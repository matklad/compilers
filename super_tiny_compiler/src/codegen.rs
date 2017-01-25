use target::{Program, ExpressionStatement, Expression};
use std::fmt::Write;

macro_rules! w {
  ($($tt:tt)*) => { write!($($tt)*).unwrap() }
}

pub fn generate(p: &Program) -> String {
    let mut buff = String::new();
    for stmt in p.body.iter() {
        generate_stmt(&mut buff, stmt);
        buff.push('\n');
    }
    buff
}

fn generate_stmt(buff: &mut String, stmt: &ExpressionStatement) {
    generate_expr(buff, &stmt.0);
    buff.push(';');
}

fn generate_expr(buff: &mut String, expr: &Expression) {
    match *expr {
        Expression::Call { ref calee, ref arguments } => {
            generate_expr(buff, calee);
            buff.push('(');
            let mut first = true;
            for arg in arguments.iter() {
                if !first {
                    buff.push_str(", ");
                }
                generate_expr(buff, arg);
                first = false
            }
            buff.push(')');
        }
        Expression::Identifier { ref value } => buff.push_str(&*value),
        Expression::NumberLiteral { ref value } => w!(buff, "{}", value),
        Expression::StringLiteral { ref value } => w!(buff, r#""{}""#, value),
    };
}

#[test]
fn test_codegen() {
    let file = ::ast::AstFile::new(super::parse_tiny(r#"(add 2 (subtract 4 2))"hello""#.to_owned()));
    let program = ::target::translate(&file);
    let actual = generate(&program);
    let expect = r#"
add(2, subtract(4, 2));
"hello";
"#.trim_left();

    assert_eq!(expect, actual);
}