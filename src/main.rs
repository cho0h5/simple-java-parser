use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Token {
    Vtype,      // for the types of variables and function
    Num,        // for signed integers
    Character,  // for a single character
    Boolstr,    // for Boolean strings
    Literal,    // for literal strings
    Id,         // for the the identifiers of variables and functions
    If,         // for if statements
    Else,       // for else statements
    While,      // for while statements
    Return,     // for return statements
    Class,      // for class declarations
    Addsub,     // for + and - arithmetic
    Multdiv,    // for * and / arithmetic operators
    Assign,     // for assignment operators
    Comp,       // for comparison operators
    Semi,       // for semicolons
    Comma,      // for commas
    Lparen,     // for (
    Rparen,     // for )
    Lbrace,     // for {
    Rbrace,     // for }
}

fn main() {
    // get file name
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("error: no input file");
    }
    let filename = &args[1];

    // file read
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("Read token");


    let tokens = read_tokens(&contents);
    println!("{:?}", tokens);
}

fn read_tokens(contents: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for word in contents.split_whitespace() {
        let token = match word {
            "vtype" => Token::Vtype,
            "num" => Token::Num,
            "character" => Token::Character,
            "boolstr" => Token::Boolstr,
            "literal" => Token::Literal,
            "id" => Token::Id,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "return" => Token::Return,
            "class" => Token::Class,
            "addsub" => Token::Addsub,
            "multdiv" => Token::Multdiv,
            "assign" => Token::Assign,
            "comp" => Token::Comp,
            "semi" => Token::Semi,
            "comma" => Token::Comma,
            "lparen" => Token::Lparen,
            "rparen" => Token::Rparen,
            "lbrace" => Token::Lbrace,
            "rbrace" => Token::Rbrace,
            _ => panic!("unknown token"),
        };
        tokens.push(token);
    }

    tokens
}
