use super::{tok::Tok, LexicalError};

mod peek_comment;
mod peek_delimiter;
mod peek_keyword;
mod peek_literal_number;
mod peek_literal_string;
mod peek_operator_comparison;
mod peek_operator_logical;
mod peek_operator_math;
mod peek_punctuation;
mod peek_symbol;
mod peek_whitespace;

pub use peek_comment::*;
pub use peek_delimiter::*;
pub use peek_keyword::*;
pub use peek_literal_number::*;
pub use peek_literal_string::*;
pub use peek_operator_comparison::*;
pub use peek_operator_logical::*;
pub use peek_operator_math::*;
pub use peek_punctuation::*;
pub use peek_symbol::*;
pub use peek_whitespace::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PeekedToken<'inp> {
    pub span_from: usize,
    pub span_to: usize,
    pub tok: Tok<'inp>,
    pub count_consumed_graphemes: usize,
}

impl<'inp> PeekedToken<'inp> {
    pub fn new(tok: Tok<'inp>, span_from: usize, span_to: usize, consumed: usize) -> Self {
        Self {
            span_from,
            span_to,
            tok,
            count_consumed_graphemes: consumed,
        }
    }
}

pub type PeekResult<'inp> = Result<PeekedToken<'inp>, LexicalError<'inp>>;
