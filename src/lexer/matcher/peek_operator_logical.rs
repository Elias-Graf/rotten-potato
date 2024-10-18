use super::{PeekResult, PeekedToken};
use crate::lexer::{tok::Tok, Grapheme};

pub fn peek_operator_logical<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &'inp str,
) -> Option<PeekResult<'inp>> {
    if let Some(result) = peek_operator_logical_two_characters(graphemes, idx, raw) {
        return Some(result);
    }

    peek_operator_logical_single_character(graphemes, idx, raw)
}

fn peek_operator_logical_two_characters<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &str,
) -> Option<PeekResult<'inp>> {
    let [first, second]: &[_; 2] = graphemes.get(idx..idx + 2)?.try_into().ok()?;

    let idx_raw_from = first.0;
    let idx_raw_to = second.0 + second.1.len();

    let slice = &raw[idx_raw_from..idx_raw_to];

    log::trace!(
        "trying to interpret grapheme: {:?} as a logical operator with two characters",
        slice
    );

    let tok = match slice {
        "||" => Tok::OperatorLogicalOr,
        "&&" => Tok::OperatorLogicalAnd,
        _ => {
            log::trace!(
                "could not match '{}' as a logical operator with two characters... returning none",
                slice
            );
            return None;
        }
    };

    let peeked_tok = PeekedToken::new(tok, idx_raw_from, idx_raw_to, 2);
    log::debug!(
        "successfully interpreted: {:?} as a logical operator with two characters",
        peeked_tok
    );
    Some(Ok(peeked_tok))
}

fn peek_operator_logical_single_character<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &'inp str,
) -> Option<PeekResult<'inp>> {
    let grapheme = graphemes.get(idx)?;

    let idx_raw_from = grapheme.0;
    let idx_raw_to = grapheme.0 + grapheme.1.len();

    let slice = &raw[idx_raw_from..idx_raw_to];

    log::trace!(
        "trying to interpret grapheme: {:?} as a logical operator with a single character",
        slice
    );

    let tok = match slice {
        "!" => Tok::OperatorLogicalNot,
        _ => {
            log::trace!(
                "could not match '{}' as a logical operator with a single character... returning none",
                slice
            );
            return None;
        }
    };

    let peeked_tok = PeekedToken::new(tok, idx_raw_from, idx_raw_to, 1);
    log::debug!(
        "successfully interpreted: {:?} as a logical character with a single character",
        peeked_tok
    );
    Some(Ok(peeked_tok))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use unicode_segmentation::UnicodeSegmentation;

    use super::*;

    #[test]
    fn all_delimiters_are_correctly_identified() {
        let _ = env_logger::builder().is_test(true).try_init();

        for (input, expected) in vec![
            ("||", PeekedToken::new(Tok::OperatorLogicalOr, 0, 2, 2)),
            ("&&", PeekedToken::new(Tok::OperatorLogicalAnd, 0, 2, 2)),
            ("!", PeekedToken::new(Tok::OperatorLogicalNot, 0, 1, 1)),
        ] {
            let graphemes: Vec<_> = input.grapheme_indices(true).collect();

            assert_eq!(
                peek_operator_logical(&graphemes, 0, input),
                Some(Ok(expected.clone())),
                "input '{}' failed to be parsed to {:?}",
                input,
                expected
            );
        }
    }
}
