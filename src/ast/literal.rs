use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Literal {
    pub args: Vec<LiteralArg>,
}

impl Literal {
    pub fn new(args: Vec<impl Into<LiteralArg>>) -> Self {
        Self {
            args: args.into_iter().map(|a| a.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LiteralArg {
    pub name: Symbol,
    pub value: LiteralArgValue,
}

impl LiteralArg {
    pub fn new(name: impl Into<Symbol>, value: impl Into<LiteralArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LiteralArgValue {
    String(String),
    Symbol(Symbol),
}

impl From<String> for LiteralArgValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Symbol> for LiteralArgValue {
    fn from(value: Symbol) -> Self {
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
    };

    #[test]
    fn content_string() {
        let (errs, ast) = test(r#"(literal :content "(button 'foo')")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(Literal::new(vec![LiteralArg::new(
                "content",
                "(button 'foo')".to_owned()
            )])
            .into())
        );
    }

    #[test]
    fn content_symbol() {
        let (errs, ast) = test(r#"(literal :content variable_containing_yuck)"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(Literal::new(vec![LiteralArg::new(
                "content",
                Symbol::new("variable_containing_yuck")
            )])
            .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<TopLevelExpr, LexicalError>) {
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
