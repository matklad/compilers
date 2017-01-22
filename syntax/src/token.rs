use std::str::Chars;
use std::iter::Peekable;

use super::NodeType;

pub struct Token<'file> {
    pub ty: NodeType,
    pub text: &'file str,
}

pub struct TokenFile {
    text: String,
    tokens: Vec<RawToken>
}

impl TokenFile {
    pub fn new(text: String, tokenizer: &Tokenizer) -> TokenFile {
        let mut builder = TokenBuilder::new();
        {
            let chars = CharIterator::new(&text);
            tokenizer(chars, &mut builder);
        }
        TokenFile {
            text: text,
            tokens: builder.into_tokens(),
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.iter().map(|t| {
            Token {
                ty: t.ty,
                text: &self.text[(t.start as usize)..(t.end as usize)],
            }
        }).collect()
    }

    pub fn dump(&self) -> String {
        self.tokens().iter()
            .map(|t| format!("{} {:?}\n", t.ty.name(), t.text))
            .collect()
    }
}

pub type Tokenizer = Fn(CharIterator, &mut TokenBuilder);


pub struct TokenBuilder {
    tokens: Vec<RawToken>,
    prev_offset: u32,
    curr_offset: u32,
}

impl TokenBuilder {
    pub fn advance(&mut self, c: char) {
        self.curr_offset += c.len_utf8() as u32
    }

    pub fn try_advance_while(
        &mut self,
        ty: NodeType,
        chars: &mut CharIterator,
        cond: &Fn(char) -> bool
    ) -> bool {
        match chars.peek() {
            None => return false,
            Some(c) if cond(c) => {
                self.advance(c);
                chars.next();
            }
            Some(_) => return false
        }

        while let Some(c) = chars.peek() {
            if !cond(c) { break; }
            self.advance(c);
            chars.next();
        }

        self.emit(ty);
        true
    }

    pub fn emit(&mut self, ty: NodeType) {
        let token = RawToken {
            ty: ty,
            start: self.prev_offset,
            end: self.curr_offset,
        };
        self.prev_offset = self.curr_offset;
        self.tokens.push(token)
    }

    pub fn try_emit(&mut self, ty: NodeType, chars: &mut CharIterator, expected: char) -> bool {
        match chars.peek() {
            Some(c) if c == expected => {
                self.advance(expected);
                chars.next();
                self.emit(ty);
                true
            }
            _ => false
        }
    }

    fn new() -> TokenBuilder {
        TokenBuilder {
            tokens: Vec::new(),
            prev_offset: 0,
            curr_offset: 0,
        }
    }

    fn into_tokens(self) -> Vec<RawToken> {
        self.tokens
    }
}

pub struct CharIterator<'a> {
    inner: Peekable<Chars<'a>>
}

impl<'a> CharIterator<'a> {
    pub fn next(&mut self) -> Option<char> {
        self.inner.next()
    }

    pub fn peek(&mut self) -> Option<char> {
        self.inner.peek().cloned()
    }

    fn new(text: &'a str) -> Self {
        CharIterator { inner: text.chars().peekable() }
    }
}

#[derive(Clone, Copy)]
struct RawToken {
    ty: NodeType,
    start: u32,
    end: u32,
}
