use std::collections::VecDeque;

use crate::parser::Node::Terminal;
use crate::parser::formatting::Tokens;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Token {
    // terminals
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

    // for EOL
    EOL,

    // non-terminals
    CODE,
    CODE_,
    VDECL,
    ASSIGN,
    RHS,
    EXPR,
    EXPR_,
    EXPR__,
    FDECL,
    ARG,
    MOREARGS,
    BLOCK,
    STMT,
    COND,
    ELSE,
    RETURN,
    CDECL,
    ODECL,
}

pub fn read_tokens(contents: &String) -> Tokens {
    let mut tokens = VecDeque::new();
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
            unknown_token => panic!("unknown token: {}", unknown_token),
        };
        tokens.push_back(Terminal(token));
    }
    tokens.push_back(Terminal(Token::EOL));

    Tokens(tokens)
}
