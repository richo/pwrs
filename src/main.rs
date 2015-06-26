extern crate rand;
extern crate getopts;

use std::io::{self, Read};
use std::fs::File;
use rand::Rng;
use getopts::Options;

const DEFAULT_DICT_FILE: &'static str = "/usr/share/dict/words";

struct Config {
    min: usize,
    max: usize,
    number: usize,
    count: usize,
    filename: String,
    downcase: bool,
}

impl Config {
    fn new() -> Config {
        Config {
            min: 4,
            max: 6,
            number: 4,
            count: 1,
            filename: DEFAULT_DICT_FILE.to_string(),
            downcase: true,
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

fn usage(opts: &Options, err: Option<String>) {
    if let Some(e) = err {
        println!("{}\n", e);
    }
    let brief = "Usage: pwrs [options]";
    println!("{}", opts.usage(&brief));
}


fn config() -> Option<Config> {
    let mut cfg = Config::new();
    let args: Vec<_> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help");
    opts.optopt("m", "min", "Minimum number of characters in words", "MIN");
    opts.optopt("M", "max", "Maximum number of characters in words", "MAX");
    opts.optopt("c", "count", "Count of passphrases to print", "COUNT");
    opts.optopt("n", "number", "Number of words to include in passphrases", "NUMBER");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            usage(&opts, Some(f.to_string()));
            // This is only in nightlies and for once I'm writing a thing that works on stable rust
            // ::std::env::set_exit_status(1);
            return None;
        }
    };

    if matches.opt_present("help") {
        usage(&opts, None);
        return None;
    }

    macro_rules! unpack_opt(
        ($name:ident) => {
            if let Some(s) = matches.opt_str(stringify!($name)) {
                match s.parse() {
                    Ok(i) => cfg.$name = i,
                    Err(e) => {
                        usage(&opts, Some(e.to_string()));
                        return None;
                    }
                }
            }
        }
    );

    unpack_opt!(min);
    unpack_opt!(max);
    unpack_opt!(count);
    unpack_opt!(number);

    Some(cfg)
}

fn main() {
    let cfg = match config() {
        Some(c) => c,
        None => return,
    };
    let words = read_words(&cfg.filename).unwrap();

    for _ in (0..cfg.count) {
        let shuffled = shuffled_words(&words, &cfg);

        let out = shuffled[0..cfg.number].connect(" ");
        if cfg.downcase {
            println!("{}",  out.to_lowercase());
        } else {
            println!("{}", out);
        }
    }
}
