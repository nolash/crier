use super::FeedMethod;
use super::FeedGet;
use feed_rs::model::Feed;
use feed_rs::parser;
//use http::Uri;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;
//use core::str::FromStr;
//use std::io::stderr;
use std::io::Write;

use crate::cache::Cache;


pub struct FsFeed {
}

pub struct FsCache {
    dir: PathBuf,
    files: HashMap<String, File>,
}

impl FeedGet for FsFeed {
    fn get(&self, s: &str, _method: Option<FeedMethod>) -> Result<Feed, u64> {
        //let uri = Uri::from_str(s).unwrap(); 
        let f = File::open(s).unwrap();
        let feed = parser::parse(f).unwrap();
        Ok(feed)
    }
}

impl FsCache {
    pub fn new(path: PathBuf) -> FsCache {
        FsCache{
            dir: path,
            files: HashMap::new(),
        }
    }
}

impl Cache for FsCache {
    fn open(&mut self, id: String) -> &mut dyn Write {
        let p: &Path;
        let fp: PathBuf;
        let s: &str;
        let mut f: File;

        if !self.files.contains_key(&id) {
            p = Path::new(self.dir.as_path());
            fp = p.join(id.clone());
            s = fp.to_str().unwrap();
            f = File::create(s).unwrap();
            self.files.insert(id.clone(), f);
        }
        return self.files.get_mut(&id).unwrap();
    }

    fn close(&mut self, id: String) -> usize {
        if self.files.contains_key(&id) {
            self.files.remove(&id);
        }
        0
    }
}
