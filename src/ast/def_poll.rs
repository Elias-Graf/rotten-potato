use crate::spanned::Spanned;

use super::atom::Atom;
use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefPoll {
    pub keyword: Spanned<()>,
    pub name: Spanned<Symbol>,
    pub args: Vec<DefPollArg>,
    pub script: Spanned<String>,
}

impl DefPoll {
    pub fn new(
        keyword: impl Into<Spanned<()>>,
        name: impl Into<Spanned<Symbol>>,
        args: Vec<impl Into<DefPollArg>>,
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
pub struct DefPollArg {
    pub name: Spanned<Symbol>,
    pub value: DefPollArgValue,
}

impl DefPollArg {
    pub fn new(name: impl Into<Spanned<Symbol>>, value: impl Into<DefPollArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefPollArgValue {
    Atom(Spanned<Atom>),
}

impl From<Spanned<Atom>> for DefPollArgValue {
    fn from(value: Spanned<Atom>) -> Self {
        Self::Atom(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        ast::{
            atom::Atom,
            def_poll::{DefPoll, DefPollArg},
            top_level_expr::TopLevelExpr,
            ParseError,
        },
        lexer::{Lexer, LexicalError},
        spanned::Spanned,
    };

    #[test]
    pub fn missing_name() {
        let (errs, ast) = test(r#"(defpoll)"#);

        assert_eq!(ast, Ok((0, TopLevelExpr::Err, 9).into()));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefPollName {
                err_span: (0, 9).into()
            }]
        );
    }

    #[test]
    pub fn missing_script() {
        let (errs, ast) = test(r#"(defpoll time)"#);

        assert_eq!(ast, Ok((0, TopLevelExpr::Err, 14).into()));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefPollScript {
                err_span: (0, 14).into()
            }]
        );
    }

    #[test]
    pub fn script_and_args() {
        // TODO: Variables in value
        //:run-while time-visible
        // TODO: string parser that works with all supported delimiters, e.g.:
        // `\``, `"`, `'`. Check specification.

        let (errs, ast) = test(
            r#"(defpoll time :interval "1s"
                :initial "initial-value"
                "date +%H:%M:%S")"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                DefPoll::new(
                    (1, (), 8),
                    (9, "time".into(), 13),
                    vec![
                        DefPollArg::new(
                            (15, "interval".into(), 23),
                            Spanned::from((24, Atom::from("1s"), 28))
                        ),
                        DefPollArg::new(
                            (46, "initial".into(), 53),
                            Spanned::from((54, Atom::from("initial-value"), 69))
                        )
                    ],
                    (86, "date +%H:%M:%S".into(), 102),
                )
                .into(),
                103
            )
                .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<Spanned<TopLevelExpr>, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::DefPollParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
