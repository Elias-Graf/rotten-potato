use super::PeekResult;
use crate::lexer::{matcher::PeekedToken, tok::Tok, Grapheme};

pub fn peek_operator_comparison<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &'inp str,
) -> Option<PeekResult<'inp>> {
    if let Some(operator) = peek_operator_comparison_with_two_characters(graphemes, idx, raw) {
        return Some(operator);
    };

    peek_operator_comparison_with_single_character(graphemes, idx, raw)
}

fn peek_operator_comparison_with_two_characters<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &str,
) -> Option<PeekResult<'inp>> {
    let [first, second]: &[_; 2] = graphemes.get(idx..idx + 2)?.try_into().ok()?;

    let idx_raw_from = first.0;
    let idx_raw_to = second.0 + second.1.len();

    let slice = &raw[idx_raw_from..idx_raw_to];

    log::trace!(
        "trying to interpret grapheme: {:?} as a comparison operator with two characters",
        slice
    );

    let tok = match slice {
        "==" => Tok::OperatorComparisonEqual,
        "!=" => Tok::OperatorComparisonNotEqual,
        "<=" => Tok::OperatorComparisonLessThanOrEquals,
        ">=" => Tok::OperatorComparisonGreaterThanOrEquals,
        _ => {
            log::trace!(
                "could not match '{}' as a comparison operator with two characters... returning none",
                slice
            );
            return None;
        }
    };

    log::debug!(
        "successfully interpreted: {:?} as a comparison operator with two characters",
        tok
    );

    Some(Ok(PeekedToken::new(tok, idx_raw_from, idx_raw_to, 2)))
}

fn peek_operator_comparison_with_single_character<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &str,
) -> Option<PeekResult<'inp>> {
    let grapheme = graphemes.get(idx)?;

    let idx_raw_from = grapheme.0;
    let idx_raw_to = grapheme.0 + grapheme.1.len();

    let slice = &raw[idx_raw_from..idx_raw_to];

    log::trace!(
        "trying to interpret grapheme: {:?} as a comparison operator with a single character",
        slice
    );

    let tok = match slice {
        ">" => Tok::OperatorComparisonGreaterThan,
        "<" => Tok::OperatorComparisonLessThan,
        _ => {
            log::trace!(
                "could not match '{}' as a comparison operator with a single character... returning none",
                slice
            );
            return None;
        }
    };

    log::debug!(
        "successfully interpreted: {:?} as a comparison operator with a signle character",
        tok
    );

    Some(Ok(PeekedToken::new(tok, idx_raw_from, idx_raw_to, 1)))
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
            (
                "==",
                PeekedToken::new(Tok::OperatorComparisonEqual, 0, 2, 2),
            ),
            (
                "!=",
                PeekedToken::new(Tok::OperatorComparisonNotEqual, 0, 2, 2),
            ),
            (
                ">=",
                PeekedToken::new(Tok::OperatorComparisonGreaterThanOrEquals, 0, 2, 2),
            ),
            (
                "<=",
                PeekedToken::new(Tok::OperatorComparisonLessThanOrEquals, 0, 2, 2),
            ),
            (
                ">",
                PeekedToken::new(Tok::OperatorComparisonGreaterThan, 0, 1, 1),
            ),
            (
                "<",
                PeekedToken::new(Tok::OperatorComparisonLessThan, 0, 1, 1),
            ),
        ] {
            let graphemes: Vec<_> = input.grapheme_indices(true).collect();

            assert_eq!(
                peek_operator_comparison(&graphemes, 0, input),
                Some(Ok(expected.clone())),
                "input '{}' failed to be parsed to {:?}",
                input,
                expected
            );
        }
    }
}
