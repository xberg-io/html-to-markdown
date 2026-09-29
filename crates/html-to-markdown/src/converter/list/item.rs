//! List item handling (li element).
//!
//! Processes list items with support for:
//! - Task list detection and rendering (checkboxes)
//! - Block-level children detection
//! - Proper bullet/number formatting
//! - Indentation and spacing

use crate::converter::list::utils::add_list_leading_separator;
use crate::converter::main_helpers::effective_max_depth;
use crate::converter::main_helpers::strip_trailing_backslash_breaks;
use crate::converter::main_helpers::tag_name_eq;
use crate::converter::main_helpers::trim_trailing_whitespace;
use crate::converter::utility::content::normalized_tag_name;
use crate::converter::walk_node;
use crate::options::{ConversionOptions, NewlineStyle};
#[cfg(feature = "visitor")]
use std::borrow::Cow;
use tl;

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle list item element (<li>).
///
/// Processes list item content with support for task lists (checkboxes),
/// proper indentation, and block-level element detection.
#[allow(clippy::too_many_arguments)]
pub fn handle_li(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    // ~keep A nested list whose enclosing <li> has no other content renders directly after
    // ~keep that parent's own bare marker on the SAME physical line (see
    // ~keep `add_list_leading_separator`'s bare-marker exclusion) -- the parent marker's own
    // ~keep printed width already reaches this item's target column, so pushing this
    // ~keep indent too double-counts it, deeply nesting single-child lists into runaway
    // ~keep padding that reparses as an indented code block (spec example 299). The indent is
    // ~keep only needed when this item genuinely starts a fresh physical line.
    let marker_line_start = (!output.is_empty() && output.ends_with('\n')).then_some(output.len());
    if ctx.list_depth > 0 && (output.is_empty() || output.ends_with('\n')) {
        let indent = match options.list_indent_type {
            crate::options::ListIndentType::Tabs => "\t".repeat(ctx.list_depth),
            // ~keep `list_indent_columns` is the cumulative width of every ancestor <li>'s own
            // ~keep marker (see Context::list_indent_columns), not a uniform per-depth value.
            crate::options::ListIndentType::Spaces => " ".repeat(ctx.list_indent_columns),
        };
        output.push_str(&indent);
    }

    let mut has_block_children = false;
    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            if let Some(info) = dom_ctx.tag_info(child_handle.get_inner(), parser) {
                if matches!(
                    info.name.as_str(),
                    "p" | "div" | "blockquote" | "pre" | "table" | "hr" | "dl"
                ) {
                    has_block_children = true;
                    break;
                }
            } else if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                let tag_name = normalized_tag_name(child_tag.name().as_utf8_str());
                if matches!(
                    tag_name.as_ref(),
                    "p" | "div" | "blockquote" | "pre" | "table" | "hr" | "dl"
                ) {
                    has_block_children = true;
                    break;
                }
            }
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn find_checkbox<'a>(
        node_handle: &tl::NodeHandle,
        parser: &'a tl::Parser<'a>,
        options: &ConversionOptions,
        ctx: &Context,
        depth: usize,
    ) -> Option<(bool, tl::NodeHandle)> {
        // ~keep This helper recurses over the li subtree independently of `walk_node`
        // ~keep (it runs before any handler dispatch), so it needs its own depth guard
        // ~keep instead of relying on the main walker's check.
        if depth >= effective_max_depth(options) {
            ctx.depth_limit_reached.set(true);
            return None;
        }
        if let Some(tl::Node::Tag(node_tag)) = node_handle.get(parser) {
            if tag_name_eq(node_tag.name().as_utf8_str(), "input") {
                let input_type = node_tag.attributes().get("type").flatten().map(|v| v.as_utf8_str());

                if input_type.as_deref() == Some("checkbox") {
                    let checked = node_tag.attributes().get("checked").is_some();
                    return Some((checked, *node_handle));
                }
            }

            // ~keep A nested list's items own the checkboxes inside it (issue #604).
            if matches!(normalized_tag_name(node_tag.name().as_utf8_str()).as_ref(), "ul" | "ol") {
                return None;
            }

            let children = node_tag.children();
            {
                for child_handle in children.top().iter() {
                    if let Some(result) = find_checkbox(child_handle, parser, options, ctx, depth + 1) {
                        return Some(result);
                    }
                }
            }
        }
        None
    }

    let (is_task_list, task_checked, checkbox_node) =
        if let Some((checked, node)) = find_checkbox(node_handle, parser, options, ctx, depth) {
            (true, checked, Some(node))
        } else {
            (false, false, None)
        };

    // ~keep This item's own marker width, used to grow `list_indent_columns` for descendants
    // ~keep (nested lists and continuation content). Unordered/task markers are always 2 wide
    // ~keep ("- "); an ordered marker's width depends on its counter's digit count ("1. " = 3,
    // ~keep "10. " = 4, ...). `list_indent_width` is honoured as a floor, not the literal width.
    let own_marker_width = if is_task_list || !ctx.in_ordered_list {
        options.list_indent_width.max(2)
    } else {
        let marker_len = format!("{}. ", ctx.list_counter).chars().count();
        options.list_indent_width.max(marker_len)
    };

    // ~keep A list inside an inline wrapper, a summary or a caption is written into that
    // ~keep buffer and gets its markers; its first line is then text, so the item is not open
    // ~keep and no block gets the column. Text after a quote or a list still takes the column
    // ~keep of the innermost item whose marker starts a list item (issue #615).
    let list_item_open = ctx.inline_depth == 0 && !ctx.text_in_markers;
    let real_item_columns = if list_item_open {
        ctx.real_item_columns
    } else {
        let enclosing = if ctx.list_item_open {
            ctx.list_indent_columns
        } else {
            ctx.real_item_columns
        };
        let hyphen_marker = is_task_list || (!ctx.in_ordered_list && unordered_bullet(ctx, options) == '-');
        let marker_is_text = ctx.escapes_hyphens && hyphen_marker;
        if !marker_is_text
            && crate::converter::list::utils::marker_starts_item(output, marker_line_start, enclosing, ctx, options)
        {
            ctx.list_indent_columns + own_marker_width
        } else {
            enclosing
        }
    };
    let li_ctx = Context {
        in_list_item: true,
        list_item_open,
        list_depth: ctx.list_depth + 1,
        list_indent_columns: ctx.list_indent_columns + own_marker_width,
        real_item_columns,
        ..ctx.clone()
    };

    if is_task_list {
        output.push('-');
        output.push(' ');
        output.push_str(if task_checked { "[x]" } else { "[ ]" });

        #[allow(clippy::ref_option)]
        fn is_checkbox_node(node_handle: &tl::NodeHandle, checkbox: &Option<tl::NodeHandle>) -> bool {
            if let Some(cb) = checkbox {
                node_handle == cb
            } else {
                false
            }
        }

        #[allow(clippy::ref_option)]
        fn contains_checkbox<'a>(
            node_handle: &tl::NodeHandle,
            parser: &'a tl::Parser<'a>,
            checkbox: &Option<tl::NodeHandle>,
            options: &ConversionOptions,
            ctx: &Context,
            depth: usize,
        ) -> bool {
            if depth >= effective_max_depth(options) {
                ctx.depth_limit_reached.set(true);
                return false;
            }
            if is_checkbox_node(node_handle, checkbox) {
                return true;
            }
            if let Some(tl::Node::Tag(node_tag)) = node_handle.get(parser) {
                let children = node_tag.children();
                {
                    for child_handle in children.top().iter() {
                        if contains_checkbox(child_handle, parser, checkbox, options, ctx, depth + 1) {
                            return true;
                        }
                    }
                }
            }
            false
        }

        #[allow(clippy::too_many_arguments, clippy::ref_option)]
        fn render_li_content<'a>(
            node_handle: &tl::NodeHandle,
            parser: &'a tl::Parser<'a>,
            output: &mut String,
            options: &ConversionOptions,
            ctx: &Context,
            depth: usize,
            checkbox: &Option<tl::NodeHandle>,
            dom_ctx: &DomContext,
        ) {
            // ~keep Independent recursion from `walk_node` while probing for the nested
            // ~keep checkbox, so it needs its own guard rather than relying on the depth
            // ~keep check inside `walk_node` (which is only reached once a leaf is found).
            if depth >= effective_max_depth(options) {
                ctx.depth_limit_reached.set(true);
                return;
            }
            if is_checkbox_node(node_handle, checkbox) {
                return;
            }

            if contains_checkbox(node_handle, parser, checkbox, options, ctx, depth) {
                if let Some(tl::Node::Tag(node_tag)) = node_handle.get(parser) {
                    let children = node_tag.children();
                    {
                        for child_handle in children.top().iter() {
                            render_li_content(child_handle, parser, output, options, ctx, depth + 1, checkbox, dom_ctx);
                        }
                    }
                }
            } else {
                walk_node(node_handle, parser, output, options, ctx, depth, dom_ctx);
            }
        }

        let mut task_text = String::new();
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                render_li_content(
                    child_handle,
                    parser,
                    &mut task_text,
                    options,
                    &li_ctx,
                    depth + 1,
                    &checkbox_node,
                    dom_ctx,
                );
            }
        }
        let trimmed_task = task_text.trim();
        // ~keep After the checkbox the line is paragraph text, so a block that is the item's
        // ~keep first content (a quote) starts on the next line at the content column.
        match crate::converter::list::utils::continuation_indent_string(li_ctx.list_indent_columns, options) {
            Some(indent)
                if li_ctx.list_item_open
                    && !ctx.in_marker_span
                    && !ctx.in_table_cell
                    && !ctx.convert_as_inline
                    && task_content_starts_with_block(tag, checkbox_node, parser) =>
            {
                output.push('\n');
                output.push_str(&indent);
            }
            _ => output.push(' '),
        }
        if !trimmed_task.is_empty() {
            output.push_str(trimmed_task);
        }
    } else {
        if ctx.in_table_cell {
            // ~keep GFM pipe cells cannot hold block content, so sibling <li>s inside a
            // ~keep cell lose their marker (see below) and would otherwise be concatenated
            // ~keep with zero separator. Reuse the same leading-separator helper used before
            // ~keep the enclosing <ul>/<ol> opens so consecutive items get the identical
            // ~keep <br> boundary already established for <p>/<div> siblings in a cell.
            add_list_leading_separator(output, ctx, options);
        } else if ctx.in_ordered_list {
            use std::fmt::Write;
            let _ = write!(output, "{}. ", ctx.list_counter);
        } else {
            output.push(unordered_bullet(ctx, options));
            output.push(' ');
        }

        let item_start_pos = output.len();
        let mut text_end_pos = output.len();

        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                let is_nested_list = if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                    let n = normalized_tag_name(child_tag.name().as_utf8_str());
                    matches!(n.as_ref(), "ul" | "ol")
                } else {
                    false
                };
                walk_node(child_handle, parser, output, options, &li_ctx, depth + 1, dom_ctx);
                if !is_nested_list {
                    text_end_pos = output.len();
                }
            }
        }

        trim_trailing_whitespace(output);

        if options.newline_style == NewlineStyle::Backslash {
            // ~keep A trailing <br> run with no following sibling has no next dispatch to
            // ~keep catch it in `walk_node`'s pre-block-dispatch strip, since the item's own
            // ~keep content is simply finished here — so this closes its own trailing run the
            // ~keep same way `paragraph.rs` closes its own (issue #464 follow-up).
            strip_trailing_backslash_breaks(output, item_start_pos);
        }

        if !ctx.in_table_cell {
            if let Some(ref sc) = ctx.structure_collector {
                let safe_end = text_end_pos.min(output.len());
                if item_start_pos <= safe_end
                    && output.is_char_boundary(item_start_pos)
                    && output.is_char_boundary(safe_end)
                {
                    let rendered = &output[item_start_pos..safe_end];
                    let content = rendered.trim();
                    if !content.is_empty() {
                        sc.borrow_mut().push_list_item(content);
                    }
                }
            }
        }

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let parent_tag = dom_ctx
                .parent_of(node_handle.get_inner())
                .and_then(|pid| dom_ctx.tag_name_for(dom_ctx.node_handle(pid).copied()?, parser))
                .map(std::borrow::Cow::into_owned);

            let index = dom_ctx.sibling_index(node_handle.get_inner()).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::ListItem,
                Cow::Borrowed("li"),
                tag,
                depth,
                index,
                parent_tag.map(Cow::Owned),
                false,
            );

            let last_line_start = output.rfind('\n').map_or(0, |pos| pos + 1);
            let last_line = &output[last_line_start..];

            let (marker, text_start) = if is_task_list {
                let task_marker = if task_checked { "- [x]" } else { "- [ ]" };
                let text_start = last_line.find(task_marker).map_or(0, |pos| pos + task_marker.len());
                (Cow::Borrowed(task_marker), text_start)
            } else if ctx.in_ordered_list {
                let marker_text = format!("{}.", ctx.list_counter);
                let text_start = last_line.find(&marker_text).map_or(0, |pos| pos + marker_text.len());
                (Cow::Owned(marker_text), text_start)
            } else {
                let bullet = unordered_bullet(ctx, options);
                let text_start = last_line.find(bullet).map_or(0, |pos| pos + 1);
                let mut buf = String::with_capacity(bullet.len_utf8());
                buf.push(bullet);
                (Cow::Owned(buf), text_start)
            };
            let text_content = last_line[text_start..].trim();

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_list_item(&node_ctx, ctx.in_ordered_list, &marker, text_content)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Custom(custom) => {
                    output.truncate(last_line_start);
                    output.push_str(&custom);
                    if !ctx.in_table_cell && !output.ends_with('\n') {
                        output.push('\n');
                    }
                    return;
                }
                VisitResult::Skip => {
                    output.truncate(last_line_start);
                    return;
                }
                VisitResult::PreserveHtml => {
                    output.truncate(last_line_start);
                    use crate::converter::serialize_node_to_html;
                    serialize_node_to_html(node_handle, parser, output);
                    if !ctx.in_table_cell && !output.ends_with('\n') {
                        output.push('\n');
                    }
                    return;
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    return;
                }
            }
        }
    }

    if !ctx.in_table_cell {
        if has_block_children || ctx.loose_list || ctx.prev_item_had_blocks {
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
    }
}

