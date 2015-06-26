extern crate rand;

use std::io::{self, Read};
use std::fs::File;
use rand::Rng;

const DEFAULT_DICT_FILE: &'static str = "/usr/share/dict/words";

struct Config {
    min: usize,
    max: usize,
    number: usize,
    count: usize,
    filename: String,
}

impl Config {
    fn new() -> Config {
        Config {
            min: 4,
            max: 6,
            number: 4,
            count: 1,
            filename: DEFAULT_DICT_FILE.to_string(),
        }
    }
}

fn read_words(filename: &String) -> io::Result<String> {
    let mut dict = File::open(filename).unwrap();
    let mut buf = String::new();
    try!(dict.read_to_string(&mut buf));
    Ok(buf)
}

fn shuffled_words<'a>(source: &'a String, cfg: &Config) -> Vec<&'a str> {
    // Not ideal, but whatever.

    let mut lines: Vec<_> = source
        .lines()
        .filter(|x| x.len() >= cfg.min && x.len() <= cfg.max)
        .collect();
    rand::thread_rng().shuffle(&mut lines);
    lines
}

fn main() {
    let cfg = Config::new();
    let words = read_words(&cfg.filename).unwrap();

    for _ in (0..cfg.count) {
        let shuffled = shuffled_words(&words, &cfg);

        for i in (0..(cfg.number-1)) {
            print!("{} ", shuffled[i]);
        }
        println!("{}", shuffled[cfg.number]);
    }
}
