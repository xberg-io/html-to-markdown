//! Visitor state management and context building.
//!
//! This module handles construction of `NodeContext` objects that represent
//! the state of DOM nodes during traversal.

use std::collections::BTreeMap;

use crate::visitor::NodeContext;
use crate::visitor::NodeType;

/// Build a `NodeContext` from current parsing state.
///
/// Creates a complete `NodeContext` suitable for passing to visitor callbacks.
/// This function collects metadata about the current node from various sources:
/// - Tag name and attributes from the HTML element
/// - Depth and parent information from the DOM tree
/// - Index among siblings for positional awareness
/// - Inline/block classification
///
/// # Parameters
///
/// - `node_type`: Coarse-grained classification (Link, Image, Heading, etc.)
/// - `tag_name`: Raw HTML tag name (e.g., "div", "h1", "custom-element")
/// - `attributes`: All HTML attributes as key-value pairs
/// - `depth`: Nesting depth in the DOM tree (0 = root)
/// - `index_in_parent`: Zero-based index among siblings
/// - `parent_tag`: Parent element's tag name (None if root)
/// - `is_inline`: Whether this element is treated as inline vs block
///
/// # Returns
///
/// A fully populated `NodeContext` ready for visitor dispatch.
///
/// # Performance
///
/// This function performs minimal allocations:
/// - Clones `tag_name` (typically 2-10 bytes)
/// - Clones `parent_tag` if present (typically 2-10 bytes)
/// - Clones the attributes `BTreeMap` (heap allocation if non-empty)
///
/// For text nodes and simple elements without attributes, allocations are minimal.
///
/// # Examples
///
/// ```text
/// let ctx = build_node_context(
///     NodeType::Heading,
///     "h1",
///     &attrs,
///     1,
///     0,
///     Some("body"),
///     false,
/// );
/// ```
#[allow(dead_code)]
#[inline]
pub fn build_node_context(
    node_type: NodeType,
    tag_name: &str,
    attributes: &BTreeMap<String, String>,
    depth: usize,
    index_in_parent: usize,
    parent_tag: Option<&str>,
    is_inline: bool,
) -> NodeContext {
    NodeContext {
        node_type,
        tag_name: tag_name.to_string(),
        attributes: attributes.clone(),
        depth,
        index_in_parent,
        parent_tag: parent_tag.map(String::from),
        is_inline,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_node_context() {
        let mut attrs = BTreeMap::new();
        attrs.insert("id".to_string(), "main".to_string());
        attrs.insert("class".to_string(), "container".to_string());

        let ctx = build_node_context(NodeType::Div, "div", &attrs, 2, 3, Some("body"), false);

        assert_eq!(ctx.node_type, NodeType::Div);
        assert_eq!(ctx.tag_name, "div");
        assert_eq!(ctx.depth, 2);
        assert_eq!(ctx.index_in_parent, 3);
        assert_eq!(ctx.parent_tag, Some("body".to_string()));
        assert!(!ctx.is_inline);
        assert_eq!(ctx.attributes.len(), 2);
        assert_eq!(ctx.attributes.get("id"), Some(&"main".to_string()));
    }

    #[test]
    fn test_build_node_context_no_parent() {
        let attrs = BTreeMap::new();

        let ctx = build_node_context(NodeType::Html, "html", &attrs, 0, 0, None, false);

        assert_eq!(ctx.node_type, NodeType::Html);
        assert_eq!(ctx.parent_tag, None);
        assert!(ctx.attributes.is_empty());
    }
}
