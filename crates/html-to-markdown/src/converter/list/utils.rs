//! Utility functions for list processing.
//!
//! Contains helper functions for loose list detection, indentation calculation,
//! list spacing, and list child processing.

use crate::converter::main_helpers::{tag_name_eq, trim_trailing_whitespace};
use crate::converter::utility::content::normalized_tag_name;
use crate::options::{ConversionOptions, ListIndentType, OutputFormat};
use tl;

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Counter value an `<ol>` starts from when it has no (or an invalid) `start` attribute.
///
/// This mirrors the HTML spec default for ordered list numbering.
pub const DEFAULT_ORDERED_LIST_START: i64 = 1;

/// Parse the `start` attribute of an `<ol>` element into a counter value.
///
/// `start` is untrusted external input: the HTML spec allows any signed integer (browsers
/// count downward from a negative `start`), and a document can supply a magnitude that
/// overflows every fixed-width integer type. Rather than panicking or wrapping, out-of-range
/// magnitudes are clamped to the `i64` bounds the render-time counter uses, and syntactically
/// invalid values (empty, non-numeric) fall back to the spec default of 1.
pub fn parse_ordered_list_start(raw: &str) -> i64 {
    let trimmed = raw.trim();
    if let Ok(value) = trimmed.parse::<i64>() {
        return value;
    }

    let (is_negative, digits) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed.strip_prefix('+').unwrap_or(trimmed)),
    };

    if !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()) {
        let clamped = if is_negative { i64::MIN } else { i64::MAX };
        tracing::warn!(
            target: "html_to_markdown::list",
            raw_value = trimmed,
            clamped_value = clamped,
            "ol start attribute magnitude out of range; clamping to i64 bounds"
        );
        return clamped;
    }

    if !trimmed.is_empty() {
        tracing::warn!(
            target: "html_to_markdown::list",
            raw_value = trimmed,
            default_value = DEFAULT_ORDERED_LIST_START,
            "ol start attribute is not a valid integer; using default start"
        );
    }
    DEFAULT_ORDERED_LIST_START
}

/// The number of tabs that reaches a list item's content column: each tab is four columns
/// wide, and a line indented by four or more columns past the content column is a code block.
pub const fn tabs_for_column(list_indent_columns: usize) -> usize {
    list_indent_columns.div_ceil(4)
}

/// Direct-child tag names that force a list item's own trailing separator (kept in sync with
/// `list/item.rs::has_block_children`'s identical match arms), for every item except the
/// list's last.
///
/// ~keep A list item containing one of these -- even without a `<p>` -- still needs a blank
/// ~keep line before/after it in our rendering to keep item boundaries unambiguous (a bare
/// ~keep `<pre>` or `<blockquote>` sibling can't just run into the next `- ` marker). Once that
/// ~keep blank line exists anywhere BETWEEN two items, every CommonMark-compliant reparse
/// ~keep concludes the *whole* list is loose (blank lines are a per-list, not per-item-pair,
/// ~keep signal) and re-wraps every item's content in `<p>`, including plain-text ones that had
/// ~keep none originally. Treating this same trigger set as "loose" up front -- not just literal
/// ~keep `<p>` -- renders every item with full blank-line separation from the first pass, which
/// ~keep is what the second-generation reparse would force anyway (spec examples 278, 308, 318).
/// ~keep Restricted to "not the last item": one of these tags in the list's OWN last item has
/// ~keep no following sibling to create a boundary blank line with, so it never actually
/// ~keep reparses the list as loose -- unlike literal `<p>`, which is excluded from this gate
/// ~keep below because it is CommonMark's actual, unconditional looseness signal regardless of
/// ~keep position (issue: an ordered list whose only loose-looking item is its last, e.g. a
/// ~keep trailing `<table>`, incorrectly gained a leading blank line without this gate).
const BLOCK_FORCING_CHILD_TAGS: [&str; 6] = ["div", "blockquote", "pre", "table", "hr", "dl"];

