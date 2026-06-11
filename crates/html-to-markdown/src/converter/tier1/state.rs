//! Tier-1 scanner state: escape context bitmask and the open-tag stack.

use crate::converter::tier1::tags::TagSpec;

/// Minimum output buffer capacity in bytes.
const OUTPUT_CAPACITY_MIN: usize = 1024;

/// Maximum output buffer capacity in bytes (256 KiB).
const OUTPUT_CAPACITY_MAX: usize = 256 * 1024;

/// Divisor applied to `input_len` to derive the initial output buffer capacity.
const OUTPUT_CAPACITY_DIVISOR: usize = 3;

/// Accumulated state for one `<table>` being assembled by the Tier-1 scanner.
///
/// When a `<table>` opens, a fresh `TableState` is pushed onto
/// `Tier1State::table_stack`.  Cells accumulate text into `current_cell`
/// instead of `Tier1State::output`.  On `</table>` the state is popped and
/// the completed GFM table is appended to the main output.
#[derive(Debug, Clone, Default)]
pub struct TableState {
    /// Completed rows; each row is a list of finished cell strings.
    pub rows: Vec<Vec<String>>,
    /// Row currently being assembled.
    pub current_row: Vec<String>,
    /// Cell text currently accumulating (active while inside a `<td>`/`<th>`).
    pub current_cell: String,
    /// True while the scanner is inside a `<thead>` section.
    pub in_thead: bool,
    /// True while the scanner is inside a `<td>` or `<th>`.
    pub in_cell: bool,
    /// True while the scanner is accumulating `<caption>` content.
    pub in_caption: bool,
    /// Raw text accumulated during a `<caption>` element.
    pub caption_buf: String,
    /// Trimmed, hyphen-escaped caption text ready for emission.
    ///
    /// `None` if no `<caption>` was seen; `Some("")` if the caption was empty
    /// (Tier-2 emits nothing for an empty caption, so we match that).
    pub caption_text: Option<String>,
    /// True after the first `<tbody>` has closed — used to detect
    /// `<tbody>` → `<tfoot>` → `<tbody>` ordering violations.
    pub seen_tbody_close: bool,
    /// True after a `<tfoot>` open has been seen.
    pub seen_tfoot: bool,
    /// True if at least one `<th>` cell has been seen in this table.
    ///
    /// Tier-2 only uses the GFM table rendering path when `table_has_header`
    /// returns true (i.e. at least one `<th>` exists).  Without a `<th>`,
    /// Tier-2 may render the table as a layout/bulleted-list table depending
    /// on other conditions (link count, row count, etc.).
    pub has_th: bool,
    /// Number of `<a>` (link) elements seen inside the table so far.
    ///
    /// Used to detect the "link-heavy navigation table" pattern that Tier-2
    /// renders as a layout table: `row_count <= 2 && link_count >= 3`.
    pub link_count: usize,
    /// Column count of the first row (used to detect inconsistent column counts
    /// across rows, which triggers Tier-2's layout-table path).
    pub first_row_col_count: Option<usize>,
    /// True when this table is nested inside another table's cell.
    ///
    /// On `</table>`, the rendered GFM markdown is appended to the parent
    /// cell buffer (rather than `state.output`).  The parent cell's later
    /// newline-collapse step then flattens the inner table to a single
    /// inline run, matching Tier-2's behaviour where inner tables emit
    /// full GFM into the cell text and the cell collapses `\n` → space.
    pub inline_mode: bool,
    /// True when at least one nested table has emitted GFM markdown into
    /// this table's current cell.  Set by `close_table` on the parent
    /// frame when popping an `inline_mode = true` child.  Read by
    /// `close_table_cell` to skip the literal-pipe bail — nested-table
    /// rendering legitimately introduces unescaped `|` characters that
    /// Tier-2 also emits without escaping.
    pub had_nested_table: bool,
    /// `colspan` attribute on the currently-open `<td>`/`<th>`.
    ///
    /// Defaults to 1.  On `</td>` / `</th>` close, `close_table_cell` pushes
    /// `(colspan - 1)` additional empty cells onto `current_row` so the row
    /// has the same column count Tier-2 sees after expanding colspan in
    /// `block/table/cells.rs`.  Without this expansion, infobox-style tables
    /// where a header row uses `<th colspan="2">Title</th>` would render as
    /// one column while the rest of the table has two, triggering Tier-2's
    /// layout-table fallback in close_table on what should be a normal GFM
    /// table (e.g. wikipedia/large_rust infobox).
    pub current_cell_colspan: u16,
}

