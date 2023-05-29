use std::fmt;
use std::collections::VecDeque;

use crate::parser::Node;
use crate::parser::Node::*;

pub struct Tokens(pub VecDeque<Node>);

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[").ok();
        for node in &self.0 {
            let token = match node {
                Terminal(token) => token,
                NonTerminal(token, _) => token,
            };
            write!(f, "{:?} ", token).ok();
        }
        write!(f, "]").ok();
        Ok(())
    }
}

pub struct Tree(pub Node);

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f, 0);
        Ok(())
    }
}

impl Node {
    fn fmt(&self, f: &mut fmt::Formatter, n: usize) {
        match self {
            Terminal(token) => {
                for _ in 0..n {
                    write!(f, "  ");
                }
                write!(f, "{:?}\n", token)
            },
            NonTerminal(token, children) => {
                for _ in 0..n {
                    write!(f, "  ");
                }
                write!(f, "{:?}\n", token);
                for child in children {
                    child.fmt(f, n + 1);
                }
                Ok(())
            }
        };
    }
}
