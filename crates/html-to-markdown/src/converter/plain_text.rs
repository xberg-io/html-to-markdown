//! Plain text extraction from parsed HTML DOM.
//!
//! Provides a fast-path text extractor that walks the DOM tree collecting only
//! visible text content with structural whitespace, bypassing the full
//! Markdown/Djot conversion pipeline.

use std::collections::HashSet;
use std::fmt::Write;

use crate::converter::preprocessing_helpers::should_drop_for_preprocessing;
use crate::options::ConversionOptions;
use crate::text;

#[cfg(feature = "visitor")]
use crate::converter::utility::content::{collect_tag_attributes, is_block_level_element};
#[cfg(feature = "visitor")]
use crate::visitor::{NodeContext, NodeType, VisitResult, VisitorHandle};
#[cfg(feature = "visitor")]
use std::collections::BTreeMap;

/// Tracks list context for proper marker emission on `<li>` elements.
#[derive(Clone, Debug)]
enum ListContext {
    /// Not inside any list.
    None,
    /// Inside `<ul>` — each `<li>` gets a `- ` prefix.
    Unordered,
    /// Inside `<ol>` — each `<li>` gets a sequential `N. ` prefix.
    /// The `next_index` is incremented after each `<li>`.
    Ordered { next_index: u32 },
}

/// Tags whose content should be skipped entirely.
const SKIP_TAGS: &[&str] = &["script", "style", "head", "template", "noscript", "svg", "math"];

/// Block-level tags that should be separated by blank lines.
const BLOCK_TAGS: &[&str] = &[
    "p",
    "div",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "blockquote",
    "section",
    "article",
    "aside",
    "main",
    "nav",
    "header",
    "footer",
    "figure",
    "figcaption",
    "details",
    "summary",
    "address",
    "hgroup",
    "search",
];

/// Shared walker state threaded through all recursive calls.
///
/// Holds the options, visitor (feature-gated), and current DOM depth.
/// Using a struct avoids feature-gated function parameters at call sites.
struct WalkState<'a> {
    options: &'a ConversionOptions,
    excluded_node_ids: &'a HashSet<u32>,
    depth: usize,
    #[cfg(feature = "visitor")]
    visitor: Option<&'a VisitorHandle>,
}

impl WalkState<'_> {
    fn descend(&self) -> Self {
        WalkState {
            options: self.options,
            excluded_node_ids: self.excluded_node_ids,
            depth: self.depth + 1,
            #[cfg(feature = "visitor")]
            visitor: self.visitor,
        }
    }
}

/// Extract plain text from a parsed DOM tree.
///
/// Walks the tree collecting visible text with structural whitespace:
/// - Block elements get blank-line separation
/// - `<br>` becomes a newline, `<hr>` a blank line
/// - `<pre>` preserves internal whitespace
/// - `<img>` outputs alt text (unless `skip_images` is set)
/// - `<script>`, `<style>`, `<head>`, `<template>`, `<noscript>` are skipped
/// - Tables: cells separated by tab, rows by newline
/// - Inline elements are recursed without markers
/// - Nodes matching `excluded_node_ids` (from `exclude_selectors`) are dropped entirely
/// - When a visitor is configured, `visit_element_start`, `visit_element_end`, and
///   `visit_text` callbacks are fired and their results are honoured.
pub fn extract_plain_text(dom: &tl::VDom, parser: &tl::Parser, options: &ConversionOptions) -> String {
    let mut buf = String::with_capacity(1024);
    let mut list_ctx = ListContext::None;

    // Pre-compute excluded node IDs from exclude_selectors.
    let excluded_node_ids: HashSet<u32> = if options.exclude_selectors.is_empty() {
        HashSet::new()
    } else {
        let mut ids = HashSet::new();
        for selector in &options.exclude_selectors {
            if let Some(iter) = dom.query_selector(selector) {
                for handle in iter {
                    ids.insert(handle.get_inner());
                }
            }
        }
        ids
    };

    let state = WalkState {
        options,
        excluded_node_ids: &excluded_node_ids,
        depth: 0,
        #[cfg(feature = "visitor")]
        visitor: options.visitor.as_ref(),
    };

    for child_handle in dom.children() {
        walk_plain(child_handle, parser, &mut buf, false, &mut list_ctx, &state);
    }

    post_process(&mut buf);
    buf
}

