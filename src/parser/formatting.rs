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
        self.0.fmt(f, true, vec![]);
        Ok(())
    }
}

impl Node {
    fn fmt(&self, f: &mut fmt::Formatter, is_last: bool, bridge: Vec<bool>) {
        match self {
            Terminal(token) => {
                for b in &bridge[..bridge.len()-1] {
                    write!(f, "{}", if *b { "│   " } else { "    " }).ok();
                }
                write!(f, "{}", if is_last { "└── " } else { "├── " }).ok();
                write!(f, "{:?}\n", token).ok();
            },
            NonTerminal(token, children) => {
                if bridge.len() != 0 {
                    for b in &bridge[..bridge.len()-1] {
                        write!(f, "{}", if *b { "│   " } else { "    " }).ok();
                    }
                    write!(f, "{}", if is_last { "└── " } else { "├── " }).ok();
                }
                write!(f, "\x1b[36m{:?}\x1b[97m\n", token).ok();
                for i in 0..children.len() {
                    let mut _bridge = bridge.clone();
                    _bridge.push(i != children.len() - 1);
                    children[i].fmt(f, i == children.len() - 1, _bridge);
                }
            }
        };
    }
}
