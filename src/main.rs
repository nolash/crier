use std::path::absolute;
use std::path::PathBuf;
use std::process;
use std::io::stdout;
use std::str::from_utf8;

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
}

impl Config {
    fn new(urls: Vec<String>) -> Config {
        Config {
            urls: urls,
        }
    }
}

fn parse() -> Config {
    let m = App::new("crier")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .arg(Arg::with_name("URLS")
        .multiple(true)
        .help("list of uris to merge"))
    .get_matches();

    Config::new(m.values_of("URLS").unwrap().map(|v| String::from(v)).collect())
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
    let mut seq = Sequencer::new().with_cache(&mut cache);

    seq.set_title("my new feed");
    seq.set_author("Foo Bar");


    env_logger::init();

    debug!("config has {} uris", cfg.urls.len());

    for v in cfg.urls {
        process_entry(&mut seq, v).unwrap_or_else(|e| process::exit(1));
    }

    seq.write_to(stdout()).unwrap();
}
