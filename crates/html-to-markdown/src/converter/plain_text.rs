//! Plain text extraction from parsed HTML DOM.
//!
//! Provides a fast-path text extractor that walks the DOM tree collecting only
//! visible text content with structural whitespace, bypassing the full
//! Markdown/Djot conversion pipeline.

use std::fmt::Write;

use crate::options::ConversionOptions;
use crate::text;

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
pub fn extract_plain_text(dom: &tl::VDom, parser: &tl::Parser, options: &ConversionOptions) -> String {
    let mut buf = String::with_capacity(1024);
    let mut list_ctx = ListContext::None;

    for child_handle in dom.children() {
        walk_plain(child_handle, parser, &mut buf, options, false, &mut list_ctx);
    }

    post_process(&mut buf);
    buf
}

/// Recursive plain-text walker.
fn walk_plain(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    buf: &mut String,
    options: &ConversionOptions,
    in_pre: bool,
    list_ctx: &mut ListContext,
) {
    let Some(node) = node_handle.get(parser) else {
        return;
    };

    match node {
        tl::Node::Raw(bytes) => {
            let raw = bytes.as_utf8_str();
            let decoded = text::decode_html_entities_cow(raw.as_ref());
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
            let tag_name = tag.name().as_utf8_str().to_ascii_lowercase();
            let tag_str = tag_name.as_str();

            // Skip invisible content
            if SKIP_TAGS.contains(&tag_str) {
                return;
            }

            match tag_str {
                "br" => {
                    buf.push('\n');
                }
                "hr" => {
                    ensure_blank_line(buf);
                }
                "pre" => {
                    ensure_blank_line(buf);
                    walk_children(tag, parser, buf, options, true, list_ctx);
                    ensure_blank_line(buf);
                }
                "img" => {
                    if !options.skip_images {
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
                    walk_table(tag, parser, buf, options);
                    ensure_blank_line(buf);
                }
                "ul" => {
                    ensure_newline(buf);
                    let mut child_ctx = ListContext::Unordered;
                    walk_children(tag, parser, buf, options, false, &mut child_ctx);
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
                    walk_children(tag, parser, buf, options, false, &mut child_ctx);
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
                    walk_children(tag, parser, buf, options, false, list_ctx);
                    ensure_newline(buf);
                }
                _ if BLOCK_TAGS.contains(&tag_str) => {
                    ensure_blank_line(buf);
                    walk_children(tag, parser, buf, options, in_pre, list_ctx);
                    ensure_blank_line(buf);
                }
                _ => {
                    // Inline elements and structural containers (html, body, etc.)
                    walk_children(tag, parser, buf, options, in_pre, list_ctx);
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
    options: &ConversionOptions,
    in_pre: bool,
    list_ctx: &mut ListContext,
) {
    let children = tag.children();
    let top = children.top();
    for child in top.iter() {
        walk_plain(child, parser, buf, options, in_pre, list_ctx);
    }
}

/// Walk a `<table>` element, extracting cells as tab-separated, rows as newline-separated.
fn walk_table(table_tag: &tl::HTMLTag, parser: &tl::Parser, buf: &mut String, options: &ConversionOptions) {
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

        for (cell_idx, cell_handle) in cell_handles.iter().enumerate() {
            if cell_idx > 0 {
                buf.push('\t');
            }
            let mut cell_buf = String::new();
            if let Some(tl::Node::Tag(cell_tag)) = cell_handle.get(parser) {
                let mut cell_list_ctx = ListContext::None;
                walk_children(cell_tag, parser, &mut cell_buf, options, false, &mut cell_list_ctx);
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

/// Collapse runs of 3 or more consecutive newlines to exactly 2 in a single pass.
fn collapse_triple_newlines(buf: &mut String) {
    let bytes = buf.as_bytes();
    let mut result = String::with_capacity(buf.len());
    let mut newline_count = 0usize;
    for &b in bytes {
        if b == b'\n' {
            newline_count += 1;
            if newline_count <= 2 {
                result.push('\n');
            }
        } else {
            newline_count = 0;
            result.push(b as char);
        }
    }
    *buf = result;
}

/// Trim trailing whitespace from every line in a buffer without allocating per-line strings.
///
/// Uses a single allocation of the same capacity, writing each line's trimmed content
/// and inserting newline separators directly.
fn trim_line_ends(buf: &mut String) {
    let mut result = String::with_capacity(buf.len());
    for line in buf.lines() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(line.trim_end());
    }
    *buf = result;
}

/// Post-process: collapse 3+ newlines to 2, trim line-end whitespace, ensure single trailing newline.
fn post_process(buf: &mut String) {
    // Collapse runs of 3+ newlines to exactly 2
    collapse_triple_newlines(buf);

    // Trim trailing whitespace from each line in-place
    trim_line_ends(buf);

    // Trim to single trailing newline
    let keep = buf.trim_end_matches('\n').len();
    if keep == 0 {
        buf.clear();
    } else {
        buf.truncate(keep);
        buf.push('\n');
    }
}
