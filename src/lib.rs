use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;
use std::iter::Iterator;
use feed_rs::model::Entry;
use feed_rs::model::Feed;
use rs_sha512::Sha512Hasher;
use chrono::DateTime;
mod io;

pub struct Sequencer {
    pub items: HashMap<u64, Vec<u8>>,
    item_keys: Vec<u64>,
    crsr: usize,
}

pub struct SequencerEntry {
    pub digest: u64,
    entry: Entry,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            items: HashMap::new(),
            crsr: 0,
            item_keys: Vec::<u64>::new(),
        }
    }

    pub fn add(&mut self, entry: Entry) -> bool {
        let o = SequencerEntry::new(entry);
        if self.items.contains_key(&o.digest) {
            return false;
        }
        self.items.insert(o.digest, o.into());
        return true;
    }

    pub fn add_all(&mut self, feed: Feed) -> i64 {
        let mut c: i64;

        c = 0;
        for v in feed.entries.iter() {
            self.add(v.clone());
            c += 1;
        }
        c
    }
}

impl Iterator for Sequencer {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let c: u64;

        c = self.item_keys[self.crsr];
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
