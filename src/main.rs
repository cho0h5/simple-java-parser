mod token_reader;
mod utils;

fn main() {
    let filename = utils::get_filename();

    let raw_contents = utils::read_file(&filename);

    let tokens = token_reader::read_tokens(&raw_contents);
    println!("{:?}", tokens);
}

