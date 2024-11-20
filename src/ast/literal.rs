use crate::spanned::Spanned;

use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Literal {
    pub keyword: Spanned<()>,
    pub args: Vec<LiteralArg>,
}

impl Literal {
    pub fn new(keyword: impl Into<Spanned<()>>, args: Vec<impl Into<LiteralArg>>) -> Self {
        Self {
            keyword: keyword.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LiteralArg {
    pub name: Spanned<Symbol>,
    pub value: LiteralArgValue,
}

impl LiteralArg {
    pub fn new(name: impl Into<Spanned<Symbol>>, value: impl Into<LiteralArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LiteralArgValue {
    String(Spanned<String>),
    Symbol(Spanned<Symbol>),
}

impl From<Spanned<String>> for LiteralArgValue {
    fn from(value: Spanned<String>) -> Self {
        Self::String(value)
    }
}

impl From<Spanned<Symbol>> for LiteralArgValue {
    fn from(value: Spanned<Symbol>) -> Self {
        Self::Symbol(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        ast::{top_level_expr::TopLevelExpr, ParseError},
        lexer::{Lexer, LexicalError},
        spanned::Spanned,
    };

    #[test]
    fn content_string() {
        let (errs, ast) = test(r#"(literal :content "(button 'foo')")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                Literal::new(
                    (1, 8),
                    vec![LiteralArg::new(
                        (10, "content".into(), 17),
                        Spanned::from((18, "(button 'foo')".to_owned(), 34))
                    )]
                )
                .into(),
                35
            )
                .into())
        );
    }

    #[test]
    fn content_symbol() {
        let (errs, ast) = test(r#"(literal :content variable_containing_yuck)"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                Literal::new(
                    (1, 8),
                    vec![LiteralArg::new(
                        (10, "content".into(), 17),
                        Spanned::from((18, Symbol::new("variable_containing_yuck"), 42))
                    )]
                )
                .into(),
                43
            )
                .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<Spanned<TopLevelExpr>, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::LiteralParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
