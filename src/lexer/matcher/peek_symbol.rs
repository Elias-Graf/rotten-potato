use super::PeekResult;
use crate::lexer::{
    grapheme_is_whitespace,
    matcher::{peek_delimiter, PeekedToken},
    tok::Tok,
    Grapheme,
};

pub fn peek_symbol<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &'inp str,
) -> Option<PeekResult<'inp>> {
    let offset_graphemes_end = graphemes.get(idx..)?.iter().enumerate().position(|(o, g)| {
        grapheme_is_whitespace(g) || peek_delimiter(graphemes, idx + o, raw).is_some()
    });

    let idx_raw_from = graphemes.get(idx)?.0;
    let (idx_raw_to, offset_graphemes_end) = offset_graphemes_end
        .map(|o| (graphemes[idx + o].0, o))
        .unwrap_or((raw.len(), graphemes.len() - idx));

    let peeked_token = PeekedToken::new(
        Tok::LiteralSymbol(&raw[idx_raw_from..idx_raw_to]),
        idx_raw_from,
        idx_raw_to,
        offset_graphemes_end,
    );

    log::trace!("successfully matched: {:?} as a symbol", peeked_token);

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

        for (raw, expected) in vec![
            ("foo", PeekedToken::new(Tok::LiteralSymbol("foo"), 0, 3, 3)),
            (
                "bar123",
                PeekedToken::new(Tok::LiteralSymbol("bar123"), 0, 6, 6),
            ),
        ] {
            let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

            assert_eq!(
                peek_symbol(&graphemes, 0, raw),
                Some(Ok(expected.clone())),
                "input '{}' failed to be parsed to {:?}",
                raw,
                expected
            );
        }
    }

    #[test]
    fn offsets_are_correctly_calculated_in_middle() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "     baz     ";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_symbol(&graphemes, 5, raw),
            Some(Ok(PeekedToken::new(Tok::LiteralSymbol("baz"), 5, 8, 3))),
        );
    }

    #[test]
    fn offsets_are_correctly_calculated_at_end() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "     baz";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_symbol(&graphemes, 5, raw),
            Some(Ok(PeekedToken::new(Tok::LiteralSymbol("baz"), 5, 8, 3))),
        );
    }
}
