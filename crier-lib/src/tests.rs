use std::clone::Clone;
use std::fs::File;
use std::io::{SeekFrom, Seek, Read};
use std::str;

//use feed_rs::model::Entry;
//use feed_rs::model::Text;
use mediatype::MediaTypeBuf;
use chrono::DateTime;
use tempfile::NamedTempFile;
use tempfile::tempdir;
use atom_syndication::Entry as OutEntry;
use atom_syndication::Feed as OutFeed;
use atom_syndication::Person;
use atom_syndication::Text;
use quick_xml::Reader as XMLReader;
use quick_xml::events::Event as XMLEvent;

use crate::Sequencer;
use crate::io::FeedGet;
use crate::meta::FeedMetadata;
use crate::Feed;
use crate::Entry;
use crate::io::fs::FsCache;
use crate::mem::MemCache;

#[cfg(feature = "fs")]
use crate::io::fs::FsFeed;


fn check_xml_title(xml: Vec<u8>, title: &str) {
    let mut rxml = XMLReader::from_str(str::from_utf8(&xml).unwrap());
    let mut xmlbuf = Vec::new();
    let mut state = 0;
    loop {
        match rxml.read_event_into(&mut xmlbuf) {
            Err(e) => panic!("cant read back xml: {:?}", e),
            Ok(XMLEvent::Eof) => break,
            Ok(XMLEvent::Start(v)) => {
                match v.name().as_ref() {
                    b"title" => {
                        state = 1;
                    },
                    _ => {
                        state = 0
                    },
                }
            },
            Ok(XMLEvent::End(v)) => {
                state = 0;
            },
            Ok(XMLEvent::Text(v)) => {
                if state > 0 {
                    assert_eq!(v.unescape().unwrap(), title);
                }
            },
            _ => (),
        }
    }
}

#[test]
fn test_entry_guard() {
    let mut r: bool;
    let mut seq = Sequencer::new();
    let mut src = Entry::default();
    let mut s: String;

    src.id = String::from("foo");
    s = String::from("inky");
    src.title = Text::plain(s);
        
    src.published = Some(DateTime::parse_from_rfc3339("2024-06-25T20:46:00+02:00").unwrap().into());
    r = seq.add(src);
    assert!(r);

    let mut src_two = Entry::default();
    src_two.id = String::from("foo");
    s = String::from("pinky");
    src_two.title = Text::plain(s);
    src_two.published = Some(DateTime::parse_from_rfc3339("2024-06-25T20:46:00+02:00").unwrap().into());
    r = seq.add(src_two);
    assert!(!r);

    let mut src_three = Entry::default();
    src_three.id = String::from("foo");
    s = String::from("blinky");
    src_three.title = Text::plain(s);

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
    //r = seq.add(feed.entries.get(0).unwrap().clone()); 
    //assert!(r);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_all() {
    let r: i64;
    let fs = FsFeed{};
    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    r = seq.add_from(feed); 
    assert_eq!(r, 16);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_mix() {
    let mut r: i64;
    let fs = FsFeed{};
    let mut feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    r = seq.add_from(feed); 
    assert_eq!(r, 16);
    feed = fs.get("testdata/test2.xml", None).unwrap();
    r = seq.add_from(feed); 
    assert_eq!(r, 10);
    assert_eq!(seq.by_ref().count(), 26);
    assert_eq!(seq.count(), 26);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_write() {
    let r: usize;
    let fs = FsFeed{};
    let f: NamedTempFile;
    let mut fr: File;

    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    seq.add_from(feed); 
    f = NamedTempFile::new().unwrap();
    fr = f.reopen().unwrap();
    r = seq.write_to(f).unwrap();
    assert_eq!(r, 16);
    assert_eq!(fr.metadata().unwrap().len(), 520327);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_write_extcache() {
    let r: usize;
    let fs = FsFeed{};
    let f: NamedTempFile;
    let fr: File;
    let mut cache: FsCache;

    let d = tempdir().unwrap();
    cache = FsCache::new(d.into_path());
        
    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    seq = seq.with_cache(&mut cache);

    seq.add_from(feed);
    f = NamedTempFile::new().unwrap();
    fr = f.reopen().unwrap();
    r = seq.write_to(f).unwrap();

    assert_eq!(r, 16);
    assert_eq!(fr.metadata().unwrap().len(), 520327);
}

#[test]
#[cfg(feature = "fs")]
fn test_sequence_order() {
    let mut seq = Sequencer::new();
    let mut entry: Entry;
    let mut s: String;
    let mut r: Vec<u8>;

    entry = Entry::default();
    entry.id = String::from("y");
    s = String::from("inky");
    entry.title = Text::plain(s);
    entry.published = Some(DateTime::parse_from_rfc3339("2024-06-25T20:46:00+02:00").unwrap().into());
    seq.add(entry);


    entry = Entry::default();
    entry.id = String::from("b");
    s = String::from("pinky");
    entry.title = Text::plain(s);
    entry.published = Some(DateTime::parse_from_rfc3339("2023-06-25T20:46:00+02:00").unwrap().into());
    seq.add(entry);

    entry = Entry::default();
    entry.id = String::from("d");
    s = String::from("blinky");
    entry.title = Text::plain(s);
    entry.published = Some(DateTime::parse_from_rfc3339("2024-06-25T20:46:00+02:00").unwrap().into());
    seq.add(entry);

    entry = Entry::default();
    entry.id = String::from("a");
    s = String::from("clyde");
    entry.title = Text::plain(s);
    entry.published = Some(DateTime::parse_from_rfc3339("2024-06-25T20:46:00+02:00").unwrap().into());
    seq.add(entry);


    // TODO find value where sort digest is reverse of lexical id
    r = seq.next().unwrap();
    check_xml_title(r, "pinky");
    r = seq.next().unwrap();
    check_xml_title(r, "blinky");
    r = seq.next().unwrap();
    check_xml_title(r, "inky");
    r = seq.next().unwrap();
    check_xml_title(r, "clyde");
}

#[test]
fn test_meta() {
    let mut o = FeedMetadata::default();
    let mut feed = OutFeed::default();

    match o.apply(&mut feed) {
        Ok(r) => {
            panic!("metadata should not be ready");
        },
        Err(e) => {},
    };

    o.set_title(String::from("foo"));
    match o.apply(&mut feed) {
        Ok(r) => {
            panic!("metadata should not be ready");
        },
        Err(e) => {},
    };

    o.set_author(Person{
                name: String::from("Foo Bar"),
                email: Some("foo@bar.com".to_string()),
                uri: Some("foo.bar.com".to_string()),
            }
    );
    o.apply(&mut feed).unwrap();
}

#[test]
fn test_rss() {
let fs = FsFeed{};
    let mut cache = MemCache::new();
    let fs = FsFeed{};

    let feed = fs.get("testdata/test.rss.xml", None).unwrap();
    let mut seq = Sequencer::new();
    seq = seq.with_cache(&mut cache);
    
    seq.add_from(feed);
}
