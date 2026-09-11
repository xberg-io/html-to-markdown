//! Cell and row handling for Markdown conversion.
//!
//! Provides functionality for processing table cells and rows, including:
//! - Row conversion to Markdown format
//! - Cell layout handling with colspan/rowspan support
//! - Layout table row conversion to list items

use crate::converter::utility::content::normalized_tag_name;
use std::borrow::Cow;
use std::collections::HashMap;

use super::cell::{cell_text_content, collect_table_cells, convert_table_cell, emit_cell_text, get_colspan_rowspan};

/// Maximum allowed table columns to prevent unbounded memory usage.
const MAX_TABLE_COLS: usize = 1000;

/// Rendered markdown of each cell visited by the column-width pre-pass, keyed by DOM node id.
///
/// ~keep The pre-pass and the render pass walk the same cell subtrees, so the pre-pass result
/// ~keep can be emitted directly instead of rendering twice — but only where the two passes are
/// ~keep provably byte-identical. The pre-pass context sets `measure_width_only` (nested tables
/// ~keep degrade to raw text, issue #406) and `skip_visitor_hooks`, and a second walk is
/// ~keep observable through any collector handle shared via the context. The caller decides via
/// ~keep `enabled` (see `builder::cell_text_reuse_allowed`); when disabled this stores nothing.
pub struct CellTextCache {
    entries: HashMap<u32, String>,
    enabled: bool,
}

impl CellTextCache {
    /// Create a cache; `enabled == false` makes every store a no-op and every lookup a miss.
    pub fn new(enabled: bool) -> Self {
        Self {
            entries: HashMap::new(),
            enabled,
        }
    }

    fn store(&mut self, node_id: u32, text: String) {
        if self.enabled {
            self.entries.insert(node_id, text);
        }
    }

    fn take(&mut self, node_id: u32) -> Option<String> {
        if self.enabled {
            self.entries.remove(&node_id)
        } else {
            None
        }
    }
}

/// Read-only conversion inputs threaded together through row/cell rendering: the parser, user
/// options, conversion `Context`, and DOM lookup context.
///
/// ~keep These four are always passed as a group across this module's row-rendering functions;
/// ~keep bundling them keeps each function's own signature down to its row/cell-specific
/// ~keep parameters instead of repeating all four everywhere (too-many-parameters).
#[derive(Clone, Copy)]
pub struct RowEnv<'p> {
    pub parser: &'p tl::Parser<'p>,
    pub options: &'p crate::options::ConversionOptions,
    pub ctx: &'p super::super::super::Context,
    pub dom_ctx: &'p super::super::super::DomContext,
}

/// Append one layout-table cell's converted text to `row_text`, space-joining after the first
/// non-empty cell.
///
/// Extracted from `append_layout_row`'s per-cell loop body — identical cell-name check,
/// inline-image handling, and whitespace trim, unchanged.
fn append_layout_cell_text(cell_handle: &tl::NodeHandle, row_text: &mut String, env: RowEnv<'_>, depth: usize) {
    let Some(tl::Node::Tag(cell_tag)) = cell_handle.get(env.parser) else {
        return;
    };
    let cell_name: Cow<'_, str> = env.dom_ctx.tag_info(cell_handle.get_inner(), env.parser).map_or_else(
        || normalized_tag_name(cell_tag.name().as_utf8_str()).into_owned().into(),
        |info| Cow::Borrowed(info.name.as_str()),
    );
    if !matches!(cell_name.as_ref(), "td" | "th" | "cell") {
        return;
    }

    let mut cell_text = String::new();
    // ~keep issue #433: honor keep_inline_images_in for images in
    // ~keep layout-table cells even though cell content is converted
    // ~keep as inline. A matching cell tag keeps images as markdown.
    let cell_allow_inline_images = env.ctx.keep_inline_images_in.contains(cell_name.as_ref());
    let cell_ctx = super::super::super::Context {
        convert_as_inline: true,
        in_layout_cell: true,
        cell_allow_inline_images,
        ..env.ctx.clone()
    };
    let cell_children = cell_tag.children();
    for cell_child in cell_children.top().iter() {
        super::super::super::walk_node(
            cell_child,
            env.parser,
            &mut cell_text,
            env.options,
            &cell_ctx,
            depth + 1,
            env.dom_ctx,
        );
    }
    let cell_content = crate::text::normalize_whitespace_cow(&cell_text);
    if !cell_content.trim().is_empty() {
        if !row_text.is_empty() {
            row_text.push(' ');
        }
        row_text.push_str(cell_content.trim());
    }
}

