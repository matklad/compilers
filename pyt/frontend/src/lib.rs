extern crate syntax;

use syntax::{RstBuilder, TokenBuilder, NodeType, WHITESPACE};

mod tokenizer;

pub mod node {
    use syntax::NodeType;

    pub const NUMBER: NodeType = NodeType(10, "number");
    pub const ID: NodeType = NodeType(11, "id");

    pub const EQ: NodeType = NodeType(12, "=");
    pub const ADD: NodeType = NodeType(13, "+");
    pub const SUB: NodeType = NodeType(14, "-");
    pub const MUL: NodeType = NodeType(15, "*");
    pub const DIV: NodeType = NodeType(16, "/");

    pub const LPAREN: NodeType = NodeType(17, "lparen");
    pub const RPAREN: NodeType = NodeType(18, "rparen");


    pub const LIT_EXPR: NodeType = NodeType(19, "lit_expr");
    pub const BIN_EXPR: NodeType = NodeType(20, "bin_expr");
    pub const CALL_EXPR: NodeType = NodeType(21, "call_expr");
    pub const PAREN_EXPR: NodeType = NodeType(22, "paren_expr");

    pub const ASSIGNMENT: NodeType = NodeType(23, "assignment");
    pub const EXPR_STMT: NodeType = NodeType(24, "expr_stmt");

    pub const FILE: NodeType = NodeType(25, "file");
}

