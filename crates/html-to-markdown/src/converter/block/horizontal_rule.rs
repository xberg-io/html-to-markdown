//! Handler for horizontal rule elements (hr).
//!
//! Converts HTML horizontal rule tags to Markdown horizontal rules (---)
//! with appropriate spacing handling based on context.

use crate::converter::block::container::HandlerContext;
use crate::converter::main_helpers::trim_trailing_whitespace;
use crate::converter::utility::siblings::get_previous_sibling_tag;
use crate::dom::{NodeHandle, Parser};
#[cfg(feature = "visitor")]
use std::borrow::Cow;

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle horizontal rule elements (hr).
///
/// Converts to Markdown horizontal rule (---) with appropriate blank line
/// spacing based on context and previous siblings.
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle(node_handle: &NodeHandle, parser: &Parser, output: &mut String, handler: HandlerContext<'_>) {
    let HandlerContext {
        options,
        ctx,
        depth: _,
        dom_ctx,
    } = handler;
    #[cfg(feature = "visitor")]
    if visit_horizontal_rule(node_handle, parser, output, handler) {
        return;
    }

    if ctx.in_marker_text() {
        join_rule_to_text(output);
        output.push_str("--- ");
        return;
    }

    // ~keep A rule after other content of a list item starts at the item's content column, or
    // ~keep it ends the list (issue #583). Whitespace after a bare marker (a `<br>` there) is
    // ~keep not content: the rule then follows the marker like a rule at the marker.
    // ~keep A cell writes no list marker, so text in a cell that looks like one is text (issue #628).
    let at_bare_marker = ctx.in_list_item
        && !ctx.in_table_cell
        && crate::converter::list::utils::trim_whitespace_after_bare_marker(output);
    // ~keep A rule that is the item's first content is `___` on the marker line. `- ---` is a
    // ~keep rule of its own, and after a blank line the rule leaves the item empty and ends the
    // ~keep list. On the next line it can make the text above a heading: an empty item after
    // ~keep text is not an item.
    if at_bare_marker {
        output.push_str("___\n");
        return;
    }
    let list_indent = list_rule_indent(output, options, ctx);
    separate_from_previous_rule(node_handle, parser, output, dom_ctx);
    if let Some(indent) = list_indent {
        output.push_str(&indent);
    }
    output.push_str("---\n");
}

fn list_rule_indent(output: &str, options: &crate::options::ConversionOptions, ctx: &Context) -> Option<String> {
    if !ctx.in_list_item || ctx.convert_as_inline || ctx.in_table_cell || output.is_empty() {
        return None;
    }
    crate::converter::list::utils::continuation_indent_string(ctx.list_indent_columns, options)
        .filter(|indent| crate::converter::list::utils::item_is_open(output, indent, ctx))
}

fn separate_from_previous_rule(node_handle: &NodeHandle, parser: &Parser, output: &mut String, dom_ctx: &DomContext) {
    if output.is_empty() {
        return;
    }
    let prev_tag = get_previous_sibling_tag(node_handle, parser, dom_ctx);
    let last_line_is_blockquote = output
        .rsplit('\n')
        .find(|line| !line.trim().is_empty())
        .is_some_and(|line| line.trim_start().starts_with('>'));
    let needs_blank_line = !matches!(prev_tag, Some("blockquote")) && !last_line_is_blockquote;

    if matches!(prev_tag, Some("blockquote")) && output.ends_with("\n\n") {
        output.truncate(output.len() - 1);
    } else if !needs_blank_line {
        if !output.ends_with('\n') {
            output.push('\n');
        }
    } else {
        trim_trailing_whitespace(output);
        if !output.ends_with("\n\n") {
            output.push_str(if output.ends_with('\n') { "\n" } else { "\n\n" });
        }
    }
}

#[cfg(feature = "visitor")]
fn visit_horizontal_rule(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    handler: HandlerContext<'_>,
) -> bool {
    use crate::visitor::{NodeContext, NodeType, VisitResult};

    let Some(ref visitor_handle) = handler.ctx.visitor else {
        return false;
    };
    let Some(crate::dom::Node::Tag(tag)) = node_handle.get(parser) else {
        return true;
    };
    let node_id = node_handle.get_inner();
    let parent_tag = handler.dom_ctx.parent_tag_name(node_id, parser);
    let index_in_parent = handler.dom_ctx.get_sibling_index(node_id).unwrap_or(0);
    let node_ctx = NodeContext::with_lazy_attributes(
        NodeType::Hr,
        Cow::Borrowed("hr"),
        tag,
        handler.depth,
        index_in_parent,
        parent_tag.map(Cow::Borrowed),
        false,
    );
    let visit_result = {
        let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
        visitor.visit_horizontal_rule(&node_ctx)
    };
    match visit_result {
        VisitResult::Continue => false,
        VisitResult::Skip => true,
        VisitResult::Custom(custom) => {
            output.push_str(&custom);
            true
        }
        VisitResult::PreserveHtml => {
            use crate::converter::utility::serialization::serialize_node;
            output.push_str(&serialize_node(node_handle, parser));
            true
        }
        VisitResult::Error(err) => {
            if handler.ctx.visitor_error.borrow().is_none() {
                *handler.ctx.visitor_error.borrow_mut() = Some(err);
            }
            true
        }
    }
}

/// Continue the text in `output` with a space for a rule written between inline markers.
///
/// ~keep Between markers a rule has no Markdown form, so it is the word `---` in the running line,
/// ~keep as in a link label. On a line of its own it would end the markers at the blank line
/// ~keep before it, or turn the text above it into a heading (issue #603). A line break before it
/// ~keep goes with the whitespace; `walk_node` already strips a backslash-style break before a block.
fn join_rule_to_text(output: &mut String) {
    let text_len = output.trim_end_matches([' ', '\t', '\n', '\r']).len();
    output.truncate(text_len);
    if !output.is_empty() {
        output.push(' ');
    }
}

/// Write a blank line before a container's `content` when it starts with a rule and `output` ends
/// in text or in a bare list marker. Between inline markers, join the rule to the text instead. The container rendered the rule into its own buffer, so the
/// rule saw nothing before it; on a marker line the rule would swallow the item (`- ---` is a rule).
pub fn separate_leading_rule(output: &mut String, content: &str, ctx: &Context) {
    if ctx.in_marker_text() {
        if content.split([' ', '\n']).next() == Some("---") {
            join_rule_to_text(output);
        }
        return;
    }
    if content.split('\n').next() != Some("---") || output.is_empty() || output.ends_with("\n\n") {
        return;
    }
    // ~keep In a cell the blank line takes the place of a cell break before the rule (issue #628).
    if ctx.in_table_cell {
        crate::converter::main_helpers::trim_trailing_whitespace(output);
    }
    // ~keep The column a list item wrote for this line stays with the rule (issue #583).
    let line_start = output.rfind('\n').map_or(0, |pos| pos + 1);
    let indent = if output[line_start..].trim().is_empty() {
        output.split_off(line_start)
    } else {
        String::new()
    };
    if !output.ends_with("\n\n") {
        output.push_str(if output.ends_with('\n') { "\n" } else { "\n\n" });
    }
    output.push_str(&indent);
}
