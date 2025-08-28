use std::env;
use std::io::{self, Read};
use std::fs::File;

use rand;
use rand::seq::IndexedRandom;
use getopts::Options;

const DEFAULT_DICT_FILE: &'static str = "/usr/share/dict/words";

struct Config {
    min: usize,
    max: usize,
    number: usize,
    count: usize,
    filename: String,
    upcase: bool,
}

impl Config {
    fn new() -> Config {
        Config {
            min: 4,
            max: 6,
            number: 4,
            count: 1,
            filename: DEFAULT_DICT_FILE.to_string(),
            upcase: false,
        }
    }
}

fn read_wordlist(cfg: &Config) -> io::Result<String> {
    let mut dict = File::open(&cfg.filename)?;
    let mut buf = String::new();
    dict.read_to_string(&mut buf)?;
    Ok(buf)
}

fn split_wordlist<'a>(buf: &'a String, cfg: &Config) -> Vec<&'a str> {
    buf.lines()
       .filter(|x| x.len() >= cfg.min && x.len() <= cfg.max)
       .collect()
}

fn select_words<'a>(words: &'a Vec<&'a str>, cfg: &Config) -> Vec<&'a &'a str> {
    let mut rng = rand::rng();
    words.choose_multiple(&mut rng, cfg.number).collect()
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
    opts.optflag("u", "upcase", "Allow words to be capitolized");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            usage(&opts, Some(f.to_string()));
            std::process::exit(1);
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

    if matches.opt_present("upcase") {
        cfg.upcase = true;
    }

    Some(cfg)
}

fn main() {
    let cfg = match config() {
        Some(c) => c,
        None => return,
    };

    let wordlist = match read_wordlist(&cfg) {
        Ok(words) => words,
        Err(e) => {
            println!("Couldn't open wordlist file: {}", e.to_string());
            return;
        }
    };

    let words = split_wordlist(&wordlist, &cfg);

    for _ in 0..cfg.count {
        let selected = select_words(&words, &cfg);

        let inter: Vec<&str> = selected.iter()
                                       .map(|x| **x)
                                       .collect();
        let out = inter.join(" ");
        if cfg.upcase {
            println!("{}", out);
        } else {
            println!("{}",  out.to_lowercase());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_defaults() {
        let cfg = Config::new();
        let buf = read_wordlist(&cfg).unwrap();
        let words = split_wordlist(&buf, &cfg);
        let _ = select_words(&words, &cfg);
    }
}
