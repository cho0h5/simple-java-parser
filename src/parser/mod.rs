mod parsing_table;

use std::collections::VecDeque;

use crate::token_reader::Token;

#[derive(Debug)]
struct StackItem {
    state: u32,
    token: Option<Token>,
}

impl StackItem {
    fn from(state: u32, token: Option<Token>) -> StackItem {
        StackItem { state: state, token: token }
    }
}

pub fn parse(tokens: VecDeque<Token>) {
    let mut stack = vec![StackItem::from(0, None)];

    // println!("{:?}", stack);
    // println!("{:?}", tokens);
    
    println!("parsing table");
    let parsing_table = parsing_table::get_parsing_table();
    println!("{:?}", parsing_table);
}
