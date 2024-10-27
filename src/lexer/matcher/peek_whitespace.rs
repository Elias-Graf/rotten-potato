use crate::lexer::{grapheme_is_whitespace, Grapheme};

pub fn peek_whitespace<'inp>(graphemes: &[Grapheme<'inp>], idx: usize, raw: &'inp str) -> usize {
    log::trace!(
        "skipping potential whitespaces, starting at grapheme index: {}",
        idx
    );

    let offset = graphemes
        .get(idx..)
        .and_then(|remainder| remainder.iter().position(|g| !grapheme_is_whitespace(g)));

    if let Some(offset) = offset {
        let grapheme_end = graphemes[idx + offset];

        let idx_raw_from = graphemes[idx].0;
        let idx_raw_to = grapheme_end.0 + grapheme_end.1.len();

        log::trace!(
            "found {} whitespace-graphemes to skip ('{}')",
            offset,
            &raw[idx_raw_from..idx_raw_to]
        );

        return offset;
    }

    graphemes.len() - idx
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use unicode_segmentation::UnicodeSegmentation;

    use super::*;

    #[test]
    fn single_whitespace() {
        for raw in [" ", "\n", "\r\n", "\t"] {
            assert_eq!(test(raw, 0), 1, "failed to consume '{}' as whitespace", raw);
        }
    }

    #[test]
    fn multiple_whitespaces() {
        for (raw, expected) in [("   ", 3), ("\t\t\t", 3), (" \r\n\t", 3)] {
            assert_eq!(
                test(raw, 0),
                expected,
                "failed to consume '{}' as whitespace",
                raw
            );
        }
    }

    #[test]
    fn offsets_are_correctly_calculated_in_the_middle() {
        assert_eq!(test("true   false", 4), 3);
    }

    #[test]
    fn offsets_are_correctly_calculated_in_the_end() {
        assert_eq!(test("true  ", 4), 2);
    }

    fn test(raw: &str, idx: usize) -> usize {
        let _ = env_logger::builder().is_test(true).try_init();
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        peek_whitespace(&graphemes, idx, raw)
    }
}
