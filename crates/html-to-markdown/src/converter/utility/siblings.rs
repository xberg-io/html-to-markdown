//! Sibling node navigation and handling.
//!
//! Utilities for working with sibling nodes in the DOM tree, including navigation functions
//! and inline/block element detection for whitespace handling.

use crate::converter::DomContext;

/// Get the tag name of the next sibling element.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn get_next_sibling_tag<'a>(
    node_handle: &tl::NodeHandle,
    parser: &'a tl::Parser,
    dom_ctx: &'a DomContext,
) -> Option<&'a str> {
    dom_ctx.next_tag_name(*node_handle, parser)
}

/// Get the tag name of the previous sibling element.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn get_previous_sibling_tag<'a>(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &'a DomContext,
) -> Option<&'a str> {
    let id = node_handle.get_inner();
    let parent = dom_ctx.parent_of(id);

    let siblings = if let Some(parent_id) = parent {
        dom_ctx.children_of(parent_id)?
    } else {
        &dom_ctx.root_children
    };

    let position = dom_ctx.sibling_index(id).or_else(|| {
        siblings
            .iter()
            .position(|handle: &tl::NodeHandle| handle.get_inner() == id)
    })?;

    for sibling in siblings.iter().take(position).rev() {
        if let Some(info) = dom_ctx.tag_info(sibling.get_inner(), parser) {
            return Some(info.name.as_str());
        }
        if let Some(tl::Node::Raw(raw)) = sibling.get(parser) {
            if !raw.as_utf8_str().trim().is_empty() {
                return None;
            }
        }
    }

    None
}

/// Check if the previous sibling is an inline tag.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn previous_sibling_is_inline_tag(node_handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    dom_ctx.previous_inline_like(*node_handle, parser)
}

/// Check if the next sibling is whitespace-only text.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn next_sibling_is_whitespace_text(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) -> bool {
    dom_ctx.next_whitespace_text(*node_handle, parser)
}

/// Check if the next sibling is an inline tag.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn next_sibling_is_inline_tag(node_handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    dom_ctx.next_inline_like(*node_handle, parser)
}

/// Check if the next sibling carries inline *content* — an inline-like tag, or a bare text node.
///
/// `next_sibling_is_inline_tag` answers a narrower question: its scan stops and returns `false` at
/// the first non-whitespace text sibling, because its callers care specifically about an adjacent
/// *element*. A newline-only inline wrapper needs the broader question, since `a<span>\n</span>b`
/// separates two words exactly as `a<span>\n</span><span>b</span>` does (issue #491); asking the
/// narrower one welded them together. Kept separate rather than widening
/// `DomContext::next_inline_like`, which is memoised and shared with an unrelated caller. ~keep
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn next_sibling_is_inline_content(node_handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    let id = node_handle.get_inner();
    let siblings = match dom_ctx.parent_of(id) {
        Some(parent_id) => match dom_ctx.children_of(parent_id) {
            Some(children) => children,
            None => return false,
        },
        None => &dom_ctx.root_children,
    };

    let Some(position) = dom_ctx
        .sibling_index(id)
        .or_else(|| siblings.iter().position(|handle| handle.get_inner() == id))
    else {
        return false;
    };

    for sibling in siblings.iter().skip(position + 1) {
        if let Some(info) = dom_ctx.tag_info(sibling.get_inner(), parser) {
            return info.is_inline_like;
        }
        if let Some(tl::Node::Raw(raw)) = sibling.get(parser) {
            if raw.as_utf8_str().trim().is_empty() {
                continue;
            }
            return true;
        }
    }

    false
}

/// Append an inline suffix to output, with smart whitespace handling.
///
/// Avoids adding spaces before siblings that are already whitespace.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn append_inline_suffix(
    output: &mut String,
    suffix: &str,
    has_core_content: bool,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) {
    if suffix.is_empty() {
        return;
    }

    if suffix == " " && has_core_content && next_sibling_is_whitespace_text(node_handle, parser, dom_ctx) {
        return;
    }

    output.push_str(suffix);
}
