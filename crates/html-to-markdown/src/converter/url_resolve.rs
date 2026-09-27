//! Resolution of relative `href`/`src` destinations against a caller-supplied
//! `base_url`, honoring a document's own `<base href>` the way a browser does.
//!
//! Computed exactly once per conversion (in `convert_api.rs`, before the
//! Tier-1/Tier-2 split) and threaded into both tiers as the same value, so
//! they cannot disagree on what a relative URL resolves to.

use std::rc::Rc;

use html5ever::tendril::StrTendril;
use html5ever::tokenizer::{BufferQueue, Tokenizer, TokenizerOpts};
use html5ever::tree_builder::{TreeBuilder, TreeBuilderOpts};
use html5ever::{LocalName, TokenizerResult, local_name, ns};
use url::Url;

use crate::rcdom::{Handle, NodeData, RcDom};

/// Compute the effective base URL for a conversion.
///
/// ~keep Precedence mirrors a browser's document-base algorithm: the first `<base>`
/// ~keep with an `href` in tree order is itself resolved against the caller's
/// ~keep `base_url` (so a page at `https://example.com/blog/` with
/// ~keep `<base href="/assets/">` gets an effective base of
/// ~keep `https://example.com/assets/`, and `<base href="https://cdn.example/">`
/// ~keep overrides the caller's base entirely, exactly as `Url::join` resolves an
/// ~keep already-absolute reference). When the `<base href>` is present but fails
/// ~keep to parse/join (malformed markup), it is ignored and the caller's
/// ~keep `base_url` alone is used rather than disabling resolution entirely.
///
/// Returns `None` when `caller_base_url` itself does not parse as an absolute
/// URL -- resolution is then a no-op everywhere, which keeps a malformed
/// `base_url` from ever panicking or corrupting output.
///
/// Reads the raw (pre-normalization) `html`; a `<base>` tag
/// that only becomes well-formed after this crate's UTF-16/NUL-byte input
/// normalization is not found, and the caller's `base_url` alone is used --
/// a safe fallback, not a correctness gap in the resolved output.
pub fn compute_effective_base(html: &str, caller_base_url: &str) -> Option<Url> {
    let caller_base = Url::parse(caller_base_url).ok()?;

    match document_base_href(html) {
        Some(href) if !href.is_empty() => Some(caller_base.join(&href).unwrap_or(caller_base)),
        // ~keep An empty `<base href="">` (or no `<base>` at all) leaves the document's own
        // ~keep URL as the base, per the HTML "document base URL" algorithm -- i.e. the
        // ~keep caller's `base_url`, unchanged.
        _ => Some(caller_base),
    }
}

/// Resolve a single `href`/`src` attribute value against `base`.
///
/// Returns `Some(resolved)` when `value` was a resolvable relative reference;
/// returns `None` when `value` should be left exactly as written -- an empty
/// value, an already-absolute URL (including non-hierarchical schemes such as
/// `mailto:`, `tel:`, `javascript:`, `data:`), or a reference that fails to
/// join against `base` (malformed input never panics and never produces a
/// corrupted destination; the caller keeps using the original text).
///
/// A bare fragment (`"#section"`) resolves against `base` -- the same
/// "resolve against the current document" rule a browser applies -- rather
/// than staying a same-page-only fragment, because `base` carries the full
/// crawled page URL: `https://example.com/blog/post#section` is exactly the
/// followable link the `base_url` option exists to produce, whereas a bare
/// `#section` left untouched is not followable outside the original page. ~keep
pub fn resolve_attribute_url(base: &Url, value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }

    // ~keep A value that already parses as a standalone (base-less) URL is either already
    // ~keep absolute (`https://…`) or a non-hierarchical/"cannot-be-a-base" scheme
    // ~keep (`mailto:`, `tel:`, `javascript:`, `data:`) -- both cases must pass through
    // ~keep unchanged, and `Url::parse` succeeding without a base is exactly the test for
    // ~keep "this text is already a complete URL reference".
    if Url::parse(value).is_ok() {
        return None;
    }

    base.join(value).ok().map(|joined| joined.to_string())
}

