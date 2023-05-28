use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn get_filename() -> String {
    match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("error: no input file"),
    }
}

pub fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");
    contents
}
