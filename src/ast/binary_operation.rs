use super::expr::PrimitiveExpr;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum BinaryOperator {
    And,
    Or,

    Add,
    Div,
    Mod,
    Mul,
    Sub,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BinaryOperation {
    pub lhs: Box<PrimitiveExpr>,
    pub op: BinaryOperator,
    pub rhs: Box<PrimitiveExpr>,
}

impl BinaryOperation {
    pub fn new(
        lhs: impl Into<PrimitiveExpr>,
        op: BinaryOperator,
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
    use super::*;
    use crate::{
        ast::ParseError,
        lexer::{Lexer, LexicalError},
    };

    mod logical {
        use pretty_assertions::assert_eq;

        use super::*;
        use crate::ast::unary_operation::UnaryOperation;

        #[test]
        fn or() {
            let (_, ast) = test("false || true");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(false, BinaryOperator::Or, true).into())
            );
        }

        #[test]
        fn and() {
            let (_, ast) = test("true && false");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(true, BinaryOperator::And, false).into())
            );
        }

        #[test]
        fn precedence() {
            let (_, ast) = test("false || true && !false");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(
                    false,
                    BinaryOperator::Or,
                    BinaryOperation::new(true, BinaryOperator::And, UnaryOperation::new_not(false))
                )
                .into())
            );
        }
    }

    mod mathematical {
        use pretty_assertions::assert_eq;

        use super::*;
        use crate::ast::atom::Atom;

        #[test]
        fn add() {
            let (_, ast) = test("10 + 10");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(
                    Atom::new_number("10"),
                    BinaryOperator::Add,
                    Atom::new_number("10")
                )
                .into())
            );
        }

        #[test]
        fn sub() {
            let (_, ast) = test("42 - 5");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(
                    Atom::new_number("42"),
                    BinaryOperator::Sub,
                    Atom::new_number("5")
                )
                .into())
            );
        }

        #[test]
        fn div() {
            let (_, ast) = test("9 / 9");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(
                    Atom::new_number("9"),
                    BinaryOperator::Div,
                    Atom::new_number("9")
                )
                .into())
            );
        }

        #[test]
        fn mul() {
            let (_, ast) = test("2 * 6");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(
                    Atom::new_number("2"),
                    BinaryOperator::Mul,
                    Atom::new_number("6")
                )
                .into())
            );
        }

        #[test]
        fn r#mod() {
            let (_, ast) = test("10 % 5");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(
                    Atom::new_number("10"),
                    BinaryOperator::Mod,
                    Atom::new_number("5")
                )
                .into())
            );
        }

        #[test]
        fn precedence() {
            let (_, ast) = test("5 + 10 * 10 - 5 / 2");

            assert_eq!(
                ast,
                Ok(BinaryOperation::new(
                    BinaryOperation::new(
                        Atom::new_number("5"),
                        BinaryOperator::Add,
                        BinaryOperation::new(
                            Atom::new_number("10"),
                            BinaryOperator::Mul,
                            Atom::new_number("10"),
                        ),
                    ),
                    BinaryOperator::Sub,
                    BinaryOperation::new(
                        Atom::new_number("5"),
                        BinaryOperator::Div,
                        Atom::new_number("2"),
                    ),
                )
                .into())
            );
        }
    }

    // TODO: Add global parenthesis support

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
