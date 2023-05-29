mod token_reader;
mod parser;
mod utils;

fn main() {
    let filename = utils::get_filename();

    let raw_contents = utils::read_file(&filename);

    let tokens = token_reader::read_tokens(&raw_contents);
    println!("Read tokens:\n{}\n", tokens);

    let parse_tree = parser::parse(tokens);
    println!("Parse tree:\n{}", parse_tree);
}

