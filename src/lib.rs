use ast::{top_level_expr::TopLevelExpr, ParseError};
use lalrpop_util::lalrpop_mod;
use lexer::LexicalError;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub grammar);

/// Parses (a list of) top level expressions, for example a complete file.
pub fn parse_top_level<'inp>(
    inp: &'inp str,
) -> Result<(Vec<TopLevelExpr>, Vec<ParseError>), LexicalError<'inp>> {
    let lexer = lexer::Lexer::new(inp);
    let parser = grammar::TopLevelParser::new();
    let mut errs = Vec::new();

    let ast = parser.parse(&mut errs, lexer)?;

    Ok((ast, errs))
}
