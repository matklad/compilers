extern crate syntax;

use syntax::{AstBuilder, TokenBuilder, NodeType};


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

fn tiny_tokenizer(builder: &mut TokenBuilder) {
    loop {
        if builder.try_emit(LPAREN, '(') || builder.try_emit(RPAREN, ')')
            || builder.try_advance_while(WHITESPACE, &char::is_whitespace)
            || builder.try_advance_while(ID, &char::is_alphabetic)
            || builder.try_advance_while(NUMBER, &|c| c.is_digit(10)) {
            continue
        }

        let c = match builder.peek() {
            Some(c) => c,
            None => break
        };

        builder.bump();
        match c {
            '"' => loop {
                match builder.peek() {
                    Some('"') => {
                        builder.bump();
                        builder.emit(STRING);
                        break;
                    }
                    Some(_) => builder.bump(),
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

fn tiny_parser(builder: &mut AstBuilder) {
    parse(builder);
}

fn parse(builder: &mut AstBuilder) -> bool {
    let ty = match builder.peek() {
        Some(ty) => ty,
        None => return false,
    };

    match ty {
        WHITESPACE => panic!("Leading ws not chomped!"),
        NUMBER | STRING => {
            builder.start(LITERAL);
            builder.bump();
            builder.finish(LITERAL);
            true
        }
        ID => {
            builder.bump();
            true
        },
        LPAREN => {
            builder.start(LIST);
            builder.bump();
            loop {
                if !parse(builder) {
                    break
                }
            }

            match builder.peek() {
                Some(ty) if ty == RPAREN => {
                    builder.bump();
                    builder.finish(LIST);
                    true
                }
                None | Some(_) => panic!("Expected RPAREN")
            }
        }
        _ => false
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