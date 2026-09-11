//! Core table building and structure calculation.
//!
//! Handles main table element conversion, column calculation, and visitor
//! pattern integration for custom table handling.

use std::borrow::Cow;

use super::cell::{collect_table_cells, get_colspan};
use super::cells::{CellTextCache, RowEnv, append_layout_row, collect_row_cell_widths, convert_table_row};
use super::scanner::{TableScan, scan_table};
use super::utils::{is_tag_name, normalized_tag_name};

/// Return the content cell of a one-cell layout wrapper containing a nested table.
/// Stop at that cell, so nested data headers do not give the wrapper table semantics.
fn nested_table_wrapper_cell(
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    scan: &TableScan,
) -> Option<(tl::NodeHandle, usize)> {
    if scan.row_counts != [1] || scan.nested_table_count == 0 || scan.has_span || !scan.has_text {
        return None;
    }
    let mut cell = None;
    let mut pending: Vec<_> = tag.children().top().iter().map(|handle| (*handle, 1)).collect();
    while let Some((handle, depth)) = pending.pop() {
        let Some(tl::Node::Tag(child)) = handle.get(parser) else {
            continue;
        };
        match child.name().as_utf8_str().to_ascii_lowercase().as_str() {
            "td" if cell.is_none() => cell = Some((handle, depth)),
            "thead" | "tbody" | "tfoot" | "tr" => {
                pending.extend(child.children().top().iter().map(|handle| (*handle, depth + 1)));
            }
            _ => return None,
        }
    }
    cell
}

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
/// Maximum column count (minimum 1, maximum `MAX_TABLE_COLS`)
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

