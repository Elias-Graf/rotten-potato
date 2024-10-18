#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Include {
    pub path: String,
}

impl Include {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        ast::{include::Include, top_level_expr::TopLevelExpr, ParseError},
        lexer::{Lexer, LexicalError},
    };

    #[test]
    pub fn missing_path() {
        let (errs, ast) = test(r#"(include)"#);

        assert_eq!(ast, Ok(TopLevelExpr::Err));
        assert_eq!(errs, Vec::new());
    }

    #[test]
    pub fn filename() {
        let (errs, ast) = test(r#"(include "my_file.yuck")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(ast, Ok(Include::new("my_file.yuck").into()));
    }

    #[test]
    pub fn path() {
        let (errs, ast) = test(r#"(include "./path/to/my_file.yuck")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(ast, Ok(Include::new("./path/to/my_file.yuck").into()));
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<TopLevelExpr, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::IncludeParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
