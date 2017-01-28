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

    pub fn emit(&mut self, ty: NodeType) {
        let token = RawToken {
            ty: ty,
            range: Range::from_to(self.prev_offset, self.curr_offset),
        };
        self.prev_offset = self.curr_offset;
        self.tokens.push(token)
    }

    pub fn try_text_token(&mut self, tokens: &[(NodeType, char)]) -> bool {
        let next = match self.peek() {
            Some(n) => n,
            None => return false,
        };

        for &(ty, c) in tokens {
            if c == next {
                self.bump();
                self.emit(ty);
                return true;
            }
        }
        false
    }

    pub fn try_pred_token(&mut self, tokens: &[(NodeType, &Fn(char) -> bool)]) -> bool {
        let next = match self.peek() {
            Some(n) => n,
            None => return false,
        };

        let (ty, cond) = match tokens.iter().find(|&&(_, cond)| cond(next)) {
            Some(&(ty, cond)) => (ty, cond),
            None => return false,
        };

        self.bump();
        while let Some(c) = self.peek() {
            if !cond(c) { break; }
            self.bump()
        }
        self.emit(ty);

        true
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
