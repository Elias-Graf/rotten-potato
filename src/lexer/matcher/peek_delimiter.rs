use super::PeekResult;
use crate::lexer::{matcher::PeekedToken, tok::Tok, Grapheme};

pub fn peek_delimiter<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    _: &'inp str,
) -> Option<PeekResult<'inp>> {
    let grapheme = graphemes.get(idx)?;

    log::trace!(
        "trying to interpret grapheme: {:?} as a delimiter",
        grapheme
    );

    let tok = match grapheme.1 {
        "(" => Tok::DelimiterLeftParen,
        ")" => Tok::DelimiterRightParen,
        "[" => Tok::DelimiterLeftBracket,
        "]" => Tok::DelimiterRightBracket,
        "," => Tok::DelimiterComma,
        _ => {
            log::trace!(
                "could not match '{}' as a delimiter... returning none",
                grapheme.1
            );
            return None;
        }
    };

    let span_from = grapheme.0;
    let span_to = grapheme.0 + grapheme.1.len();
    let peeked_token = PeekedToken::new(tok, span_from, span_to, 1);

    log::trace!("successfully matched: {:?} as a delimiter", peeked_token);

    Some(Ok(peeked_token))
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
            ("(", Tok::DelimiterLeftParen),
            (")", Tok::DelimiterRightParen),
            ("[", Tok::DelimiterLeftBracket),
            ("]", Tok::DelimiterRightBracket),
            (",", Tok::DelimiterComma),
        ] {
            let graphemes: Vec<_> = input.grapheme_indices(true).collect();

            assert_eq!(
                peek_delimiter(&graphemes, 0, input),
                Some(Ok(PeekedToken::new(expected.clone(), 0, 1, 1))),
                "input '{}' failed to be parsed to {:?}",
                input,
                expected
            );
        }
    }

    #[test]
    fn offsets_are_correctly_calculated_in_middle() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "   (    ";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_delimiter(&graphemes, 3, raw),
            Some(Ok(PeekedToken::new(Tok::DelimiterLeftParen, 3, 4, 1))),
        );
    }

    #[test]
    fn offsets_are_correctly_calculated_at_end() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "    (";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_delimiter(&graphemes, 4, raw),
            Some(Ok(PeekedToken::new(Tok::DelimiterLeftParen, 4, 5, 1))),
        );
    }
}
