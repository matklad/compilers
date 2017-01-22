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
        tokenizer(&text, &mut builder);
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

pub type Tokenizer = Fn(&str, &mut TokenBuilder);


pub struct TokenBuilder {
    tokens: Vec<RawToken>,
    prev_offset: u32,
    curr_offset: u32,
}

impl TokenBuilder {
    pub fn advance(&mut self, c: char) {
        self.curr_offset += c.len_utf8() as u32
    }

    pub fn emit(&mut self, tt: NodeType) {
        let token = RawToken {
            ty: tt,
            start: self.prev_offset,
            end: self.curr_offset,
        };
        self.prev_offset = self.curr_offset;
        self.tokens.push(token)
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

#[derive(Clone, Copy)]
struct RawToken {
    ty: NodeType,
    start: u32,
    end: u32,
}
