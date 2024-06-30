use super::Sequencer;
use feed_rs::model::Entry;
use super::io::FeedGet;
use super::io::fs::Fs;

#[test]
fn test_entry_guard() {
    let mut r: bool;
    let mut seq = Sequencer::new();
    let mut src = Entry::default();
    src.id = String::from("foo");
    r = seq.add(src);
    assert!(r);

    let mut src_two = Entry::default();
    src_two.id = String::from("foo");
    r = seq.add(src_two);
    assert!(!r);
}

#[test]
fn test_feed_get() {
    let r: bool;
    let fs = Fs{};
    let feed = fs.get("testdata/test.atom.xml", None).unwrap();
    let mut seq = Sequencer::new();
    r = seq.add(feed.entries.get(0).unwrap().clone()); 
    assert!(r);
}
