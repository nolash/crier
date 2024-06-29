use std::collections::HashMap;
use std::iter::Iterator;
use feed_rs::model::Entry;

pub struct Sequencer {
    pub items: HashMap<u32, Entry>,
    item_keys: Vec<u32>,
    crsr: usize,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            items: HashMap::new(),
            crsr: 0,
            item_keys: Vec::<u32>::new(),
        }
    }
}

impl Iterator for Sequencer {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let c: u32;

        c = self.item_keys[self.crsr];
        return Some(self.items[&c].clone());
    }
}