/// Resolve a node's normalized tag name via the `DomContext` cache, falling back to the raw
/// `tl` tag when no cached `TagInfo` exists for it.
fn resolve_tag_name(node_handle: tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> Option<String> {
    if let Some(info) = dom_ctx.tag_info(node_handle.get_inner(), parser) {
        return Some(info.name.clone());
    }
    match node_handle.get(parser) {
        Some(tl::Node::Tag(tag)) => Some(normalized_tag_name(tag.name().as_utf8_str()).into_owned()),
        _ => None,
    }
}

/// Check if a list (ul or ol) is "loose".
///
/// A loose list is one where any list item contains block-level elements like paragraphs
/// (`<p>`), or any other element that forces our own rendering to add a blank-line separator
/// (see `BLOCK_FORCING_CHILD_TAGS`), or a nested sublist that is itself loose (a loose nested
/// list's own trailing blank line becomes the boundary before the next item of THIS list when
/// it is that item's last content). In loose lists, all items should have blank line
/// separation (ending with \n\n) regardless of their own content.
///
/// # Examples
///
/// ```html
/// <!-- Loose list (has <p> in an item) -->
/// <ul>
///   <li><p>Item 1</p></li>
///   <li>Item 2</li>  <!-- Also gets \n\n ending -->
/// </ul>
///
/// <!-- Tight list (no block elements) -->
/// <ul>
///   <li>Item 1</li>
///   <li>Item 2</li>
/// </ul>
/// ```
pub fn is_loose_list(node_handle: tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    let Some(tl::Node::Tag(tag)) = node_handle.get(parser) else {
        return false;
    };

    let children = tag.children();
    let items: Vec<tl::NodeHandle> = children
        .top()
        .iter()
        .copied()
        .filter(|child_handle| {
            dom_ctx.tag_info(child_handle.get_inner(), parser).map_or_else(
                || {
                    matches!(
                        child_handle.get(parser),
                        Some(tl::Node::Tag(child_tag))
                            if tag_name_eq(child_tag.name().as_utf8_str(), "li")
                    )
                },
                |info| info.name == "li",
            )
        })
        .collect();
    let Some(last_index) = items.len().checked_sub(1) else {
        return false;
    };

    for (index, item_handle) in items.iter().enumerate() {
        let Some(tl::Node::Tag(item_tag)) = item_handle.get(parser) else {
            continue;
        };
        let is_last = index == last_index;
        let li_children = item_tag.children();
        for li_child_handle in li_children.top().iter() {
            let Some(name) = resolve_tag_name(*li_child_handle, parser, dom_ctx) else {
                continue;
            };
            if name == "p" {
                return true;
            }
            if is_last {
                continue;
            }
            if BLOCK_FORCING_CHILD_TAGS.contains(&name.as_str()) {
                return true;
            }
            if matches!(name.as_str(), "ul" | "ol") && is_loose_list(*li_child_handle, parser, dom_ctx) {
                return true;
            }
        }
    }
    false
}

/// Add list continuation indentation to output.
///
/// Used when block elements (like <p> or <div>) appear inside list items.
/// Adds appropriate line separation and indentation to continue the list item.
///
/// # Arguments
///
/// * `output` - The output string to append to
/// * `list_indent_columns` - The item's content column
/// * `blank_line` - If true, adds blank line separation (\n\n); if false, single newline (\n)
///
/// # Examples
///
/// ```text
/// Paragraph continuation (blank_line = true):
///   * First para
///
///       Second para  (blank line + indentation)
///
/// Div continuation (blank_line = false):
///   * First div
///       Second div   (single newline + indentation)
/// ```
pub fn add_list_continuation_indent(
    output: &mut String,
    list_indent_columns: usize,
    blank_line: bool,
    options: &ConversionOptions,
) {
    trim_trailing_whitespace(output);

    if blank_line {
        if !output.ends_with("\n\n") {
            if output.ends_with('\n') {
                output.push('\n');
            } else {
                output.push_str("\n\n");
            }
        }
    } else if !output.ends_with('\n') {
        output.push('\n');
    }

    match options.list_indent_type {
        ListIndentType::Tabs => {
            for _ in 0..tabs_for_column(list_indent_columns) {
                output.push('\t');
            }
        }
        // ~keep `list_indent_columns` is the item's content column (see
        // ~keep Context::list_indent_columns), not a uniform per-depth value.
        ListIndentType::Spaces => {
            for _ in 0..list_indent_columns {
                output.push(' ');
            }
        }
    }
}

/// Calculate the indentation string for list continuations based on depth and options.
pub fn continuation_indent_string(list_indent_columns: usize, options: &ConversionOptions) -> Option<String> {
    match options.list_indent_type {
        ListIndentType::Tabs => {
            let tabs = tabs_for_column(list_indent_columns);
            if tabs == 0 {
                return None;
            }
            Some("\t".repeat(tabs))
        }
        // ~keep `list_indent_columns` is the item's content column (see
        // ~keep Context::list_indent_columns), not a uniform per-depth value.
        ListIndentType::Spaces => {
            if list_indent_columns == 0 {
                return None;
            }
            Some(" ".repeat(list_indent_columns))
        }
    }
}

/// Write the list item's continuation indent when `output` is at the start of a line inside the
/// item, so the text or hard break written next stays in the item. Not in verbatim content or in a
/// detached buffer, where `output` is not the item's own text.
pub fn indent_list_item_line_start(output: &mut String, ctx: &Context, options: &ConversionOptions) {
    if ctx.in_list_item
        && !ctx.in_code
        && !ctx.in_ruby
        && !ctx.in_table_cell
        && !ctx.convert_as_inline
        && output.ends_with('\n')
        && !output.ends_with("\n\n")
    {
        if let Some(indent) = continuation_indent_string(ctx.list_indent_columns, options) {
            output.push_str(&indent);
        }
    }
}

/// The column that the continuation indent for `list_indent_columns` reaches.
pub fn indent_column(list_indent_columns: usize, options: &ConversionOptions) -> usize {
    continuation_indent_string(list_indent_columns, options).map_or(0, |indent| {
        crate::converter::utility::escaping::leading_indent(&indent).1
    })
}

/// Whether a line at the list item's content column can start a block: it is within 3 columns
/// of the content column of the innermost item whose marker starts a list item. Further in, the
/// line is the text of that item's paragraph.
pub fn block_is_real(ctx: &Context, options: &ConversionOptions) -> bool {
    indent_column(ctx.list_indent_columns, options).saturating_sub(indent_column(ctx.real_item_columns, options)) < 4
}

/// The column a block that starts its own line in the list item is written at: the item's content
/// column, or the column of the item whose marker starts a list item where the content column
/// starts no block in a quote whose first line is outside it.
///
/// ~keep That quote holds no paragraph for a line at the content column to continue, so the
/// ~keep line would be an indented code block.
pub fn block_columns(ctx: &Context, options: &ConversionOptions) -> usize {
    if ctx.quote_starts_after_markers && !block_is_real(ctx, options) {
        ctx.real_item_columns
    } else {
        ctx.list_indent_columns
    }
}

/// Whether `marker`, written by a list item between markers on the line starting at
/// `marker_line_start`, starts a real list item inside the item at `enclosing_columns`.
///
/// ~keep The marker must start its own line: the buffer's first line follows the opening
/// ~keep marker. Measured from the enclosing item's content column, the marker line with the
/// ~keep item's content after it must open a block that can interrupt a paragraph (the check
/// ~keep that escapes a link label's continuation lines). Where no paragraph is open, any
/// ~keep marker within 3 columns starts an item.
pub fn marker_starts_item(
    output: &str,
    marker_line_start: Option<usize>,
    marker: &str,
    enclosing_columns: usize,
    previous: (&PreviousMarker, usize),
    options: &ConversionOptions,
) -> bool {
    let Some(line_start) = marker_line_start else {
        return false;
    };
    let enclosing_column = indent_column(enclosing_columns, options);
    let marker_column = crate::converter::utility::escaping::leading_indent(&output[line_start..]).1;
    let column = marker_column.saturating_sub(enclosing_column);
    if !paragraph_is_open_before(output, line_start, enclosing_column, previous) {
        return column < 4;
    }
    let line = format!("{}{marker}x", " ".repeat(column));
    crate::converter::utility::escaping::line_opens_block(&line)
}

/// Whether the first item of a list, whose marker `line` starts at `line_start` in `output` inside
/// the item at `enclosing_columns`, needs a blank line before it: the line cannot interrupt a
/// paragraph, and a paragraph is open there. `line` has no indent.
///
/// ~keep In `CommonMark` a marker line with content interrupts a paragraph unless its marker is
/// ~keep an ordered one other than `1.` (issue #662). A marker line without content (an empty
/// ~keep item) cannot, and a lone `-` is a heading underline (issue #667). In Djot no list can
/// ~keep (issue #670). The items after the first follow a list item.
pub fn list_needs_blank_line(
    (output, line_start): (&str, usize),
    line: &str,
    enclosing_columns: usize,
    previous: (&PreviousMarker, usize),
    options: &ConversionOptions,
) -> bool {
    !marker_line_interrupts(line, options)
        && paragraph_is_open_before(output, line_start, indent_column(enclosing_columns, options), previous)
}

/// Whether the marker `line` of a list's first item, without its indent, can interrupt a paragraph.
fn marker_line_interrupts(line: &str, options: &ConversionOptions) -> bool {
    options.output_format != OutputFormat::Djot && marker_line_can_interrupt(line)
}

/// Whether the marker `line`, without its indent, can interrupt a `CommonMark` paragraph.
fn marker_line_can_interrupt(line: &str) -> bool {
    use crate::converter::utility::escaping::{is_heading_underline, line_opens_block};
    line_opens_block(line) && !is_heading_underline(line.trim_start_matches([' ', '\t']))
}

/// Whether the item that wrote its marker line at `line_start` in `output`, on the line after text
/// inside its list, needs a blank line before that line: the line cannot interrupt the text.
///
/// ~keep The text before the item was checked with an item that has content (issue #625). A marker
/// ~keep line without content cannot interrupt, and a lone `-` is a heading underline (issue #667).
pub fn marker_line_after_text_needs_blank_line(output: &str, line_start: usize) -> bool {
    !marker_line_can_interrupt(written_marker_line(output, line_start))
}

/// The marker line that the first item of a list wrote at `line_start` in `output`, without its
/// indent.
///
/// ~keep The stand-in line has no indent either: the marker sits where the list's items start.
fn written_marker_line(output: &str, line_start: usize) -> &str {
    let written = &output[line_start..];
    written[..written.find('\n').unwrap_or(written.len())].trim_start_matches([' ', '\t'])
}

/// The indent that puts a marker on the next line at the column where it starts at `end` on the
/// current line of `output`: the line up to `end`, with every byte but a tab written as a space.
fn column_of_written_marker(output: &str, end: usize) -> String {
    let start = output[..end].rfind('\n').map_or(0, |pos| pos + 1);
    output[start..end]
        .chars()
        .map(|ch| if ch == '\t' { '\t' } else { ' ' })
        .collect()
}

/// Whether the line of `output` that holds `end`, a line of bare list markers up to `end`, reads as
/// a thematic break from its start or from one of its markers on.
fn bare_marker_line_is_rule(output: &str, end: usize) -> bool {
    let start = output[..end].rfind('\n').map_or(0, |pos| pos + 1);
    let line_end = output[end..].find('\n').map_or(output.len(), |pos| end + pos);
    let mut rest = output[start..line_end].trim_start_matches([' ', '\t']);
    loop {
        if crate::converter::utility::escaping::is_rule(rest) {
            return true;
        }
        match strip_leading_bare_marker(rest) {
            Some(next) => rest = next,
            None => return false,
        }
    }
}

/// The end of the last ordered list and the buffers it can still end.
///
/// ~keep A list continues after blank lines when the next marker has the same type (issue #666),
/// ~keep so an ordered list that starts where the output still ends with an ordered list, in the
/// ~keep same place, writes the other delimiter. Elements such as `<section>` write into a buffer
/// ~keep of their own, rewrite it and append it to their parent's. `walk_node` reports each node's
/// ~keep start and end here. Nodes that pass one buffer down form a run, and the key names the run
/// ~keep that holds the list's end and the offset of that end. When the node that started the run
/// ~keep ends, its buffer can go away, so the key waits for its parent. A later child of the same
/// ~keep parent takes it back when its output is the same buffer: the same address, still holding
/// ~keep the list's last bytes at the end offset. Otherwise the key moves to the end of the
/// ~keep parent's output when the parent ends, after any rewrite. So the key never reads a buffer
/// ~keep that is gone, and each byte after the end is checked once. A list whose last line opens a
/// ~keep block left of its items' content is already closed, so it stores nothing. A stale answer
/// ~keep only switches a marker that did not need it.
#[derive(Clone, Default)]
pub struct LastList(std::rc::Rc<std::cell::RefCell<ListTracker>>);

#[derive(Default)]
struct ListTracker {
    frames: Vec<Frame>,
    key: Option<ListEnd>,
}

/// A node being rendered: the address of its output and the first node of the run that writes
/// into the same output.
#[derive(Clone, Copy)]
struct Frame {
    buffer: usize,
    run_start: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Owner {
    /// The run, by its first node, whose buffer holds the end.
    Run(usize),
    /// The run ended; the key waits for the parent node (`None` above the root nodes).
    Waiting(Option<usize>),
}

/// How many bytes before the end a later child compares to take the key back.
const END_BYTES: usize = 32;

struct ListEnd {
    owner: Owner,
    buffer: usize,
    end: usize,
    checked: usize,
    end_bytes: [u8; END_BYTES],
    place: ListPlace,
    delimiter: char,
}

impl ListEnd {
    /// The key for a list whose text ends `output`.
    fn new(owner: Owner, buffer: usize, output: &str, place: ListPlace, delimiter: char) -> Self {
        let mut key = Self {
            owner,
            buffer,
            end: output.len(),
            checked: output.len(),
            end_bytes: [0; END_BYTES],
            place,
            delimiter,
        };
        key.take_end_bytes(output);
        key
    }

    fn take_end_bytes(&mut self, output: &str) {
        let bytes = &output.as_bytes()[self.end.saturating_sub(END_BYTES)..self.end];
        self.end_bytes = [0; END_BYTES];
        self.end_bytes[..bytes.len()].copy_from_slice(bytes);
    }

    fn holds_end_bytes(&self, output: &str) -> bool {
        let start = self.end.saturating_sub(END_BYTES);
        output.as_bytes().get(start..self.end) == Some(&self.end_bytes[..self.end - start])
    }

    /// Whether `output`, the buffer that holds the end, still reaches the end and has only
    /// whitespace after it.
    fn clean_after(&mut self, output: &str) -> bool {
        let start = self.checked.min(output.len()).max(self.end);
        self.checked = output.len();
        output
            .as_bytes()
            .get(start..)
            .is_some_and(|rest| rest.iter().all(u8::is_ascii_whitespace))
    }
}

/// Where a list sits: the content column of the item around it and the quote depth.
#[derive(Clone, Copy, PartialEq, Eq)]
struct ListPlace {
    columns: usize,
    quotes: usize,
}

impl ListPlace {
    const fn of(ctx: &Context) -> Self {
        Self {
            columns: ctx.list_indent_columns,
            quotes: ctx.blockquote_depth,
        }
    }
}

impl LastList {
    /// A node starts rendering into `output`.
    pub fn enter(&self, output: &String) {
        let mut tracker = self.0.borrow_mut();
        let buffer = std::ptr::from_ref::<String>(output) as usize;
        let index = tracker.frames.len();
        let run_start = match tracker.frames.last() {
            Some(parent) if parent.buffer == buffer => parent.run_start,
            _ => index,
        };
        tracker.frames.push(Frame { buffer, run_start });
        let ListTracker { key, .. } = &mut *tracker;
        if let Some(waiting) = key.as_mut().filter(|key| {
            key.owner == Owner::Waiting(index.checked_sub(1)) && key.buffer == buffer && run_start == index
        }) {
            if waiting.holds_end_bytes(output) {
                waiting.owner = Owner::Run(run_start);
                if !waiting.clean_after(output) {
                    *key = None;
                }
            }
        }
    }

    /// The node that started last ends; `output` is its output.
    pub fn leave(&self, output: &str) {
        let mut tracker = self.0.borrow_mut();
        let Some(frame) = tracker.frames.pop() else { return };
        let index = tracker.frames.len();
        let ListTracker { key, .. } = &mut *tracker;
        let Some(list_end) = key.as_mut() else { return };
        if list_end.owner == Owner::Waiting(Some(index)) {
            *list_end = ListEnd::new(
                Owner::Run(frame.run_start),
                frame.buffer,
                output,
                list_end.place,
                list_end.delimiter,
            );
        }
        if list_end.owner == Owner::Run(frame.run_start) {
            if !list_end.clean_after(output) {
                *key = None;
            } else if frame.run_start == index {
                list_end.owner = Owner::Waiting(index.checked_sub(1));
            }
        }
    }

    /// Store the end of an ordered list whose items wrote `delimiter` at the end of `output`.
    pub fn set(&self, output: &str, ctx: &Context, delimiter: char) {
        use crate::converter::utility::escaping::{leading_indent, opens_block};
        let mut tracker = self.0.borrow_mut();
        let Some(frame) = tracker.frames.last().copied() else {
            return;
        };
        let content = output.trim_end_matches(|c: char| c.is_ascii_whitespace());
        let line = &content[content.rfind('\n').map_or(0, |pos| pos + 1)..];
        let (indent, column) = leading_indent(line);
        let rest = &line[indent..];
        let closed =
            column < ctx.list_indent_columns + 2 && opens_block(rest) && strip_leading_bare_marker(rest).is_none();
        tracker.key = (!closed).then(|| {
            ListEnd::new(
                Owner::Run(frame.run_start),
                frame.buffer,
                output,
                ListPlace::of(ctx),
                delimiter,
            )
        });
    }

    /// The delimiter of the list that `output` still ends with, for a list that starts at the end
    /// of `output`. A list in an outer buffer counts while `output` holds only whitespace.
    fn delimiter_before(&self, output: &str, ctx: &Context) -> Option<char> {
        let mut tracker = self.0.borrow_mut();
        let run = tracker.frames.last()?.run_start;
        let key = tracker.key.as_mut()?;
        let open = if key.owner == Owner::Run(run) {
            key.clean_after(output)
        } else {
            output.bytes().all(|byte| byte.is_ascii_whitespace())
        };
        (open && key.place == ListPlace::of(ctx)).then_some(key.delimiter)
    }
}

/// The delimiter of an ordered list that starts at the end of `output`: `)` right after an
/// ordered list that wrote `.`, so the two stay two lists (issue #666).
pub fn switched_delimiter(output: &str, ctx: &Context) -> Option<char> {
    // ~keep A list in heading text, inline text or text between markers is text: it continues
    // ~keep nothing.
    (!ctx.convert_as_inline
        && ctx.inline_depth == 0
        && !ctx.text_in_markers
        && !ctx.in_marker_span
        && ctx.last_list.delimiter_before(output, ctx) == Some('.'))
    .then_some(')')
}

/// Whether a paragraph is open before the line at `line_start`, stored for the next marker line.
fn paragraph_is_open_before(
    output: &str,
    line_start: usize,
    enclosing_column: usize,
    (previous, buffer): (&PreviousMarker, usize),
) -> bool {
    let open = paragraph_is_open(
        &output[..line_start],
        enclosing_column,
        previous.get(buffer, output, enclosing_column),
    );
    previous.set(buffer, output, line_start, enclosing_column, open);
    open
}

/// Whether a paragraph is open at the end of `output` in the item whose content starts at
/// `enclosing_column`: only then must a marker line interrupt it to start a list item.
///
/// ~keep `CommonMark` checks the interrupt rule only when the deepest open block a line reaches
/// ~keep is a paragraph (issue #633). The lines within 3 columns of the item's content column
/// ~keep decide it; a deeper line is inside the block above it. A blank line, or a line that
/// ~keep opens a block, leaves no paragraph open: after a list item's line the open block is
/// ~keep the list.
/// ~keep A marker line that cannot interrupt a paragraph is an item only when none was open
/// ~keep before it, so the walk looks past it. Any other line is paragraph text, and so is the
/// ~keep buffer's first line, which follows the opening marker.
fn paragraph_is_open(output: &str, enclosing_column: usize, previous: Option<(usize, bool)>) -> bool {
    use crate::converter::utility::escaping::{leading_indent, opens_block};
    let text = output.strip_suffix('\n').unwrap_or(output);
    let mut end = text.len();
    loop {
        let start = text[..end].rfind('\n').map_or(0, |pos| pos + 1);
        if start == 0 {
            return true;
        }
        let line = &text[start..end];
        end = start - 1;
        if line.trim().is_empty() {
            return false;
        }
        let (indent, column) = leading_indent(line);
        if column < enclosing_column {
            return true;
        }
        if column - enclosing_column >= 4 {
            continue;
        }
        let rest = &line[indent..];
        if opens_block(rest) {
            return false;
        }
        if strip_leading_bare_marker(rest).is_none() {
            return true;
        }
        // ~keep The previous marker line of this item's lists: its own check already walked
        // ~keep back from here, so the walk stays linear in the number of items.
        if let Some((_, open)) = previous.filter(|&(line_start, _)| line_start == start) {
            return open;
        }
    }
}

/// The answer of the paragraph check at the previous marker line of an item's lists: the address
/// of the buffer, the line start, the line before it, the enclosing content column and whether a
/// paragraph was open before the line.
///
/// ~keep The lists share it through the context. The buffer address, the enclosing content column
/// ~keep and the line before the marker line must match: in another buffer, in a list at another
/// ~keep depth, or after a write that changed the end of the buffer, the line start means nothing.
#[derive(Clone, Default)]
pub struct PreviousMarker(std::rc::Rc<std::cell::RefCell<Option<PreviousMarkerState>>>);

struct PreviousMarkerState {
    buffer: usize,
    line_start: usize,
    line_before: String,
    enclosing_column: usize,
    open: bool,
}

impl PreviousMarker {
    fn get(&self, buffer: usize, output: &str, enclosing_column: usize) -> Option<(usize, bool)> {
        let state = self.0.borrow();
        let state = state.as_ref()?;
        (state.buffer == buffer
            && state.enclosing_column == enclosing_column
            && output
                .get(..state.line_start)
                .is_some_and(|before| before.ends_with(state.line_before.as_str())))
        .then_some((state.line_start, state.open))
    }

    fn set(&self, buffer: usize, output: &str, line_start: usize, enclosing_column: usize, open: bool) {
        let before = &output[..line_start];
        let line_before = &before[before.trim_end_matches('\n').rfind('\n').map_or(0, |pos| pos + 1)..];
        *self.0.borrow_mut() = Some(PreviousMarkerState {
            buffer,
            line_start,
            line_before: line_before.to_string(),
            enclosing_column,
            open,
        });
    }
}

/// If this list is immediately preceded by an HTML comment whose own immediately preceding
/// sibling is a list of this same tag (`ul`/`ol`), return that comment's literal source text.
///
/// ~keep `CommonMark` merges two adjacent lists of the same type into one list unless
/// ~keep something else -- and per the spec, only a raw HTML comment qualifies -- sits between
/// ~keep them. This converter otherwise drops every HTML comment unconditionally (a real
/// ~keep content-preservation policy for stray markup elsewhere), but dropping THIS one
/// ~keep specific comment discards the only thing keeping the two lists apart, so it un-merges
/// ~keep them on every reparse and the next conversion pass never recovers a matching
/// ~keep separator (spec example 308). A comment anywhere else (inline text, the sole content
/// ~keep of a block) is unrelated to this ambiguity and keeps the existing strip behavior --
/// ~keep this check only fires for the exact position where CommonMark assigns the comment
/// ~keep separator meaning.
pub fn preceding_same_type_list_separator_comment(
    node_handle: tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &DomContext,
    tag_name: &str,
) -> Option<String> {
    let id = node_handle.get_inner();
    let siblings = match dom_ctx.parent_of(id) {
        Some(parent_id) => dom_ctx.children_of(parent_id)?,
        None => &dom_ctx.root_children,
    };
    let position = dom_ctx
        .sibling_index(id)
        .or_else(|| siblings.iter().position(|handle| handle.get_inner() == id))?;

    // ~keep The source text between two block siblings (e.g. the "\n" between `</ul>` and
    // ~keep `<!-- -->`) parses as its own whitespace-only Raw sibling node -- skip those to
    // ~keep find the nearest MEANINGFUL sibling on each side, exactly like
    // ~keep `get_previous_sibling_tag` does for the tag-name-only lookup.
    let mut cursor = position;
    let comment_text = loop {
        cursor = cursor.checked_sub(1)?;
        match siblings.get(cursor)?.get(parser) {
            Some(tl::Node::Comment(bytes)) => break bytes.as_utf8_str().into_owned(),
            Some(tl::Node::Raw(raw)) if raw.as_utf8_str().trim().is_empty() => {}
            _ => return None,
        }
    };

    let previous_list_name = loop {
        cursor = cursor.checked_sub(1)?;
        let sibling = *siblings.get(cursor)?;
        if let Some(tl::Node::Raw(raw)) = sibling.get(parser) {
            if raw.as_utf8_str().trim().is_empty() {
                continue;
            }
        }
        break resolve_tag_name(sibling, parser, dom_ctx)?;
    };

    if previous_list_name == tag_name {
        Some(comment_text)
    } else {
        None
    }
}

/// Strip one bare list marker -- a single bullet char (`-`, `*`, `+`) followed by a space,
/// or one-or-more ASCII digits followed by `". "` or `") "` -- from the front of `text`,
/// returning what remains after it. Returns `None` when `text` does not start with a marker.
fn strip_leading_bare_marker(text: &str) -> Option<&str> {
    let digit_count = text.bytes().take_while(u8::is_ascii_digit).count();
    if digit_count > 0 {
        let rest = &text[digit_count..];
        if let Some(rest) = rest.strip_prefix(". ").or_else(|| rest.strip_prefix(") ")) {
            return Some(rest);
        }
    }
    let mut chars = text.chars();
    let first = chars.next()?;
    if matches!(first, '-' | '*' | '+') {
        return chars.as_str().strip_prefix(' ');
    }
    None
}

/// Whether the current line of `output` (from the last `\n`, or the very start of the
/// buffer) is nothing but one or more bare list markers -- concatenated bullets
/// (`"- "`, `"* "`, `"+ "`) and/or ordered markers (`"N. "`) -- with optional leading
/// indentation and no other content.
///
/// ~keep A plain suffix check like `output.ends_with("* ")` also matches the closing
/// ~keep `"**"` of `<strong>` (or the closing `"*"` of `<em>`) immediately followed by a
/// ~keep migrated trailing space, e.g. `"**b** "`: its last two bytes are literally `'*'`
/// ~keep and `' '`, indistinguishable by suffix alone from a real bare `"* "` bullet. That
/// ~keep false positive suppressed the newline before a nested list, flattening it onto
/// ~keep the parent line and destroying it on reparse. Requiring the WHOLE line (after
/// ~keep stripping only leading indentation) to decompose into nothing but marker tokens
/// ~keep rules that out: real inline content preceding a marker-looking tail is not itself
/// ~keep a marker, so the decomposition fails and the check correctly returns `false`. This
/// ~keep also naturally handles several single-child lists nested directly inside each
/// ~keep other, whose bare markers stack on one physical line with nothing else between
/// ~keep them (CommonMark spec example 299: `"1. - 2. foo"`).
///
/// ~keep `pub` (not `pub(crate)`, which clippy's `redundant_pub_crate` flags here since
/// ~keep `list::utils` is itself only `pub` within the crate): the same ambiguity affects
/// ~keep the "is this block the item's first content, or a continuation?" decision in
/// ~keep `handlers/blockquote.rs`, `handlers/code_block.rs`, and `block/div.rs`, which now
/// ~keep reuse this instead of repeating a hardcoded, `+`-missing, two-byte suffix check.
pub fn line_is_bare_list_marker(output: &str) -> bool {
    let line_start = output.rfind('\n').map_or(0, |pos| pos + 1);
    let mut rest = output[line_start..].trim_start_matches([' ', '\t']);
    if rest.is_empty() {
        return false;
    }
    while let Some(next) = strip_leading_bare_marker(rest) {
        if next.is_empty() {
            return true;
        }
        rest = next;
    }
    false
}

/// Whether the list item that `output` ends inside is still open: the item is open where this
/// buffer starts (`ctx.list_item_open`), and every non-blank line since the item's marker line
/// starts at the item's content column (`indent`). The buffer's first line counts as at the
/// column: the container that owns the buffer puts it there.
///
/// ~keep A line at a shallower column is a block that already left the item, and nothing
/// ~keep reopens it. Writing the content column after it opens an indented code block once the
/// ~keep column is 4 or more (issue #583).
/// ~keep Each block of an item asks this, so the answer for the lines before the last one is
/// ~keep kept in the item's context and the next check reads only the lines written since
/// ~keep (issue #649).
pub fn item_is_open(output: &str, indent: &str, ctx: &Context) -> bool {
    if !ctx.list_item_open {
        return false;
    }
    let content = output.trim_end();
    if content.is_empty() {
        return true;
    }
    let last_start = content.rfind('\n').map_or(0, |pos| pos + 1);
    let before = lines_are_open(&output[..last_start], indent, &ctx.item_lines);
    let last = output[last_start..].split('\n').next().unwrap_or_default();
    match before {
        _ if strip_leading_bare_marker(last.trim_start_matches([' ', '\t'])).is_some() => true,
        None => true,
        Some(_) if indent.is_empty() || !last.starts_with(indent) => false,
        Some(open) => open,
    }
}

/// Whether the item is open after the complete lines `output` (it ends at a line start), or
/// `None` when they are all blank. Stores the answer in `scan` for the next check.
fn lines_are_open(output: &str, indent: &str, scan: &ItemLineScan) -> Option<bool> {
    let buffer = output.as_ptr() as usize;
    let (start, below) = scan
        .read(buffer, indent, output)
        .unwrap_or_else(|| (last_marker_line_start(output), None));
    let mut open = below;
    for line in output[start..].split('\n').filter(|line| !line.trim().is_empty()) {
        open = Some(match open {
            _ if strip_leading_bare_marker(line.trim_start_matches([' ', '\t'])).is_some() => true,
            None => true,
            Some(_) if indent.is_empty() || !line.starts_with(indent) => false,
            Some(open) => open,
        });
    }
    scan.write(buffer, indent, output, open);
    open
}

/// The start of the last line of `output` that is a bare list marker line, or 0 when there is none.
///
/// ~keep A marker line opens the item whatever the lines before it say, so a check with no stored
/// ~keep answer reads forward from there: it reads the lines of the innermost item, not the buffer.
fn last_marker_line_start(output: &str) -> usize {
    let mut end = output.len();
    while end > 0 {
        let start = output[..end - 1].rfind('\n').map_or(0, |pos| pos + 1);
        if strip_leading_bare_marker(output[start..end].trim_start_matches([' ', '\t'])).is_some() {
            return start;
        }
        end = start;
    }
    0
}

/// The answer of the last check of an item's lines in one buffer: the buffer address, the
/// indent, the end of the lines it covers, the last of those lines, and the answer.
///
/// ~keep The walk reads lines forward from the item's start, so the answer after a line depends
/// ~keep only on the answer before it and the line itself; the next check starts from the stored
/// ~keep end. Only the end of a buffer changes after it is written, and it changes by whitespace
/// ~keep and line breaks. The stored last line must still be there: a later write that removed
/// ~keep it removed the end the answer covers.
#[derive(Clone, Default)]
pub struct ItemLineScan(std::rc::Rc<std::cell::RefCell<Option<ItemLineScanState>>>);

struct ItemLineScanState {
    buffer: usize,
    indent: String,
    end: usize,
    last_line_start: usize,
    last_line: String,
    open: Option<bool>,
}

impl ItemLineScan {
    /// A scan for a quote or a container that writes a buffer of its own: the answer kept for
    /// the enclosing buffer stays.
    pub fn new_item() -> Self {
        Self::default()
    }

    fn read(&self, buffer: usize, indent: &str, output: &str) -> Option<(usize, Option<bool>)> {
        let state = self.0.borrow();
        let state = state.as_ref()?;
        (state.buffer == buffer
            && state.indent == indent
            && output.get(state.last_line_start..state.end) == Some(state.last_line.as_str()))
        .then_some((state.end, state.open))
    }

    fn write(&self, buffer: usize, indent: &str, output: &str, open: Option<bool>) {
        let last_line_start = output.trim_end().rfind('\n').map_or(0, |pos| pos + 1);
        *self.0.borrow_mut() = Some(ItemLineScanState {
            buffer,
            indent: indent.to_string(),
            end: output.len(),
            last_line_start,
            last_line: output[last_line_start..].to_string(),
            open,
        });
    }
}

/// The context for the children of a container that renders them into a buffer of its own:
/// they see whether the list item is still open where that buffer will be written.
pub fn nested_block_context(output: &str, ctx: &Context, options: &ConversionOptions) -> Context {
    let indent = continuation_indent_string(ctx.list_indent_columns, options).unwrap_or_default();
    Context {
        list_item_open: item_is_open(output, &indent, ctx),
        item_lines: ItemLineScan::new_item(),
        ..ctx.clone()
    }
}

/// Trim whitespace that follows a bare list marker at the end of `output` back to the marker's
/// own space, and say whether `output` ends in a bare marker afterwards.
///
/// ~keep Whitespace-only text after the marker (kept in strict whitespace mode) is not content:
/// ~keep counting it made the first block of the item start after a blank line, which ends the
/// ~keep item in `CommonMark` (issue #583).
pub fn trim_whitespace_after_bare_marker(output: &mut String) -> bool {
    let content_end = output.trim_end().len();
    if content_end == output.len() {
        return line_is_bare_list_marker(output);
    }
    let tail = output[content_end..].to_string();
    output.truncate(content_end);
    output.push(' ');
    if line_is_bare_list_marker(output) {
        return true;
    }
    output.truncate(content_end);
    output.push_str(&tail);
    false
}

/// Start a block inside a list item: on the marker line when the item has no content yet, after
/// a blank line at the content column while the item is open, and after a blank line at the
/// start of the line once the item has ended (issue #583).
pub fn start_block_in_list_item(output: &mut String, ctx: &Context, options: &ConversionOptions) {
    if trim_whitespace_after_bare_marker(output) {
        return;
    }
    let indent = continuation_indent_string(ctx.list_indent_columns, options).unwrap_or_default();
    if item_is_open(output, &indent, ctx) {
        add_list_continuation_indent(output, ctx.list_indent_columns, true, options);
    } else {
        trim_trailing_whitespace(output);
        if !output.ends_with("\n\n") {
            output.push_str(if output.ends_with('\n') { "\n" } else { "\n\n" });
        }
    }
}

/// Add appropriate leading separator before a list.
///
/// Lists need different separators depending on context:
/// - In table cells: a line-break separator if there's already content -- a literal `<br>`
///   under `br_in_tables`, otherwise a single space (see `emit_table_cell_break`)
/// - Outside lists: blank line (\n\n) if needed
/// - Inside list items: blank line before nested list
///
/// ~keep A GFM pipe cell cannot hold a literal newline, so the two adjacent `<li>`s (or the
/// ~keep prior cell content and this list) need an in-cell substitute for the line break a
/// ~keep block boundary would normally get. `br_in_tables` governs exactly this substitution
/// ~keep everywhere else it applies (`main_helpers::emit_table_cell_break`, consulted by
/// ~keep `<br>`/`<div>`/`<p>` continuations in a cell) — reusing it here keeps list-in-cell
/// ~keep breaks consistent with every other break-in-cell site instead of hardcoding `<br>`
/// ~keep regardless of the option (issue: `br_in_tables: false` was ignored for list items
/// ~keep sharing a table cell).
pub fn add_list_leading_separator(output: &mut String, ctx: &Context, options: &ConversionOptions) {
    if ctx.in_table_cell {
        let is_table_continuation =
            !output.is_empty() && !output.ends_with('|') && !output.ends_with(' ') && !output.ends_with("<br>");
        if is_table_continuation {
            crate::converter::main_helpers::emit_table_cell_break(output, options.br_in_tables);
        }
        return;
    }

    if !output.is_empty() && !ctx.in_list {
        let needs_newline = !output.ends_with("\n\n") && !line_is_bare_list_marker(output);
        if needs_newline {
            output.push_str("\n\n");
        }
        return;
    }

    if ctx.in_list_item && !output.is_empty() {
        if line_is_bare_list_marker(output) {
            return;
        }

        // ~keep A loose list wraps every item's leading text in a real <p> on any
        // ~keep CommonMark-compliant reparse (looseness is a per-list, not per-item,
        // ~keep property), and `block/paragraph.rs` always follows a <p> with a blank line
        // ~keep even inside a list item. So a nested list that is this item's next sibling
        // ~keep needs that same blank line here when the CONTAINING list is loose, even
        // ~keep though the leading text itself arrived as bare inline text with no <p> --
        // ~keep otherwise this pass's tighter join reparses with the blank line the loose
        // ~keep list demands, moving the nested list's `<p>`-wrapped leading item further
        // ~keep from a fixpoint instead of closer (spec example 319).
        if ctx.loose_list {
            trim_trailing_whitespace(output);
            if !output.ends_with("\n\n") {
                if output.ends_with('\n') {
                    output.push('\n');
                } else {
                    output.push_str("\n\n");
                }
            }
        } else if !output.ends_with('\n') {
            trim_trailing_whitespace(output);
            output.push('\n');
        }
    }
}

/// Add appropriate trailing separator after a nested list.
///
/// Nested lists inside list items need trailing newlines to separate
/// from following content. In loose lists, use blank line (\n\n). In tight lists, single newline (\n).
pub fn add_nested_list_trailing_separator(output: &mut String, ctx: &Context) {
    if !ctx.in_list_item {
        return;
    }

    if ctx.loose_list {
        if !output.ends_with("\n\n") {
            if !output.ends_with('\n') {
                output.push('\n');
            }
            output.push('\n');
        }
    } else if !output.ends_with('\n') {
        output.push('\n');
    }
}

/// Calculate the nesting depth for a list.
///
/// If we're in a list but NOT in a list item, this is incorrectly nested HTML
/// and we need to increment the depth. If in a list item, the depth was already
/// incremented by the <li> element.
pub const fn calculate_list_nesting_depth(ctx: &Context) -> usize {
    if ctx.in_list && !ctx.in_list_item {
        ctx.list_depth + 1
    } else {
        ctx.list_depth
    }
}

/// Check if a node is a list item element.
pub fn is_list_item(node_handle: tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    if let Some(info) = dom_ctx.tag_info(node_handle.get_inner(), parser) {
        return info.name == "li";
    }
    matches!(
        node_handle.get(parser),
        Some(tl::Node::Tag(tag)) if tag_name_eq(tag.name().as_utf8_str(), "li")
    )
}

/// Process a list's children, tracking which items had block elements.
///
/// This is used to determine proper spacing between list items.
/// Returns true if the last processed item had block children.
#[allow(clippy::too_many_arguments)]
pub fn process_list_children(
    node_handle: tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    is_ordered: bool,
    is_loose: bool,
    nested_depth: usize,
    start_counter: i64,
    delimiter: Option<char>,
    dom_ctx: &DomContext,
) {
    let mut counter = start_counter;
    let mut counter_saturated = false;
    let mut first_item = true;

    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let children = tag.children();
        {
            // ~keep Build the per-list context once; only `list_counter` varies
            // ~keep per iteration, so mutate that field in place instead of
            // ~keep cloning ctx for every <li>.  Tier-2 hot-spot pass III.
            let mut list_ctx = Context {
                in_ordered_list: is_ordered,
                list_counter: if is_ordered { counter } else { 0 },
                in_list: true,
                list_depth: nested_depth,
                ul_depth: if is_ordered { ctx.ul_depth } else { ctx.ul_depth + 1 },
                loose_list: is_loose,
                prev_item_had_blocks: false,
                ordered_delimiter: delimiter,
                ..ctx.clone()
            };

            for child_handle in children.top().iter() {
                if let Some(tl::Node::Raw(bytes)) = child_handle.get(parser) {
                    if bytes.as_utf8_str().trim().is_empty() {
                        continue;
                    }
                }

                if is_ordered {
                    list_ctx.list_counter = counter;
                }
                // ~keep A first marker that cannot interrupt the paragraph before it starts after a
                // ~keep blank line (issues #662, #670). Between inline markers the list is text. The
                // ~keep marker character does not change the answer, so `N. x` and `- x` stand for
                // ~keep every item with content. Whether the item has content is known once it is
                // ~keep written, so a marker line that could interrupt is checked again then (issue
                // ~keep #667), except in a marker-only wrapper's text. The paragraph is read once
                // ~keep per list either way.
                let mut marker_line_start = None;
                let mut bare_marker_end = None;
                if first_item && is_list_item(*child_handle, parser, dom_ctx) {
                    first_item = false;
                    let item_block = ctx.in_list_item && ctx.inline_depth == 0 && !ctx.text_in_markers;
                    if item_block && line_is_bare_list_marker(output) {
                        bare_marker_end = Some(output.len());
                    } else if item_block && output.ends_with('\n') {
                        let marker = if is_ordered {
                            format!("{counter}. x")
                        } else {
                            String::from("- x")
                        };
                        if list_needs_blank_line(
                            (output, output.len()),
                            &marker,
                            ctx.real_item_columns,
                            (&ctx.previous_marker, std::ptr::from_ref::<String>(output) as usize),
                            options,
                        ) {
                            output.push('\n');
                        } else if !ctx.in_marker_span && marker_line_interrupts(&marker, options) {
                            marker_line_start = Some(output.len());
                        }
                    }
                }

                use crate::converter::walk_node;
                walk_node(child_handle, parser, output, options, &list_ctx, depth + 1, dom_ctx);

                if let Some(line_start) = marker_line_start {
                    let line = written_marker_line(output, line_start);
                    if list_needs_blank_line(
                        (output, line_start),
                        line,
                        ctx.real_item_columns,
                        (&ctx.previous_marker, std::ptr::from_ref::<String>(output) as usize),
                        options,
                    ) {
                        output.insert(line_start, '\n');
                    }
                }
                // ~keep An empty item that ends a line of bare markers can complete a thematic
                // ~keep break (`- - -`), so its marker starts the next line instead, at the column
                // ~keep it has on this line (issue #667).
                if let Some(end) = bare_marker_end {
                    if bare_marker_line_is_rule(output, end) {
                        let indent = column_of_written_marker(output, end);
                        output.replace_range(end - 1..end, &format!("\n{indent}"));
                    }
                }

                if is_ordered && is_list_item(*child_handle, parser, dom_ctx) {
                    if counter == i64::MAX {
                        if !counter_saturated {
                            tracing::warn!(
                                target: "html_to_markdown::list",
                                counter,
                                "ordered list counter reached i64::MAX; subsequent items repeat this value"
                            );
                            counter_saturated = true;
                        }
                    } else {
                        counter += 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Render `first` as a child of a root node with an ordered list at its end, then enter a
    /// second child whose output is the same `String` holding `second`, and return the owner of
    /// the key at that point.
    fn owner_after_sibling_enters(first: &str, second: &str) -> Option<Owner> {
        let list = LastList::default();
        let root = String::new();
        let mut buffer = String::from(first);
        list.enter(&root);
        list.enter(&buffer);
        let place = ListPlace { columns: 0, quotes: 0 };
        let address = std::ptr::from_ref::<String>(&buffer) as usize;
        list.0.borrow_mut().key = Some(ListEnd::new(Owner::Run(1), address, &buffer, place, '.'));
        list.leave(&buffer);
        buffer.clear();
        buffer.push_str(second);
        list.enter(&buffer);
        list.0.borrow().key.as_ref().map(|key| key.owner)
    }

    #[test]
    fn a_later_child_takes_the_list_end_back_only_from_the_same_buffer() {
        assert_eq!(owner_after_sibling_enters("1. a\n", "1. a\n\n"), Some(Owner::Run(1)));
        // ~keep A new buffer at the reused address does not hold the list's bytes at the end offset.
        assert_eq!(
            owner_after_sibling_enters("1. a\n", "zzzzzz\n"),
            Some(Owner::Waiting(Some(0)))
        );
        assert_eq!(owner_after_sibling_enters("1. a\n", "1. a\nx"), None);
    }

    #[test]
    fn a_buffer_that_no_longer_reaches_the_list_end_is_not_clean() {
        let place = ListPlace { columns: 0, quotes: 0 };
        let mut key = ListEnd::new(Owner::Run(0), 0, "1. a\n", place, '.');
        assert!(key.clean_after("1. a\n\n"));
        assert!(!key.clean_after("1."));
    }

    #[test]
    fn a_later_child_with_another_buffer_leaves_the_list_end_waiting() {
        let list = LastList::default();
        let root = String::new();
        let first = String::from("1. a\n");
        let second = first.clone();
        list.enter(&root);
        list.enter(&first);
        let place = ListPlace { columns: 0, quotes: 0 };
        let address = std::ptr::from_ref::<String>(&first) as usize;
        list.0.borrow_mut().key = Some(ListEnd::new(Owner::Run(1), address, &first, place, '.'));
        list.leave(&first);
        list.enter(&second);
        let owner = list.0.borrow().key.as_ref().map(|key| key.owner);
        assert_eq!(owner, Some(Owner::Waiting(Some(0))));
    }

    #[test]
    fn item_line_scan_reads_the_lines_again_after_its_last_line_changed() {
        let scan = ItemLineScan::new_item();
        let mut output = String::from("- a\n  b\n");
        assert_eq!(lines_are_open(&output, "  ", &scan), Some(true));
        output.clear();
        output.push_str("a\nb\nzzz\n");
        assert_eq!(lines_are_open(&output, "  ", &scan), Some(false));
    }

    #[test]
    fn item_line_scan_reads_the_lines_of_another_buffer_again() {
        let scan = ItemLineScan::new_item();
        let first = String::from("- a\n  b\n");
        let second = String::from("a\nb\n  b\n");
        assert_eq!(lines_are_open(&first, "  ", &scan), Some(true));
        assert_eq!(lines_are_open(&second, "  ", &scan), Some(false));
    }

    #[test]
    fn item_line_scan_reads_the_lines_again_for_another_indent() {
        let scan = ItemLineScan::new_item();
        let output = String::from("- a\n  b\n");
        assert_eq!(lines_are_open(&output, "  ", &scan), Some(true));
        assert_eq!(lines_are_open(&output, "    ", &scan), Some(false));
    }

    #[test]
    fn previous_marker_answers_only_for_its_buffer_column_and_line_before_its_marker_line() {
        let previous = PreviousMarker::default();
        previous.set(1, "p\n- a\n", 2, 0, true);
        assert_eq!(previous.get(1, "p\n- a\n- b\n", 0), Some((2, true)));
        assert_eq!(previous.get(1, "p\n- a\n- b\n", 2), None);
        assert_eq!(previous.get(2, "p\n- a\n- b\n", 0), None);
        assert_eq!(previous.get(1, "q\n- a\n- b\n", 0), None);
    }
}
