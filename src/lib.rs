use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;
use std::iter::Iterator;
use feed_rs::model::Entry;
use rs_sha512::Sha512Hasher;

pub struct Sequencer {
    pub items: HashMap<u32, Vec<u8>>,
    item_keys: Vec<u32>,
    crsr: usize,
}

pub struct SequencerEntry {
    pub digest: u32,
    entry: Entry,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            items: HashMap::new(),
            crsr: 0,
            item_keys: Vec::<u32>::new(),
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
}

impl Iterator for Sequencer {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let c: u32;

        c = self.item_keys[self.crsr];
        return Some(self.items[&c].clone());
    }
}

impl SequencerEntry {
    pub fn new(entry: Entry) -> SequencerEntry {
        let mut o = SequencerEntry {
            entry: entry,
            digest: 0,
        };
        let mut h = Sha512Hasher::default();
        o.hash(&mut h);
        o.digest = h.finish() as u32;
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
mod tests {
    use super::Sequencer;
    use feed_rs::model::Entry;

    #[test]
    fn test_entry_guard() {
        let mut r: bool;
        let mut seq = Sequencer::new();
        let mut src = Entry::default();
        src.id = String::from("foo");
        r = seq.add(src);
        assert!(r);


        let mut src_two = Entry::default();
        src_two.id = String::from("foo");
        r = seq.add(src_two);
        assert!(!r);
    }
}
