use super::{expr::PrimitiveExpr, symbol::Symbol};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionCall {
    pub name: Symbol,
    pub args: Vec<PrimitiveExpr>,
}

impl FunctionCall {
    pub fn new(name: impl Into<Symbol>, args: Vec<PrimitiveExpr>) -> Self {
        Self {
            name: name.into(),
            args,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        ast::{atom::Atom, ParseError},
        lexer::{Lexer, LexicalError},
        spanned::Spanned,
    };

    #[test]
    fn strlength() {
        let (errs, ast) = test(r#"strlength("value")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                FunctionCall::new(Symbol::new("strlength"), vec!["value".into()]).into(),
                18
            )
                .into())
        );
    }

    #[test]
    fn substring() {
        let (errs, ast) = test(r#"substring("yuck", 1, 3)"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                FunctionCall::new(
                    Symbol::new("substring"),
                    vec![
                        "yuck".into(),
                        Atom::new_number("1").into(),
                        Atom::new_number("3").into()
                    ]
                )
                .into(),
                23
            )
                .into())
        );
    }

    #[test]
    fn nested() {
        let (errs, ast) = test(r#"strlength(trim("foo"))"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                FunctionCall::new(
                    Symbol::new("strlength"),
                    vec![FunctionCall::new(Symbol::new("trim"), vec!["foo".into()]).into()]
                )
                .into(),
                22
            )
                .into())
        );
    }

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