/// Recursive plain-text walker.
fn walk_plain(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    buf: &mut String,
    in_pre: bool,
    list_ctx: &mut ListContext,
    state: &WalkState<'_>,
) {
    let Some(node) = node_handle.get(parser) else {
        return;
    };

    match node {
        tl::Node::Raw(bytes) => {
            let raw = bytes.as_utf8_str();
            let decoded = text::decode_html_entities_cow(raw.as_ref());

            #[cfg(feature = "visitor")]
            if let Some(visitor_handle) = state.visitor {
                let text_str: &str = &decoded;
                let node_ctx = NodeContext {
                    node_type: NodeType::Text,
                    tag_name: String::new(),
                    attributes: BTreeMap::new(),
                    depth: state.depth,
                    index_in_parent: 0,
                    parent_tag: None,
                    is_inline: true,
                };
                let result = visitor_handle
                    .lock()
                    .expect("visitor mutex poisoned")
                    .visit_text(&node_ctx, text_str);
                match result {
                    VisitResult::Skip => return,
                    VisitResult::Custom(custom) => {
                        buf.push_str(&custom);
                        return;
                    }
                    _ => {}
                }
            }

            if in_pre {
                buf.push_str(&decoded);
            } else {
                let normalized = text::normalize_whitespace_cow(&decoded);
                if !normalized.is_empty() {
                    // Avoid leading space at start of a new line
                    if normalized.as_ref() == " " && buf.ends_with('\n') {
                        return;
                    }
                    buf.push_str(&normalized);
                }
            }
        }
        tl::Node::Tag(tag) => {
            // Drop elements matching exclude_selectors, including all their descendants.
            if !state.excluded_node_ids.is_empty() && state.excluded_node_ids.contains(&node_handle.get_inner()) {
                return;
            }

            let tag_name = tag.name().as_utf8_str().to_ascii_lowercase();
            let tag_str = tag_name.as_str();

            // Skip invisible content
            if SKIP_TAGS.contains(&tag_str) {
                return;
            }

            // Apply preprocessing: drop nav/footer/aside/noise elements
            // (shared logic with the markdown path).
            if should_drop_for_preprocessing(tag_str, tag, state.options) {
                return;
            }

            // --- visitor: element start ---
            #[cfg(feature = "visitor")]
            if let Some(visitor_handle) = state.visitor {
                let attributes = collect_tag_attributes(tag);
                let node_ctx = NodeContext {
                    node_type: NodeType::Element,
                    tag_name: tag_str.to_string(),
                    attributes,
                    depth: state.depth,
                    index_in_parent: 0,
                    parent_tag: None,
                    is_inline: !is_block_level_element(tag_str),
                };
                let result = visitor_handle
                    .lock()
                    .expect("visitor mutex poisoned")
                    .visit_element_start(&node_ctx);
                match result {
                    VisitResult::Skip => return,
                    VisitResult::Custom(custom) => {
                        buf.push_str(&custom);
                        // Still call visit_element_end with the custom content as context.
                        let end_result = visitor_handle
                            .lock()
                            .expect("visitor mutex poisoned")
                            .visit_element_end(&node_ctx, &custom);
                        match end_result {
                            VisitResult::Custom(replacement) => {
                                let trim_len = buf.len() - custom.len();
                                buf.truncate(trim_len);
                                buf.push_str(&replacement);
                            }
                            VisitResult::Skip => {
                                let trim_len = buf.len() - custom.len();
                                buf.truncate(trim_len);
                            }
                            _ => {}
                        }
                        return;
                    }
                    _ => {}
                }
            }

            // Record the buf position before this element's content so visit_element_end
            // can truncate back to it for Custom/Skip results.
            #[cfg(feature = "visitor")]
            let element_output_start = buf.len();

            let child_state = state.descend();

            match tag_str {
                "br" => {
                    buf.push('\n');
                }
                "hr" => {
                    ensure_blank_line(buf);
                }
                "pre" => {
                    ensure_blank_line(buf);
                    walk_children(tag, parser, buf, true, list_ctx, &child_state);
                    ensure_blank_line(buf);
                }
                "img" => {
                    if !state.options.skip_images {
                        if let Some(Some(alt)) = tag.attributes().get("alt") {
                            let alt_text = alt.as_utf8_str();
                            if !alt_text.is_empty() {
                                buf.push_str(alt_text.as_ref());
                            }
                        }
                    }
                }
                "table" => {
                    ensure_blank_line(buf);
                    walk_table(tag, parser, buf, &child_state);
                    ensure_blank_line(buf);
                }
                "ul" => {
                    ensure_newline(buf);
                    let mut child_ctx = ListContext::Unordered;
                    walk_children(tag, parser, buf, false, &mut child_ctx, &child_state);
                    ensure_newline(buf);
                }
                "ol" => {
                    let start = tag
                        .attributes()
                        .get("start")
                        .flatten()
                        .and_then(|v| v.as_utf8_str().parse::<u32>().ok())
                        .unwrap_or(1);
                    ensure_newline(buf);
                    let mut child_ctx = ListContext::Ordered { next_index: start };
                    walk_children(tag, parser, buf, false, &mut child_ctx, &child_state);
                    ensure_newline(buf);
                }
                "li" => {
                    ensure_newline(buf);
                    match list_ctx {
                        ListContext::Unordered => {
                            buf.push_str("- ");
                        }
                        ListContext::Ordered { next_index } => {
                            let _ = write!(buf, "{}. ", next_index);
                            *next_index += 1;
                        }
                        ListContext::None => {
                            // <li> outside a list — emit with bullet as fallback
                            buf.push_str("- ");
                        }
                    }
                    walk_children(tag, parser, buf, false, list_ctx, &child_state);
                    ensure_newline(buf);
                }
                _ if BLOCK_TAGS.contains(&tag_str) => {
                    ensure_blank_line(buf);
                    walk_children(tag, parser, buf, in_pre, list_ctx, &child_state);
                    ensure_blank_line(buf);
                }
                _ => {
                    // Inline elements and structural containers (html, body, etc.)
                    walk_children(tag, parser, buf, in_pre, list_ctx, &child_state);
                }
            }

            // --- visitor: element end ---
            #[cfg(feature = "visitor")]
            if let Some(visitor_handle) = state.visitor {
                let attributes = collect_tag_attributes(tag);
                let node_ctx = NodeContext {
                    node_type: NodeType::Element,
                    tag_name: tag_str.to_string(),
                    attributes,
                    depth: state.depth,
                    index_in_parent: 0,
                    parent_tag: None,
                    is_inline: !is_block_level_element(tag_str),
                };
                // Clamp safe_start in case children truncated the buffer.
                let safe_start = element_output_start.min(buf.len());
                let element_content = &buf[safe_start..];
                let result = visitor_handle
                    .lock()
                    .expect("visitor mutex poisoned")
                    .visit_element_end(&node_ctx, element_content);
                match result {
                    VisitResult::Custom(custom) => {
                        buf.truncate(safe_start);
                        buf.push_str(&custom);
                    }
                    VisitResult::Skip => {
                        buf.truncate(safe_start);
                    }
                    _ => {}
                }
            }
        }
        tl::Node::Comment(_) => {}
    }
}

