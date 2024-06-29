use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;
use std::iter::Iterator;
use feed_rs::model::Entry;

pub struct Sequencer {
    pub items: HashMap<u32, Vec<u8>>,
    item_keys: Vec<u32>,
    crsr: usize,
}

pub struct SequencerEntry<'a> {
    entry: Entry,
    summer: &'a dyn Hasher,
    digest: Vec<u8>,
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

impl<'a> SequencerEntry<'a> {
    pub fn new(entry: Entry, summer: &'a dyn Hasher) -> SequencerEntry<'a> {
        SequencerEntry {
            entry: entry,
            summer: summer,
            digest: Vec::new(),
        }
    }
}

//impl<'a> Hash for SequencerEntry<'a> {
//    fn hash(&self) -> u64 {
//        Vec::<u64>::new()
//    }
//}
//

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
        let h = Sha512Hasher::default();
        let src = Entry::default();
        let entry = SequencerEntry::new(src, &h);
    }
}
