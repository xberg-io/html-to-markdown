//! Content extraction and manipulation utilities.
//!
//! Functions for extracting and processing element content, including text collection
//! and empty element detection.

use crate::text;
use std::borrow::Cow;
#[cfg(feature = "visitor")]
use std::collections::BTreeMap;

// Forward declare DomContext from parent module to avoid circular imports
pub(crate) use crate::converter::DomContext;

/// Collect all attributes from an HTML tag as a `BTreeMap<String, String>`.
///
/// Boolean attributes (those with `None` as the value) are skipped; only
/// attributes that carry an explicit value are included.
#[cfg(feature = "visitor")]
pub(crate) fn collect_tag_attributes(tag: &tl::HTMLTag) -> BTreeMap<String, String> {
    tag.attributes()
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|val| (k.to_string(), val.to_string())))
        .collect()
}

/// Chomp whitespace from inline element content, preserving line breaks.
///
/// Similar to `text::chomp` but handles line breaks from `<br>` tags specially.
/// Line breaks are extracted as suffix to be placed outside formatting.
/// Returns (prefix, suffix, `trimmed_text`).
pub(crate) fn chomp_inline(text: &str) -> (&str, &str, &str) {
    if text.is_empty() {
        return ("", "", "");
    }

    let prefix = if text.starts_with(&[' ', '\t'][..]) { " " } else { "" };

    let has_trailing_linebreak = text.ends_with("  \n") || text.ends_with("\\\n");

    let suffix = if has_trailing_linebreak {
        if text.ends_with("  \n") { "  \n" } else { "\\\n" }
    } else if text.ends_with(&[' ', '\t'][..]) {
        " "
    } else {
        ""
    };

    let trimmed = if has_trailing_linebreak {
        text.strip_suffix("  \n").map_or_else(
            || text.strip_suffix("\\\n").map_or_else(|| text.trim(), |s| s.trim()),
            |s| s.trim(),
        )
    } else {
        text.trim()
    };

    (prefix, suffix, trimmed)
}

/// Get the text content of a node and its children.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn get_text_content(node_handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> String {
    dom_ctx.text_content(*node_handle, parser)
}

/// Collect inline text for link labels, skipping block-level descendants.
#[allow(clippy::match_wildcard_for_single_variants)]
pub(crate) fn collect_link_label_text(
    children: &[tl::NodeHandle],
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) -> (String, Vec<tl::NodeHandle>, bool) {
    let mut text = String::new();
    let mut saw_block = false;
    let mut block_nodes = Vec::new();
    let mut stack: Vec<_> = children.iter().rev().copied().collect();

    while let Some(handle) = stack.pop() {
        if let Some(node) = handle.get(parser) {
            match node {
                tl::Node::Raw(bytes) => {
                    let raw = bytes.as_utf8_str();
                    let decoded = text::decode_html_entities_cow(raw.as_ref());
                    text.push_str(decoded.as_ref());
                }
                tl::Node::Tag(tag) => {
                    let is_block = dom_ctx.tag_info(handle.get_inner(), parser).map_or_else(
                        || {
                            let tag_name = normalized_tag_name(tag.name().as_utf8_str());
                            is_block_level_element(tag_name.as_ref())
                        },
                        |info| info.is_block,
                    );
                    if is_block {
                        saw_block = true;
                        block_nodes.push(handle);
                        continue;
                    }

                    if let Some(children) = dom_ctx.children_of(handle.get_inner()) {
                        for child in children.iter().rev() {
                            stack.push(*child);
                        }
                    } else {
                        let tag_children = tag.children();
                        let mut child_nodes: Vec<_> = tag_children.top().iter().copied().collect();
                        child_nodes.reverse();
                        stack.extend(child_nodes);
                    }
                }
                _ => {}
            }
        }
    }

    (text, block_nodes, saw_block)
}

/// Normalize a link label by collapsing newlines and normalizing whitespace.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn normalize_link_label(label: &str) -> String {
    let mut needs_collapse = false;
    for ch in label.chars() {
        if ch == '\n' || ch == '\r' {
            needs_collapse = true;
            break;
        }
    }

    let collapsed = if needs_collapse {
        let mut collapsed = String::with_capacity(label.len());
        for ch in label.chars() {
            if ch == '\n' || ch == '\r' {
                collapsed.push(' ');
            } else {
                collapsed.push(ch);
            }
        }
        Cow::Owned(collapsed)
    } else {
        Cow::Borrowed(label)
    };

    let normalized = text::normalize_whitespace_cow(collapsed.as_ref());
    normalized.as_ref().trim().to_string()
}

/// Normalize a tag name to lowercase, preserving borrowed input when possible.
pub(crate) fn normalized_tag_name(raw: Cow<'_, str>) -> Cow<'_, str> {
    if raw.as_bytes().iter().any(u8::is_ascii_uppercase) {
        let mut owned = raw.into_owned();
        owned.make_ascii_lowercase();
        Cow::Owned(owned)
    } else {
        raw
    }
}

/// Check if an element is block-level (not inline).
pub(crate) fn is_block_level_element(tag_name: &str) -> bool {
    is_block_level_name(tag_name, crate::converter::main_helpers::is_inline_element(tag_name))
}

/// Returns the largest valid char boundary index at or before `index`.
///
/// If `index` is already a char boundary it is returned unchanged.
/// Otherwise it walks backwards to find one.  Returns 0 if no boundary
/// is found before `index`.
pub fn floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        s.len()
    } else {
        let mut i = index;
        while i > 0 && !s.is_char_boundary(i) {
            i -= 1;
        }
        i
    }
}

/// Escape special Markdown characters in a link label.
///
/// Handles bracket escaping to prevent unintended link label termination.
/// Tracks bracket depth and escapes closing brackets when depth is zero.
///
/// # Examples
/// ```text
/// Input:  "[link]"
/// Output: "[link\\]"
///
/// Input:  "[outer [inner]]"
/// Output: "[outer [inner]]"
/// ```
pub(crate) fn escape_link_label(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(text.len());
    let mut backslash_count = 0usize;
    let mut bracket_depth = 0usize;

    for ch in text.chars() {
        if ch == '\\' {
            result.push('\\');
            backslash_count += 1;
            continue;
        }

        let is_escaped = backslash_count % 2 == 1;
        backslash_count = 0;

        match ch {
            '[' if !is_escaped => {
                bracket_depth = bracket_depth.saturating_add(1);
                result.push('[');
            }
            ']' if !is_escaped => {
                if bracket_depth == 0 {
                    result.push('\\');
                } else {
                    bracket_depth -= 1;
                }
                result.push(']');
            }
            _ => result.push(ch),
        }
    }

    result
}

/// Helper for block-level element detection.
pub(crate) fn is_block_level_name(tag_name: &str, is_inline: bool) -> bool {
    !is_inline
        && matches!(
            tag_name,
            "address"
                | "article"
                | "aside"
                | "blockquote"
                | "canvas"
                | "dd"
                | "div"
                | "dl"
                | "dt"
                | "fieldset"
                | "figcaption"
                | "figure"
                | "footer"
                | "form"
                | "h1"
                | "h2"
                | "h3"
                | "h4"
                | "h5"
                | "h6"
                | "header"
                | "hr"
                | "li"
                | "main"
                | "nav"
                | "ol"
                | "p"
                | "pre"
                | "section"
                | "table"
                | "tfoot"
                | "ul"
        )
}
