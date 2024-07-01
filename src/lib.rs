use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;
use std::iter::Iterator;
use std::io::Write;
use std::error::Error as StdError;

use feed_rs::model::Entry;
use feed_rs::model::Feed;
use rs_sha512::Sha512Hasher;
use chrono::DateTime;
use chrono::Local;
use atom_syndication::Feed as OutFeed;

mod meta;
mod io;
use meta::FeedMetadata;

#[derive(Debug)]
pub enum Error {
    WriteError,
}


pub struct Sequencer {
    metadata: FeedMetadata,
    pub items: HashMap<u64, Vec<u8>>,
    item_keys: Vec<u64>,
    crsr: usize,
    limit: usize,
}

pub struct SequencerEntry {
    pub digest: u64,
    entry: Entry,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        let mut o = Sequencer {
            metadata: FeedMetadata::default(),
            items: HashMap::new(),
            crsr: 0,
            limit: 0,
            item_keys: Vec::new(),
        };

        #[cfg(test)]
        o.metadata.force();

        o
    }

    pub fn add(&mut self, entry: Entry) -> bool {
        let o = SequencerEntry::new(entry);
        if self.items.contains_key(&o.digest) {
            return false;
        }
        self.items.insert(o.digest, o.into());
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

    fn write_to(&mut self, w: impl Write) -> Result<usize, Error> {
        let mut r: usize;
        let mut feed = OutFeed::default();
        feed.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
        feed.set_updated(Local::now().to_utc());

        match self.metadata.apply(&mut feed) {
            Err(v) => {
                return Err(Error::WriteError);
            },
            Ok(_) => {
            },

        }

        match feed.write_to(w) {
            Err(v) => {
                return Err(Error::WriteError);
            },
            Ok(_) => {
            },
        }

        r = 0;
        for v in self {
            r += 1;
        }

        Ok(r)
    }
}

impl Iterator for Sequencer {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let c: u64;

        if self.limit == 0 {
            self.item_keys = Vec::new();
            for k in  self.items.keys() {
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
    pub fn new(entry: Entry) -> SequencerEntry {
        let mut have_date: bool;
        let mut id_part: u32;
        let mut o = SequencerEntry {
            entry: entry,
            digest: 0,
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
            match &o.entry.updated {
                Some(v) => {
                    id_part = v.timestamp() as u32;
                    o.digest = id_part as u64;
                    o.digest <<= 32;
                    have_date = true;
                },
                None => {
                },
            }
        }
        
        let mut h = Sha512Hasher::default();
        o.hash(&mut h);
        id_part = h.finish() as u32;
        o.digest += id_part as u64;
        o
    }
}

impl Into<Vec<u8>> for SequencerEntry {
    fn into(self) -> Vec<u8> {
        return String::from(self.entry.id).into_bytes();
    }
}

impl Hash for SequencerEntry {
    fn hash<H: Hasher>(&self, h: &mut H) {
            h.write(self.entry.id.as_bytes());
    }
}

#[cfg(test)]
mod tests;
