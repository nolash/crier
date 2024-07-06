use super::FeedMethod;
use super::FeedGet;
use feed_rs::model::Feed;
use feed_rs::parser;
//use http::Uri;
//use std::path::Path;
use std::fs::File;
//use core::str::FromStr;
//use std::io::stderr;

use crate::cache::Cache;


pub struct FsFeed {
}

impl FeedGet for FsFeed {
    fn get(&self, s: &str, _method: Option<FeedMethod>) -> Result<Feed, u64> {
        //let uri = Uri::from_str(s).unwrap(); 
        let f = File::open(s).unwrap();
        let feed = parser::parse(f).unwrap();
        Ok(feed)
    }
}

//impl Cache for FsCache {

//}
