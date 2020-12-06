mod utils {
    use crate::ast::Expression;
    use crate::parser;

    lazy_static! {
        static ref PARSER: parser::ExpressionParser = parser::ExpressionParser::new();
    }

    pub fn parse(token: &'static str) -> Expression {
        PARSER.parse(token).unwrap()
    }
}

mod literals;
mod bin_op;
mod block_expr;
mod small_prog;