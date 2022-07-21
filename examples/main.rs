use cst::get_entry;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn ui_text(file: &str, key: usize) {
    let path = Path::new(file);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!(
            "{}{}****************{}",
            get_entry(s, key),
            LINE_ENDING,
            LINE_ENDING
        ),
    }
}

fn main() {
    let mut entries = [1, 2, 3, 4];
    entries.shuffle(&mut thread_rng());
    for entry in entries {
        ui_text("example.cst", entry);
    }
}
