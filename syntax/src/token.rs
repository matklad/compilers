use std::str::Chars;
use std::iter::Peekable;

use super::{NodeType, Range};

#[derive(Debug, Clone, Copy)]
pub struct Token<'file> {
    pub ty: NodeType,
    pub text: &'file str,
    pub range: Range,
}

pub struct TokenFile {
    text: String,
    tokens: Vec<RawToken>
}

impl TokenFile {
    pub fn new(text: String, tokenizer: &Tokenizer) -> TokenFile {
        let tokens = {
            let chars = text.chars().peekable();
            let mut builder = TokenBuilder::new(chars);
            tokenizer(&mut builder);
            builder.into_tokens()
        };
        TokenFile {
            text: text,
            tokens: tokens,
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.iter().map(|t| {
            Token {
                ty: t.ty,
                text: &self.text[t.range],
                range: t.range,
            }
        }).collect()
    }

    pub fn dump(&self) -> String {
        self.tokens().iter()
            .map(|t| format!("{} {:?}\n", t.ty.name(), t.text))
            .collect()
    }

    pub fn into_text(self) -> String {
        self.text
    }
}

pub type Tokenizer = Fn(&mut TokenBuilder);

type CharIter<'a> = Peekable<Chars<'a>>;

pub struct TokenBuilder<'a> {
    chars: CharIter<'a>,
    tokens: Vec<RawToken>,
    prev_offset: u32,
    curr_offset: u32,
}

impl<'a> TokenBuilder<'a> {
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    pub fn bump(&mut self) {
        let c = self.chars.next().expect("EOF");
        self.curr_offset += c.len_utf8() as u32
    }

    pub fn try_advance_while(
        &mut self,
        ty: NodeType,
        cond: &Fn(char) -> bool
    ) -> bool {
        match self.peek() {
            None => return false,
            Some(c) if cond(c) => {
                self.bump();
            }
            Some(_) => return false
        }

        while let Some(c) = self.peek() {
            if !cond(c) { break; }
            self.bump()
        }

        self.emit(ty);
        true
    }

    pub fn emit(&mut self, ty: NodeType) {
        let token = RawToken {
            ty: ty,
            range: Range::from_to(self.prev_offset, self.curr_offset),
        };
        self.prev_offset = self.curr_offset;
        self.tokens.push(token)
    }

    pub fn try_emit(&mut self, ty: NodeType, expected: char) -> bool {
        match self.peek() {
            Some(c) if c == expected => {
                self.bump();
                self.emit(ty);
                true
            }
            _ => false
        }
    }

    pub fn error(&mut self) {
        self.emit(::ERROR);
    }

    fn new(chars: CharIter) -> TokenBuilder {
        TokenBuilder {
            chars: chars,
            tokens: Vec::new(),
            prev_offset: 0,
            curr_offset: 0,
        }
    }

    fn into_tokens(self) -> Vec<RawToken> {
        self.tokens
    }
}

#[derive(Clone, Copy)]
struct RawToken {
    ty: NodeType,
    range: Range,
}
