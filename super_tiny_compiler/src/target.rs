use ast::{self, AstFile, AstElement};
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Program {
    pub body: Vec<ExpressionStatement>
}

#[derive(Debug)]
pub struct ExpressionStatement(pub Expression);

#[derive(Debug)]
pub enum Expression {
    Call {
        calee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    Identifier {
        value: String
    },

    NumberLiteral {
        value: u32
    },

    StringLiteral {
        value: String
    }
}

pub fn translate(ast: &AstFile) -> Program {
    translate_program(ast.root())
}

fn translate_program(program: ast::Program) -> Program {
    Program {
        body: Vec::from_iter(program.elements()
            .into_iter()
            .map(translate_element)
            .map(ExpressionStatement))
    }
}

fn translate_element(element: ast::ListElement) -> Expression {
    match element {
        ast::ListElement::List(call) => translate_call(call),
        ast::ListElement::Literal(lit) => translate_literal(lit),
        ast::ListElement::Variable(var) => translate_identifier(var),
    }
}

fn translate_call(element: ast::List) -> Expression {
    let mut args = element.elements();
    Expression::Call {
        calee: Box::new(translate_element(args.remove(0))),
        arguments: Vec::from_iter(args.into_iter().map(translate_element)),
    }
}

fn translate_literal(element: ast::Literal) -> Expression {
    match element.value() {
        ast::LiteralValue::Int(value) => Expression::NumberLiteral { value: value },
        ast::LiteralValue::String(value) => Expression::StringLiteral { value: value.to_owned() },
    }
}

fn translate_identifier(element: ast::Variable) -> Expression {
    Expression::Identifier { value: element.node().text().to_owned() }
}

#[test]
fn test_translation() {
    let source = AstFile::new(super::parse_tiny(r#"hello (1 "hi")"#.to_owned()));
    let target = translate(&source);
    let actual = format!("{:#?}", target);
    let expected = r#"
Program {
    body: [
        ExpressionStatement(
            Identifier {
                value: "hello"
            }
        ),
        ExpressionStatement(
            Call {
                calee: NumberLiteral {
                    value: 1
                },
                arguments: [
                    StringLiteral {
                        value: "hi"
                    }
                ]
            }
        )
    ]
}"#;

    if actual.trim() != expected.trim() {
        panic!("Mismatch!\nActual:\n{}\nExpected:\n{}\n", actual, expected)
    }
}