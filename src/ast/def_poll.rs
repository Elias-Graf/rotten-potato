use super::atom::Atom;
use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefPoll {
    pub name: Symbol,
    pub args: Vec<DefPollArg>,
    pub script: String,
}

impl DefPoll {
    pub fn new(name: impl Into<Symbol>, args: Vec<impl Into<DefPollArg>>, script: String) -> Self {
        Self {
            name: name.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
            script,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefPollArg {
    pub name: Symbol,
    pub value: DefPollArgValue,
}

impl DefPollArg {
    pub fn new(name: impl Into<Symbol>, value: impl Into<DefPollArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefPollArgValue {
    Atom(Atom),
}

impl From<Atom> for DefPollArgValue {
    fn from(value: Atom) -> Self {
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
    };

    #[test]
    pub fn missing_name() {
        let (errs, ast) = test(r#"(defpoll)"#);

        assert_eq!(ast, Ok(TopLevelExpr::Err));
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

        assert_eq!(ast, Ok(TopLevelExpr::Err));
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
            r#"
            (defpoll time :interval "1s"
              :initial "initial-value"
              "date +%H:%M:%S")"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(DefPoll::new(
                "time",
                vec![
                    DefPollArg::new("interval", Atom::from("1s")),
                    DefPollArg::new("initial", Atom::from("initial-value"))
                ],
                "date +%H:%M:%S".to_owned(),
            )
            .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<TopLevelExpr, LexicalError>) {
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
