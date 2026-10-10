//! The white space rule of running text, decided in one place for both converters.
//!
//! The rule is CSS `white-space: normal` as a browser applies it to the decoded text of the
//! parsed tree:
//!
//! 1. Collapsible white space is exactly the five characters of [`is_collapsible`]. A no-break
//!    space and every other Unicode space is a character.
//! 2. A segment break (a line feed) is removed when the character before or after its run,
//!    across inline element boundaries and comments, is a zero-width space; otherwise it is
//!    a space.
//! 3. A run of collapsible white space, across inline element boundaries, comments and empty
//!    inline elements, is one space.
//! 4. That space is written only where it is owed ([`space_is_owed`]): not at the start of a
//!    line and not after a space already written. Markdown marks are not characters: the space
//!    goes outside the marks.
//! 5. Inside `pre`, code and in strict mode nothing above applies: the caller does not ask.

use crate::converter::utility::content::ZERO_WIDTH_SPACE;

/// Whether `character` is collapsible white space: a space, a tab, a line feed, a carriage
/// return or a form feed. No other character is.
#[must_use]
pub const fn is_collapsible(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{c}')
}

/// A run of collapsible white space at an edge of a text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
    /// The run holds a segment break (a line feed).
    pub segment_break: bool,
}

impl Run {
    fn of(white_space: &str) -> Option<Self> {
        (!white_space.is_empty()).then(|| Self {
            segment_break: white_space.contains('\n'),
        })
    }

    /// Whether the run is written as one space. It is not when it holds a segment break that
    /// the zero-width space `before` or `after` the run removes.
    #[must_use]
    pub fn writes_space(self, before: Option<char>, after: Option<char>) -> bool {
        !(self.segment_break && segment_break_is_removed(before, after))
    }
}

/// The decoded text of a text node, classified by the collapsible white space at its edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextClass<'text> {
    /// Every character of the text is collapsible white space (or the text is empty).
    WhiteSpaceOnly(Option<Run>),
    /// The text has at least one character that is not collapsible white space.
    Text {
        /// The collapsible white space before the first character.
        prefix: Option<Run>,
        /// The text between the two runs, the first and the last character not collapsible.
        core: &'text str,
        /// The collapsible white space after the last character.
        suffix: Option<Run>,
    },
}

/// Classifies `decoded` by rule 1: only the five collapsible characters count as white space.
#[must_use]
pub fn classify_text(decoded: &str) -> TextClass<'_> {
    let core = decoded.trim_matches(is_collapsible);
    if core.is_empty() {
        return TextClass::WhiteSpaceOnly(Run::of(decoded));
    }
    let start = decoded.len() - decoded.trim_start_matches(is_collapsible).len();
    let end = start + core.len();
    TextClass::Text {
        prefix: Run::of(&decoded[..start]),
        core,
        suffix: Run::of(&decoded[end..]),
    }
}

/// Rule 2: a segment break is removed when the character `before` or `after` its run is a
/// zero-width space. `None` is a block boundary or the end of the text.
#[must_use]
pub fn segment_break_is_removed(before: Option<char>, after: Option<char>) -> bool {
    before == Some(ZERO_WIDTH_SPACE) || after == Some(ZERO_WIDTH_SPACE)
}

/// Rule 4: whether white space of the source owes one space at the end of `written`. Both
/// converters ask here before they write the space of a text or of an inline element.
///
/// ~keep White space collapses as in CSS: a space after a space is the same run, whatever
/// ~keep element boundary lies between the two, and the start of a line keeps none.
#[must_use]
pub fn space_is_owed(written: &str) -> bool {
    !written.ends_with([' ', '\n'])
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUN: Option<Run> = Some(Run { segment_break: false });
    const BREAK: Option<Run> = Some(Run { segment_break: true });

    #[test]
    fn the_five_characters_are_collapsible() {
        for character in [' ', '\t', '\n', '\r', '\u{c}'] {
            assert!(is_collapsible(character), "{character:?}");
        }
    }

    #[test]
    fn a_no_break_space_and_the_other_unicode_spaces_are_characters() {
        for character in [
            '\u{a0}', '\u{2002}', '\u{2003}', '\u{3000}', '\u{200b}', '\u{feff}', 'a',
        ] {
            assert!(!is_collapsible(character), "{character:?}");
        }
    }

    #[test]
    fn a_text_of_the_five_characters_is_white_space_only() {
        assert_eq!(classify_text(" \t\r\u{c}"), TextClass::WhiteSpaceOnly(RUN));
        assert_eq!(classify_text(" \n "), TextClass::WhiteSpaceOnly(BREAK));
        assert_eq!(classify_text(""), TextClass::WhiteSpaceOnly(None));
    }

    #[test]
    fn a_no_break_space_alone_is_text() {
        assert_eq!(
            classify_text("\u{a0}"),
            TextClass::Text {
                prefix: None,
                core: "\u{a0}",
                suffix: None
            }
        );
        assert_eq!(
            classify_text(" \u{a0} "),
            TextClass::Text {
                prefix: RUN,
                core: "\u{a0}",
                suffix: RUN
            }
        );
    }

    #[test]
    fn the_edges_of_a_text_are_its_runs() {
        assert_eq!(
            classify_text("\u{c}\none"),
            TextClass::Text {
                prefix: BREAK,
                core: "one",
                suffix: None
            }
        );
        assert_eq!(
            classify_text("one\u{c}\n"),
            TextClass::Text {
                prefix: None,
                core: "one",
                suffix: BREAK
            }
        );
        assert_eq!(
            classify_text("one two\t"),
            TextClass::Text {
                prefix: None,
                core: "one two",
                suffix: RUN
            }
        );
    }

    #[test]
    fn a_segment_break_beside_a_zero_width_space_is_removed_on_either_side() {
        assert!(segment_break_is_removed(Some('\u{200b}'), Some('G')));
        assert!(segment_break_is_removed(Some('s'), Some('\u{200b}')));
        assert!(segment_break_is_removed(Some('\u{200b}'), None));
        assert!(segment_break_is_removed(None, Some('\u{200b}')));
        assert!(!segment_break_is_removed(Some('s'), Some('G')));
        assert!(!segment_break_is_removed(None, None));
        assert!(!segment_break_is_removed(Some('\u{a0}'), Some('\u{feff}')));
    }

    #[test]
    fn a_run_writes_one_space_unless_its_segment_break_is_removed() {
        let run = Run { segment_break: false };
        assert!(run.writes_space(Some('\u{200b}'), Some('\u{200b}')));
        let with_break = Run { segment_break: true };
        assert!(with_break.writes_space(Some('s'), Some('G')));
        assert!(!with_break.writes_space(Some('\u{200b}'), Some('G')));
        assert!(!with_break.writes_space(Some('s'), Some('\u{200b}')));
    }

    #[test]
    fn a_space_is_owed_except_after_a_space_or_at_a_line_start() {
        assert!(space_is_owed("one"));
        assert!(space_is_owed("**one**"));
        assert!(space_is_owed("one\u{a0}"));
        assert!(!space_is_owed("one "));
        assert!(!space_is_owed("one\n"));
    }
}
