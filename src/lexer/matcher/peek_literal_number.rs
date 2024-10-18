use crate::lexer::{
    matcher::{self},
    tok::Tok,
    Grapheme,
};

pub fn peek_literal_number<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &'inp str,
) -> Option<matcher::PeekResult<'inp>> {
    let digit_first = graphemes.get(idx)?;

    if !grapheme_is_digit(digit_first) {
        return None;
    }

    let offset_next_non_digit = graphemes
        .get(idx + 1..)
        .and_then(|reminder| reminder.iter().position(|g| !grapheme_is_digit(g)))
        .map(|o| o + 1);

    let idx_inp_start = digit_first.0;
    let (idx_inp_end, offset) = offset_next_non_digit
        .map(|o| (graphemes[idx + o].0, o))
        .unwrap_or((raw.len(), graphemes.len() - idx));

    Some(Ok(matcher::PeekedToken::new(
        Tok::LiteralNumber(&raw[idx_inp_start..idx_inp_end]),
        idx_inp_start,
        idx_inp_end,
        offset,
    )))
}

fn grapheme_is_digit(grapheme: &Grapheme<'_>) -> bool {
    let chars: Vec<_> = grapheme.1.chars().collect();

    if chars.len() > 1 {
        // Multi-symbol graphemes, or even grapheme clusters, are not considered digits.
        return false;
    }

    if let Some(char) = chars.first() {
        return char.is_ascii_digit();
    }

    return false;
}

#[cfg(test)]
mod tests {
    use matcher::PeekedToken;
    use pretty_assertions::assert_eq;
    use unicode_segmentation::UnicodeSegmentation;

    use super::*;

    #[test]
    fn single_digit() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "8";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_number(&graphemes, 0, raw),
            Some(Ok(PeekedToken::new(Tok::LiteralNumber("8"), 0, 1, 1)))
        );
    }

    #[test]
    fn multiple_digits() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "987654321";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_number(&graphemes, 0, raw),
            Some(Ok(PeekedToken::new(
                Tok::LiteralNumber("987654321"),
                0,
                9,
                9
            )))
        );
    }

    #[test]
    fn offsets_are_correctly_calculated_in_the_middle() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "    3 ";

        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_number(&graphemes, 4, raw),
            Some(Ok(PeekedToken::new(Tok::LiteralNumber("3"), 4, 5, 1)))
        );
    }

    #[test]
    fn offsets_are_correctly_calculated_at_end() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "  0";

        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_number(&graphemes, 2, raw),
            Some(Ok(PeekedToken::new(Tok::LiteralNumber("0"), 2, 3, 1)))
        );
    }
}
