#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Symbol(pub String);

impl Symbol {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        ast::ParseError,
        lexer::{Lexer, LexicalError},
    };

    #[test]
    fn symbols_are_parsed() {
        for input in vec!["foo", "bar123"] {
            let (errs, ast) = test(input);

            assert_eq!(errs, Vec::new());
            assert_eq!(ast, Ok(input.into()))
        }
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<Symbol, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::SymbolParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
