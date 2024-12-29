use ast::{top_level_expr::TopLevelExpr, ParseError};
use lalrpop_util::lalrpop_mod;
use lexer::{tok::Tok, LexicalError};
use spanned::Spanned;

pub mod ast;
pub mod lexer;
pub mod spanned;

lalrpop_mod!(
    #[allow(clippy::all)]
    pub grammar
);

pub fn parse_top_level<'inp>(
    tokens: Vec<(usize, Tok<'inp>, usize)>,
) -> Result<(Vec<Spanned<TopLevelExpr>>, Vec<ParseError>), LexicalError<'inp>> {
    let parser = grammar::TopLevelParser::new();
    let mut errs = Vec::new();

    match parser.parse(&mut errs, tokens) {
        Ok(nodes) => Ok((nodes, errs)),
        Err(err) => match err {
            lalrpop_util::ParseError::User { error } => Err(error),
            e => {
                // LALRPOP should generally not fail, all the problems should be caught by the
                // grammar and or processing.
                log::error!("unexpected parsing error: {:?}", e);
                Ok((Vec::new(), errs))
            }
        },
    }
}