/// The `href` of the first HTML `<base>` with one, in tree order, as a browser picks it:
/// html5ever builds the document, so a `<base>` in a comment, in raw text, in `<template>`
/// contents or in SVG does not count, and foster parenting and `<frameset>` apply. ~keep
fn document_base_href(html: &str) -> Option<String> {
    // ~keep A `<base>` element only comes from a start tag named `base`, so a document
    // ~keep without `<base` (in any case) is not parsed.
    let bytes = html.as_bytes();
    let has_base_tag = memchr::memchr_iter(b'<', bytes).any(|at| {
        bytes
            .get(at + 1..at + 5)
            .is_some_and(|n| n.eq_ignore_ascii_case(b"base"))
    });
    if !has_base_tag {
        return None;
    }

    let tokenizer = Tokenizer::new(
        TreeBuilder::new(RcDom::default(), TreeBuilderOpts::default()),
        TokenizerOpts::default(),
    );
    let input = BufferQueue::default();
    let mut fed = 0;
    while fed < html.len() {
        // ~keep Fed in pieces cut after a `>`, an ASCII byte and so a char boundary.
        let until = memchr::memchr(b'>', &bytes[(fed + PARSE_PIECE).min(html.len())..])
            .map_or(html.len(), |at| fed + PARSE_PIECE + at + 1);
        input.push_back(StrTendril::from(&html[fed..until]));
        while !matches!(tokenizer.feed(&input), TokenizerResult::Done) {}
        fed = until;
        // ~keep Nothing is ever placed before a child of `<head>`, so a `<base href>` already
        // ~keep there is the first in tree order and the rest of the document is not parsed.
        if let Some(href) = head_base_href(&tokenizer.sink.sink.document) {
            return Some(href);
        }
    }
    tokenizer.end();
    first_base_href(&tokenizer.sink.sink.document)
}

/// Bytes of source fed to html5ever between checks of `<head>` for a `<base href>`.
const PARSE_PIECE: usize = 4096;

/// The `href` of a `<base>` element, or `None` for any other node or a `<base>` without one.
fn base_href(node: &Handle) -> Option<String> {
    let NodeData::Element { name, attrs, .. } = &node.data else {
        return None;
    };
    if name.ns != ns!(html) || name.local != local_name!("base") {
        return None;
    }
    attrs
        .borrow()
        .iter()
        .find(|attr| attr.name.local == local_name!("href"))
        .map(|attr| attr.value.to_string())
}

/// The `href` of the first `<base>` with one among the children of the document's `<head>`.
fn head_base_href(document: &Handle) -> Option<String> {
    let html = find_element(document, local_name!("html"))?;
    let head = find_element(&html, local_name!("head"))?;
    head.children.borrow().iter().find_map(base_href)
}

/// The first child of `parent` that is the HTML element `local`.
fn find_element(parent: &Handle, local: LocalName) -> Option<Handle> {
    parent
        .children
        .borrow()
        .iter()
        .find(|child| matches!(&child.data, NodeData::Element { name, .. } if name.ns == ns!(html) && name.local == local))
        .cloned()
}

