extern crate rand;

use std::io::Read;
use std::fs::File;
use rand::Rng;

const DICT_FILE: &'static str = "/usr/share/dict/words";

fn main() {
    let mut dict = File::open(DICT_FILE).unwrap();
    // Not ideal, but whatever.
    let mut buf = String::new();
    dict.read_to_string(&mut buf);

    let mut lines: Vec<_> = buf
        .lines()
        .filter(|x| x.len() > 3 && x.len() < 7)
        .collect();
    rand::thread_rng().shuffle(&mut lines);
    for i in (0..4) {
        print!("{} ", lines[i]);
    }
}
