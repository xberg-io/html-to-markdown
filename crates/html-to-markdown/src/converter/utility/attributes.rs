//! Attribute handling and extraction utilities.
//!
//! Functions for working with element attributes, semantic detection, and hOCR document detection.

use crate::converter::DomContext;
use crate::converter::utility::content::normalized_tag_name;

/// Check if a tag has main content semantics based on role or class.
pub fn tag_has_main_semantics(tag: &tl::HTMLTag) -> bool {
    if let Some(Some(role)) = tag.attributes().get("role") {
        let lowered = role.as_utf8_str().to_ascii_lowercase();
        if matches!(lowered.as_str(), "main" | "article" | "document" | "region") {
            return true;
        }
    }

    if let Some(Some(class_bytes)) = tag.attributes().get("class") {
        let class_value = class_bytes.as_utf8_str().to_ascii_lowercase();
        const MAIN_CLASS_HINTS: &[&str] = &[
            "mw-body",
            "mw-parser-output",
            "content-body",
            "content-container",
            "article-body",
            "article-content",
            "main-content",
            "page-content",
            "entry-content",
            "post-content",
            "document-body",
        ];
        if MAIN_CLASS_HINTS.iter().any(|hint| class_value.contains(hint)) {
            return true;
        }
    }

    false
}

/// Navigation-hint keywords matched against `class` and `id` token values.
///
/// Shared between [`element_has_navigation_hint`] (Tier-2 DOM path) and the
/// Tier-1 byte-scanner's preprocessing skip logic so both paths use the same
/// canonical keyword list.
pub const NAV_KEYWORDS: &[&str] = &[
    "nav",
    "navigation",
    "navbar",
    "breadcrumbs",
    "breadcrumb",
    "toc",
    "sidebar",
    "sidenav",
    "menu",
    "menubar",
    "mainmenu",
    "subnav",
    "tabs",
    "tablist",
    "toolbar",
    "pager",
    "pagination",
    "skipnav",
    "skip-link",
    "skiplinks",
    "site-nav",
    "site-menu",
    "site-header",
    "site-footer",
    "topbar",
    "bottombar",
    "masthead",
    "vector-nav",
    "vector-header",
    "vector-footer",
];

/// Check if an element has navigation-related hints in its attributes.
pub fn element_has_navigation_hint(tag: &tl::HTMLTag) -> bool {
    if attribute_matches_any(tag, "role", &["navigation", "menubar", "tablist", "toolbar"]) {
        return true;
    }

    if attribute_contains_any(
        tag,
        "aria-label",
        &["navigation", "menu", "contents", "table of contents", "toc"],
    ) {
        return true;
    }

    attribute_matches_any(tag, "class", NAV_KEYWORDS) || attribute_matches_any(tag, "id", NAV_KEYWORDS)
}

/// Check if an attribute value matches any of the given keywords (space or custom-separator aware).
pub fn attribute_matches_any(tag: &tl::HTMLTag, attr: &str, keywords: &[&str]) -> bool {
    let Some(attr_value) = tag.attributes().get(attr) else {
        return false;
    };
    let Some(value) = attr_value else {
        return false;
    };
    let raw = value.as_utf8_str();
    raw.split_whitespace()
        .map(|token| {
            token
                .chars()
                .map(|c| match c {
                    '_' | ':' | '.' | '/' => '-',
                    _ => c,
                })
                .collect::<String>()
                .to_ascii_lowercase()
        })
        .filter(|token| !token.is_empty())
        .any(|token| keywords.iter().any(|kw| token == *kw))
}

/// Check if an attribute contains any of the given keywords (substring match).
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn attribute_contains_any(tag: &tl::HTMLTag, attr: &str, keywords: &[&str]) -> bool {
    let Some(attr_value) = tag.attributes().get(attr) else {
        return false;
    };
    let Some(value) = attr_value else {
        return false;
    };
    let lower = value.as_utf8_str().to_ascii_lowercase();
    keywords.iter().any(|kw| lower.contains(*kw))
}

/// Check if a node has a semantic content ancestor (main, article, section).
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn has_semantic_content_ancestor(node_handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    let mut current_id = node_handle.get_inner();
    while let Some(parent_id) = dom_ctx.parent_of(current_id) {
        if let Some(parent_info) = dom_ctx.tag_info(parent_id, parser) {
            if matches!(parent_info.name.as_str(), "main" | "article" | "section") {
                return true;
            }
        }
        if let Some(parent_handle) = dom_ctx.node_handle(parent_id) {
            if let Some(tl::Node::Tag(parent_tag)) = parent_handle.get(parser) {
                let parent_name = normalized_tag_name(parent_tag.name().as_utf8_str());
                if matches!(parent_name.as_ref(), "main" | "article" | "section") {
                    return true;
                }
                if tag_has_main_semantics(parent_tag) {
                    return true;
                }
            }
        }
        current_id = parent_id;
    }
    false
}
