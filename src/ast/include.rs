use crate::spanned::Spanned;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Include {
    pub keyword: Spanned<()>,
    pub path: Spanned<String>,
}

impl Include {
    pub fn new(keyword: impl Into<Spanned<()>>, path: impl Into<Spanned<String>>) -> Self {
        Self {
            keyword: keyword.into(),
            path: path.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use pretty_assertions::assert_eq;

    use crate::{
        ast::{include::Include, top_level_expr::TopLevelExpr, ParseError},
        lexer::{Lexer, LexicalError},
        spanned::Spanned,
    };

    #[test]
    pub fn missing_path() {
        let (errs, ast) = test(r#"   (include)   "#);

        expect![[r#"
            Ok(
                Spanned(
                    3,
                    Err,
                    12,
                ),
            )
        "#]]
        .assert_debug_eq(&ast);
        expect![[r#"
            [
                ExpectedIncludePath {
                    err_span: SourceSpan {
                        offset: SourceOffset(
                            3,
                        ),
                        length: 9,
                    },
                },
            ]
        "#]]
        .assert_debug_eq(&errs);
    }

    #[test]
    pub fn filename() {
        let (errs, ast) = test(r#"(include "my_file.yuck")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                Include::new((1, 8), (9, "my_file.yuck".into(), 23)).into(),
                24
            )
                .into())
        );
    }

    #[test]
    pub fn path() {
        let (errs, ast) = test(r#"(include "./path/to/my_file.yuck")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                Include::new((1, 8), (9, "./path/to/my_file.yuck".into(), 33)).into(),
                34
            )
                .into())
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