bitflags::bitflags! {
    /// Ambient escape contexts.  Set when we enter a tag that changes escape rules.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct EscapeCtx: u8 {
        /// Inside a `<code>` span.
        const CODE       = 1 << 1;
        /// Inside a `<pre>` block.
        const PRE        = 1 << 2;
        /// Inside an `<a>` link.
        const LINK       = 1 << 3;
        /// Inside a `<blockquote>`.
        const BLOCKQUOTE = 1 << 4;
        /// Inside a heading element.
        const HEADING    = 1 << 5;
    }
}

/// One frame on the open-tag stack.
#[derive(Debug, Clone)]
pub struct OpenTag {
    /// Static tag specification for this element.
    pub spec: &'static TagSpec,
    /// Where this tag's content begins in the input buffer (byte index).
    pub content_start: usize,
    /// Snapshot of `escape_ctx` BEFORE this tag set its bits, so we can restore on close.
    pub prev_escape_ctx: EscapeCtx,
    /// For ordered list items: the current item counter (1-based).
    pub list_index: u16,
    /// For ordered lists: the start counter value.
    pub ol_start: u16,
    /// Byte range of the tag name in the original input (original case; callers
    /// must lowercase before comparing).
    pub name_range: std::ops::Range<usize>,
}

/// Minimum capacity for each summary accumulation buffer.
const SUMMARY_BUF_CAPACITY: usize = 64;

/// Discriminator for entries in the wrap-buffer stack.
///
/// Both `<summary>` (Phase R) and `<figcaption>` (Phase FF-2) collect
/// children into an accumulation buffer before wrapping with delimiters,
/// but they differ on strong-marker suppression: Tier-2 sets
/// `in_strong: true` for summary children (suppresses nested `**…**`)
/// but uses the default context for figcaption children.  Tag each
/// buffer with its kind so `Tier1State::summary_at_top` can answer the
/// suppression check correctly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapKind {
    /// `<summary>` strong-wrap buffer.
    Summary,
    /// `<figcaption>` italic-wrap buffer.
    Figcaption,
}