/// Whether the first content of the task item `tag` after its checkbox is a block that writes
/// a block opener: a quote, a list, a heading or a code block.
///
/// ~keep Text that reads like an opener stays text. A rule is left out: on the line under the
/// ~keep checkbox it would make the checkbox a heading.
fn task_content_starts_with_block(tag: &tl::HTMLTag, checkbox: Option<tl::NodeHandle>, parser: &tl::Parser) -> bool {
    for child_handle in tag.children().top().iter() {
        if Some(*child_handle) == checkbox {
            continue;
        }
        match child_handle.get(parser) {
            Some(tl::Node::Raw(text)) if text.as_utf8_str().trim().is_empty() => {}
            Some(tl::Node::Comment(_)) => {}
            Some(tl::Node::Tag(child_tag)) => {
                return matches!(
                    normalized_tag_name(child_tag.name().as_utf8_str()).as_ref(),
                    "blockquote" | "ul" | "ol" | "pre" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
                );
            }
            _ => return false,
        }
    }
    false
}

/// The bullet of an unordered list item at `ctx.ul_depth`: the list's bullets cycle by depth.
fn unordered_bullet(ctx: &Context, options: &ConversionOptions) -> char {
    let bullets: Vec<char> = options.bullets.chars().collect();
    let bullet_index = if ctx.ul_depth > 0 { ctx.ul_depth - 1 } else { 0 };
    if bullets.is_empty() {
        '*'
    } else {
        bullets[bullet_index % bullets.len()]
    }
}
