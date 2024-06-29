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
    entry: Entry,
    digest: u64,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            items: HashMap::new(),
            crsr: 0,
            item_keys: Vec::<u32>::new(),
        }
    }

    pub fn add<H: Hash>(&self, entry: SequencerEntry) {
//        entry.hash(&self, entry);
    }
}

impl SequencerEntry {
    pub fn new(entry: Entry, summer: &mut Sha512Hasher) -> SequencerEntry {
        let mut o = SequencerEntry {
            entry: entry,
            digest: 0,
        };
        o.hash(summer);
        o.digest  = summer.finish();
        o
    }
}

impl Hash for SequencerEntry {
    fn hash<H: Hasher>(&self, h: &mut H) {
            h.write(self.entry.id.as_bytes());
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

#[cfg(test)]
mod tests {
    use rs_sha512::Sha512Hasher;
    use super::SequencerEntry;
    use feed_rs::model::Entry;

    #[test]
    fn test_entry() {
        let mut h = Sha512Hasher::default();
        let src = Entry::default();
        let entry = SequencerEntry::new(src, &mut h);
    }
}
