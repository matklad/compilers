extern crate syntax;

use syntax::{CharIterator, TokenBuilder, NodeType};
use syntax::{AstBuilder, TokenIterator};


fn main() {}

use syntax::WHITESPACE;

const LPAREN: NodeType = NodeType(03, "lparen");
const RPAREN: NodeType = NodeType(04, "rparen");
const NUMBER: NodeType = NodeType(05, "number");
const STRING: NodeType = NodeType(06, "string");
const ID: NodeType = NodeType(07, "id");

const TINY_FILE: NodeType = NodeType(08, "file");
const LITERAL: NodeType = NodeType(09, "literal");
const LIST: NodeType = NodeType(10, "list");

fn tiny_tokenizer(chars: CharIterator, builder: &mut TokenBuilder) {
    let mut chars = chars;
    loop {
        if builder.try_emit(LPAREN, &mut chars, '(') || builder.try_emit(RPAREN, &mut chars, ')')
            || builder.try_advance_while(WHITESPACE, &mut chars, &char::is_whitespace)
            || builder.try_advance_while(ID, &mut chars, &char::is_alphabetic)
            || builder.try_advance_while(NUMBER, &mut chars, &|c| c.is_digit(10)) {
            continue
        }

        let c = match chars.next() {
            Some(c) => c,
            None => break
        };

        builder.advance(c);
        match c {
            '"' => loop {
                match chars.next() {
                    Some('"') => {
                        builder.advance('"');
                        builder.emit(STRING);
                        break;
                    }
                    Some(c) => builder.advance(c),
                    None => {
                        builder.error();
                        break;
                    }
                }
            },

            _ => builder.error()
        }
    }
}

fn tiny_parser(tokens: TokenIterator, builder: &mut AstBuilder) {
    parse_literal(tokens, builder);
}

fn parse_literal(mut tokens: TokenIterator, builder: &mut AstBuilder) {
    if let Some(t) = tokens.next() {
        match t.ty {
            NUMBER | STRING => {
                builder.start(LITERAL);
                builder.advance(t);
                builder.finish(LITERAL);
            }
            ID => builder.advance(t),
            LPAREN => {
                builder.start(LIST);
                builder.advance(t);
                if let Some(t) = tokens.next() {
                    if t.ty == RPAREN {
                        builder.advance(t)
                    } else {
                        panic!("Unexpected token: {:?}", t)
                    }
                } else {
                    panic!("Unexpected eof")
                }

                builder.finish(LIST);
            }
            _ => panic!("Unexpected token: {:?}", t)
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
  number "92"
    "#);

    check_parser(r#""Hi!""#, r#"
literal
  string "\"Hi!\""
    "#);

    check_parser(r"foo", r#"
id "foo"
    "#);

    check_parser(r"()", r#"
list
  lparen "("
  rparen ")"
    "#);


    //        check_parser(r#"(foo 82 "hello")"#, r#"
    //    list
    //      id "foo"
    //      literal
    //        number "82"
    //      literal
    //        string "\"hello\""
    //        "#);
}