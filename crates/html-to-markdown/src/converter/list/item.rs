//! List item handling (li element).
//!
//! Processes list items with support for:
//! - Task list detection and rendering (checkboxes)
//! - Block-level children detection
//! - Proper bullet/number formatting
//! - Indentation and spacing

use crate::converter::main_helpers::tag_name_eq;
use crate::converter::main_helpers::trim_trailing_whitespace;
#[cfg(feature = "visitor")]
use crate::converter::utility::content::collect_tag_attributes;
use crate::converter::utility::content::normalized_tag_name;
use crate::converter::walk_node;
use crate::options::ConversionOptions;
use tl;

// Type aliases for Context and DomContext to avoid circular imports
type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle list item element (<li>).
///
/// Processes list item content with support for task lists (checkboxes),
/// proper indentation, and block-level element detection.
#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_li(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    if ctx.list_depth > 0 {
        let indent = match options.list_indent_type {
            crate::options::ListIndentType::Tabs => "\t".repeat(ctx.list_depth),
            crate::options::ListIndentType::Spaces => " ".repeat(ctx.list_depth * options.list_indent_width),
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
    fn find_checkbox<'a>(node_handle: &tl::NodeHandle, parser: &'a tl::Parser<'a>) -> Option<(bool, tl::NodeHandle)> {
        if let Some(tl::Node::Tag(node_tag)) = node_handle.get(parser) {
            if tag_name_eq(node_tag.name().as_utf8_str(), "input") {
                let input_type = node_tag.attributes().get("type").flatten().map(|v| v.as_utf8_str());

                if input_type.as_deref() == Some("checkbox") {
                    let checked = node_tag.attributes().get("checked").is_some();
                    return Some((checked, *node_handle));
                }
            }

            let children = node_tag.children();
            {
                for child_handle in children.top().iter() {
                    if let Some(result) = find_checkbox(child_handle, parser) {
                        return Some(result);
                    }
                }
            }
        }
        None
    }

    let (is_task_list, task_checked, checkbox_node) = if let Some((checked, node)) = find_checkbox(node_handle, parser)
    {
        (true, checked, Some(node))
    } else {
        (false, false, None)
    };

    let li_ctx = Context {
        in_list_item: true,
        list_depth: ctx.list_depth + 1,
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
        ) -> bool {
            if is_checkbox_node(node_handle, checkbox) {
                return true;
            }
            if let Some(tl::Node::Tag(node_tag)) = node_handle.get(parser) {
                let children = node_tag.children();
                {
                    for child_handle in children.top().iter() {
                        if contains_checkbox(child_handle, parser, checkbox) {
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
            if is_checkbox_node(node_handle, checkbox) {
                return;
            }

            if contains_checkbox(node_handle, parser, checkbox) {
                if let Some(tl::Node::Tag(node_tag)) = node_handle.get(parser) {
                    let children = node_tag.children();
                    {
                        for child_handle in children.top().iter() {
                            render_li_content(child_handle, parser, output, options, ctx, depth, checkbox, dom_ctx);
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
        output.push(' ');
        let trimmed_task = task_text.trim();
        if !trimmed_task.is_empty() {
            output.push_str(trimmed_task);
        }
    } else {
        if !ctx.in_table_cell {
            if ctx.in_ordered_list {
                use std::fmt::Write;
                let _ = write!(output, "{}. ", ctx.list_counter);
            } else {
                let bullets: Vec<char> = options.bullets.chars().collect();
                let bullet_index = if ctx.ul_depth > 0 { ctx.ul_depth - 1 } else { 0 };
                let bullet = if bullets.is_empty() {
                    '*'
                } else {
                    bullets[bullet_index % bullets.len()]
                };
                output.push(bullet);
                output.push(' ');
            }
        }

        let item_start_pos = output.len();

        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, &li_ctx, depth + 1, dom_ctx);
            }
        }

        trim_trailing_whitespace(output);

        if !ctx.in_table_cell {
            if let Some(ref sc) = ctx.structure_collector {
                if item_start_pos <= output.len() && output.is_char_boundary(item_start_pos) {
                    let rendered = &output[item_start_pos..];
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
            use std::collections::BTreeMap;

            let attributes: BTreeMap<String, String> = collect_tag_attributes(tag);

            let parent_tag = dom_ctx
                .parent_of(node_handle.get_inner())
                .and_then(|pid| dom_ctx.tag_name_for(dom_ctx.node_handle(pid).copied()?, parser))
                .map(|s| s.to_string());

            let index = dom_ctx.sibling_index(node_handle.get_inner()).unwrap_or(0);

            let node_ctx = NodeContext {
                node_type: NodeType::ListItem,
                tag_name: "li".to_string(),
                attributes,
                depth,
                index_in_parent: index,
                parent_tag,
                is_inline: false,
            };

            let last_line_start = output.rfind('\n').map_or(0, |pos| pos + 1);
            let last_line = &output[last_line_start..];

            let (marker, text_content) = if is_task_list {
                let task_marker = if task_checked { "- [x]" } else { "- [ ]" };
                let text_start = last_line.find(task_marker).map_or(0, |pos| pos + task_marker.len());
                (task_marker.to_string(), last_line[text_start..].trim().to_string())
            } else if ctx.in_ordered_list {
                let marker_text = format!("{}.", ctx.list_counter);
                let text_start = last_line.find(&marker_text).map_or(0, |pos| pos + marker_text.len());
                (marker_text, last_line[text_start..].trim().to_string())
            } else {
                let bullets: Vec<char> = options.bullets.chars().collect();
                let bullet_index = if ctx.ul_depth > 0 { ctx.ul_depth - 1 } else { 0 };
                let bullet = if bullets.is_empty() {
                    '*'
                } else {
                    bullets[bullet_index % bullets.len()]
                };
                let bullet_str = bullet.to_string();
                let text_start = last_line.find(bullet).map_or(0, |pos| pos + 1);
                (bullet_str, last_line[text_start..].trim().to_string())
            };

            let visit_result = {
                let mut visitor = visitor_handle.borrow_mut();
                visitor.visit_list_item(&node_ctx, ctx.in_ordered_list, &marker, &text_content)
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
