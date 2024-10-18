use super::PeekResult;
use crate::lexer::{matcher::PeekedToken, tok::Tok, Grapheme, LexicalError};

pub fn peek_literal_string<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &'inp str,
) -> Option<PeekResult<'inp>> {
    let quote_open = graphemes.get(idx)?;

    if quote_open.1 != "\"" {
        return None;
    }

    let Some(offset_grapheme_quote_close) = graphemes
        .get(idx + 1..)?
        .iter()
        .position(|g| g.1 == "\"")
        .map(|o| o + 1)
    else {
        return Some(Err(LexicalError::UnterminatedLiteralString {
            src: raw,
            span: (idx, raw.len() - idx).into(),
        }));
    };

    let quote_close = graphemes[idx + offset_grapheme_quote_close];

    log::trace!(">>> {:?}", quote_close);

    let idx_consumed_from = graphemes[idx].0;
    let idx_consumed_to = quote_close.0 + quote_close.1.len();

    let char_first = graphemes[idx + 1];
    let char_last = graphemes[idx + offset_grapheme_quote_close - 1];

    let idx_slice_from = char_first.0;
    let idx_slice_to = char_last.0 + char_last.1.len();

    let slice = &raw[idx_slice_from..idx_slice_to];

    log::trace!(
        "constructing string out of {} graphemes, indexes: {} to {}, value: '{}', (indexes with quotes: {} to {})",
        offset_grapheme_quote_close,
        idx_slice_from,
        idx_slice_to,
        slice,
        idx_consumed_from,
        idx_consumed_to
    );

    let peeked_token = PeekedToken::new(
        Tok::LiteralString(slice),
        idx_consumed_from,
        idx_consumed_to,
        offset_grapheme_quote_close + 1,
    );

    Some(Ok(peeked_token))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use unicode_segmentation::UnicodeSegmentation;

    use super::*;

    #[test]
    fn unterminated() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = r#"   "unterminated"#;
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_string(&graphemes, 3, raw),
            Some(Err(LexicalError::UnterminatedLiteralString {
                src: raw,
                span: (3, 13).into()
            }))
        );
    }

    #[test]
    fn correctly_identified_and_remove_quotes() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = r#""hello world""#;
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_string(&graphemes, 0, raw),
            Some(Ok(PeekedToken::new(
                Tok::LiteralString("hello world"),
                0,
                13,
                13,
            )))
        );
    }

    #[test]
    fn empty() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = r#""""#;
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_string(&graphemes, 0, raw),
            Some(Ok(PeekedToken::new(Tok::LiteralString(""), 0, 2, 2))),
        );
    }

    #[test]
    fn offsets_are_correctly_calculated_in_the_middle() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = r#"  "foo" "#;

        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_string(&graphemes, 2, raw),
            Some(Ok(PeekedToken::new(Tok::LiteralString("foo"), 2, 7, 5))),
        );
    }

    #[test]
    fn offsets_are_correctly_calculated_at_end() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = r#"     "foo""#;
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_literal_string(&graphemes, 5, raw),
            Some(Ok(PeekedToken::new(Tok::LiteralString("foo"), 5, 10, 5))),
        );
    }
}
