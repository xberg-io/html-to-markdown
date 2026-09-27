//! Resolution of relative `href`/`src` destinations against a caller-supplied
//! `base_url`, honoring a document's own `<base href>` the way a browser does.
//!
//! Computed exactly once per conversion (in `convert_api.rs`, before the
//! Tier-1/Tier-2 split) and threaded into both tiers as the same value, so
//! they cannot disagree on what a relative URL resolves to.

use std::cell::Cell;
use std::rc::Rc;

use html5ever::tendril::StrTendril;
use html5ever::tokenizer::{
    BufferQueue, StartTag, TagToken, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};
use html5ever::tree_builder::{TreeBuilder, TreeBuilderOpts};
use html5ever::{TokenizerResult, local_name, ns};
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
/// ~keep to parse/join (malformed markup), or resolves to a `data:` or `javascript:`
/// ~keep URL, it is ignored and the caller's `base_url` alone is used, as the HTML
/// ~keep "frozen base URL" steps fall back to the document's own URL.
///
/// Returns `None` when `caller_base_url` itself does not parse as an absolute
/// URL -- resolution is then a no-op everywhere, which keeps a malformed
/// `base_url` from ever panicking or corrupting output.
///
/// `document_base_href` is [`document_base_href`] of the conversion's input.
pub fn compute_effective_base(document_base_href: Option<&str>, caller_base_url: &str) -> Option<Url> {
    let caller_base = Url::parse(caller_base_url).ok()?;

    match document_base_href {
        Some(href) if !href.is_empty() => Some(
            caller_base
                .join(href)
                .ok()
                .filter(|base| !matches!(base.scheme(), "data" | "javascript"))
                .unwrap_or(caller_base),
        ),
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
///
/// Reads the raw (pre-normalization) `html`; a `<base>` tag that only becomes well-formed
/// after this crate's UTF-16/NUL-byte input normalization is not found. Both the effective
/// base and the `base` metadata come from this one reading.
pub fn document_base_href(html: &str) -> Option<String> {
    let parse = parse_base_href(html);
    tracing::debug!(
        target: "html_to_markdown::convert",
        read = parse.read,
        walks = parse.walks,
        "document base href parsed"
    );
    parse.href
}

/// [`document_base_href`], with what finding it cost.
struct BaseParse {
    href: Option<String>,
    /// Bytes of `html` fed to the parser.
    read: usize,
    /// Walks of the partial tree made while parsing.
    walks: usize,
}

fn parse_base_href(html: &str) -> BaseParse {
    // ~keep A `<base>` element only comes from a start tag named `base`, so a document
    // ~keep without `<base` (in any case) is not parsed.
    let bytes = html.as_bytes();
    if !has_start_tag(bytes, b"base") {
        return BaseParse {
            href: None,
            read: 0,
            walks: 0,
        };
    }
    let mut has_frameset = None;

    let tokenizer = Tokenizer::new(
        BaseTagWatch {
            tree_builder: TreeBuilder::new(RcDom::default(), TreeBuilderOpts::default()),
            saw_base: Cell::new(false),
        },
        TokenizerOpts::default(),
    );
    let input = BufferQueue::default();
    let mut fed = 0;
    let mut walks = 0;
    while fed < html.len() {
        // ~keep Fed in pieces cut after a `>`, an ASCII byte and so a char boundary.
        let until = memchr::memchr(b'>', &bytes[(fed + PARSE_PIECE).min(html.len())..])
            .map_or(html.len(), |at| fed + PARSE_PIECE + at + 1);
        input.push_back(StrTendril::from(&html[fed..until]));
        while !matches!(tokenizer.feed(&input), TokenizerResult::Done) {}
        fed = until;
        // ~keep Only a `base` start tag the tokenizer emitted can add a `<base>`, so the tree is
        // ~keep walked after that piece, whatever piece holds the bytes of the tag.
        if !tokenizer.sink.saw_base.replace(false) {
            continue;
        }
        // ~keep Later nodes go in after every node already in the tree, except a node foster
        // ~keep parented in front of an open table and a `<frameset>` replacing the body. So a
        // ~keep first `<base href>` outside those two reaches is final, and the rest is not parsed.
        walks += 1;
        if let Some(base) = first_base(&tokenizer.sink.tree_builder.sink.document) {
            let replaceable = base.in_body && *has_frameset.get_or_insert_with(|| has_start_tag(bytes, b"frameset"));
            if !(base.in_table || replaceable) {
                return BaseParse {
                    href: Some(base.href),
                    read: fed,
                    walks,
                };
            }
        }
    }
    tokenizer.end();
    BaseParse {
        href: first_base(&tokenizer.sink.tree_builder.sink.document).map(|base| base.href),
        read: fed,
        walks,
    }
}

/// A token sink that passes every token to the tree builder and notes a `base` start tag.
/// The tree builder's `end` only pops open elements, which `RcDom` ignores, so it is not passed.
struct BaseTagWatch {
    tree_builder: TreeBuilder<Handle, RcDom>,
    /// A `base` start tag reached the tree builder since the flag was last cleared.
    saw_base: Cell<bool>,
}

impl TokenSink for BaseTagWatch {
    type Handle = Handle;

    fn process_token(&self, token: Token, line_number: u64) -> TokenSinkResult<Handle> {
        if let TagToken(tag) = &token {
            if tag.kind == StartTag && tag.name == local_name!("base") {
                self.saw_base.set(true);
            }
        }
        self.tree_builder.process_token(token, line_number)
    }

    fn adjusted_current_node_present_but_not_in_html_namespace(&self) -> bool {
        self.tree_builder
            .adjusted_current_node_present_but_not_in_html_namespace()
    }
}

/// Bytes of source fed to html5ever between looks for a final `<base href>`.
const PARSE_PIECE: usize = 4096;

/// Whether `bytes` holds `<` followed by `name` in any case.
fn has_start_tag(bytes: &[u8], name: &[u8]) -> bool {
    memchr::memchr_iter(b'<', bytes).any(|at| {
        bytes
            .get(at + 1..at + 1 + name.len())
            .is_some_and(|n| n.eq_ignore_ascii_case(name))
    })
}

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

/// The first `<base>` with an `href` in a tree, and where it sits.
struct FirstBase {
    href: String,
    /// Below a `<table>`, where foster parenting can later insert a node in front of it.
    in_table: bool,
    /// Below `<body>`, which a later `<frameset>` can replace.
    in_body: bool,
}

/// The first `<base>` with an `href` below `root`, in tree order. `<template>` contents are
/// not children, so they are not searched.
fn first_base(root: &Handle) -> Option<FirstBase> {
    // ~keep The walk holds clones while the tree is alive: dropping the last handle to a
    // ~keep node empties its whole subtree (`rcdom::Node`'s drop).
    let mut pending = vec![(Rc::clone(root), false, false)];
    while let Some((node, in_table, in_body)) = pending.pop() {
        if let Some(href) = base_href(&node) {
            return Some(FirstBase {
                href,
                in_table,
                in_body,
            });
        }
        let (in_table, in_body) = match &node.data {
            NodeData::Element { name, .. } if name.ns == ns!(html) => (
                in_table || name.local == local_name!("table"),
                in_body || name.local == local_name!("body"),
            ),
            _ => (in_table, in_body),
        };
        pending.extend(
            node.children
                .borrow()
                .iter()
                .rev()
                .map(|child| (Rc::clone(child), in_table, in_body)),
        );
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
        let effective = compute_effective_base(
            document_base_href("<html><body></body></html>").as_deref(),
            "https://example.com/blog/",
        );
        assert_eq!(effective.unwrap().as_str(), "https://example.com/blog/");
    }

    #[test]
    fn test_compute_effective_base_relative_document_base_resolves_against_caller_base() {
        let html = r#"<html><head><base href="/assets/"></head><body></body></html>"#;
        let effective = compute_effective_base(
            document_base_href(html).as_deref(),
            "https://example.com/blog/post.html",
        );
        assert_eq!(effective.unwrap().as_str(), "https://example.com/assets/");
    }

    #[test]
    fn test_compute_effective_base_absolute_document_base_overrides_caller_base() {
        let html = r#"<html><head><base href="https://cdn.example.com/"></head><body></body></html>"#;
        let effective = compute_effective_base(
            document_base_href(html).as_deref(),
            "https://example.com/blog/post.html",
        );
        assert_eq!(effective.unwrap().as_str(), "https://cdn.example.com/");
    }

    #[test]
    fn test_compute_effective_base_empty_document_base_href_uses_caller_base() {
        let html = r#"<html><head><base href=""></head><body></body></html>"#;
        let effective = compute_effective_base(
            document_base_href(html).as_deref(),
            "https://example.com/blog/post.html",
        );
        assert_eq!(effective.unwrap().as_str(), "https://example.com/blog/post.html");
    }

    #[test]
    fn test_compute_effective_base_malformed_caller_base_disables_resolution() {
        let effective = compute_effective_base(document_base_href("<html></html>").as_deref(), "not a url");
        assert_eq!(effective, None);
    }

    #[test]
    fn test_compute_effective_base_finds_base_href_outside_head_range() {
        // ~keep A `<base>` before any `<head>` still counts: the parser places it in the
        // ~keep head it opens.
        let html = r#"<base href="/x/"><body></body>"#;
        let effective = compute_effective_base(document_base_href(html).as_deref(), "https://example.com/");
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
        compute_effective_base(document_base_href(html).as_deref(), PAGE)
            .unwrap()
            .to_string()
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
    fn test_compute_effective_base_ignores_a_base_inside_svg_cdata() {
        // ~keep In SVG, `<![CDATA[` runs to `]]>`; read as a bogus comment it would end at the
        // ~keep first `>` and let `<p>` break out of SVG ahead of the `<base>`.
        assert_eq!(
            effective(r#"<body><svg><![CDATA[x><p><base href="https://evil.example/">]]></svg>"#),
            PAGE
        );
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

    #[test]
    fn test_compute_effective_base_falls_back_to_the_page_for_a_data_or_javascript_base() {
        for href in ["data:text/html,x", "javascript:void(0)"] {
            let html = format!(r#"<head><base href="{href}"><base href="/second/"></head>"#);
            assert_eq!(effective(&html), PAGE, "{href}");
        }
        let html = r#"<head><base href="https://cdn.example/"></head>"#;
        assert_eq!(effective(html), "https://cdn.example/");
    }

    #[test]
    fn test_parse_base_href_stops_at_a_first_base_in_the_body() {
        let html = format!(
            r#"<head></head><body><p>text</p><base href="/in-body/">{}"#,
            "<p>more</p>".repeat(PARSE_PIECE)
        );
        let parse = parse_base_href(&html);
        assert_eq!(parse.href.as_deref(), Some("/in-body/"));
        assert!(
            parse.read <= 2 * PARSE_PIECE,
            "read {} of {} bytes",
            parse.read,
            html.len()
        );
    }

    #[test]
    fn test_parse_base_href_stops_at_a_first_base_in_the_head() {
        let html = format!(
            r#"<head><base href="/in-head/"></head><body>{}"#,
            "<p>more</p>".repeat(PARSE_PIECE)
        );
        let parse = parse_base_href(&html);
        assert_eq!(parse.href.as_deref(), Some("/in-head/"));
        assert!(
            parse.read <= 2 * PARSE_PIECE,
            "read {} of {} bytes",
            parse.read,
            html.len()
        );
    }

    #[test]
    fn test_parse_base_href_stops_at_a_first_base_with_a_gt_in_a_quoted_value() {
        // ~keep The first piece ends at the `>` inside the quoted value, mid tag.
        let href = format!("https://a.example/?q={}>", "x".repeat(PARSE_PIECE));
        let html = format!(
            r#"<head><base href="{href}"></head><body>{}"#,
            "<p>more</p>".repeat(PARSE_PIECE)
        );
        let parse = parse_base_href(&html);
        assert_eq!(parse.href.as_deref(), Some(href.as_str()));
        assert!(
            parse.read <= 3 * PARSE_PIECE,
            "read {} of {} bytes",
            parse.read,
            html.len()
        );
    }

    #[test]
    fn test_parse_base_href_does_not_parse_a_page_without_base_text() {
        assert_eq!(parse_base_href("<html><body><p>x</p></body></html>").read, 0);
    }

    #[test]
    fn test_parse_base_href_walks_the_tree_only_after_a_base_start_tag() {
        // ~keep The comment holds `<base` text but no tag, `</base>` is an end tag, and the
        // ~keep template's `<base>` is a start tag.
        let padding = "<p>more</p>".repeat(PARSE_PIECE);
        let html = format!(
            r#"<!-- <base href="/x/"> --><body>{padding}</base>{padding}<template><base href="/t/"></template>{padding}"#
        );
        let parse = parse_base_href(&html);
        assert_eq!((parse.href, parse.read), (None, html.len()));
        assert_eq!(parse.walks, 1);
    }

    #[test]
    fn test_compute_effective_base_ignores_a_body_base_a_frameset_pieces_later_replaces() {
        // ~keep `<div>` keeps the frameset allowed, where text would not.
        let padding = "<div></div>".repeat(PARSE_PIECE);
        let html =
            format!(r#"<head></head><div><base href="https://evil.example/"></div>{padding}<frameset></frameset>"#);
        assert_eq!(effective(&html), PAGE);
    }
}