/// Walk all children of a tag.
fn walk_children(
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    buf: &mut String,
    in_pre: bool,
    list_ctx: &mut ListContext,
    state: &WalkState<'_>,
) {
    let children = tag.children();
    let top = children.top();
    for child in top.iter() {
        walk_plain(child, parser, buf, in_pre, list_ctx, state);
    }
}

/// Walk a `<table>` element, extracting cells as tab-separated, rows as newline-separated.
fn walk_table(table_tag: &tl::HTMLTag, parser: &tl::Parser, buf: &mut String, state: &WalkState<'_>) {
    // Collect all <tr> node handles by recursing into the table
    let mut row_handles = Vec::new();
    collect_descendant_handles(table_tag, parser, "tr", &mut row_handles);

    for (row_idx, row_handle) in row_handles.iter().enumerate() {
        if row_idx > 0 {
            buf.push('\n');
        }
        let Some(tl::Node::Tag(row_tag)) = row_handle.get(parser) else {
            continue;
        };

        // Collect direct <th>/<td> children
        let mut cell_handles = Vec::new();
        let row_children = row_tag.children();
        let row_top = row_children.top();
        for child in row_top.iter() {
            if let Some(tl::Node::Tag(child_tag)) = child.get(parser) {
                let name = child_tag.name().as_utf8_str();
                if name.eq_ignore_ascii_case("th") || name.eq_ignore_ascii_case("td") {
                    cell_handles.push(*child);
                }
            }
        }

        let cell_state = state.descend();
        for (cell_idx, cell_handle) in cell_handles.iter().enumerate() {
            if cell_idx > 0 {
                buf.push('\t');
            }
            let mut cell_buf = String::new();
            if let Some(tl::Node::Tag(cell_tag)) = cell_handle.get(parser) {
                let mut cell_list_ctx = ListContext::None;
                walk_children(cell_tag, parser, &mut cell_buf, false, &mut cell_list_ctx, &cell_state);
            }
            buf.push_str(cell_buf.trim());
        }
    }
}