/// Mutable state threaded through the entire Tier-1 scan pass.
pub struct Tier1State {
    /// Open-tag stack; one frame per currently-open element.
    pub stack: Vec<OpenTag>,
    /// Current escape-context bitmask.
    pub escape_ctx: EscapeCtx,
    /// Accumulated Markdown output.
    pub output: String,
    /// Current list nesting depth (0 = top level). Counts both `<ul>` and `<ol>`.
    pub list_depth: u16,
    /// Current unordered-list nesting depth.
    ///
    /// Counts only `<ul>` opens.  Used to cycle through `options.bullets` so
    /// that nested lists produce byte-identical output to Tier-2 (which selects
    /// `bullets[(ul_depth - 1) % bullets.len()]` per item).
    pub ul_depth: u16,
    /// Tracks the output length before the most recent block-open separator was appended,
    /// so we can detect whether any content was actually emitted inside a block.
    pub last_block_sep_pos: usize,
    /// Stack of in-progress table states.  Pushed on `<table>` open, popped on
    /// `</table>` close.  Depth > 1 is a nested-table bail.
    pub table_stack: Vec<TableState>,
    /// Stack of `(href, title)` pairs for currently-open `<a>` elements.
    ///
    /// HTML5 forbids nested `<a>`, but the stack handles malformed input safely.
    /// Holding link state off `OpenTag` saves two `Option<String>` slots per
    /// every non-link tag frame (24 bytes × every tag on Wikipedia pages with
    /// thousands of tags), and avoids per-frame `Clone` cost.
    pub link_stack: Vec<(Option<String>, Option<String>)>,
    /// Byte range of `<head>…</head>` content (between the tags) in the
    /// input the scanner walked.  Populated by the `TagKind::Ignored`
    /// dispatch when a non-void Ignored tag (`<head>`) is encountered;
    /// `tier1::run` forwards the slice to `head_metadata::extract_frontmatter`
    /// so the YAML frontmatter pass still works without a `PrescanReport`.
    pub head_range: Option<std::ops::Range<usize>>,
    /// Language code extracted from the current `<pre>` or its nested `<code>`
    /// child's `class` attribute (`language-X` or `lang-X`).  Used by
    /// `close_pre` to emit the language tag after the opening backtick fence.
    /// Reset to `None` after each `</pre>` so nested same-level blocks don't
    /// inherit a stale language.
    pub pre_lang: Option<String>,
    /// Stack of `<summary>` and `<figcaption>` accumulation buffers.
    ///
    /// Pushed when a non-cell `<summary>`/`<figcaption>` opens; all child
    /// text accumulates here instead of in `output`.  On close, the buffer
    /// is popped, trimmed, and emitted with the wrap delimiters into the
    /// parent destination (`**…**\n\n` for Summary, `*…*\n\n` for
    /// Figcaption).
    ///
    /// Each entry carries a [`WrapKind`] discriminator so the strong-marker
    /// suppression in scanner.rs can distinguish "currently inside a
    /// summary" from "currently inside a figcaption" — Tier-2 only sets the
    /// `in_strong: true` collection context for summary children.
    ///
    /// A stack (rather than a single `Option`) handles pathological nesting
    /// without panicking.
    pub summary_buf_stack: Vec<(WrapKind, String)>,
    /// Tier-2 runs HTML through an html5ever roundtrip when the source
    /// contains custom-element tags; the roundtrip canonicalizes
    /// attribute entities (e.g. `&#x22;` → `&quot;`).  Tier-1 sets this
    /// flag at the start of `scan` so the image emit path mirrors the
    /// canonicalization for byte-equality (Phase DD).
    pub canonicalize_attr_entities: bool,
}

impl Tier1State {
    /// Create a new `Tier1State` pre-allocating output capacity based on `input_len`.
    #[must_use]
    pub fn new(input_len: usize) -> Self {
        Self {
            stack: Vec::with_capacity(16),
            escape_ctx: EscapeCtx::empty(),
            output: String::with_capacity(
                (input_len / OUTPUT_CAPACITY_DIVISOR).clamp(OUTPUT_CAPACITY_MIN, OUTPUT_CAPACITY_MAX),
            ),
            list_depth: 0,
            ul_depth: 0,
            last_block_sep_pos: 0,
            table_stack: Vec::new(),
            link_stack: Vec::new(),
            head_range: None,
            pre_lang: None,
            summary_buf_stack: Vec::new(),
            canonicalize_attr_entities: false,
        }
    }

    /// Return a mutable reference to the current accumulation target.
    ///
    /// Priority order (highest first):
    /// 1. Summary buffer top — when a `<summary>` accumulation buffer is active,
    ///    all text (including text from inside table cells) accumulates here.
    ///    This mirrors Tier-2's behaviour where `handle_summary` processes
    ///    children into a local `content` buffer regardless of outer context.
    /// 2. Table cell — when inside a `<td>`/`<th>`, text accumulates in the
    ///    cell buffer (only when not already inside a summary).
    /// 3. Table caption — when inside a `<caption>`, text accumulates in the
    ///    caption buffer.
    /// 4. `self.output` — the main output buffer (default).
    ///
    /// This is the single dispatch point for "where does inline text land."
    pub fn cell_or_output_mut(&mut self) -> &mut String {
        if let Some((_, buf)) = self.summary_buf_stack.last_mut() {
            return buf;
        }
        if let Some(ts) = self.table_stack.last_mut() {
            if ts.in_cell {
                return &mut ts.current_cell;
            }
            if ts.in_caption {
                return &mut ts.caption_buf;
            }
        }
        &mut self.output
    }

