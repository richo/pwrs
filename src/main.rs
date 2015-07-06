#![cfg_attr(test, feature(test))]
#![cfg_attr(test, allow(dead_code))]
#[cfg(test)]
extern crate test;

extern crate rand;
extern crate getopts;

use std::env;
use std::io::{self, Read};
use std::fs::File;
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

fn read_wordlist(cfg: &Config) -> io::Result<String> {
    let mut dict = File::open(&cfg.filename).unwrap();
    let mut buf = String::new();
    try!(dict.read_to_string(&mut buf));
    Ok(buf)
}

fn split_wordlist<'a>(buf: &'a String, cfg: &Config) -> Vec<&'a str> {
    buf.lines()
       .filter(|x| x.len() >= cfg.min && x.len() <= cfg.max)
       .collect()
}

fn select_words<'a>(words: &'a Vec<&'a str>, cfg: &Config) -> Vec<&'a &'a str> {
    let mut rng = rand::thread_rng();
    rand::sample(&mut rng, words.iter(), cfg.number)
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
            // env::set_exit_status(1);
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

    if let Ok(path) = env::var("PWRS_WORDLIST") {
        cfg.filename = path;
    }

    Some(cfg)
}

fn main() {
    let cfg = match config() {
        Some(c) => c,
        None => return,
    };
    let wordlist = read_wordlist(&cfg).unwrap();
    let words = split_wordlist(&wordlist, &cfg);

    for _ in (0..cfg.count) {
        let selected = select_words(&words, &cfg);

        let inter: Vec<&str> = selected.iter()
                                       .map(|x| **x)
                                       .collect();
        let out = inter.connect(" ");
        if cfg.downcase {
            println!("{}",  out.to_lowercase());
        } else {
            println!("{}", out);
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[bench]
    fn with_defaults(b: &mut Bencher) {
        let cfg = ::Config::new();
        let buf = ::read_wordlist(&cfg).unwrap();
        let words = ::split_wordlist(&buf, &cfg);
        b.iter(|| {
            let _ = ::select_words(&words, &cfg);
        })
    }
}
