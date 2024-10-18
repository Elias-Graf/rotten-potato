use super::expr::PrimitiveExpr;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum UnaryOperation {
    Not(Box<PrimitiveExpr>),
}

impl UnaryOperation {
    pub fn new_not(value: impl Into<PrimitiveExpr>) -> Self {
        Self::Not(Box::new(value.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::ParseError,
        lexer::{Lexer, LexicalError},
    };

    #[test]
    fn not() {
        let (errs, ast) = test("!false");

        assert_eq!(errs, Vec::new());
        assert_eq!(ast, Ok(UnaryOperation::new_not(false).into()));
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<PrimitiveExpr, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::TernaryOperationParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
