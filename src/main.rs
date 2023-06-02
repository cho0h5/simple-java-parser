mod token_reader;
mod parser;

use std::process;
use std::env;
use std::fs;

use token_reader::UnknownTokenError;
use parser::ParsingError;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("\x1b[31merror\x1b[37m: no input file");
            process::exit(1);
        },
    };
    println!("File name: {}\n", filename);

    let raw_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            println!("\x1b[31merror\x1b[37m: something went wrong during reading file");
            process::exit(1);
        },
    };

    let tokens = match token_reader::read_tokens(&raw_contents) {
        Ok(tokens) => tokens,
        Err(UnknownTokenError(token)) => {
            println!("\x1b[31merror\x1b[37m: unknown token: {}", token);
            process::exit(1);
        },
    };
    println!("Read tokens:\n{}\n", tokens);

    let parse_tree = match parser::parse(tokens) {
        Ok(tree) => tree,
        Err(ParsingError(expected, found)) => {
            println!("\x1b[31merror\x1b[37m: parsing error");
            println!("\texpected: {:?}", expected);
            println!("\tbut found: {:?}", found);
            process::exit(1);
        },
    };
    println!("Parse tree:\n{}", parse_tree);
}

