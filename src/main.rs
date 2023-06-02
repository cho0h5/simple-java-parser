mod token_reader;
mod parser;
mod utils;

use std::process;

use token_reader::UnknownTokenError;

fn main() {
    let filename = utils::get_filename();
    println!("File name: {}\n", filename);

    let raw_contents = utils::read_file(&filename);

    let tokens = match token_reader::read_tokens(&raw_contents) {
        Ok(tokens) => tokens,
        Err(UnknownTokenError(token)) => {
            println!("\x1b[31merror\x1b[37m: unknown token: {}", token);
            process::exit(1);
        },
    };
    println!("Read tokens:\n{}\n", tokens);

    let parse_tree = parser::parse(tokens);
    println!("Parse tree:\n{}", parse_tree);
}

