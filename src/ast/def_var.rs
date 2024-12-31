use crate::spanned::Spanned;

use super::atom::Atom;
use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefVar {
    pub keyword: Spanned<()>,
    pub name: Spanned<Symbol>,
    pub value: DefVarValue,
}

impl DefVar {
    pub fn new(
        keyword: impl Into<Spanned<()>>,
        name: impl Into<Spanned<Symbol>>,
        value: impl Into<DefVarValue>,
    ) -> Self {
        Self {
            keyword: keyword.into(),
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefVarValue {
    Atom(Spanned<Atom>),
}

impl From<Spanned<Atom>> for DefVarValue {
    fn from(value: Spanned<Atom>) -> Self {
        Self::Atom(value)
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use pretty_assertions::assert_eq;

    use crate::{
        ast::{atom::Atom, def_var::DefVar, top_level_expr::TopLevelExpr, ParseError},
        lexer::{Lexer, LexicalError},
        spanned::Spanned,
    };

    #[test]
    fn missing_name() {
        let (errs, ast) = test(r#"   (defvar)   "#);

        expect![[r#"
            Ok(
                Spanned(
                    3,
                    Err,
                    11,
                ),
            )
        "#]]
        .assert_debug_eq(&ast);
        expect![[r#"
            [
                ExpectedDefVarName {
                    err_span: SourceSpan {
                        offset: SourceOffset(
                            3,
                        ),
                        length: 8,
                    },
                },
            ]
        "#]]
        .assert_debug_eq(&errs);
    }

    #[test]
    fn missing_value() {
        let (errs, ast) = test(r#"   (defvar foo)   "#);

        expect![[r#"
            Ok(
                Spanned(
                    3,
                    Err,
                    15,
                ),
            )
        "#]]
        .assert_debug_eq(&ast);
        expect![[r#"
            [
                ExpectedDefVarValue {
                    err_span: SourceSpan {
                        offset: SourceOffset(
                            3,
                        ),
                        length: 12,
                    },
                },
            ]
        "#]]
        .assert_debug_eq(&errs);
    }

    #[test]
    fn atom_value() {
        let (errs, ast) = test(r#"(defvar foo "some value")"#);

        assert_eq!(errs, Vec::<ParseError>::new(),);
        assert_eq!(
            ast,
            Ok((
                0,
                DefVar::new(
                    (1, (), 7),
                    (8, "foo".into(), 11),
                    Spanned::from((12, Atom::from("some value").into(), 24))
                )
                .into(),
                25
            )
                .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<Spanned<TopLevelExpr>, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::DefVarParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
