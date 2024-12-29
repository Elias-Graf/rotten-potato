use crate::lexer::matcher::PeekedToken;
use crate::lexer::{Grapheme, Tok};

use super::PeekResult;

pub fn peek_comment<'inp>(
    graphemes: &[Grapheme<'inp>],
    idx: usize,
    raw: &'inp str,
) -> Option<PeekResult<'inp>> {
    let comment_symbol = graphemes.get(idx)?;
    if comment_symbol.1 != ";" {
        return None;
    }

    let (count_consumed, comment_to) = graphemes
        .get(idx..)?
        .iter()
        .position(|g| g.1 == "\n")
        .map(|offset_end| {
            let char_last = graphemes[idx + offset_end - 1];
            (offset_end, char_last.0 + char_last.1.len())
        })
        .unwrap_or_else(|| {
            // Safety: `last()` will always return a value, since per definition, the input must
            // have at least one value: the comment symbol.
            let char_last = unsafe { graphemes.last().unwrap_unchecked() };
            (graphemes.len() - idx, char_last.0 + char_last.1.len())
        });

    let comment_from = graphemes
        .get(idx + 1)
        .map(|char_first| char_first.0)
        // Comment at the very end of the file with no content:
        .unwrap_or(comment_to);

    Some(Ok(PeekedToken::new(
        Tok::Comment(&raw[comment_from..comment_to]),
        comment_symbol.0,
        comment_to,
        count_consumed,
    )))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use unicode_segmentation::UnicodeSegmentation;

    use super::*;

    #[test]
    fn whole_line() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "; This is a variable, which saves a name, very impotent comment
             (defvar name 123)";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_comment(&graphemes, 0, raw),
            Some(Ok(PeekedToken::new(
                Tok::Comment(" This is a variable, which saves a name, very impotent comment"),
                0,
                63,
                63
            )))
        );
    }

    #[test]
    fn end_of_line() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "(foo) ; bar
            (baz)";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_comment(&graphemes, 6, raw),
            Some(Ok(PeekedToken::new(Tok::Comment(" bar"), 6, 11, 5)))
        );
    }

    #[test]
    fn end_of_input() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = "; bar";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_comment(&graphemes, 0, raw),
            Some(Ok(PeekedToken::new(Tok::Comment(" bar"), 0, 5, 5)))
        );
    }

    #[test]
    fn end_of_line_empty_comment() {
        let _ = env_logger::builder().is_test(true).try_init();

        let raw = ";";
        let graphemes: Vec<_> = raw.grapheme_indices(true).collect();

        assert_eq!(
            peek_comment(&graphemes, 0, raw),
            Some(Ok(PeekedToken::new(Tok::Comment(""), 0, 1, 1)))
        );
    }
}