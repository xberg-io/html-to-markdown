//! Bail reasons emitted by the Tier-1 scanner.
//!
//! When the scanner encounters a condition it cannot handle correctly it returns
//! one of these variants. The dispatcher in `lib.rs::convert` catches the error,
//! logs it at `tracing::info`, and falls back to the Tier-2 path with the
//! original (pre-prescan) input.

use std::fmt;

/// Reasons the Tier-1 scanner may bail out and hand off to Tier-2.
#[derive(Debug, Clone)]
pub enum BailReason {
    /// The classifier decided Tier-2 is required for this input / option set.
    Classifier,

    /// An open-tag stack mismatch was detected mid-stream.
    DepthMismatch {
        /// Tag name where the mismatch was detected.
        tag: String,
        /// Depth expected by the scanner's stack.
        expected: u8,
        /// Depth seen in the source.
        actual: u8,
    },

    /// Reached end-of-file with one or more unclosed block elements.
    EofWithOpenBlock {
        /// Number of unclosed block elements at EOF.
        open_count: usize,
    },

    /// A literal `<` (not a valid tag open) was encountered in the stream.
    LiteralLt {
        /// Byte offset of the `<` in the input.
        offset: usize,
    },

    /// A CDATA section was encountered.
    Cdata {
        /// Byte offset of `<![CDATA[` in the input.
        offset: usize,
    },

    /// An unknown custom element (tag name containing `-`) was encountered.
    UnknownCustomElement {
        /// The element name.
        name: Box<str>,
        /// Byte offset of the `<` in the input.
        offset: usize,
    },

    // ── Table-specific bail reasons ───────────────────────────────────────────
    /// A `<td>` or `<th>` had a `rowspan` or `colspan` attribute with a value
    /// other than 1 (absent attribute counts as 1).
    TableRowspanColspan,

    /// A block-level element was opened inside a `<td>` or `<th>` (e.g.
    /// `<td><p>text</p></td>`).  Tier-1 only supports inline cell content.
    TableBlockChildInCell,

    /// A nested `<table>` was opened while a table is already being assembled
    /// (i.e. `table_stack` is non-empty).
    TableNestedTable,

    /// A `<caption>` element was encountered inside a table.
    TableCaption,

    /// Table sections appear in an unsupported order, e.g. `<tbody>` after
    /// `<tfoot>` close, or `<thead>` after any section that already closed.
    TableSectionOrder,

    /// A named HTML entity (e.g. `&mdash;`, `&laquo;`) was encountered that is
    /// not in Tier-1's 45-entry decode table, or a numeric character reference
    /// was malformed / mapped to an invalid Unicode code point.
    ///
    /// Tier-1 would pass the entity through verbatim, but Tier-2 decodes it to
    /// the correct character, so the outputs would diverge.  Bail so the
    /// dispatcher falls back to Tier-2.
    UnknownEntity {
        /// The entity name between `&` and `;` (e.g. `"mdash"`, `"#x2014"`).
        name: Box<str>,
        /// Byte offset in the HTML input where the `&` was found.
        offset: usize,
    },
}

impl fmt::Display for BailReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Classifier => write!(f, "classifier forced tier-2"),
            Self::DepthMismatch { tag, expected, actual } => {
                write!(
                    f,
                    "depth mismatch for </{tag}>: expected {expected} open(s), got {actual}"
                )
            }
            Self::EofWithOpenBlock { open_count } => {
                write!(f, "EOF with {open_count} unclosed block element(s)")
            }
            Self::LiteralLt { offset } => {
                write!(f, "literal '<' at byte offset {offset}")
            }
            Self::Cdata { offset } => {
                write!(f, "CDATA section at byte offset {offset}")
            }
            Self::UnknownCustomElement { name, offset } => {
                write!(f, "unknown custom element <{name}> at byte offset {offset}")
            }
            Self::TableRowspanColspan => {
                write!(f, "table cell has rowspan or colspan != 1")
            }
            Self::TableBlockChildInCell => {
                write!(f, "block-level element inside table cell")
            }
            Self::TableNestedTable => {
                write!(f, "nested <table> inside a table cell")
            }
            Self::TableCaption => {
                write!(f, "<caption> element in table")
            }
            Self::TableSectionOrder => {
                write!(f, "table sections in unsupported order")
            }
            Self::UnknownEntity { name, offset } => {
                write!(f, "unknown HTML entity &{name}; at byte offset {offset}")
            }
        }
    }
}
