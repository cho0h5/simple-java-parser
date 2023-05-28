use std::collections::HashMap;
use TableElement::*;
use crate::token_reader::Token;
use crate::token_reader::Token::*;
use crate::utils;

#[derive(Debug)]
pub enum TableElement {
    Shift(u32),
    Reduce(u32),
    Goto(u32),
    Accepted,
}

pub fn get_parsing_table() -> Vec<HashMap<Token, TableElement>> {
    let mut table = vec![];

    table
}
