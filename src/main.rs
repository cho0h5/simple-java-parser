mod token_reader;
mod parser;

use std::process;
use std::env;
use std::fs;

use token_reader::UnknownTokenError;
use parser::ParsingError;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => {
            print!("\x1b[32m[1/4]\x1b[37m ");
            println!("File name: {}\n", filename);
            filename
        },
        None => {
            println!("\x1b[31m[1/4] error\x1b[37m: no input file");
            process::exit(1);
        },
    };

    let raw_contents = match fs::read_to_string(filename) {
        Ok(contents) => {
            print!("\x1b[32m[2/4]\x1b[37m ");
            println!("File contents: \n{}\n", contents.trim());
            contents
        },
        Err(_) => {
            println!("\x1b[31m[2/4] error\x1b[37m: something went wrong during reading file");
            process::exit(1);
        },
    };

    let tokens = match token_reader::read_tokens(&raw_contents) {
        Ok(tokens) => {
            print!("\x1b[32m[3/4]\x1b[37m ");
            println!("Read tokens:\n{}\n", tokens);
            tokens
        },
        Err(UnknownTokenError(token)) => {
            println!("\x1b[31m[3/4] error\x1b[37m: unknown token: {}", token);
            process::exit(1);
        },
    };

    match parser::parse(tokens) {
        Ok(tree) => {
            print!("\x1b[32m[4/4]\x1b[37m ");
            println!("Parse tree:\n{}", tree);
            println!("Accepted!");
            tree
        },
        Err(ParsingError(expected, found)) => {
            println!("\x1b[31m[4/4] error\x1b[37m: parsing error");
            println!("\texpected: {:?}", expected);
            println!("\tbut found: {:?}", found);
            process::exit(1);
        },
    };
}
