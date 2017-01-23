use std::fmt;

mod token;
mod rst;

pub use token::{TokenFile, Token, Tokenizer, TokenBuilder};

pub use rst::{RstFile, Node, Parser, RstBuilder};


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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Range {
    lo: u32,
    hi: u32
}

impl Range {
    pub fn lo(&self) -> u32 {
        self.lo
    }
    pub fn hi(&self) -> u32 {
        self.hi
    }
    pub fn from_to(lo: u32, hi: u32) -> Range {
        assert!(lo < hi);
        Range { lo: lo, hi: hi }
    }
}

impl std::ops::Index<Range> for str {
    type Output = str;
    fn index(&self, index: Range) -> &Self::Output {
        &self[index.lo as usize..index.hi as usize]
    }
}

impl std::ops::Index<Range> for String {
    type Output = str;
    fn index(&self, index: Range) -> &Self::Output {
        &self[index.lo as usize..index.hi as usize]
    }
}


impl fmt::Debug for Range {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}; {})", self.lo, self.hi)
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
    let f = RstFile::new(f, file_type, parser);
    let actual = f.dump();
    assert!(actual.trim() == expected.trim(), "\nInput:\n{}\n\nOutput:\n{}\n\nExpected:\n{}\n", text, actual, expected);
}
