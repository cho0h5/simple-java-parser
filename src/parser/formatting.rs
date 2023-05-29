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
        self.0.fmt(f, 0, true);
        Ok(())
    }
}

impl Node {
    fn fmt(&self, f: &mut fmt::Formatter, n: usize, isLast: bool) {
        match self {
            Terminal(token) => {
                for _ in 0..n-1 {
                    write!(f, "    ");
                }
                write!(f, "{}", if isLast { "└── " } else { "├── " });
                write!(f, "{:?}\n", token)
            },
            NonTerminal(token, children) => {
                if n != 0 {
                    for _ in 0..n-1 {
                        write!(f, "    ");
                    }
                    write!(f, "{}", if isLast { "└── " } else { "├── " });
                }
                write!(f, "\x1b[36m{:?}\x1b[97m\n", token);
                for i in 0..children.len() {
                    children[i].fmt(f, n + 1, i == children.len() - 1);
                }
                Ok(())
            }
        };
    }
}
