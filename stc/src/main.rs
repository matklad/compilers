extern crate syntax;

use syntax::{TokenBuilder, NodeType};
use syntax::{AstBuilder, TokenIterator};


fn main() {}

use syntax::{ERROR, WHITESPACE};
const LPAREN: NodeType = NodeType(03, "lparen");
const RPAREN: NodeType = NodeType(04, "rparen");
const NUMBER: NodeType = NodeType(05, "number");
const STRING: NodeType = NodeType(06, "string");
const ID: NodeType = NodeType(07, "id");

const TINY_FILE: NodeType = NodeType(08, "file");
const LITERAL: NodeType = NodeType(09, "literal");

fn tiny_tokenizer(text: &str, builder: &mut TokenBuilder) {
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        builder.advance(c);
        match c {
            c if c.is_whitespace() => {
                while let Some(&c) = chars.peek() {
                    if !c.is_whitespace() {
                        break;
                    }
                    builder.advance(c);
                    chars.next();
                }
                builder.emit(WHITESPACE)
            }

            '(' => builder.emit(LPAREN),
            ')' => builder.emit(RPAREN),

            c if c.is_alphabetic() => {
                while let Some(&c) = chars.peek() {
                    if !c.is_alphabetic() {
                        break;
                    }
                    builder.advance(c);
                    chars.next();
                }
                builder.emit(ID)
            }

            c if c.is_digit(10) => {
                while let Some(&c) = chars.peek() {
                    if !c.is_digit(10) {
                        break;
                    }
                    builder.advance(c);
                    chars.next();
                }
                builder.emit(NUMBER)
            }

            '"' => loop {
                match chars.next() {
                    Some('"') => {
                        builder.advance('"');
                        builder.emit(STRING);
                        break;
                    }
                    Some(c) => builder.advance(c),
                    None => {
                        builder.emit(ERROR);
                        break;
                    }
                }
            },

            _ => builder.emit(ERROR)
        }
    }
}

fn tiny_parser(tokens: TokenIterator, builder: &mut AstBuilder) {
    parse_literal(tokens, builder);
}

fn parse_literal(mut tokens: TokenIterator, builder: &mut AstBuilder) {
    if let Some(t) = tokens.next() {
        if t.ty == NUMBER || t.ty == STRING {
            builder.start(LITERAL);
            builder.advance(t);
            builder.finish(LITERAL);
        }
    }
}


fn check_tokenizer(text: &str, expected: &str) {
    syntax::check_tokenizer(&tiny_tokenizer, text, expected);
}

fn check_parser(text: &str, expected: &str) {
    syntax::check_parser(&tiny_tokenizer, &tiny_parser, TINY_FILE, text, expected);
}


#[test]
fn test_tokenizer() {
    check_tokenizer(r#"(foo "hello"  1)"#, r#"
lparen "("
id "foo"
whitespace " "
string "\"hello\""
whitespace "  "
number "1"
rparen ")"
"#);
}



#[test]
fn test_parser() {
    check_parser("92", r#"
literal
  number
    "#)
}