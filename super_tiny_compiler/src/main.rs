extern crate syntax;

use std::io::Read;

mod rst;
mod ast;
mod target;
mod codegen;

pub fn parse_tiny(text: String) -> syntax::RstFile {
    let tokens = syntax::TokenFile::new(text, &rst::tiny_tokenizer);
    syntax::RstFile::new(tokens, rst::TINY_FILE, &rst::tiny_parser)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let rst = parse_tiny(input);
    let ast = ast::AstFile::new(rst);
    let program = target::translate(&ast);
    let code = codegen::generate(&program);
    println!("{}", code);
}

