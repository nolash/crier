use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;
use std::iter::Iterator;
use std::io::Write;
use std::fmt::Debug;
use std::io::BufWriter;
use std::str::FromStr;

use rs_sha512::Sha512Hasher;
use chrono::Local;
use atom_syndication::Feed as Feed;
use atom_syndication::Entry as Entry;
use atom_syndication::TextType as OutTextType;
use atom_syndication::Text as OutText;
use atom_syndication::Content as OutContent;
use atom_syndication::Person as OutPerson;
use atom_syndication::Category as OutCategory;
use atom_syndication::FixedDateTime;
use atom_syndication::Person;
use itertools::Itertools;

pub mod io;
pub mod mem;

mod meta;
mod cache;
mod rss;
mod log;
use meta::FeedMetadata;
use mem::CacheWriter;
use cache::Cache;

#[derive(Debug)]
pub enum Error {
    WriteError,
    CacheError,
    ParseError,
    IncompleteError,
}

pub struct Sequencer<'a> {
    metadata: FeedMetadata,
    pub items: HashMap<u64, Vec<u8>>,
    item_keys: Vec<u64>,
    crsr: usize,
    limit: usize,
    default_cache: CacheWriter, //HashMap<String, Vec<u8>>,
    cache: Option<&'a mut dyn Cache>,
}

pub struct SequencerEntry {
    pub digest: u64,
    entry: Entry,
    out: Vec<u8>,
}

impl<'a> Sequencer<'a> {
    pub fn new() -> Sequencer<'a> {
        let mut o = Sequencer {
            metadata: FeedMetadata::default(),
            items: HashMap::new(),
            crsr: 0,
            limit: 0,
            item_keys: Vec::new(),
            default_cache: CacheWriter::new(), //HashMap::new(),
            cache: None,
        };

        #[cfg(test)]
        o.metadata.force();

        o
    }

    pub fn with_cache(mut self, w: &'a mut impl Cache) -> Sequencer<'a> {
        self.cache = Some(w);
        return self;
    }

    pub fn set_author(&mut self, name: &str) -> bool {
        self.metadata.set_author(Person{
            name: String::from(name),
            email: None,
            uri: None,
        })
    }

    pub fn set_title(&mut self, title: &str) -> bool {
        self.metadata.set_title(String::from(title))
    }

    pub fn add(&mut self, entry: Entry) -> bool {
        let w: &mut dyn Write;
        let mut id: String;

        id = entry.id.to_string();
        match &mut self.cache {
            Some(v) => {
                w = v.open(id);
            },
            None => {
                w = &mut self.default_cache;
            },
        }

        id = entry.id.to_string();
        let o = SequencerEntry::new(entry, w);
        if self.items.contains_key(&o.digest) {
            return false;
        }
        self.items.insert(o.digest, o.into());
        match &mut self.cache {
            Some(v) => {
                v.close(id);
            },
            None => {
            },
        }
        return true;
    }

    pub fn add_from(&mut self, feed: Feed) -> i64 {
        let mut c: i64;

        c = 0;
        for v in feed.entries.iter() {
            self.add(v.clone());
            c += 1;
        }
        c
    }

    pub fn write_to(&mut self, w: impl Write) -> Result<usize, Error> {
        let mut r: usize;
        let mut feed = Feed::default();
        let mut entry: Entry;
        let mut entries: Vec<Entry>;
        let mut b: &str;
        feed.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
        feed.set_updated(Local::now().to_utc());

        match self.metadata.apply(&mut feed) {
            Err(_v) => {
                return Err(Error::WriteError);
            },
            Ok(_) => {
            },
        }

        entries = Vec::new();
        r = 0;
        for v in self {
            b = std::str::from_utf8(v.as_slice()).unwrap();
            match Entry::from_str(b) {
                Err(e) => {
                    println!("fromstrerr {:?}", e);
                    return Err(Error::CacheError);
                },
                Ok(o) => {
                    entries.push(o);
                },
            }
            r += 1;
        }
        feed.set_entries(entries);

        match feed.write_to(w) {
            Err(_v) => {
                return Err(Error::WriteError);
            },
            Ok(_) => {
            },
        }

        Ok(r)
    }
}

impl<'a> Iterator for Sequencer<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let c: u64;

        if self.limit == 0 {
            self.item_keys = Vec::new();
            for k in  self.items.keys().sorted() {
                self.item_keys.push(k.clone());
                self.limit += 1;
            }
        }

        if self.limit == 0 {
            return None;
        }

        if self.crsr == self.limit {
            self.limit = 0;
            self.crsr = 0;
            return None;
        }

        c = self.item_keys[self.crsr];
        self.crsr += 1;
        return Some(self.items[&c].clone());
    }
}

impl SequencerEntry {
    pub fn new(entry: Entry, exporter: &mut dyn Write) -> SequencerEntry {
        let mut have_date: bool;
        let mut id_part: u32;
        let mut o = SequencerEntry {
            entry: entry,
            digest: 0,
            out: Vec::new(),
        };

        have_date = false;
        match &o.entry.published {
            Some(v) => {
                id_part = v.timestamp() as u32;
                o.digest = id_part as u64;
                o.digest <<= 32;
                have_date = true;
            },
            None => {
            },
        }

        if !have_date {
            id_part = o.entry.updated.timestamp() as u32;
            o.digest = id_part as u64;
            o.digest <<= 32;
            have_date = true;
        }
        
        let mut h = Sha512Hasher::default();
        o.hash(&mut h);
        id_part = h.finish() as u32;
        o.digest += id_part as u64;
        o
    }

    /// TODO: get size heuristics from already written values (either that or replace underlying
    /// in-memory writer implementation with something that doesnt wrap on flush.
    fn to_writer(&self, v: Vec<u8>) -> BufWriter<Vec<u8>> {
        BufWriter::with_capacity(10241024, v)
    }

}

/// TODO: split out field translations to separate module
impl Into<Vec<u8>> for SequencerEntry {
    fn into(self) -> Vec<u8> {
        let mut out_entry: Entry;
        let mut b: Vec<u8>;
        let mut w: BufWriter<Vec<u8>>;
        let o: &SequencerEntry;

        o = &self;
        b = Vec::new();
        w = o.to_writer(b);

        w = self.entry.write_to(w).unwrap();
        b = Vec::from(w.buffer());
        b
    }
}

impl Hash for SequencerEntry {
    fn hash<H: Hasher>(&self, h: &mut H) {
            h.write(self.entry.id.as_bytes());
    }
}

#[cfg(test)]
mod tests;
