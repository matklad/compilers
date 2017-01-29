use syntax::RstBuilder;

use node::{ID, EQ, ASSIGNMENT, EXPR_STMT, NUMBER, LIT_EXPR};

pub fn parse(builder: &mut RstBuilder) {
    loop {
        if !parse_stmt(builder) {
            return
        }
        builder.skip_ws();
    }
}

fn parse_stmt(builder: &mut RstBuilder) -> bool {
    if let Some(ID) = builder.peek() {
        builder.start(ASSIGNMENT);
        builder.eat(ID);
        builder.skip_ws();
        if let Some(EQ) = builder.peek() {
            builder.eat(EQ);
            parse_expr(builder);
            builder.finish(ASSIGNMENT);
            return true;
        } else {
            return false;
//            builder.backtrack(ASSIGNMENT)
        }
    }

    builder.start(EXPR_STMT);
    parse_expr(builder);
    builder.finish(EXPR_STMT);
    true
}

fn parse_expr(builder: &mut RstBuilder) -> bool {
    builder.start(LIT_EXPR);
    builder.eat(NUMBER);
    builder.finish(LIT_EXPR);
    true
}