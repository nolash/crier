use std::clone::Clone;
use std::fs::File;

use feed_rs::model::Entry;
use chrono::DateTime;
use tempfile::NamedTempFile;
use tempfile::TempDir;

use crate::Sequencer;
use crate::io::FeedGet;
use crate::io::fs::FsCache;

#[cfg(feature = "fs")]
use crate::io::fs::FsFeed;


#[test]
fn test_entry_guard() {
    let mut r: bool;
    let mut seq = Sequencer::new();
    let mut src = Entry::default();
    src.id = String::from("foo");
    //src.published = Some(DateTime::<Utc>::default());
    src.published = Some(DateTime::parse_from_rfc3339("2024-06-25T20:46:00+02:00").unwrap().into());
    r = seq.add(src);
    assert!(r);

    let mut src_two = Entry::default();
    src_two.id = String::from("foo");
    src_two.published = Some(DateTime::parse_from_rfc3339("2024-06-25T20:46:00+02:00").unwrap().into());
    r = seq.add(src_two);
    assert!(!r);

    let mut src_three = Entry::default();
    src_three.id = String::from("foo");
    src_three.published = Some(DateTime::parse_from_rfc3339("2024-06-25T20:46:00+03:00").unwrap().into());
    r = seq.add(src_three);
    assert!(r);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_get() {
    let r: bool;
    let fs = FsFeed{};
    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    r = seq.add(feed.entries.get(0).unwrap().clone()); 
    assert!(r);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_all() {
    let r: i64;
    let fs = FsFeed{};
    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    r = seq.add_from(feed); 
    assert_eq!(r, 15);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_mix() {
    let mut r: i64;
    let fs = FsFeed{};
    let mut feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    r = seq.add_from(feed); 
    assert_eq!(r, 15);
    feed = fs.get("testdata/test2.xml", None).unwrap();
    r = seq.add_from(feed); 
    assert_eq!(r, 10);
    assert_eq!(seq.by_ref().count(), 25);
    assert_eq!(seq.count(), 25);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_write() {
    let r: usize;
    let fs = FsFeed{};
    let f: NamedTempFile;
    let fr: File;

    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    seq.add_from(feed); 
    f = NamedTempFile::new().unwrap();
    fr = f.reopen().unwrap();
    r = seq.write_to(f).unwrap();
    assert_eq!(r, 15);
    assert_eq!(fr.metadata().unwrap().len(), 254);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_write_extcache() {
    let r: usize;
    let fs = FsFeed{};
    let d: TempDir;
    let f: NamedTempFile;
    let fr: File;
    let mut cache: FsCache;

    d = TempDir::new().unwrap();
    cache = FsCache::new(d.into_path());
        
    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    seq = seq.with_cache(&mut cache);

    seq.add_from(feed); 
    f = NamedTempFile::new().unwrap();
    fr = f.reopen().unwrap();
    r = seq.write_to(f).unwrap();
    assert_eq!(r, 15);
    assert_eq!(fr.metadata().unwrap().len(), 254);
}
