use super::{atom::Atom, symbol::Symbol};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefListen {
    pub name: Symbol,
    pub args: Vec<DefListenArg>,
    pub script: String,
}

impl DefListen {
    pub fn new(
        name: impl Into<Symbol>,
        args: Vec<impl Into<DefListenArg>>,
        script: String,
    ) -> Self {
        Self {
            name: name.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
            script,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefListenArg {
    pub name: Symbol,
    pub value: DefListenArgValue,
}

impl DefListenArg {
    pub fn new(name: impl Into<Symbol>, value: impl Into<DefListenArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefListenArgValue {
    Atom(Atom),
}

impl From<Atom> for DefListenArgValue {
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
            def_listen::{DefListen, DefListenArg},
            top_level_expr::TopLevelExpr,
            ParseError,
        },
        lexer::{Lexer, LexicalError},
        spanned::Spanned,
    };

    #[test]
    fn missing_name() {
        let (errs, ast) = test(r#"(deflisten)"#);

        assert_eq!(ast, Ok((0, TopLevelExpr::Err, 11).into()));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefListenName {
                err_span: (0, 11).into()
            }]
        );
    }

    #[test]
    fn missing_script() {
        let (errs, ast) = test(r#"(deflisten foo)"#);

        assert_eq!(ast, Ok((0, TopLevelExpr::Err, 15).into()));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefListenScript {
                err_span: (0, 15).into()
            }]
        );
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
                    "foo",
                    vec![DefListenArg::new("initial", Atom::from("whatever"))],
                    "tail -F /tmp/some_file".to_owned()
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
