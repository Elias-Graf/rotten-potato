use ast::{top_level_expr::TopLevelExpr, ParseError};
use lalrpop_util::lalrpop_mod;
use lexer::LexicalError;
use spanned::Spanned;

pub mod ast;
pub mod lexer;
pub mod spanned;

lalrpop_mod!(
    #[allow(clippy::all)]
    pub grammar
);

/// Parses (a list of) top level expressions, for example a complete file.
pub fn parse_top_level(
    inp: &str,
) -> Result<(Vec<Spanned<TopLevelExpr>>, Vec<ParseError>), LexicalError> {
    let lexer = lexer::Lexer::new(inp);
    let parser = grammar::TopLevelParser::new();
    let mut errs = Vec::new();

    let ast = parser.parse(&mut errs, lexer)?;

    Ok((ast, errs))
}
