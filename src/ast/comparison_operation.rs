use super::expr::PrimitiveExpr;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ComparisonOperator {
    Eq,
    Gt,
    Gte,
    Lt,
    Lte,
    Neq,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ComparisonOperation {
    lhs: Box<PrimitiveExpr>,
    op: ComparisonOperator,
    rhs: Box<PrimitiveExpr>,
}

impl ComparisonOperation {
    pub fn new(
        lhs: impl Into<PrimitiveExpr>,
        op: ComparisonOperator,
        rhs: impl Into<PrimitiveExpr>,
    ) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            op,
            rhs: Box::new(rhs.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        ast::{atom::Atom, function_call::FunctionCall, ParseError},
        lexer::{Lexer, LexicalError},
    };

    #[test]
    fn eq() {
        let (errs, ast) = test("5 == 5");

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(ComparisonOperation::new(
                Atom::new_number("5"),
                ComparisonOperator::Eq,
                Atom::new_number("5")
            )
            .into())
        );
    }

    #[test]
    fn neq() {
        let (errs, ast) = test("4 != 2");

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(ComparisonOperation::new(
                Atom::new_number("4"),
                ComparisonOperator::Neq,
                Atom::new_number("2")
            )
            .into())
        );
    }

    #[test]
    fn gt() {
        let (errs, ast) = test("25 > 99");

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(ComparisonOperation::new(
                Atom::new_number("25"),
                ComparisonOperator::Gt,
                Atom::new_number("99")
            )
            .into())
        );
    }

    #[test]
    fn lt() {
        let (errs, ast) = test("333 < 21");

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(ComparisonOperation::new(
                Atom::new_number("333"),
                ComparisonOperator::Lt,
                Atom::new_number("21")
            )
            .into())
        );
    }

    #[test]
    fn gte() {
        let (errs, ast) = test("8 >= 10");

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(ComparisonOperation::new(
                Atom::new_number("8"),
                ComparisonOperator::Gte,
                Atom::new_number("10")
            )
            .into())
        );
    }

    #[test]
    fn lte() {
        let (errs, ast) = test("1 <= 66");

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(ComparisonOperation::new(
                Atom::new_number("1"),
                ComparisonOperator::Lte,
                Atom::new_number("66")
            )
            .into())
        );
    }

    #[test]
    fn compound() {
        let (errs, ast) = test(r#"strlength("foo") > 5"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(ComparisonOperation::new(
                FunctionCall::new("strlength", vec![Atom::from("foo").into()]),
                ComparisonOperator::Gt,
                Atom::new_number("5")
            )
            .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<PrimitiveExpr, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::BinaryOperationParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
