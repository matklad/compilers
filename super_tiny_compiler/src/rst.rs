use syntax::{self, RstBuilder, TokenBuilder, NodeType, WHITESPACE};

pub const LPAREN: NodeType = NodeType(03, "lparen");
pub const RPAREN: NodeType = NodeType(04, "rparen");
pub const NUMBER: NodeType = NodeType(05, "number");
pub const STRING: NodeType = NodeType(06, "string");
pub const ID: NodeType = NodeType(07, "id");

pub const TINY_FILE: NodeType = NodeType(08, "file");
pub const LITERAL: NodeType = NodeType(09, "literal");
pub const LIST: NodeType = NodeType(10, "list");

pub fn tiny_tokenizer(builder: &mut TokenBuilder) {
    let text_tokens = [(LPAREN, '('), (RPAREN, ')')];
    let pred_tokens: &[(NodeType, &Fn(char) -> bool)] = &[
        (ID, &char::is_alphabetic),
        (WHITESPACE, &char::is_whitespace),
        (NUMBER, &|c| c.is_digit(10)),
    ];
    loop {
        if builder.try_text_token(&text_tokens) || builder.try_pred_token(&pred_tokens){
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

pub fn tiny_parser(builder: &mut RstBuilder) {
    loop {
        builder.skip_ws();
        if !parse(builder) {
            break
        }
    }
}

fn parse(builder: &mut RstBuilder) -> bool {
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
                builder.skip_ws();
                if !parse(builder) {
                    break
                }
                builder.skip_ws();
            }

            builder.eat(RPAREN);
            builder.finish(LIST);
            true
        }
        _ => false
    }
}

#[cfg(test)]
fn check_tokenizer(text: &str, expected: &str) {
    syntax::check_tokenizer(&tiny_tokenizer, text, expected);
}

#[cfg(test)]
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


    check_parser(r#"(foo 82 "hello")"#, r#"
list
  lparen "("
  id "foo"
  whitespace " "
  literal
    number "82"
  whitespace " "
  literal
    string "\"hello\""
  rparen ")"
    "#);
}