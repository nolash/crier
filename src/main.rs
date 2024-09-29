use std::path::absolute;
use std::path::PathBuf;
use std::process;
use std::io::stdout;
use std::str::from_utf8;

use uuid::Uuid;

use clap::Arg;
use clap::App;

use log::debug;
use log::info;
use env_logger;

use crier::Sequencer;
use crier::io::FeedGet;
use crier::mem::MemCache;
use crier::io::fs::FsFeed;
use crier::Error;

struct Config {
    urls: Vec<String>,
    author: String,
    title: String,
    id: String,
}

impl Config {
    fn new(id: String, title: String, author: String, urls: Vec<String>) -> Config {
        Config {
            urls: urls,
            title: title,
            author: author,
            id: id,
        }
    }
}

fn parse() -> Config {
    let mut o = App::new("crier")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"));

    o = o.arg(
        Arg::with_name("title")
            .long("title")
            .short("t")
            .value_name("Aggregated feed title")
            .takes_value(true)
            .required(true)
    );

    o = o.arg(
        Arg::with_name("author")
            .long("author")
            .short("a")
            .value_name("Aggregated feed author")
            .takes_value(true)
            .required(true)
    );

    // TODO: implement auto generate id when missing
    o = o.arg(
        Arg::with_name("id")
            .long("id")
            .short("i")
            .value_name("Aggregated feed id uuid value")
            .takes_value(true)
            .required(true)
    );

    o = o.arg(Arg::with_name("URLS")
        .multiple(true)
        .help("list of uris to merge"));

    let m = o.get_matches();

    Config::new(
        String::from(m.value_of("id").unwrap()),
        String::from(m.value_of("title").unwrap()),
        String::from(m.value_of("author").unwrap()),
        m.values_of("URLS").unwrap().map(|v| String::from(v)).collect())
}

fn add_feed(seq: &mut Sequencer, getter: impl FeedGet, uri: String) -> Result<i64, Error> {
    match getter.get(uri.as_str(), None) {
        Ok(v) => {
            let r = seq.add_from(v);
            info!("got {} results from {}", r, uri);
            return Ok(r);
        },
        Err(e) => {
            return Err(Error::ParseError);
        },
    };
}

fn process_entry(seq: &mut Sequencer, uri: String) -> Result<(), Error> {
    let v: PathBuf;
    let fp: String;
    let fs = FsFeed{};

    debug!("processing {}", uri);
    match absolute(uri) {
        Ok(r) => {
            fp = String::from(r.to_str().unwrap());
        },
        Err(e) => {
            return Err(Error::ParseError);
        }
    };

    match add_feed(seq, fs, fp) {
        Ok(r) => {
            return Ok(());
        },
        Err(e) => {
            return Err(e);
        },
    }
}

fn main() {
    let cfg = parse();
    let mut cache = MemCache::new();

    let id: Vec<u8> = cfg.id.into();
    let mut seq = Sequencer::new(id).with_cache(&mut cache);

    seq.set_title(cfg.title.as_str());
    seq.set_author(cfg.author.as_str());

    env_logger::init();

    debug!("config has {} uris", cfg.urls.len());

    for v in cfg.urls {
        process_entry(&mut seq, v).unwrap_or_else(|e| process::exit(1));
    }

    seq.write_to(stdout()).unwrap();
}
