use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::Error;

use log::info;
use log::debug;

use rss::Channel;
use rss::Item;
use rss::extension::dublincore::DublinCoreExtension;
use atom_syndication::Feed;
use atom_syndication::Entry;
use atom_syndication::Text;
use atom_syndication::TextType;
use atom_syndication::FixedDateTime;
use atom_syndication::Content;
use atom_syndication::Category;
use chrono::naive::NaiveDateTime;
use chrono::Local;
use chrono::offset::Utc;

/// try to coerce the item field into a valid date
fn parse_date(v: &String) -> Result<FixedDateTime, Error> {
    match FixedDateTime::parse_from_rfc2822(v.as_str()) {
        Ok(r) => {
            return Ok(r);
        },
        Err(e) => {},
    };
    match FixedDateTime::parse_from_rfc3339(v.as_str()) {
        Ok(r) => {
            return Ok(r);
        },
        Err(e) => {},
    };
    match FixedDateTime::parse_from_str(v.as_str(), "%Y-%m-%dT%H:%M:%S") {
        Ok(r) => {
            return Ok(r);
        },
        Err(e) => {
        },
    };
    match NaiveDateTime::parse_from_str(v.as_str(), "%Y-%m-%dT%H:%M:%S") {
        Ok(r) => {
            return Ok(r.and_utc().fixed_offset());
        },
        Err(e) => {
        },
    };


    Err(Error::ParseError)
}

/// try different item fields to determine the date
fn get_base_date(ipt: &Item) -> Result<FixedDateTime, Error> {
    let mut ds = String::new();

    match &ipt.pub_date {
        Some(v) => {
            ds.push_str(v.as_str());
        },
        _ => {},
    };
    match parse_date(&ds) {
        Ok(v) => {
            return Ok(v);
        },
        Err(e) => {},
    };

    match &ipt.dublin_core_ext {
        Some(v) => {
            for vv in v.dates() {
                match parse_date(vv) {
                    Ok(vvv) => {
                        return Ok(vvv);
                    },
                    Err(e) => {
                        debug!("no date");
                    },
                }
            }
        },
        _ => {},
    }

    Err(Error::IncompleteError)
}

/// coerce the rss item into an atom entry
fn translate_item(ipt: Item) -> Result<Entry, Error> {
    let mut opt = Entry::default();

    match &ipt.title {
        Some(v) => {
            opt.set_title(Text::plain(v));
        },
        _ => {},
    };

    match get_base_date(&ipt) {
        Ok(v) => {
            opt.set_published(v.clone());
            opt.set_updated(v);
        },
        Err(e) => {
            return Err(e);
        }
    };
   
    match ipt.description {
        Some(v) => {
            opt.set_summary(Some(Text::plain(v)));
        },
        _ => {},
    };

    match ipt.content {
        Some(v) => {
            let mut r = Content::default();
            r.set_content_type(Some(String::from("text/html")));
            r.set_value(Some(v));
            match ipt.source {
                Some(v) => {
                    r.set_src(v.url);
                },
                _ => {},
            }
            opt.set_content(Some(r));
        },
        _ => {},
    };

    match ipt.guid {
        Some(v) => {
            if v.is_permalink() {
                opt.set_id(String::from(v.value()));
            }
        },
        _ => {
            match ipt.link {
                Some(v) => {
                    opt.set_id(v.clone());
                },
                _ => {},
            }
        },
    };

    for v in ipt.categories {
        let mut cat = Category::default();
        cat.set_term(String::from(v.name()));
        cat.set_label(Some(v.name));
        match v.domain {
            Some(v) => {
                cat.set_scheme(Some(v));
            },
            _ => {},
        };
        opt.categories.push(cat);
    }

    Ok(opt)
}


fn translate(ipt: Channel, allow_fail: bool) -> Result<Feed, Error> {
    let mut entries: Vec<Entry>;
    let mut opt = Feed::default();
    
    opt.set_title(Text::plain(&ipt.title));

    opt.set_subtitle(Some(Text::plain(&ipt.description)));

    entries = vec!();
    for v in ipt.into_items() {
        match translate_item(v) {
            Ok(v) => {
                entries.push(v);
            },
            Err(e) => {
                if !allow_fail {
                    return Err(Error::IncompleteError);
                }
            },
        }
    }

    opt.set_entries(entries);
    opt.set_updated(Local::now().to_utc());
    Ok(opt)
}

pub fn from_file(fp: &str, allow_entry_fail: bool) -> Result<Feed, Error> {
    let mut o: Channel;
    let r: Feed;
    let p: &Path; 
    let mut f: File;
    //let mut b: BufReader; // how to explicitly declare 

    p = Path::new(fp);
    f = File::open(p).unwrap();
    let mut b = BufReader::new(f);

    match Feed::read_from(b) {
        Ok(v) => {
            return Ok(v);
        },
        Err(e) => {},
    };

    f = File::open(p).unwrap();
    b = BufReader::new(f);

    match Channel::read_from(b) {
        Ok(v) => {
            o = v;
        },
        Err(e) => {
            return Err(Error::ParseError);
        },
    };
    o.set_dublin_core_ext(DublinCoreExtension::default());
    translate(o, allow_entry_fail)
}

mod test {
    use std::path::Path;
    use atom_syndication::Feed;
    use env_logger;

    #[test]
    fn test_rss_from_file() {
        env_logger::init();
        let mut r: Feed;
        match super::from_file("testdata/test.rss.xml", false) {
            Ok(v) => {
            },
            Err(e) => {
                panic!("{:?}", e);
            },
        };
        match super::from_file("testdata/test.atom.xml", false) {
            Ok(v) => {
            },
            Err(e) => {
                panic!("expected fail");
            },
        };
    }
}
