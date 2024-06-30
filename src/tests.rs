use super::Sequencer;
use feed_rs::model::Entry;
use super::io::FeedGet;
use chrono::DateTime;
use chrono::offset::Utc;

#[cfg(feature = "fs")]
use super::io::fs::Fs;

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
    let fs = Fs{};
    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    r = seq.add(feed.entries.get(0).unwrap().clone()); 
    assert!(r);
}

#[test]
#[cfg(feature = "fs")]
fn test_feed_all() {
    let r: i64;
    let fs = Fs{};
    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    r = seq.add_all(feed); 
    assert_eq!(r, 15);
}