/// Whether each cell's markdown may be rendered once in the width pre-pass and reused
/// verbatim in the render pass.
///
/// ~keep The pre-pass runs with `measure_width_only` (a nested `<table>` degrades to raw text,
/// ~keep issue #406) and `skip_visitor_hooks`, so its rendering equals the render pass only
/// ~keep when the table holds no nested table and no visitor is installed.
///
/// ~keep A collector shared through the context also observes the walk, and two of them feed
/// ~keep back into the emitted bytes or into a caller-visible document model whose entries are
/// ~keep positional: the reference collector numbers `[n]` link labels that appear in the
/// ~keep output, and the structure/inline-image collectors index what they see. Those keep the
/// ~keep two-pass path. The metadata collector does not affect emitted bytes, so reuse stays on
/// ~keep for it — its entries stop being recorded twice per table cell, which is the double
/// ~keep walk's bug, not a behaviour worth preserving.
const fn cell_text_reuse_allowed(ctx: &super::super::super::Context, table_scan: &TableScan) -> bool {
    if table_scan.nested_table_count > 0 {
        return false;
    }
    if ctx.structure_collector.is_some() || ctx.reference_collector.is_some() {
        return false;
    }
    #[cfg(feature = "inline-images")]
    if ctx.inline_collector.is_some() {
        return false;
    }
    #[cfg(feature = "visitor")]
    if ctx.visitor.is_some() {
        return false;
    }
    true
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

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Table,
                Cow::Borrowed("table"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                false,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
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
        // Keep the normal table renderer at the traversal boundary: it emits the
        // truncated table structure and records the usual depth-limit warning.
        let wrapper_cell = nested_table_wrapper_cell(tag, parser, &table_scan).filter(|(_, cell_depth)| {
            depth + cell_depth + 1 < crate::converter::main_helpers::effective_max_depth(options)
        });
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

        if wrapper_cell.is_none()
            && !table_scan.has_header
            && !table_scan.has_caption
            && (looks_like_layout || is_blank_table || (row_count <= 2 && link_count >= 3))
        {
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
                                        append_layout_row(
                                            row_handle,
                                            output,
                                            RowEnv {
                                                parser,
                                                options,
                                                ctx,
                                                dom_ctx,
                                            },
                                            depth + 1,
                                        );
                                    }
                                }
                            }
                        }
                        "tr" | "row" => {
                            append_layout_row(
                                child_handle,
                                output,
                                RowEnv {
                                    parser,
                                    options,
                                    ctx,
                                    dom_ctx,
                                },
                                depth + 1,
                            );
                        }
                        "colgroup" | "col" => {}
                        _ => {
                            // ~keep Handle non-table-structure elements (like <a>, <img>, etc.) that may be
                            // ~keep direct children of layout tables (e.g., Blogger table wrappers)
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

        if let Some((cell_handle, cell_depth)) = wrapper_cell {
            if let Some(tl::Node::Tag(cell)) = cell_handle.get(parser) {
                for child in cell.children().top().iter() {
                    super::super::super::walk_node(
                        child,
                        parser,
                        output,
                        options,
                        ctx,
                        depth + cell_depth + 1,
                        dom_ctx,
                    );
                }
            }
        } else {
            let mut row_index = 0;
            // ~keep The header separator row's column count must cover every row's width, not
            // ~keep just the first row: a later row with more actual cells than the header
            // ~keep ("ragged" table) would otherwise render a separator declaring fewer columns
            // ~keep than that row provides, and GFM-compliant renderers silently drop cells past
            // ~keep the declared column count (issue #13).
            let total_cols = table_total_columns(node_handle, parser, dom_ctx);
            let mut rowspan_tracker = vec![None; total_cols];

            let reuse_cell_text = !options.compact_tables && cell_text_reuse_allowed(ctx, &table_scan);
            let mut cell_cache = CellTextCache::new(reuse_cell_text);
            // ~keep Populated by `convert_table_row` when a row's sole cell holds a nested
            // ~keep table (issue #484): rendered separately, after this table, instead of
            // ~keep being flattened into a line of escaped pipes.
            let mut deferred_tables: Vec<String> = Vec::new();

            // ~keep Pre-pass: compute per-column max content widths for aligned padding.
            // ~keep Uses a rowspan tracker so spanned columns are skipped just as they
            // ~keep are in the render pass, keeping column indices correctly aligned.
            // ~keep Skipped entirely when compact_tables is true — passing an empty slice
            // ~keep to convert_table_row disables all padding and reduces separator dashes
            // ~keep to the GFM minimum (---).
            let col_widths: Vec<usize> = if options.compact_tables {
                Vec::new()
            } else {
                // ~keep Exactly one walk of a cell may reach each collector, and the context's
                // ~keep collector handles are `Rc`s that `..ctx.clone()` shares rather than copies.
                // ~keep With reuse on, the render pass emits this pass's cached markdown without
                // ~keep walking the cell again, so this pass is that one walk and keeps the handles.
                // ~keep With reuse off the render pass walks and records, so the handles are detached
                // ~keep here — the width measurement is an internal detail and must not be visible in
                // ~keep `ConversionResult`. Detaching propagates to the whole subtree because every
                // ~keep nested context is built from this one by `..clone()`.
                // ~keep For the structure and inline-image collectors the reuse-on branch is
                // ~keep unreachable rather than merely unused: `cell_text_reuse_allowed` returns false
                // ~keep whenever either is set, so neither can ever be the pass that records. The
                // ~keep guard is kept so the two rules stay coupled if that function changes.
                let mut prepass_ctx = super::super::super::Context {
                    skip_visitor_hooks: true,
                    measure_width_only: true,
                    ..ctx.clone()
                };
                if !reuse_cell_text {
                    #[cfg(feature = "metadata")]
                    {
                        prepass_ctx.metadata_collector = None;
                    }
                    prepass_ctx.structure_collector = None;
                    #[cfg(feature = "inline-images")]
                    {
                        prepass_ctx.inline_collector = None;
                    }
                }
                let mut widths: Vec<usize> = Vec::new();
                let mut prepass_rowspan: Vec<Option<usize>> = vec![None; total_cols];
                let children = tag.children();
                for child_handle in children.top().iter() {
                    if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                        let tag_name = normalized_tag_name(child_tag.name().as_utf8_str());
                        match tag_name.as_ref() {
                            "thead" | "tbody" | "tfoot" => {
                                for row_handle in child_tag.children().top().iter() {
                                    if is_tag_name(row_handle, parser, dom_ctx, "tr") {
                                        collect_row_cell_widths(
                                            row_handle,
                                            RowEnv {
                                                parser,
                                                options,
                                                ctx: &prepass_ctx,
                                                dom_ctx,
                                            },
                                            &mut widths,
                                            &mut prepass_rowspan,
                                            &mut cell_cache,
                                            depth + 1,
                                        );
                                    }
                                }
                            }
                            "tr" | "row" => {
                                collect_row_cell_widths(
                                    child_handle,
                                    RowEnv {
                                        parser,
                                        options,
                                        ctx: &prepass_ctx,
                                        dom_ctx,
                                    },
                                    &mut widths,
                                    &mut prepass_rowspan,
                                    &mut cell_cache,
                                    depth + 1,
                                );
                            }
                            _ => {}
                        }
                    }
                }
                widths
            };

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
                                            depth + 1,
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
                                                    total_cols,
                                                    dom_ctx,
                                                    depth + 1,
                                                    is_header_section,
                                                    &col_widths,
                                                    &mut cell_cache,
                                                    &mut deferred_tables,
                                                );
                                                row_index += 1;
                                            }
                                        }
                                    }
                                }
                            }

                            "tr" | "row" => {
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
                                    total_cols,
                                    dom_ctx,
                                    depth + 1,
                                    row_index == 0,
                                    &col_widths,
                                    &mut cell_cache,
                                    &mut deferred_tables,
                                );
                                row_index += 1;
                            }

                            "colgroup" | "col" => {}

                            _ => {
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

            // ~keep Render each deferred nested table (issue #484) as its own separate GFM
            // ~keep table, immediately after this one. GFM cannot express real nesting, so
            // ~keep this is the closest usable substitute to what 3.8.3 rendered before the
            // ~keep escaped-flatten fallback (issue #469) took over this shape too.
            for nested in &deferred_tables {
                if !output.ends_with('\n') {
                    output.push('\n');
                }
                if !output.ends_with("\n\n") {
                    output.push('\n');
                }
                output.push_str(nested);
                output.push('\n');
            }
        }

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Table,
                Cow::Borrowed("table"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                false,
            );

            let table_content = &output[table_output_start..];

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
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

    /// Regression test for issue #406: deeply-nested layout tables caused
    /// exponential output growth because `cell_text_content` recursed into
    /// nested `<table>` elements (including their separator rows) when
    /// computing outer column widths.  With 5 levels of nesting the output
    /// must stay well under 10 KB.
    #[test]
    fn deeply_nested_layout_tables_do_not_produce_runaway_output() {
        // ~keep Build a 5-level-deep nested layout table.
        // ~keep Each cell contains another complete table, mirroring the structure
        // ~keep seen in email digests that triggered the regression.
        let inner = "<table><tr><td>leaf</td></tr></table>";
        let level1 = format!("<table><tr><td>{inner}</td></tr></table>");
        let level2 = format!("<table><tr><td>{level1}</td></tr></table>");
        let level3 = format!("<table><tr><td>{level2}</td></tr></table>");
        let level4 = format!("<table><tr><td>{level3}</td></tr></table>");
        let level5 = format!("<table><tr><td>{level4}</td></tr></table>");

        let result = crate::convert(&level5, None).unwrap();
        let content = result.content.unwrap_or_default();

        const MAX_BYTES: usize = 10_000;
        assert!(
            content.len() < MAX_BYTES,
            "nested layout table output should be < {MAX_BYTES} bytes, got {} bytes (issue #406 regression)",
            content.len()
        );
    }
}
