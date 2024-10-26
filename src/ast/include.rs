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
        spanned::Spanned,
    };

    #[test]
    pub fn missing_path() {
        let (errs, ast) = test(r#"(include)"#);

        assert_eq!(ast, Ok((0, TopLevelExpr::Err, 9).into()));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedIncludePath {
                err_span: (0, 9).into()
            }]
        );
    }

    #[test]
    pub fn filename() {
        let (errs, ast) = test(r#"(include "my_file.yuck")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(ast, Ok((0, Include::new("my_file.yuck").into(), 24).into()));
    }

    #[test]
    pub fn path() {
        let (errs, ast) = test(r#"(include "./path/to/my_file.yuck")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((0, Include::new("./path/to/my_file.yuck").into(), 34).into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<Spanned<TopLevelExpr>, LexicalError>) {
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