/// Append a layout table row as a list item.
///
/// For tables used for visual layout, converts rows to list items
/// instead of table format for better readability.
///
/// # Arguments
/// * `row_handle` - Handle to the row element
/// * `output` - Mutable string to append content
/// * `env` - Parser, options, conversion context, and DOM context
/// * `depth` - Current recursion depth (the row's own depth; cell content is walked at `depth + 1`)
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn append_layout_row(row_handle: &tl::NodeHandle, output: &mut String, env: RowEnv<'_>, depth: usize) {
    let Some(tl::Node::Tag(row_tag)) = row_handle.get(env.parser) else {
        return;
    };
    let mut row_text = String::new();
    let row_children = row_tag.children();
    for cell_handle in row_children.top().iter() {
        append_layout_cell_text(cell_handle, &mut row_text, env, depth);
    }

    let trimmed = row_text.trim();
    if !trimmed.is_empty() {
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }
        let marker = layout_row_bullet(env.options, env.ctx);
        let formatted = strip_leading_bullet(trimmed, env.options);
        output.push(marker);
        output.push(' ');
        output.push_str(formatted);
        output.push('\n');
    }
}

/// The list marker a layout row renders with.
///
/// A layout table's rows are list items, so they cycle through `options.bullets` by nesting
/// depth exactly as `list::item` does. The row sits one level deeper than its surroundings --
/// `ul_depth` is still the *enclosing* depth here, where `list::item` has already counted its
/// own `<ul>` -- so the index is `ul_depth` rather than `ul_depth - 1`. A layout table nested
/// in a list therefore takes the next marker instead of repeating its parent's (issue #472).
fn layout_row_bullet(options: &crate::options::ConversionOptions, ctx: &super::super::super::Context) -> char {
    let bullets: Vec<char> = options.bullets.chars().collect();
    if bullets.is_empty() {
        return '*';
    }
    bullets[ctx.ul_depth % bullets.len()]
}

/// Drop a leading list marker the cell content already produced, so the row's own marker is not
/// doubled up. Any configured bullet counts, not just `-`: the marker being stripped was written
/// by this same cycling rule, so hardcoding one character missed every other configured set.
fn strip_leading_bullet<'a>(trimmed: &'a str, options: &crate::options::ConversionOptions) -> &'a str {
    for bullet in options.bullets.chars().chain(['-', '*', '+']) {
        let mut marker = String::with_capacity(2);
        marker.push(bullet);
        marker.push(' ');
        if let Some(rest) = trimmed.strip_prefix(marker.as_str()) {
            return rest.trim_start();
        }
    }
    trimmed
}

/// Advance `col` past any columns whose rowspan tracker still has rows remaining, decrementing
/// each as it is skipped and clearing the tracker entry once exhausted.
///
/// Extracted from `collect_row_cell_widths`'s inner loop — identical decrement-and-clear logic,
/// unchanged.
fn skip_rowspan_columns(col: &mut usize, rowspan_tracker: &mut [Option<usize>]) {
    while *col < rowspan_tracker.len() {
        let Some(Some(remaining)) = rowspan_tracker.get_mut(*col) else {
            break;
        };
        if *remaining == 0 {
            break;
        }
        *remaining -= 1;
        if *remaining == 0 {
            rowspan_tracker[*col] = None;
        }
        *col += 1;
    }
}

