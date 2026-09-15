//! Utility functions for table processing.
//!
//! Provides helper functions for tag name normalization and comparison.

pub(super) use crate::converter::main_helpers::tag_name_eq;
pub(super) use crate::converter::utility::content::normalized_tag_name;

/// Check if a node has a specific tag name.
///
/// Handles both direct tag matching and DOM context-based tag resolution.
///
/// # Arguments
/// * `node_handle` - Handle to the node
/// * `parser` - HTML parser instance
/// * `dom_ctx` - DOM context for tag name resolution
/// * `name` - Expected tag name
///
/// # Returns
/// True if node has the specified tag name
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_tag_name(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &super::super::super::DomContext,
    name: &str,
) -> bool {
    if let Some(info) = dom_ctx.tag_info(node_handle.get_inner(), parser) {
        return info.name == name;
    }
    matches!(
        node_handle.get(parser),
        Some(tl::Node::Tag(tag)) if tag_name_eq(tag.name().as_utf8_str(), name)
    )
}

/// Check whether a node is a `<table>`, or has one anywhere in its subtree, without
/// crossing into a `<table>`'s own descendants once one is found.
///
/// # Arguments
/// * `node_handle` - Handle to the node to search from (included in the search)
/// * `parser` - HTML parser instance
/// * `dom_ctx` - DOM context for tag name and children resolution
///
/// # Returns
/// True if `node_handle` is a `table`, or a `table` is reachable through non-table
/// descendants (e.g. a `<div>` wrapper).
// ~keep An explicit `Vec` work-list, not recursion -- adversarial deep markup can nest
// ~keep thousands of wrapper elements without ever using a `<table>` tag, and a recursive
// ~keep walk over that shape overflows the native stack (see
// ~keep tests/deep_nesting_overflow.rs). Mirrors `scanner.rs::scan_own_structure`'s
// ~keep work-list, which has always descended through wrapper elements to find a nested
// ~keep table -- `render_cell_text`'s single-node tag check disagreed with that and let a
// ~keep div-wrapped nested table fall through as unescaped flattened text (issue #488). No
// ~keep depth cap: returning at the first table found means the search never crosses a
// ~keep table boundary, so each nesting level only scans its own non-table nodes and total
// ~keep cost stays linear.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_or_contains_table(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &super::super::super::DomContext,
) -> bool {
    let mut work = vec![*node_handle];
    while let Some(handle) = work.pop() {
        if is_tag_name(&handle, parser, dom_ctx, "table") {
            return true;
        }
        let Some(tl::Node::Tag(tag)) = handle.get(parser) else {
            continue;
        };
        if let Some(children) = dom_ctx.children_of(handle.get_inner()) {
            work.extend(children.iter().copied());
        } else {
            work.extend(tag.children().top().iter().copied());
        }
    }
    false
}
