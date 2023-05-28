mod parsing_table;

use std::collections::VecDeque;

use crate::token_reader::Token;
use crate::parser::parsing_table::TableElement::*;

#[derive(Debug)]
struct StackItem {
    state: usize,
    token: Option<Token>,
}

impl StackItem {
    fn from(state: usize, token: Option<Token>) -> StackItem {
        StackItem { state: state, token: token }
    }
}

pub fn parse(mut tokens: VecDeque<Token>) {
    let parsing_table = parsing_table::get_parsing_table();
    let reduction_table = parsing_table::get_reduction_table();
    let mut stack = vec![StackItem::from(0, None)];

    loop {
        let current_state = stack.last().unwrap().state;
        let next_token = tokens[0];

        println!("[target] {} {:?}", current_state, next_token);
        if !parsing_table[current_state].contains_key(&next_token) {
            unimplemented!();
        }
        println!("[try] {:?}", parsing_table[current_state][&next_token]);

        match parsing_table[current_state][&next_token] {
            Shift(state) => shift(&mut tokens, &mut stack, state),
            Reduce(state) => unimplemented!(),
            Goto(state) => unimplemented!(),
            Accepted => unimplemented!()
        };
    }
}

fn shift(tokens: &mut VecDeque<Token>, stack: &mut Vec<StackItem>, state: usize) {
    let next_token = tokens.pop_front().unwrap();
    stack.push(StackItem::from(state, Some(next_token)));
    println!("[shift] {} {:?}", state, next_token);
    println!("stack: {:?}", stack);
    println!("tokens: {:?}\n", tokens);
}

// fn reduce(stack: &mut Vec<StackItem>, 
