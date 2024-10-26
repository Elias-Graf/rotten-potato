use crate::spanned::Spanned;

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
    pub lhs: Box<Spanned<PrimitiveExpr>>,
    pub op: BinaryOperator,
    pub rhs: Box<Spanned<PrimitiveExpr>>,
}

impl BinaryOperation {
    pub fn new(
        lhs: impl Into<Spanned<PrimitiveExpr>>,
        op: BinaryOperator,
        rhs: impl Into<Spanned<PrimitiveExpr>>,
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
                Ok((
                    0,
                    BinaryOperation::new(
                        (0, false.into(), 5),
                        BinaryOperator::Or,
                        (9, true.into(), 13)
                    )
                    .into(),
                    13
                )
                    .into())
            );
        }

        #[test]
        fn and() {
            let (_, ast) = test("true && false");

            assert_eq!(
                ast,
                Ok((
                    0,
                    BinaryOperation::new(
                        (0, true.into(), 4),
                        BinaryOperator::And,
                        (8, false.into(), 13)
                    )
                    .into(),
                    13
                )
                    .into())
            );
        }

        #[test]
        fn precedence() {
            let (_, ast) = test("false || true && !false");

            assert_eq!(
                ast,
                Ok((
                    0,
                    BinaryOperation::new(
                        (0, false.into(), 5),
                        BinaryOperator::Or,
                        (
                            9,
                            BinaryOperation::new(
                                (9, true.into(), 13),
                                BinaryOperator::And,
                                (0, UnaryOperation::new_not(false).into(), 0),
                            )
                            .into(),
                            23
                        )
                    )
                    .into(),
                    23
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
                Ok((
                    0,
                    BinaryOperation::new(
                        (0, Atom::new_number("10").into(), 2),
                        BinaryOperator::Add,
                        (5, Atom::new_number("10").into(), 7),
                    )
                    .into(),
                    7
                )
                    .into())
            );
        }

        #[test]
        fn sub() {
            let (_, ast) = test("42 - 5");

            assert_eq!(
                ast,
                Ok((
                    0,
                    BinaryOperation::new(
                        (0, Atom::new_number("42").into(), 2),
                        BinaryOperator::Sub,
                        (5, Atom::new_number("5").into(), 6),
                    )
                    .into(),
                    6
                )
                    .into())
            );
        }

        #[test]
        fn div() {
            let (_, ast) = test("9 / 9");

            assert_eq!(
                ast,
                Ok((
                    0,
                    BinaryOperation::new(
                        (0, Atom::new_number("9").into(), 1),
                        BinaryOperator::Div,
                        (4, Atom::new_number("9").into(), 5),
                    )
                    .into(),
                    5
                )
                    .into())
            );
        }

        #[test]
        fn mul() {
            let (_, ast) = test("2 * 6");

            assert_eq!(
                ast,
                Ok((
                    0,
                    BinaryOperation::new(
                        (0, Atom::new_number("2").into(), 1),
                        BinaryOperator::Mul,
                        (4, Atom::new_number("6").into(), 5),
                    )
                    .into(),
                    5
                )
                    .into())
            );
        }

        #[test]
        fn r#mod() {
            let (_, ast) = test("10 % 5");

            assert_eq!(
                ast,
                Ok((
                    0,
                    BinaryOperation::new(
                        (0, Atom::new_number("10").into(), 2),
                        BinaryOperator::Mod,
                        (5, Atom::new_number("5").into(), 6),
                    )
                    .into(),
                    6
                )
                    .into())
            );
        }

        #[test]
        fn precedence() {
            let (_, ast) = test("5 + 10 * 10 - 5 / 2");

            assert_eq!(
                ast,
                Ok((
                    0,
                    BinaryOperation::new(
                        (
                            0,
                            BinaryOperation::new(
                                (0, Atom::new_number("5").into(), 1),
                                BinaryOperator::Add,
                                (
                                    4,
                                    BinaryOperation::new(
                                        (4, Atom::new_number("10").into(), 6),
                                        BinaryOperator::Mul,
                                        (9, Atom::new_number("10").into(), 11),
                                    )
                                    .into(),
                                    11
                                ),
                            )
                            .into(),
                            11
                        ),
                        BinaryOperator::Sub,
                        (
                            14,
                            BinaryOperation::new(
                                (14, Atom::new_number("5").into(), 15),
                                BinaryOperator::Div,
                                (18, Atom::new_number("2").into(), 19),
                            )
                            .into(),
                            19
                        ),
                    )
                    .into(),
                    19
                )
                    .into())
            );
        }
    }

    // TODO: Add global parenthesis support

    fn test(
        inp: &str,
    ) -> (
        Vec<ParseError>,
        Result<Spanned<PrimitiveExpr>, LexicalError>,
    ) {
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
