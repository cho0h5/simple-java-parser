use std::fmt;
use std::collections::VecDeque;

use crate::parser::Node;

pub struct Tokens(pub VecDeque<Node>);

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 234123)
    }
}

pub struct Tree(pub Node);

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 234)
    }
}
