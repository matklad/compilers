use std::fmt;

mod token;
mod ast;

pub use token::{TokenFile, Token, Tokenizer, TokenBuilder, CharIterator};

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


pub fn check_tokenizer(tokenizer: &Tokenizer, text: &str, expected: &str) {
    let text = text.trim();
    let f = TokenFile::new(text.to_owned(), tokenizer);
    let actual = f.dump();
    assert!(actual.trim() == expected.trim(), "\nInput:\n{}\n\nOutput:\n{}\n\nExpected:\n{}", text, actual, expected);
}

pub fn check_parser(tokenizer: &Tokenizer, parser: &Parser, file_type: NodeType, text: &str, expected: &str) {
    let text = text.trim();
    let f = TokenFile::new(text.to_owned(), tokenizer);
    let f = AstFile::new(f, file_type, parser);
    let actual = f.dump();
    assert!(actual.trim() == expected.trim(), "\nInput:\n{}\n\nOutput:\n{}\n\nExpected:\n{}\n", text, actual, expected);
}
