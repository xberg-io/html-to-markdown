//! Table element handler for HTML to Markdown conversion.
//!
//! This module provides specialized handling for table elements including:
//! - Table structure detection and scanning (TableScan)
//! - Row and cell conversion to Markdown table format
//! - Cell content processing with colspan/rowspan support
//! - Layout table detection (tables used for visual layout)
//! - Integration with the visitor pattern for custom table handling
//!
//! Tables are converted to Markdown pipe-delimited format with header separators.
//! Layout tables (tables without proper semantic headers) may be converted to lists
//! instead of tables for better readability.

pub mod builder;
pub mod caption;
pub mod cell;
pub mod cells;
pub mod layout;
pub mod scanner;
pub(super) mod utils;

// Re-export types from parent module for submodule access
pub use super::super::{Context, DomContext};

// Re-export for use in converter.rs
pub(crate) use builder::handle_table;
pub(crate) use caption::handle_caption;

/// Dispatches table element handling to the main convert_table function.
///
/// # Usage in converter.rs
/// ```text
/// if "table" == tag_name {
///     crate::converter::block::table::handle_table(
///         node_handle,
///         parser,
///         output,
///         options,
///         ctx,
///         dom_ctx,
///         depth,
///     );
///     return;
/// }
/// ```
pub fn dispatch_table_handler(
    tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::Context,
    depth: usize,
    dom_ctx: &super::super::DomContext,
) -> bool {
    match tag_name {
        "table" => {
            builder::handle_table(node_handle, parser, output, options, ctx, dom_ctx, depth);
            true
        }
        _ => false,
    }
}

/// Handles table output with context-aware formatting.
///
/// This function wraps `handle_table` with list indentation and whitespace management logic.
/// It handles two distinct contexts:
///
/// 1. **List Context** (`ctx.in_list_item = true`):
///    - Indents table content using `indent_table_for_list` for proper nesting
///    - Preserves special caption formatting (lines starting with `*`)
///    - Adds newlines for proper list item separation
///
/// 2. **Normal Context**:
///    - Ensures proper spacing with double newlines around tables
///    - Handles various output state scenarios (empty, ends with newline, etc.)
///
/// # Arguments
/// * `node_handle` - The table node handle
/// * `parser` - The HTML parser instance
/// * `output` - The output string being built
/// * `options` - Conversion options (includes list indent type)
/// * `ctx` - Conversion context (includes list state)
/// * `dom_ctx` - DOM context for tree structure info
/// * `depth` - Current nesting depth
pub(crate) fn handle_table_with_context(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::Context,
    dom_ctx: &super::super::DomContext,
    depth: usize,
) {
    let mut table_output = String::new();
    builder::handle_table(node_handle, parser, &mut table_output, options, ctx, dom_ctx, depth);

    // Feed the table into the structure collector when document structure extraction is enabled.
    if let Some(ref sc) = ctx.structure_collector {
        if let Some(grid) = collect_table_grid(node_handle, parser, options, ctx, dom_ctx) {
            sc.borrow_mut().push_table(grid);
        }
    }

    if ctx.in_list_item {
        let has_caption = table_output.starts_with('*');

        if !has_caption {
            use crate::converter::main_helpers::trim_trailing_whitespace;
            trim_trailing_whitespace(output);
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
        }

        let indented = layout::indent_table_for_list(&table_output, ctx.list_depth, options);
        output.push_str(&indented);
    } else {
        if !output.ends_with("\n\n") {
            if output.is_empty() || !output.ends_with('\n') {
                output.push_str("\n\n");
            } else {
                output.push('\n');
            }
        }
        output.push_str(&table_output);
    }

    if !output.ends_with('\n') {
        output.push('\n');
    }
}

/// Collect a [`crate::types::TableGrid`] from the DOM for the structure collector.
///
/// Walks the table's rows and cells, extracting text content and span attributes
/// to build a structured grid representation.
fn collect_table_grid(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::Context,
    dom_ctx: &super::super::DomContext,
) -> Option<crate::types::TableGrid> {
    use utils::{is_tag_name, normalized_tag_name};

    let tl::Node::Tag(tag) = node_handle.get(parser)? else {
        return None;
    };

    let mut grid_cells = Vec::new();
    let mut row_index: u32 = 0;
    let mut max_cols: u32 = 0;
    let mut cell_handles = Vec::new();

    let children = tag.children();
    for child_handle in children.top().iter() {
        if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
            let tag_name = normalized_tag_name(child_tag.name().as_utf8_str());
            match tag_name.as_ref() {
                "thead" | "tbody" | "tfoot" => {
                    let is_header_section = tag_name.as_ref() == "thead";
                    for row_handle in child_tag.children().top().iter() {
                        if is_tag_name(row_handle, parser, dom_ctx, "tr") {
                            collect_grid_row(
                                row_handle,
                                parser,
                                options,
                                ctx,
                                dom_ctx,
                                &mut cell_handles,
                                &mut grid_cells,
                                &mut row_index,
                                &mut max_cols,
                                is_header_section,
                            );
                        }
                    }
                }
                "tr" | "row" => {
                    let is_first = row_index == 0;
                    collect_grid_row(
                        child_handle,
                        parser,
                        options,
                        ctx,
                        dom_ctx,
                        &mut cell_handles,
                        &mut grid_cells,
                        &mut row_index,
                        &mut max_cols,
                        is_first,
                    );
                }
                _ => {}
            }
        }
    }

    if row_index == 0 {
        return None;
    }

    Some(crate::types::TableGrid {
        rows: row_index,
        cols: max_cols,
        cells: grid_cells,
    })
}

/// Process a single table row for grid collection.
#[allow(clippy::too_many_arguments)]
fn collect_grid_row(
    row_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::Context,
    dom_ctx: &super::super::DomContext,
    cell_handles: &mut Vec<tl::NodeHandle>,
    grid_cells: &mut Vec<crate::types::GridCell>,
    row_index: &mut u32,
    max_cols: &mut u32,
    is_header_section: bool,
) {
    use cell::{collect_table_cells, get_colspan_rowspan};

    collect_table_cells(row_handle, parser, dom_ctx, cell_handles);

    let mut col_index: u32 = 0;
    for cell_handle in cell_handles.iter() {
        let is_header = is_header_section
            || dom_ctx
                .tag_name_for(*cell_handle, parser)
                .is_some_and(|name| name.as_ref() == "th");

        let mut text = String::new();
        let cell_ctx = super::super::Context {
            in_table_cell: true,
            ..ctx.clone()
        };
        if let Some(tl::Node::Tag(cell_tag)) = cell_handle.get(parser) {
            for child_handle in cell_tag.children().top().iter() {
                super::super::walk_node(child_handle, parser, &mut text, options, &cell_ctx, 0, dom_ctx);
            }
        }
        let content = crate::text::normalize_whitespace_cow(&text).trim().to_string();

        let (colspan, rowspan) = get_colspan_rowspan(cell_handle, parser);

        grid_cells.push(crate::types::GridCell {
            content,
            row: *row_index,
            col: col_index,
            row_span: rowspan as u32,
            col_span: colspan as u32,
            is_header,
        });

        col_index += colspan as u32;
    }
    if col_index > *max_cols {
        *max_cols = col_index;
    }
    *row_index += 1;
}
