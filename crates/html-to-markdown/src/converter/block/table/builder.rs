//! Core table building and structure calculation.
//!
//! Handles main table element conversion, column calculation, and visitor
//! pattern integration for custom table handling.

use std::borrow::Cow;

use super::cell::{collect_table_cells, get_colspan};
use super::cells::{append_layout_row, convert_table_row};
use super::scanner::scan_table;
use super::utils::{is_tag_name, normalized_tag_name};
#[cfg(feature = "visitor")]
use crate::converter::utility::content::collect_tag_attributes;

/// Maximum allowed table columns to prevent unbounded memory usage.
const MAX_TABLE_COLS: usize = 1000;

/// Calculate total columns in a table.
///
/// Scans all rows and cells to determine the maximum column count,
/// accounting for colspan values.
///
/// # Arguments
/// * `node_handle` - Handle to the table element
/// * `parser` - HTML parser instance
/// * `dom_ctx` - DOM context for tag name resolution
///
/// # Returns
/// Maximum column count (minimum 1, maximum MAX_TABLE_COLS)
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn table_total_columns(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &super::super::super::DomContext,
) -> usize {
    let mut max_cols = 0usize;
    let mut cells = Vec::new();

    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let children = tag.children();
        for child_handle in children.top().iter() {
            if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                let tag_name = dom_ctx
                    .tag_name_for(*child_handle, parser)
                    .unwrap_or_else(|| normalized_tag_name(child_tag.name().as_utf8_str()));
                match tag_name.as_ref() {
                    "thead" | "tbody" | "tfoot" => {
                        for row_handle in child_tag.children().top().iter() {
                            if is_tag_name(row_handle, parser, dom_ctx, "tr") {
                                collect_table_cells(row_handle, parser, dom_ctx, &mut cells);
                                let col_count = cells
                                    .iter()
                                    .fold(0usize, |acc, h| acc.saturating_add(get_colspan(h, parser)));
                                max_cols = max_cols.max(col_count);
                            }
                        }
                    }
                    "tr" | "row" => {
                        collect_table_cells(child_handle, parser, dom_ctx, &mut cells);
                        let col_count = cells
                            .iter()
                            .fold(0usize, |acc, h| acc.saturating_add(get_colspan(h, parser)));
                        max_cols = max_cols.max(col_count);
                    }
                    _ => {}
                }
            }
        }
    }

    max_cols.clamp(1, MAX_TABLE_COLS)
}

