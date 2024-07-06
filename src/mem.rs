use std::collections::HashMap;
use std::io::Write;
use std::io::Error;

use crate::cache::Cache;


pub struct CacheWriter {
    data: Vec<u8>,
}

pub struct MemCache {
    files: HashMap<String, CacheWriter>,
}

impl Write for CacheWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.data.extend_from_slice(buf);
        Ok(self.data.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}


impl CacheWriter {
    pub fn new() -> CacheWriter {
        CacheWriter{
            data: Vec::new(),
        }
    }
}

impl MemCache {
    pub fn new() -> MemCache {
        MemCache{
            files: HashMap::new(),
        }
    }
}

impl Cache for MemCache {
    fn open(&mut self, id: String) -> &mut dyn Write {
        let w: CacheWriter;
        if !self.files.contains_key(&id) {
            w = CacheWriter::new();
            self.files.insert(id.clone(), w);
        }
        self.files.get_mut(&id).unwrap()
    }

    fn close(&mut self, _id: String) -> usize {
        return 0;
    }
}
