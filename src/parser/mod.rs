mod parsing_table;
pub mod formatting;

use std::collections::VecDeque;

use crate::token_reader::Token;

use crate::parser::parsing_table::TableElement::*;
use crate::parser::parsing_table::Reduction;

use crate::parser::formatting::Tokens;
use crate::parser::formatting::Tree;

use Node::*;

#[derive(Debug)]
pub enum Node {
    Terminal(Token),
    NonTerminal(Token, Vec<Node>),
}

#[derive(Debug)]
struct StackItem {
    state: usize,
    tree: Option<Node>,
}

impl StackItem {
    fn from(state: usize, tree: Option<Node>) -> Self {
        StackItem { state: state, tree: tree }
    }
}

pub fn parse(tokens: Tokens) -> Tree {
    let mut tokens = tokens.0;
    let parsing_table = parsing_table::get_parsing_table();
    let reduction_table = parsing_table::get_reduction_table();
    let mut stack = vec![StackItem::from(0, None)];

    loop {
        let current_state = stack.last().unwrap().state;
        let next_token = match tokens[0] {
            Terminal(token) => token,
            NonTerminal(token, _) => token,
        };

        if !parsing_table[current_state].contains_key(&next_token) {
            unimplemented!();
        }

        match parsing_table[current_state][&next_token] {
            Shift(next_state) => shift_goto(&mut tokens, &mut stack, next_state),
            Reduce(reduction_index) => reduce(&mut tokens, &mut stack, reduction_table[reduction_index]),
            Goto(next_state) => shift_goto(&mut tokens, &mut stack, next_state),
            Accepted => break,
        };
    }

    Tree(stack.pop().unwrap().tree.unwrap())
}

fn shift_goto(tokens: &mut VecDeque<Node>, stack: &mut Vec<StackItem>, next_state: usize) {
    let next_token = tokens.pop_front().unwrap();
    stack.push(StackItem::from(next_state, Some(next_token)));
}

fn reduce(tokens: &mut VecDeque<Node>, stack: &mut Vec<StackItem>, reduction: Reduction) {
    let mut children: Vec<Node> = vec![];
    for _ in 0..reduction.right {
        children.push(stack.pop().unwrap().tree.unwrap());
    }
    children.reverse();

    tokens.push_front(NonTerminal(reduction.left, children));
}