/// Recursively collect all descendant `NodeHandle`s matching `target_tag` (by cloning handles).
fn collect_descendant_handles(
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    target_tag: &str,
    result: &mut Vec<tl::NodeHandle>,
) {
    let children = tag.children();
    let top = children.top();
    for child in top.iter() {
        if let Some(tl::Node::Tag(child_tag)) = child.get(parser) {
            if child_tag.name().as_utf8_str().eq_ignore_ascii_case(target_tag) {
                result.push(*child);
            } else {
                collect_descendant_handles(child_tag, parser, target_tag, result);
            }
        }
    }
}

/// Ensure the buffer ends with a blank line (two newlines).
fn ensure_blank_line(buf: &mut String) {
    if buf.is_empty() {
        return;
    }
    // Strip trailing horizontal whitespace
    while buf.ends_with(' ') || buf.ends_with('\t') {
        buf.pop();
    }
    let current_newlines = buf.chars().rev().take_while(|&c| c == '\n').count();
    for _ in current_newlines..2 {
        buf.push('\n');
    }
}

/// Ensure the buffer ends with at least one newline.
fn ensure_newline(buf: &mut String) {
    if buf.is_empty() {
        return;
    }
    if !buf.ends_with('\n') {
        buf.push('\n');
    }
}

/// Single-pass post-processor: trims trailing whitespace per line, collapses runs of 3+
/// newlines to exactly 2, and normalizes the trailing newline. Walking `chars()` (not
/// bytes) ensures multibyte UTF-8 codepoints are never split.
///
/// A line that is empty after trailing-whitespace trim is treated as part of the adjacent
/// newline run (it does not reset the consecutive-newline counter), so space-only lines
/// between paragraphs collapse to a single blank line without absorbing a counter slot.
///
/// Kept as a named function rather than inlined so the markdown converter's symmetrical
/// `post_process` call site reads consistently with this one.
fn normalize_plain_output(buf: &mut String) {
    let input = std::mem::take(buf);
    let mut out = String::with_capacity(input.len());
    let mut line_start = 0usize;
    let mut consecutive_newlines = 0usize;
    for ch in input.chars() {
        if ch == '\n' {
            let trimmed_len = out[line_start..].trim_end().len();
            out.truncate(line_start + trimmed_len);
            // Only reset the newline counter when the trimmed line has content; a
            // whitespace-only line continues the surrounding newline run.
            if out.len() > line_start {
                consecutive_newlines = 0;
            }
            consecutive_newlines += 1;
            if consecutive_newlines <= 2 {
                out.push('\n');
                line_start = out.len();
            }
        } else {
            // Only non-whitespace content resets the newline counter; whitespace chars
            // may still belong to a space-only line that will be collapsed on the next '\n'.
            if !ch.is_whitespace() {
                consecutive_newlines = 0;
            }
            out.push(ch);
        }
    }
    let keep = out.trim_end_matches('\n').len();
    out.truncate(keep);
    if !out.is_empty() {
        out.push('\n');
    }
    *buf = out;
}

/// Post-process the accumulated plain-text buffer.
fn post_process(buf: &mut String) {
    normalize_plain_output(buf);
}
