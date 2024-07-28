//use http::Uri;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;
//use core::str::FromStr;
//use std::io::stderr;
use std::io::Write;

use atom_syndication::Feed;

use super::FeedMethod;
use super::FeedGet;
use crate::cache::Cache;
use crate::rss::from_file as rss_from_file;


pub struct FsFeed {
}

pub struct FsCache {
    dir: PathBuf,
    files: HashMap<String, File>,
}

impl FeedGet for FsFeed {
    fn get(&self, s: &str, _method: Option<FeedMethod>) -> Result<Feed, u64> {
        let feed: Feed;
        //let uri = Uri::from_str(s).unwrap(); 
        match rss_from_file(s, false) {
            Ok(v) => {
                feed = v;
            },
            Err(e) => {
                return Err(0);
            },
        };
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
        let mut s: String;
        let mut ids: String;
        let mut f: File;

        if !self.files.contains_key(&id) {
            p = Path::new(self.dir.as_path());
       
            ids = id.clone();
            ids = ids.replace("/", "%2F");
            ids = ids.replace("\\", "%5C");

            fp = p.join(ids);
            s = String::from(fp.to_str().unwrap());
            f = File::create(s.as_str()).unwrap();
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