/// Collect the rendered text content of every cell in a row for width calculation.
///
/// `rowspan_tracker` mirrors the tracker in `convert_table_row` so that spanned
/// columns are skipped in the width pre-pass just as they are skipped in rendering.
/// Pass a shared tracker across all row calls to correctly handle multi-row spans.
///
/// `depth` is the row's own recursion depth; cell content is measured at `depth + 1`.
///
/// # Arguments
/// * `node_handle` - Handle to the row element
/// * `env` - Parser, options, conversion context, and DOM context
/// * `col_widths` - Per-column max content widths accumulated so far
/// * `rowspan_tracker` - Mutable array tracking rowspan remainder for each column
/// * `cell_cache` - Rendered markdown to store for later reuse by the render pass
/// * `depth` - Row's own recursion depth
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn collect_row_cell_widths(
    node_handle: &tl::NodeHandle,
    env: RowEnv<'_>,
    col_widths: &mut Vec<usize>,
    rowspan_tracker: &mut Vec<Option<usize>>,
    cell_cache: &mut CellTextCache,
    depth: usize,
) {
    let mut cells = Vec::new();
    collect_table_cells(node_handle, env.parser, env.dom_ctx, &mut cells);

    let mut col = 0usize;
    let mut cell_iter = cells.iter();

    loop {
        skip_rowspan_columns(&mut col, rowspan_tracker);

        let Some(cell_handle) = cell_iter.next() else {
            break;
        };

        let text = cell_text_content(cell_handle, env.parser, env.options, env.ctx, env.dom_ctx, depth + 1);
        const MAX_CELL_WIDTH: usize = 200;
        let width = text.chars().count().min(MAX_CELL_WIDTH);
        cell_cache.store(cell_handle.get_inner(), text);

        if col >= col_widths.len() {
            col_widths.resize(col + 1, 0);
        }
        if width > col_widths[col] {
            col_widths[col] = width;
        }

        let (colspan, rowspan) = get_colspan_rowspan(cell_handle, env.parser);

        if rowspan > 1 {
            if col >= rowspan_tracker.len() {
                rowspan_tracker.resize(col + 1, None);
            }
            rowspan_tracker[col] = Some(rowspan - 1);
        }

        col = col.saturating_add(colspan);
    }
}

/// Emit one cell of a rendered row, reusing the pre-pass rendering when it is cached.
///
/// `env.ctx` is used as the cell's own context (already carrying `in_table_cell = true`).
/// `depth` is the cell's own recursion depth. `deferred_tables` is forwarded to
/// [`convert_table_cell`]; see [`super::cell::render_cell_text`] for what it does.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn emit_row_cell(
    cell_handle: &tl::NodeHandle,
    row_text: &mut String,
    env: RowEnv<'_>,
    col_width: Option<usize>,
    depth: usize,
    cell_cache: &mut CellTextCache,
    deferred_tables: Option<&mut Vec<String>>,
) {
    if let Some(text) = cell_cache.take(cell_handle.get_inner()) {
        emit_cell_text(cell_handle, env.parser, row_text, &text, col_width);
    } else {
        convert_table_cell(
            cell_handle,
            env.parser,
            row_text,
            env.options,
            env.ctx,
            "",
            env.dom_ctx,
            col_width,
            depth,
            deferred_tables,
        );
    }
}

/// Minimum separator dash count per column (matches `---`).
const MIN_SEPARATOR_DASHES: usize = 3;