/// The `href` of the first `<base>` with one below `root`, in tree order. `<template>`
/// contents are not children, so they are not searched.
fn first_base_href(root: &Handle) -> Option<String> {
    // ~keep The walk holds clones while the tree is alive: dropping the last handle to a
    // ~keep node empties its whole subtree (`rcdom::Node`'s drop).
    let mut pending = vec![Rc::clone(root)];
    while let Some(node) = pending.pop() {
        if let Some(href) = base_href(&node) {
            return Some(href);
        }
        pending.extend(node.children.borrow().iter().rev().cloned());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base(url: &str) -> Url {
        Url::parse(url).unwrap()
    }

    #[test]
    fn test_resolve_leaves_absolute_url_unchanged() {
        let result = resolve_attribute_url(&base("https://example.com/page"), "https://other.example/x");
        assert_eq!(result, None);
    }

    #[test]
    fn test_resolve_protocol_relative_uses_base_scheme() {
        let result = resolve_attribute_url(&base("https://example.com/page"), "//cdn.example.com/x.png");
        assert_eq!(result, Some("https://cdn.example.com/x.png".to_string()));
    }

    #[test]
    fn test_resolve_fragment_resolves_against_full_page_url() {
        let result = resolve_attribute_url(&base("https://example.com/blog/post.html"), "#section");
        assert_eq!(result, Some("https://example.com/blog/post.html#section".to_string()));
    }

    #[test]
    fn test_resolve_leaves_mailto_unchanged() {
        assert_eq!(
            resolve_attribute_url(&base("https://example.com/"), "mailto:foo@bar.com"),
            None
        );
    }

    #[test]
    fn test_resolve_leaves_tel_unchanged() {
        assert_eq!(resolve_attribute_url(&base("https://example.com/"), "tel:+12345"), None);
    }

    #[test]
    fn test_resolve_leaves_javascript_unchanged() {
        assert_eq!(
            resolve_attribute_url(&base("https://example.com/"), "javascript:alert(1)"),
            None
        );
    }

    #[test]
    fn test_resolve_leaves_data_uri_unchanged() {
        assert_eq!(
            resolve_attribute_url(&base("https://example.com/"), "data:image/png;base64,AAA"),
            None
        );
    }

    #[test]
    fn test_resolve_leaves_empty_href_unchanged() {
        assert_eq!(resolve_attribute_url(&base("https://example.com/"), ""), None);
    }

    #[test]
    fn test_resolve_leaves_malformed_url_unchanged() {
        assert_eq!(
            resolve_attribute_url(&base("https://example.com/"), "http://[not-a-valid-host"),
            None
        );
    }

    #[test]
    fn test_resolve_relative_path_joins_against_base_directory() {
        let result = resolve_attribute_url(&base("https://example.com/blog/index.html"), "post/child.html");
        assert_eq!(result, Some("https://example.com/blog/post/child.html".to_string()));
    }

    #[test]
    fn test_resolve_absolute_path_replaces_base_path() {
        let result = resolve_attribute_url(&base("https://example.com/blog/index.html"), "/about");
        assert_eq!(result, Some("https://example.com/about".to_string()));
    }

    #[test]
    fn test_compute_effective_base_no_document_base_uses_caller_base() {
        let effective = compute_effective_base("<html><body></body></html>", "https://example.com/blog/");
        assert_eq!(effective.unwrap().as_str(), "https://example.com/blog/");
    }

    #[test]
    fn test_compute_effective_base_relative_document_base_resolves_against_caller_base() {
        let html = r#"<html><head><base href="/assets/"></head><body></body></html>"#;
        let effective = compute_effective_base(html, "https://example.com/blog/post.html");
        assert_eq!(effective.unwrap().as_str(), "https://example.com/assets/");
    }

    #[test]
    fn test_compute_effective_base_absolute_document_base_overrides_caller_base() {
        let html = r#"<html><head><base href="https://cdn.example.com/"></head><body></body></html>"#;
        let effective = compute_effective_base(html, "https://example.com/blog/post.html");
        assert_eq!(effective.unwrap().as_str(), "https://cdn.example.com/");
    }

    #[test]
    fn test_compute_effective_base_empty_document_base_href_uses_caller_base() {
        let html = r#"<html><head><base href=""></head><body></body></html>"#;
        let effective = compute_effective_base(html, "https://example.com/blog/post.html");
        assert_eq!(effective.unwrap().as_str(), "https://example.com/blog/post.html");
    }

    #[test]
    fn test_compute_effective_base_malformed_caller_base_disables_resolution() {
        let effective = compute_effective_base("<html></html>", "not a url");
        assert_eq!(effective, None);
    }

    #[test]
    fn test_compute_effective_base_finds_base_href_outside_head_range() {
        // ~keep A `<base>` before any `<head>` still counts: the parser places it in the
        // ~keep head it opens.
        let html = r#"<base href="/x/"><body></body>"#;
        let effective = compute_effective_base(html, "https://example.com/");
        assert_eq!(effective.unwrap().as_str(), "https://example.com/x/");
    }

    #[test]
    fn test_document_base_href_decodes_entities() {
        let html = r#"<head><base href="/a&amp;b"></head>"#;
        assert_eq!(document_base_href(html), Some("/a&b".to_string()));
    }

    #[test]
    fn test_document_base_href_ignores_tag_named_basefoo() {
        let html = r#"<basefoo href="/wrong"></basefoo><base href="/right">"#;
        assert_eq!(document_base_href(html), Some("/right".to_string()));
    }

    const PAGE: &str = "https://example.com/blog/post.html";

    fn effective(html: &str) -> String {
        compute_effective_base(html, PAGE).unwrap().to_string()
    }

    #[test]
    fn test_compute_effective_base_ignores_a_base_inside_a_comment() {
        let html = r#"<head><!-- <base href="https://evil.example/"> --></head><body></body>"#;
        assert_eq!(effective(html), PAGE);
    }

    #[test]
    fn test_compute_effective_base_ignores_a_base_inside_raw_text() {
        for element in [
            "title", "textarea", "script", "style", "xmp", "iframe", "noembed", "noframes", "noscript",
        ] {
            let html = format!(
                r#"<head><{element}><base href="https://evil.example/"></{element}><base href="/real/"></head>"#
            );
            assert_eq!(effective(&html), "https://example.com/real/", "inside <{element}>");
        }
    }

    #[test]
    fn test_compute_effective_base_ignores_a_base_after_plaintext() {
        let html = r#"<body><plaintext><base href="https://evil.example/">"#;
        assert_eq!(effective(html), PAGE);
    }

    #[test]
    fn test_compute_effective_base_takes_a_base_in_the_body_when_it_is_the_first() {
        let html = r#"<head></head><body><p>text</p><base href="/in-body/"></body>"#;
        assert_eq!(effective(html), "https://example.com/in-body/");
    }

    #[test]
    fn test_compute_effective_base_takes_the_first_base_in_tree_order_not_source_order() {
        // ~keep The loose `<base>` is foster-parented in front of the table, so it precedes
        // ~keep the one inside the cell in tree order although it comes later in the source.
        let html = r#"<body><table><tr><td><base href="/in-cell/"></td></tr><base href="/loose/"></table>"#;
        assert_eq!(effective(html), "https://example.com/loose/");
    }

    #[test]
    fn test_compute_effective_base_keeps_tree_order_across_parse_pieces() {
        // ~keep The cell's `<base>` is parsed pieces before the loose one that precedes it.
        let padding = "x".repeat(3 * PARSE_PIECE);
        let html =
            format!(r#"<body><table><tr><td><base href="/in-cell/">{padding}</td></tr><base href="/loose/"></table>"#);
        assert_eq!(effective(&html), "https://example.com/loose/");
    }

    #[test]
    fn test_compute_effective_base_cuts_parse_pieces_on_char_boundaries() {
        let padding = "\u{e9}".repeat(3 * PARSE_PIECE);
        // ~keep The odd-length prefix puts a byte offset of `PARSE_PIECE` inside a two-byte `é`.
        let html = format!(r#"<body><p title="x{padding}">{padding}</p><base href="/late/"></body>"#);
        assert_eq!(effective(&html), "https://example.com/late/");
    }

    #[test]
    fn test_compute_effective_base_ignores_a_base_in_a_body_a_frameset_replaces() {
        // ~keep The body must be implied: an explicit `<body>` tag stops a frameset replacing it.
        let html = r#"<head></head><div><base href="https://evil.example/"></div><frameset></frameset>"#;
        assert_eq!(effective(html), PAGE);
    }

    #[test]
    fn test_compute_effective_base_ignores_a_base_inside_a_template() {
        let html = r#"<head><template><base href="https://evil.example/"></template></head>"#;
        assert_eq!(effective(html), PAGE);
    }

    #[test]
    fn test_compute_effective_base_ignores_a_base_inside_svg() {
        let html = r#"<body><svg><base href="https://evil.example/"></base></svg></body>"#;
        assert_eq!(effective(html), PAGE);
    }

    #[test]
    fn test_compute_effective_base_skips_a_first_base_without_href() {
        let html = r#"<head><base target="_blank"><base href="/second/"></head>"#;
        assert_eq!(effective(html), "https://example.com/second/");
    }

    #[test]
    fn test_compute_effective_base_first_base_with_empty_href_wins_over_a_later_one() {
        let html = r#"<head><base href=""><base href="https://evil.example/"></head>"#;
        assert_eq!(effective(html), PAGE);
    }

    #[test]
    fn test_compute_effective_base_reads_an_upper_case_base_tag() {
        let html = r#"<HEAD><BASE HREF="/upper/"></HEAD>"#;
        assert_eq!(effective(html), "https://example.com/upper/");
    }

    #[test]
    fn test_compute_effective_base_reads_a_tag_name_ended_by_a_form_feed() {
        let html = "<head><base\x0chref=\"/ff/\"></head>";
        assert_eq!(effective(html), "https://example.com/ff/");
    }
}
