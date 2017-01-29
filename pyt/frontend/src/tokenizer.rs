use node::{NUMBER, ID, EQ, ADD, SUB, MUL, DIV, LPAREN, RPAREN};
use syntax::{TokenBuilder, NodeType, WHITESPACE};


pub fn tokenize(builder: &mut TokenBuilder) {
    let text_tokens = [
        (ADD, '+'), (SUB, '-'), (MUL, '*'), (DIV, '/'),
        (EQ, '='), (LPAREN, '('), (RPAREN, ')')
    ];

    let pred_tokens: &[(NodeType, &Fn(char) -> bool)] = &[
        (ID, &char::is_alphabetic),
        (WHITESPACE, &char::is_whitespace),
        (NUMBER, &|c| c.is_digit(10)),
    ];

    loop {
        if builder.try_text_token(&text_tokens) || builder.try_pred_token(&pred_tokens) {
            continue
        }
        match builder.peek() {
            Some(_) => {
                builder.bump();
                builder.error();
            }
            None => break
        };
    }
}
