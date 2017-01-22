use std::fmt;

mod token;
mod ast;

pub use token::{TokenFile, Token, Tokenizer, TokenBuilder};

pub use ast::{AstFile, Node, TokenIterator, Parser, AstBuilder};


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeType(pub u32, pub &'static str);

pub const ERROR: NodeType = NodeType(0, "error");
pub const WHITESPACE: NodeType = NodeType(1, "whitespace");

impl NodeType {
    pub fn name(&self) -> &'static str {
        self.1
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "NodeType({})", self.name())
    }
}


