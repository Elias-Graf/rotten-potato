use super::expr::PrimitiveExpr;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TernaryOperation {
    pub cond: Box<PrimitiveExpr>,
    pub true_branch: Box<PrimitiveExpr>,
    pub false_branch: Box<PrimitiveExpr>,
}

impl TernaryOperation {
    pub fn new(
        cond: impl Into<PrimitiveExpr>,
        true_branch: impl Into<PrimitiveExpr>,
        false_branch: impl Into<PrimitiveExpr>,
    ) -> Self {
        Self {
            cond: Box::new(cond.into()),
            true_branch: Box::new(true_branch.into()),
            false_branch: Box::new(false_branch.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::ast::atom::Atom;
    use crate::{
        ast::{
            comparison_operation::{ComparisonOperation, ComparisonOperator},
            function_call::FunctionCall,
            ParseError,
        },
        lexer::{Lexer, LexicalError},
    };

    #[test]
    fn simple() {
        let (_, ast) = test(r#"true ? 5 : 6"#);

        assert_eq!(
            ast,
            Ok(TernaryOperation::new(true, Atom::new_number("5"), Atom::new_number("6")).into())
        );
    }

    #[test]
    fn with_exprs() {
        let (_, ast) = test(r#"strlength(trim("hello")) > 0 ? "content" : "no content""#);

        assert_eq!(
            ast,
            Ok(TernaryOperation::new(
                ComparisonOperation::new(
                    (
                        0,
                        FunctionCall::new(
                            "strlength",
                            vec![FunctionCall::new("trim", vec!["hello".into()]).into()]
                        )
                        .into(),
                        0
                    ),
                    ComparisonOperator::Gt,
                    (27, Atom::new_number("0").into(), 28),
                ),
                "content",
                "no content"
            )
            .into())
        );
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