/// Convert an entire table element to Markdown.
///
/// Main entry point for table conversion. Analyzes table structure to determine
/// if it should be rendered as a Markdown table or converted to list format.
/// Handles layout tables, blank tables, and tables with semantic meaning.
/// Integrates with visitor pattern for custom table handling.
///
/// # Arguments
/// * `node_handle` - Handle to the table element
/// * `parser` - HTML parser instance
/// * `output` - Mutable string to append table content
/// * `options` - Conversion options
/// * `ctx` - Conversion context (visitor, etc)
/// * `dom_ctx` - DOM context
/// * `depth` - Nesting depth
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn handle_table(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::super::Context,
    dom_ctx: &super::super::super::DomContext,
    depth: usize,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        #[cfg(feature = "visitor")]
        let table_output_start = output.len();

        #[cfg(feature = "visitor")]
        let mut table_start_custom: Option<String> = None;

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};
            use std::collections::BTreeMap;

            let attributes: BTreeMap<String, String> = collect_tag_attributes(tag);

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext {
                node_type: NodeType::Table,
                tag_name: "table".to_string(),
                attributes,
                depth,
                index_in_parent,
                parent_tag,
                is_inline: false,
            };

            let visit_result = {
                let mut visitor = visitor_handle.borrow_mut();
                visitor.visit_table_start(&node_ctx)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Skip => return,
                VisitResult::Custom(custom) => {
                    table_start_custom = Some(custom);
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

        let table_scan = scan_table(node_handle, parser, dom_ctx);
        let row_count = table_scan.row_counts.len();
        let mut distinct_counts: Vec<_> = table_scan.row_counts.iter().copied().filter(|c| *c > 0).collect();
        distinct_counts.sort_unstable();
        distinct_counts.dedup();

        let has_border_zero = tag
            .attributes()
            .get("border")
            .is_some_and(|v| v.as_ref().is_some_and(|b| b.as_utf8_str() == "0"));
        let looks_like_layout =
            table_scan.nested_table_count > 1 || distinct_counts.len() > 1 || (table_scan.has_span && has_border_zero);
        let link_count = table_scan.link_count;
        let is_blank_table = !table_scan.has_text;

        if !table_scan.has_header
            && !table_scan.has_caption
            && (looks_like_layout || is_blank_table || (row_count <= 2 && link_count >= 3))
        {
            // Skip truly blank tables (no text, no links, no images)
            if is_blank_table && link_count == 0 {
                return;
            }

            let table_children = tag.children();
            for child_handle in table_children.top().iter() {
                if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                    let tag_name = normalized_tag_name(child_tag.name().as_utf8_str());
                    match tag_name.as_ref() {
                        "thead" | "tbody" | "tfoot" => {
                            for row_handle in child_tag.children().top().iter() {
                                if let Some(tl::Node::Tag(row_tag)) = row_handle.get(parser) {
                                    let row_tag_name = normalized_tag_name(row_tag.name().as_utf8_str());
                                    if matches!(row_tag_name.as_ref(), "tr" | "row") {
                                        append_layout_row(row_handle, parser, output, options, ctx, dom_ctx);
                                    }
                                }
                            }
                        }
                        "tr" | "row" => append_layout_row(child_handle, parser, output, options, ctx, dom_ctx),
                        "colgroup" | "col" => {}
                        _ => {
                            // Handle non-table-structure elements (like <a>, <img>, etc.) that may be
                            // direct children of layout tables (e.g., Blogger table wrappers)
                            super::super::super::walk_node(
                                child_handle,
                                parser,
                                output,
                                options,
                                ctx,
                                depth + 1,
                                dom_ctx,
                            );
                        }
                    }
                }
            }
            if !output.ends_with('\n') {
                output.push('\n');
            }
            return;
        }

        let mut row_index = 0;
        let total_cols = table_total_columns(node_handle, parser, dom_ctx);
        let mut first_row_cols: Option<usize> = None;
        let mut rowspan_tracker = vec![None; total_cols];
        let mut row_cells = Vec::new();

        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                    let tag_name: Cow<'_, str> = dom_ctx.tag_info(child_handle.get_inner(), parser).map_or_else(
                        || normalized_tag_name(child_tag.name().as_utf8_str()).into_owned().into(),
                        |info| Cow::Borrowed(info.name.as_str()),
                    );

                    match tag_name.as_ref() {
                        "caption" => {
                            let mut text = String::new();
                            let grandchildren = child_tag.children();
                            {
                                for grandchild_handle in grandchildren.top().iter() {
                                    super::super::super::walk_node(
                                        grandchild_handle,
                                        parser,
                                        &mut text,
                                        options,
                                        ctx,
                                        0,
                                        dom_ctx,
                                    );
                                }
                            }
                            let text = text.trim();
                            if !text.is_empty() {
                                let escaped_text = text.replace('-', r"\-");
                                output.push('*');
                                output.push_str(&escaped_text);
                                output.push_str("*\n\n");
                            }
                        }

                        "thead" | "tbody" | "tfoot" => {
                            let is_header_section = tag_name.as_ref() == "thead";
                            let section_children = child_tag.children();
                            {
                                for row_handle in section_children.top().iter() {
                                    if let Some(tl::Node::Tag(row_tag)) = row_handle.get(parser) {
                                        let row_tag_name = dom_ctx
                                            .tag_name_for(*row_handle, parser)
                                            .unwrap_or_else(|| normalized_tag_name(row_tag.name().as_utf8_str()));
                                        if matches!(row_tag_name.as_ref(), "tr" | "row") {
                                            if first_row_cols.is_none() {
                                                collect_table_cells(row_handle, parser, dom_ctx, &mut row_cells);
                                                let cols = row_cells
                                                    .iter()
                                                    .fold(0usize, |acc, h| acc.saturating_add(get_colspan(h, parser)));
                                                first_row_cols = Some(cols.clamp(1, MAX_TABLE_COLS));
                                            }
                                            convert_table_row(
                                                row_handle,
                                                parser,
                                                output,
                                                options,
                                                ctx,
                                                row_index,
                                                table_scan.has_span,
                                                &mut rowspan_tracker,
                                                total_cols,
                                                first_row_cols.unwrap_or(total_cols),
                                                dom_ctx,
                                                depth + 1,
                                                is_header_section,
                                            );
                                            row_index += 1;
                                        }
                                    }
                                }
                            }
                        }

                        "tr" | "row" => {
                            if first_row_cols.is_none() {
                                collect_table_cells(child_handle, parser, dom_ctx, &mut row_cells);
                                let cols = row_cells
                                    .iter()
                                    .fold(0usize, |acc, h| acc.saturating_add(get_colspan(h, parser)));
                                first_row_cols = Some(cols.clamp(1, MAX_TABLE_COLS));
                            }
                            convert_table_row(
                                child_handle,
                                parser,
                                output,
                                options,
                                ctx,
                                row_index,
                                table_scan.has_span,
                                &mut rowspan_tracker,
                                total_cols,
                                first_row_cols.unwrap_or(total_cols),
                                dom_ctx,
                                depth + 1,
                                row_index == 0,
                            );
                            row_index += 1;
                        }

                        "colgroup" | "col" => {}

                        _ => {
                            // Handle non-table-structure elements (like <a>, <img>, etc.) that may be
                            // direct children of tables without proper structure (e.g., Blogger table wrappers)
                            super::super::super::walk_node(
                                child_handle,
                                parser,
                                output,
                                options,
                                ctx,
                                depth + 1,
                                dom_ctx,
                            );
                        }
                    }
                }
            }
        }

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};
            use std::collections::BTreeMap;

            let attributes: BTreeMap<String, String> = collect_tag_attributes(tag);

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext {
                node_type: NodeType::Table,
                tag_name: "table".to_string(),
                attributes,
                depth,
                index_in_parent,
                parent_tag,
                is_inline: false,
            };

            let table_content = &output[table_output_start..];

            let visit_result = {
                let mut visitor = visitor_handle.borrow_mut();
                visitor.visit_table_end(&node_ctx, table_content)
            };
            match visit_result {
                VisitResult::Continue => {
                    if let Some(custom_start) = table_start_custom {
                        output.insert_str(table_output_start, &custom_start);
                    }
                }
                VisitResult::Custom(custom) => {
                    let rows_output = output[table_output_start..].to_string();
                    output.truncate(table_output_start);
                    if let Some(custom_start) = table_start_custom {
                        output.push_str(&custom_start);
                    }
                    output.push_str(&rows_output);
                    output.push_str(&custom);
                }
                VisitResult::Skip => {
                    output.truncate(table_output_start);
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                }
                VisitResult::PreserveHtml => {
                    output.truncate(table_output_start);
                    output.push_str(&super::super::super::serialize_node(node_handle, parser));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_nested_table_stays_as_table() {
        let html = r"<table><tr><td>Label</td><td><table><tr><td>A</td><td>B</td></tr></table></td></tr></table>";
        let result = crate::convert(html, None).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(content.contains('|'), "should produce pipe table, not list");
    }
}
