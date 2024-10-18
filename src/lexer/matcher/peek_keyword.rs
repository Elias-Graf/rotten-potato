use super::PeekResult;
use crate::lexer::{
    grapheme_is_whitespace,
    matcher::{peek_delimiter, PeekedToken},
    tok::Tok,
    Grapheme,
};

pub fn peek_keyword<'inp>(
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

    let slice = &raw[idx_raw_from..idx_raw_to];

    log::trace!(
        "trying to interpret slice: '{}', grapheme {} to +{} as keyword",
        slice,
        idx,
        offset_graphemes_end
    );

    let tok = match slice {
        "include" => Tok::KeywordInclude,
        "true" => Tok::KeywordTrue,
        "false" => Tok::KeywordFalse,
        "deflisten" => Tok::KeywordDefListen,
        "defpoll" => Tok::KeywordDefPoll,
        "defvar" => Tok::KeywordDefVar,
        "defwidget" => Tok::KeywordDefWidget,
        "defwindow" => Tok::KeywordDefWindow,
        "literal" => Tok::KeywordLiteral,
        unknown => {
            log::trace!("failed to match '{}' as a keyword", unknown);
            return None;
        }
    };

    let peeked_token = PeekedToken::new(tok, idx_raw_from, idx_raw_to, offset_graphemes_end);

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
            ("include", PeekedToken::new(Tok::KeywordInclude, 0, 7, 7)),
            ("true", PeekedToken::new(Tok::KeywordTrue, 0, 4, 4)),
            ("false", PeekedToken::new(Tok::KeywordFalse, 0, 5, 5)),
            (
                "deflisten",
                PeekedToken::new(Tok::KeywordDefListen, 0, 9, 9),
            ),
            ("defpoll", PeekedToken::new(Tok::KeywordDefPoll, 0, 7, 7)),
            ("defvar", PeekedToken::new(Tok::KeywordDefVar, 0, 6, 6)),
            (
                "defwidget",
                PeekedToken::new(Tok::KeywordDefWidget, 0, 9, 9),
            ),
            (
                "defwindow",
                PeekedToken::new(Tok::KeywordDefWindow, 0, 9, 9),
            ),
            ("literal", PeekedToken::new(Tok::KeywordLiteral, 0, 7, 7)),
        ] {
            let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

            assert_eq!(
                peek_keyword(&graphemes, 0, raw),
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

        let raw = "     true     ";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_keyword(&graphemes, 5, raw),
            Some(Ok(PeekedToken::new(Tok::KeywordTrue, 5, 9, 4))),
        );
    }

    #[test]
    fn offsets_are_correctly_calculated_at_end() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "     true";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_keyword(&graphemes, 5, raw),
            Some(Ok(PeekedToken::new(Tok::KeywordTrue, 5, 9, 4))),
        );
    }
}
