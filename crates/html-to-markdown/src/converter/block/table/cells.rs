//! Cell and row handling for Markdown conversion.
//!
//! Provides functionality for processing table cells and rows, including:
//! - Row conversion to Markdown format
//! - Cell layout handling with colspan/rowspan support
//! - Layout table row conversion to list items

use crate::converter::utility::content::normalized_tag_name;
use std::borrow::Cow;

use super::cell::{cell_text_content, collect_table_cells, convert_table_cell, get_colspan_rowspan};

/// Maximum allowed table columns to prevent unbounded memory usage.
const MAX_TABLE_COLS: usize = 1000;

/// Append a layout table row as a list item.
///
/// For tables used for visual layout, converts rows to list items
/// instead of table format for better readability.
///
/// # Arguments
/// * `row_handle` - Handle to the row element
/// * `parser` - HTML parser instance
/// * `output` - Mutable string to append content
/// * `options` - Conversion options
/// * `ctx` - Conversion context
/// * `dom_ctx` - DOM context
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn append_layout_row(
    row_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::super::Context,
    dom_ctx: &super::super::super::DomContext,
) {
    if let Some(tl::Node::Tag(row_tag)) = row_handle.get(parser) {
        let mut row_text = String::new();
        let row_children = row_tag.children();
        for cell_handle in row_children.top().iter() {
            if let Some(tl::Node::Tag(cell_tag)) = cell_handle.get(parser) {
                let cell_name: Cow<'_, str> = dom_ctx.tag_info(cell_handle.get_inner(), parser).map_or_else(
                    || normalized_tag_name(cell_tag.name().as_utf8_str()).into_owned().into(),
                    |info| Cow::Borrowed(info.name.as_str()),
                );
                if matches!(cell_name.as_ref(), "td" | "th" | "cell") {
                    let mut cell_text = String::new();
                    let cell_ctx = super::super::super::Context {
                        convert_as_inline: true,
                        ..ctx.clone()
                    };
                    let cell_children = cell_tag.children();
                    for cell_child in cell_children.top().iter() {
                        super::super::super::walk_node(
                            cell_child,
                            parser,
                            &mut cell_text,
                            options,
                            &cell_ctx,
                            0,
                            dom_ctx,
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
            }
        }

        let trimmed = row_text.trim();
        if !trimmed.is_empty() {
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
            let formatted = trimmed.strip_prefix("- ").unwrap_or(trimmed).trim_start();
            output.push_str("- ");
            output.push_str(formatted);
            output.push('\n');
        }
    }
}

/// Collect the rendered text content of every cell in a row for width calculation.
///
/// `rowspan_tracker` mirrors the tracker in `convert_table_row` so that spanned
/// columns are skipped in the width pre-pass just as they are skipped in rendering.
/// Pass a shared tracker across all row calls to correctly handle multi-row spans.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn collect_row_cell_widths(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::super::Context,
    dom_ctx: &super::super::super::DomContext,
    col_widths: &mut Vec<usize>,
    rowspan_tracker: &mut Vec<Option<usize>>,
) {
    let mut cells = Vec::new();
    collect_table_cells(node_handle, parser, dom_ctx, &mut cells);

    let mut col = 0usize;
    let mut cell_iter = cells.iter();

    loop {
        // Skip columns that are filled by a rowspan from a previous row.
        while col < rowspan_tracker.len() {
            if let Some(Some(remaining)) = rowspan_tracker.get_mut(col) {
                if *remaining > 0 {
                    *remaining -= 1;
                    if *remaining == 0 {
                        rowspan_tracker[col] = None;
                    }
                    col += 1;
                    continue;
                }
            }
            break;
        }

        let Some(cell_handle) = cell_iter.next() else {
            break;
        };

        let text = cell_text_content(cell_handle, parser, options, ctx, dom_ctx);
        let width = text.chars().count();

        // Grow the widths vec if needed.
        if col >= col_widths.len() {
            col_widths.resize(col + 1, 0);
        }
        if width > col_widths[col] {
            col_widths[col] = width;
        }

        let (colspan, rowspan) = get_colspan_rowspan(cell_handle, parser);

        // Record rowspan for future rows.
        if rowspan > 1 {
            if col >= rowspan_tracker.len() {
                rowspan_tracker.resize(col + 1, None);
            }
            rowspan_tracker[col] = Some(rowspan - 1);
        }

        col = col.saturating_add(colspan);
    }
}

/// Minimum separator dash count per column (matches `---`).
const MIN_SEPARATOR_DASHES: usize = 3;

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
) {
    let mut row_text = String::with_capacity(256);
    let mut cells = Vec::new();

    collect_table_cells(node_handle, parser, dom_ctx, &mut cells);

    #[cfg(feature = "visitor")]
    let cell_contents: Vec<String> = if ctx.visitor.is_some() {
        cells
            .iter()
            .map(|cell_handle| {
                let mut text = String::new();
                let cell_ctx = super::super::super::Context {
                    in_table_cell: true,
                    ..ctx.clone()
                };
                if let Some(tl::Node::Tag(tag)) = cell_handle.get(parser) {
                    for child_handle in tag.children().top().iter() {
                        super::super::super::walk_node(child_handle, parser, &mut text, options, &cell_ctx, 0, dom_ctx);
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

    if has_span {
        let mut col_index = 0;
        let mut cell_iter = cells.iter();

        loop {
            if col_index < total_cols {
                if let Some(Some(remaining_rows)) = rowspan_tracker.get_mut(col_index) {
                    if *remaining_rows > 0 {
                        let width = col_widths.get(col_index).copied();
                        row_text.push(' ');
                        if let Some(w) = width {
                            for _ in 0..w {
                                row_text.push(' ');
                            }
                        }
                        row_text.push_str(" |");
                        *remaining_rows -= 1;
                        if *remaining_rows == 0 {
                            rowspan_tracker[col_index] = None;
                        }
                        col_index += 1;
                        continue;
                    }
                }
            }

            if let Some(cell_handle) = cell_iter.next() {
                let col_width = col_widths.get(col_index).copied();
                convert_table_cell(cell_handle, parser, &mut row_text, options, ctx, "", dom_ctx, col_width);

                let (colspan, rowspan) = get_colspan_rowspan(cell_handle, parser);

                if rowspan > 1 && col_index < total_cols {
                    rowspan_tracker[col_index] = Some(rowspan - 1);
                }

                col_index = col_index.saturating_add(colspan);
            } else {
                break;
            }
        }
    } else {
        for (cell_idx, cell_handle) in cells.iter().enumerate() {
            let col_width = col_widths.get(cell_idx).copied();
            convert_table_cell(cell_handle, parser, &mut row_text, options, ctx, "", dom_ctx, col_width);
        }
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
