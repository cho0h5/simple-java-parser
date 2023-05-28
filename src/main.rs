mod token_reader;
mod parser;
mod utils;

fn main() {
    let filename = utils::get_filename();

    let raw_contents = utils::read_file(&filename);

    let mut tokens = token_reader::read_tokens(&raw_contents);
    println!("Parsed tokens: {:?}", tokens);

    parser::parse(tokens);

}

