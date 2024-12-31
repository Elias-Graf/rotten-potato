use crate::spanned::Spanned;

use super::{atom::Atom, symbol::Symbol};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefListen {
    pub keyword: Spanned<()>,
    pub name: Spanned<Symbol>,
    pub args: Vec<DefListenArg>,
    pub script: Spanned<String>,
}

impl DefListen {
    pub fn new(
        keyword: impl Into<Spanned<()>>,
        name: impl Into<Spanned<Symbol>>,
        args: Vec<impl Into<DefListenArg>>,
        script: impl Into<Spanned<String>>,
    ) -> Self {
        Self {
            keyword: keyword.into(),
            name: name.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
            script: script.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefListenArg {
    pub name: Spanned<Symbol>,
    pub value: DefListenArgValue,
}

impl DefListenArg {
    pub fn new(name: impl Into<Spanned<Symbol>>, value: impl Into<DefListenArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefListenArgValue {
    Atom(Spanned<Atom>),
}

impl From<Spanned<Atom>> for DefListenArgValue {
    fn from(value: Spanned<Atom>) -> Self {
        Self::Atom(value)
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use pretty_assertions::assert_eq;

    use crate::{
        ast::{
            atom::Atom,
            def_listen::{DefListen, DefListenArg},
            top_level_expr::TopLevelExpr,
            ParseError,
        },
        lexer::{Lexer, LexicalError},
        spanned::Spanned,
    };

    #[test]
    fn missing_name() {
        let (errs, ast) = test(r#"   (deflisten)   "#);

        expect![[r#"
            Ok(
                Spanned(
                    3,
                    Err,
                    14,
                ),
            )
        "#]]
        .assert_debug_eq(&ast);
        expect![[r#"
            [
                ExpectedDefListenName {
                    err_span: SourceSpan {
                        offset: SourceOffset(
                            3,
                        ),
                        length: 11,
                    },
                },
            ]
        "#]]
        .assert_debug_eq(&errs);
    }

    #[test]
    fn missing_script() {
        let (errs, ast) = test(r#"   (deflisten foo)   "#);

        expect![[r#"
            Ok(
                Spanned(
                    3,
                    Err,
                    18,
                ),
            )
        "#]]
        .assert_debug_eq(&ast);
        expect![[r#"
            [
                ExpectedDefListenScript {
                    err_span: SourceSpan {
                        offset: SourceOffset(
                            3,
                        ),
                        length: 15,
                    },
                },
            ]
        "#]]
        .assert_debug_eq(&errs);
    }

    #[test]
    fn name_and_args() {
        let (errs, ast) = test(
            r#"(deflisten foo :initial "whatever"
                "tail -F /tmp/some_file")"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                DefListen::new(
                    (1, (), 10),
                    (11, "foo".into(), 14),
                    vec![DefListenArg::new(
                        (16, "initial".into(), 23),
                        Spanned::from((24, Atom::from("whatever"), 34))
                    )],
                    (51, "tail -F /tmp/some_file".into(), 75)
                )
                .into(),
                76
            )
                .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<Spanned<TopLevelExpr>, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::DefListenParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