/// If `col_index` still has pending rowspan rows, emit its blank continuation cell (padded to
/// `col_widths`) into `row_text` and advance `col_index` past it.
///
/// Returns `true` when a continuation cell was emitted (caller should retry from the top of the
/// loop) and `false` when `col_index` is not a pending rowspan continuation.
///
/// Extracted from `convert_table_row`'s has-span loop — identical padding and rowspan-tracker
/// bookkeeping, unchanged.
fn emit_rowspan_continuation(
    col_index: &mut usize,
    total_cols: usize,
    rowspan_tracker: &mut [Option<usize>],
    col_widths: &[usize],
    row_text: &mut String,
) -> bool {
    if *col_index >= total_cols {
        return false;
    }
    let Some(Some(remaining_rows)) = rowspan_tracker.get_mut(*col_index) else {
        return false;
    };
    if *remaining_rows == 0 {
        return false;
    }

    let width = col_widths.get(*col_index).copied();
    row_text.push(' ');
    if let Some(w) = width {
        for _ in 0..w {
            row_text.push(' ');
        }
    }
    row_text.push_str(" |");
    *remaining_rows -= 1;
    if *remaining_rows == 0 {
        rowspan_tracker[*col_index] = None;
    }
    *col_index += 1;
    true
}

/// Convert a table row (tr) to Markdown format.
///
/// Processes all cells in a row, handling colspan and rowspan for proper
/// column alignment. Renders header separator row after the first row.
/// Integrates with visitor pattern for custom row handling.
///
/// # Arguments
/// * `node_handle` - Handle to the row element
/// * `parser` - HTML parser instance
/// * `output` - Mutable string to append row content
/// * `options` - Conversion options
/// * `ctx` - Conversion context (visitor, etc)
/// * `row_index` - Index of this row in the table
/// * `has_span` - Whether table has colspan/rowspan
/// * `rowspan_tracker` - Mutable array tracking rowspan remainder for each column
/// * `total_cols` - Total columns in the table
/// * `header_cols` - Columns to render in separator row
/// * `dom_ctx` - DOM context
/// * `depth` - Nesting depth
/// * `is_header` - Whether this is a header row
/// * `col_widths` - Per-column max content widths for padding (empty = no padding)
/// * `cell_cache` - Markdown already rendered for these cells by the width pre-pass
/// * `deferred_tables` - Collects a nested table's markdown when this row's sole cell holds
///   one; see [`super::cell::render_cell_text`]. The caller renders these separately, after the
///   enclosing table, once every row has been processed (issue #484).
#[allow(clippy::too_many_arguments)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn convert_table_row(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::super::Context,
    row_index: usize,
    has_span: bool,
    rowspan_tracker: &mut [Option<usize>],
    total_cols: usize,
    header_cols: usize,
    dom_ctx: &super::super::super::DomContext,
    depth: usize,
    is_header: bool,
    col_widths: &[usize],
    cell_cache: &mut CellTextCache,
    deferred_tables: &mut Vec<String>,
) {
    let mut row_text = String::with_capacity(256);
    let mut cells = Vec::new();

    collect_table_cells(node_handle, parser, dom_ctx, &mut cells);
    // ~keep A nested table may only be deferred out of a cell that shares its row with no
    // ~keep other cell -- pulling it out of a row with a sibling would leave that sibling's
    // ~keep column position undefined (issue #469 locks the sibling-cell shape to the
    // ~keep existing flatten-and-escape behavior; issue #484 is the single-cell-row shape).
    let is_single_cell_row = cells.len() == 1;

    #[cfg(feature = "visitor")]
    let cell_contents: Vec<String> = if ctx.visitor.is_some() {
        // ~keep Same rule as the width pre-pass: this walk only feeds the `visit_table_row`
        // ~keep callback, the render pass below walks these cells again (a visitor disables
        // ~keep cell-text reuse), so every shared `Rc` collector is detached to keep exactly
        // ~keep one recording walk per cell.
        let mut collect_ctx = super::super::super::Context {
            in_table_cell: true,
            ..ctx.clone()
        };
        #[cfg(feature = "metadata")]
        {
            collect_ctx.metadata_collector = None;
        }
        collect_ctx.structure_collector = None;
        #[cfg(feature = "inline-images")]
        {
            collect_ctx.inline_collector = None;
        }
        cells
            .iter()
            .map(|cell_handle| {
                let mut text = String::new();
                if let Some(tl::Node::Tag(tag)) = cell_handle.get(parser) {
                    for child_handle in tag.children().top().iter() {
                        super::super::super::walk_node(
                            child_handle,
                            parser,
                            &mut text,
                            options,
                            &collect_ctx,
                            depth + 1,
                            dom_ctx,
                        );
                    }
                }
                crate::text::normalize_whitespace_cow(&text).trim().to_string()
            })
            .collect()
    } else {
        Vec::new()
    };

    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::TableRow,
                Cow::Borrowed("tr"),
                tag,
                depth,
                row_index,
                Some(Cow::Borrowed("table")),
                false,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_table_row(&node_ctx, &cell_contents, is_header)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Skip => return,
                VisitResult::Custom(custom) => {
                    output.push_str(&custom);
                    return;
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    return;
                }
                VisitResult::PreserveHtml => {
                    output.push_str(&super::super::super::serialize_node(node_handle, parser));
                    return;
                }
            }
        }
    }

    // ~keep Build the per-cell context once for the entire row.  Tier-2 hot-spot
    // ~keep pass III: avoids cloning `Context` (which holds several Rc<HashSet> and
    // ~keep optional collector handles) on every cell in wikipedia-class tables.
    let cell_ctx = super::super::super::Context {
        in_table_cell: true,
        ..ctx.clone()
    };

    let row_env = RowEnv {
        parser,
        options,
        ctx: &cell_ctx,
        dom_ctx,
    };

    let mut filled_cols = if has_span {
        let mut col_index = 0;
        let mut cell_iter = cells.iter();

        loop {
            if emit_rowspan_continuation(&mut col_index, total_cols, rowspan_tracker, col_widths, &mut row_text) {
                continue;
            }

            if let Some(cell_handle) = cell_iter.next() {
                let col_width = col_widths.get(col_index).copied();
                let deferred = is_single_cell_row.then_some(&mut *deferred_tables);
                emit_row_cell(
                    cell_handle,
                    &mut row_text,
                    row_env,
                    col_width,
                    depth + 1,
                    cell_cache,
                    deferred,
                );

                let (colspan, rowspan) = get_colspan_rowspan(cell_handle, parser);

                if rowspan > 1 && col_index < total_cols {
                    rowspan_tracker[col_index] = Some(rowspan - 1);
                }

                col_index = col_index.saturating_add(colspan);
            } else {
                break;
            }
        }
        col_index
    } else {
        for (cell_idx, cell_handle) in cells.iter().enumerate() {
            let col_width = col_widths.get(cell_idx).copied();
            let deferred = is_single_cell_row.then_some(&mut *deferred_tables);
            emit_row_cell(
                cell_handle,
                &mut row_text,
                row_env,
                col_width,
                depth + 1,
                cell_cache,
                deferred,
            );
        }
        cells.len()
    };

    // ~keep A ragged row with fewer actual cells than the table's widest row must still
    // ~keep declare `total_cols` columns: GFM requires the header row's cell count to match
    // ~keep the delimiter row exactly, or the whole construct fails to parse as a table at
    // ~keep all (issue #13). Padding every row keeps column counts consistent throughout.
    while filled_cols < total_cols {
        let width = col_widths.get(filled_cols).copied();
        row_text.push(' ');
        if let Some(w) = width {
            for _ in 0..w {
                row_text.push(' ');
            }
        }
        row_text.push_str(" |");
        filled_cols += 1;
    }

    output.push('|');
    output.push_str(&row_text);
    output.push('\n');

    let is_first_row = row_index == 0;
    if is_first_row {
        let total_cols = header_cols.clamp(1, MAX_TABLE_COLS);
        output.push_str("| ");
        for i in 0..total_cols {
            if i > 0 {
                output.push_str(" | ");
            }
            let dash_count = col_widths.get(i).copied().unwrap_or(0).max(MIN_SEPARATOR_DASHES);
            for _ in 0..dash_count {
                output.push('-');
            }
        }
        output.push_str(" |\n");
    }
}
