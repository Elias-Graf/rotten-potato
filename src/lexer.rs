use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

pub mod matcher;
pub mod tok;

pub use tok::*;

// TODO: Rework to struct containing shared properties and the variant.
// After that, remove unnecessary methods in impl.
#[derive(Clone, Debug, Error, Diagnostic, PartialEq, PartialOrd, Eq)]
pub enum LexicalError<'inp> {
    #[error("Unrecognized token")]
    UnrecognizedToken {
        #[source_code]
        src: &'inp str,
        #[label("The token '{char}' could be not consumed")]
        span: SourceSpan,
        char: &'inp str,
    },
    #[error("Unterminated string literal")]
    UnterminatedLiteralString {
        #[source_code]
        src: &'inp str,
        #[label("The string literal is missing its closing quote ('\"')")]
        span: SourceSpan,
    },
}

impl LexicalError<'_> {
    pub fn src(&self) -> &str {
        match self {
            LexicalError::UnrecognizedToken { src, .. } => src,
            LexicalError::UnterminatedLiteralString { src, .. } => src,
        }
    }

    pub fn span(&self) -> &SourceSpan {
        match self {
            LexicalError::UnrecognizedToken { span, .. } => span,
            LexicalError::UnterminatedLiteralString { span, .. } => span,
        }
    }
}

// TODO: What's the point of this? Can this just be removed?
impl<'inp> From<lalrpop_util::ParseError<usize, Tok<'inp>, LexicalError<'inp>>>
    for LexicalError<'inp>
{
    fn from(value: lalrpop_util::ParseError<usize, Tok<'inp>, LexicalError<'inp>>) -> Self {
        match value {
            lalrpop_util::ParseError::User { error } => error,
            // TODO: See if any of the lalrpop error need to be handled / should be converted to
            // lexical errors.
            unhandled => todo!("handle lalrpop error: {:?}", unhandled),
        }
    }
}

pub type Grapheme<'inp> = (usize, &'inp str);

pub struct Lexer<'inp> {
    raw: &'inp str,
    graphemes: Vec<(usize, &'inp str)>,
    curr_idx: usize,
}

impl<'inp> Iterator for Lexer<'inp> {
    type Item = LexerResult<'inp>;

    fn next(&mut self) -> Option<Self::Item> {
        let consumed_whitespaces =
            matcher::peek_whitespace(&self.graphemes, self.curr_idx, self.raw);
        if consumed_whitespaces > 0 {
            log::debug!("skipped {} whitespace graphemes", consumed_whitespaces);
            self.curr_idx += consumed_whitespaces;
        }

        if self.curr_idx >= self.graphemes.len() {
            log::debug!("reached end of grapheme vector, returning None");
            return None;
        }

        type MatcherFn =
            for<'inp> fn(&[Grapheme<'inp>], usize, &'inp str) -> Option<matcher::PeekResult<'inp>>;

        const MATCHERS: &[(&str, MatcherFn)] = &[
            ("keyword", matcher::peek_keyword),
            ("delimiter", matcher::peek_delimiter),
            ("punctuation", matcher::peek_punctuation),
            ("literal string", matcher::peek_literal_string),
            ("literal number", matcher::peek_literal_number),
            ("operator math", matcher::peek_operator_math),
            ("operator comparison", matcher::peek_operator_comparison),
            ("operator logical", matcher::peek_operator_logical),
            ("symbol", matcher::peek_symbol),
        ];

        log::debug!(
            "beginning to consume next token, grapheme index: {}, first grapheme: {:?}",
            self.curr_idx,
            self.graphemes[self.curr_idx]
        );

        for (name, fn_ptr) in MATCHERS {
            log::debug!("trying to consume input as {}", name);
            if let Some(res) = fn_ptr(&self.graphemes, self.curr_idx, self.raw) {
                match res {
                    Ok(peeked_tok) => {
                        let count_consumed_graphemes = peeked_tok.count_consumed_graphemes;
                        self.curr_idx += count_consumed_graphemes;

                        let lexer_result: LexerResult = peeked_tok.into();
                        log::debug!(
                            "consumed input as {:?} consisting of {} graphemes",
                            lexer_result,
                            count_consumed_graphemes
                        );
                        return Some(lexer_result);
                    }
                    Err(e) => return Some(Err(e)),
                }
            }
        }

        if let Some(char) = self.graphemes.get(self.curr_idx) {
            return Some(Err(LexicalError::UnrecognizedToken {
                src: self.raw,
                span: (char.0, 1).into(),
                char: char.1,
            }));
        };

        // TODO: Convert to lexer error
        panic!("unexpected end of input");
    }
}

impl<'inp> Lexer<'inp> {
    pub fn new(raw: &'inp str) -> Self {
        let input_grapheme_indices = raw.grapheme_indices(true).collect();

        log::debug!("lexer constructed with input: '{}'", raw);
        log::trace!(
            "constructed new lexer with the following graphemes: {:?}",
            input_grapheme_indices
        );

        Lexer {
            raw,
            curr_idx: 0,
            graphemes: input_grapheme_indices,
        }
    }
}

pub type LexerResult<'inp> = Result<(usize, Tok<'inp>, usize), LexicalError<'inp>>;

impl<'inp> From<matcher::PeekedToken<'inp>> for LexerResult<'inp> {
    fn from(value: matcher::PeekedToken<'inp>) -> Self {
        Ok((value.span_from, value.tok, value.span_to))
    }
}

pub fn grapheme_is_whitespace(grapheme: &Grapheme<'_>) -> bool {
    grapheme.1.chars().all(|c| c.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_whitespace() {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new("()\n   ");

        assert_eq!(
            lexer.collect::<Vec<_>>(),
            vec![
                Ok((0, Tok::DelimiterLeftParen, 1)),
                Ok((1, Tok::DelimiterRightParen, 2)),
            ]
        );
    }
}