    /// True when the scanner is currently accumulating `<summary>` or
    /// `<figcaption>` content (any wrap-buffer is on the stack).
    ///
    /// Whitespace and text-normalization paths use this — both wrap kinds
    /// share the same collection-mode semantics.
    #[must_use]
    pub fn in_summary(&self) -> bool {
        !self.summary_buf_stack.is_empty()
    }

    /// True when the topmost wrap-buffer is specifically a `<summary>`
    /// (Phase R), as opposed to a `<figcaption>` (Phase FF-2).
    ///
    /// Used to suppress nested `**…**` strong markers, since Tier-2 only
    /// sets `in_strong: true` for summary children.
    #[must_use]
    pub fn summary_at_top(&self) -> bool {
        matches!(self.summary_buf_stack.last(), Some((WrapKind::Summary, _)))
    }

    /// Push a fresh wrap accumulation buffer onto the stack, tagged with
    /// the given [`WrapKind`].
    pub fn push_summary_buf(&mut self, kind: WrapKind) {
        self.summary_buf_stack
            .push((kind, String::with_capacity(SUMMARY_BUF_CAPACITY)));
    }

    /// Pop the top wrap accumulation buffer and return it (kind discarded).
    pub fn pop_summary_buf(&mut self) -> Option<String> {
        self.summary_buf_stack.pop().map(|(_, buf)| buf)
    }

    /// True when the scanner is currently accumulating `<caption>` content.
    #[must_use]
    pub fn in_table_caption(&self) -> bool {
        self.table_stack.last().is_some_and(|ts| ts.in_caption)
    }

    /// True when the scanner is currently inside a table cell.
    #[must_use]
    pub fn in_table_cell(&self) -> bool {
        self.table_stack.last().is_some_and(|ts| ts.in_cell)
    }

    /// True when ANY frame on the table stack has `in_cell = true`.
    ///
    /// With nested tables (Phase HH), the inner table's frame may have
    /// `in_cell = false` while an outer frame still has `in_cell = true`
    /// — for example between `<td>` siblings of the inner table while the
    /// outer cell that wraps it is still open.  Use this for inter-cell
    /// whitespace guards that must drop text outside *any* active cell.
    #[must_use]
    pub fn in_any_table_cell(&self) -> bool {
        self.table_stack.iter().any(|ts| ts.in_cell)
    }

    /// Ensure the output ends with exactly two newlines (blank-line separator).
    /// If the output is empty, do nothing.
    ///
    /// Trailing ASCII spaces / tabs (introduced by the inter-tag whitespace
    /// preservation in `flush_text`) are trimmed before the separator is
    /// appended.  Without this trim, `<span>foo</span> <div>bar</div>` would
    /// emit `foo \n\nbar` with a stray trailing space — a regression flagged
    /// during Phase U development.  Trimming here makes "preserve a space
    /// optimistically, drop on block boundary" safe.
    pub fn ensure_blank_line(&mut self) {
        let out = &mut self.output;
        if out.is_empty() {
            return;
        }
        // Drop any trailing horizontal whitespace before deciding the separator.
        while out.ends_with(' ') || out.ends_with('\t') {
            out.pop();
        }
        if out.ends_with("\n\n") {
            return;
        }
        if out.ends_with('\n') {
            out.push('\n');
        } else if out.is_empty() {
            // Output was entirely trailing whitespace before the trim — nothing to separate.
        } else {
            out.push_str("\n\n");
        }
    }

    /// Ensure the output ends with at least one newline.
    pub fn ensure_newline(&mut self) {
        if !self.output.is_empty() && !self.output.ends_with('\n') {
            self.output.push('\n');
        }
    }
}

/// Trim trailing ASCII horizontal whitespace (spaces and tabs) from a string
/// buffer.  Used before emitting block separators to drop the optimistic
/// inter-tag space pushed by `flush_text` (Phase U-2).
pub(crate) fn trim_trailing_horizontal(buf: &mut String) {
    while buf.ends_with(' ') || buf.ends_with('\t') {
        buf.pop();
    }
}
