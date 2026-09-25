//! Resolution of relative `href`/`src` destinations against a caller-supplied
//! `base_url`, honoring a document's own `<base href>` the way a browser does.
//!
//! Computed exactly once per conversion (in `convert_api.rs`, before the
//! Tier-1/Tier-2 split) and threaded into both tiers as the same value, so
//! they cannot disagree on what a relative URL resolves to.

use url::Url;

/// Compute the effective base URL for a conversion.
///
/// ~keep Precedence mirrors a browser's document-base algorithm: a `<base href>`
/// ~keep found anywhere in `html` is itself resolved against the caller's
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
/// Scans the raw (pre-normalization) `html` for simplicity; a `<base>` tag
/// that only becomes well-formed after this crate's UTF-16/NUL-byte input
/// normalization is not found, and the caller's `base_url` alone is used --
/// a safe fallback, not a correctness gap in the resolved output.
pub fn compute_effective_base(html: &str, caller_base_url: &str) -> Option<Url> {
    let caller_base = Url::parse(caller_base_url).ok()?;

    match scan_document_base_href(html) {
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

/// Scan raw `html` for the first `<base href="…">` in document order,
/// independent of `extract_metadata` -- `base_url` resolution must work even
/// when frontmatter/metadata extraction is disabled. ~keep
fn scan_document_base_href(html: &str) -> Option<String> {
    let bytes = html.as_bytes();
    let mut idx = 0usize;

    while let Some(offset) = memchr::memchr(b'<', &bytes[idx..]) {
        let tag_start = idx + offset;
        let name_start = tag_start + 1;

        if !super::prescan::matches_tag_start(bytes, name_start, b"base") {
            idx = tag_start + 1;
            continue;
        }

        let tag_end = super::prescan::find_tag_end(bytes, name_start + b"base".len())?;

        if let Some(href) = extract_href_attr(&html[tag_start..tag_end]) {
            return Some(href);
        }

        idx = tag_end;
    }

    None
}

/// Extract the (entity-decoded) value of an `href` attribute from a single
/// tag's source text (e.g. `<base href="/x">`).
fn extract_href_attr(tag_text: &str) -> Option<String> {
    let bytes = tag_text.as_bytes();
    let lower = tag_text.to_ascii_lowercase();
    let mut search_from = 0usize;

    while let Some(rel) = lower[search_from..].find("href") {
        let pos = search_from + rel;
        search_from = pos + "href".len();

        let preceded_by_boundary = pos == 0 || bytes[pos - 1].is_ascii_whitespace();
        if !preceded_by_boundary {
            continue;
        }

        let mut after = pos + "href".len();
        while after < bytes.len() && bytes[after].is_ascii_whitespace() {
            after += 1;
        }
        if bytes.get(after) != Some(&b'=') {
            continue;
        }
        after += 1;
        while after < bytes.len() && bytes[after].is_ascii_whitespace() {
            after += 1;
        }

        let value = match bytes.get(after) {
            Some(b'"') => {
                let start = after + 1;
                let end = start + tag_text[start..].find('"')?;
                &tag_text[start..end]
            }
            Some(b'\'') => {
                let start = after + 1;
                let end = start + tag_text[start..].find('\'')?;
                &tag_text[start..end]
            }
            Some(_) => {
                let start = after;
                let end = tag_text[start..]
                    .find(|c: char| c.is_whitespace() || c == '>' || c == '/')
                    .map_or(tag_text.len(), |i| start + i);
                &tag_text[start..end]
            }
            None => return None,
        };

        return Some(html_escape::decode_html_entities(value).into_owned());
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
        // ~keep The scanner looks for the first `<base>` anywhere in the document,
        // ~keep matching a lenient real-world tolerance rather than strictly requiring
        // ~keep well-formed `<head>` nesting.
        let html = r#"<base href="/x/"><body></body>"#;
        let effective = compute_effective_base(html, "https://example.com/");
        assert_eq!(effective.unwrap().as_str(), "https://example.com/x/");
    }

    #[test]
    fn test_scan_document_base_href_decodes_entities() {
        let html = r#"<head><base href="/a&amp;b"></head>"#;
        assert_eq!(scan_document_base_href(html), Some("/a&b".to_string()));
    }

    #[test]
    fn test_scan_document_base_href_ignores_tag_named_basefoo() {
        let html = r#"<basefoo href="/wrong"></basefoo><base href="/right">"#;
        assert_eq!(scan_document_base_href(html), Some("/right".to_string()));
    }
}
