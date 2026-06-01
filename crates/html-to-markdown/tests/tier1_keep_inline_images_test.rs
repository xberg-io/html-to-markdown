//! Integration tests for Tier-1 `keep_inline_images_in` support.
//!
//! Verifies that Tier-1 handles `keep_inline_images_in` natively and produces
//! output byte-identical to Tier-2 for the same inputs.
//!
//! Tier-2 semantics (confirmed by reading converter.rs lines 3030–3057):
//!
//! - `keep_inline_images_in` empty → emit `![alt](src)` unconditionally
//!   (outside headings this is always the case; inside headings Tier-2 sets
//!   `convert_as_inline = true`, so images strip to alt-only unless overridden
//!   by `keep_inline_images_in`).
//! - `keep_inline_images_in` non-empty and `<img>` has a heading ancestor whose
//!   lowercased tag name is in the list → emit `![alt](src)`.
//! - `keep_inline_images_in` non-empty and `<img>` is in a heading whose tag is
//!   NOT in the list → emit alt-text only.
//! - `<img>` outside any heading: always emit `![alt](src)` regardless of
//!   `keep_inline_images_in` (Tier-2 only gates on `ctx.in_heading`).

#[cfg(feature = "inline-images")]
use html_to_markdown_rs::{ConversionOptions, convert};

// ── Helpers (basic, no testkit) ───────────────────────────────────────────────

/// Convert using `Auto` tier selection (exercising Tier-1 for simple inputs).
#[cfg(feature = "inline-images")]
fn auto(html: &str, keep: &[&str]) -> String {
    let opts = ConversionOptions {
        keep_inline_images_in: keep.iter().map(|s| s.to_string()).collect(),
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).expect("conversion must succeed").content.unwrap_or_default()
}

// ── 1. Default (keep_inline_images_in empty) — image preserved ────────────────

#[test]
#[cfg(feature = "inline-images")]
fn default_empty_list_preserves_image_in_paragraph() {
    // With an empty keep list, images should always be emitted as markdown.
    let html = "<p><img src=\"x.png\" alt=\"A\"></p>";
    let result = auto(html, &[]);
    assert!(
        result.contains("![A](x.png)"),
        "expected markdown image in output, got: {result:?}"
    );
}

// ── 2. keep_inline_images_in=["a"] — image inside anchor preserved ────────────

#[test]
#[cfg(feature = "inline-images")]
fn image_inside_matching_heading_ancestor_preserved() {
    // <img> inside <h1> with "h1" in the keep list → image kept as markdown.
    let html = "<h1><img src=\"x.png\" alt=\"A\"></h1>";
    let result = auto(html, &["h1"]);
    assert!(
        result.contains("![A](x.png)"),
        "expected markdown image in h1 output, got: {result:?}"
    );
}

// ── 3. keep_inline_images_in=["a"] — image in paragraph strips to alt ─────────

#[test]
#[cfg(feature = "inline-images")]
fn image_in_heading_without_match_strips_to_alt() {
    // <img> inside <h2> but keep list only has "h1" → alt-text only.
    let html = "<h2><img src=\"x.png\" alt=\"A\"></h2>";
    let result = auto(html, &["h1"]);
    assert!(
        !result.contains("!["),
        "expected no markdown image syntax, got: {result:?}"
    );
    assert!(result.contains('A'), "expected alt text in output, got: {result:?}");
}

// ── 4. keep_inline_images_in=["h1","h2"] — image in h1 preserved ──────────────

#[test]
#[cfg(feature = "inline-images")]
fn image_in_h1_preserved_with_h1_h2_keep_list() {
    let html = "<h1><img src=\"x.png\" alt=\"Logo\"></h1>";
    let result = auto(html, &["h1", "h2"]);
    assert!(
        result.contains("![Logo](x.png)"),
        "expected markdown image in h1 output, got: {result:?}"
    );
}

// ── 5. Deep nesting: <h1><span><strong><img> with keep=["h1"] ─────────────────

#[test]
#[cfg(feature = "inline-images")]
fn image_in_deeply_nested_heading_preserved() {
    // Even with intermediate inline elements, heading ancestor should be found.
    let html = "<h1><span><strong><img src=\"x.png\" alt=\"A\"></strong></span></h1>";
    let result = auto(html, &["h1"]);
    assert!(
        result.contains("![A](x.png)"),
        "expected markdown image in deeply-nested h1, got: {result:?}"
    );
}

// ── 6. Byte-equality with Tier-2 ─────────────────────────────────────────────

#[cfg(feature = "testkit")]
mod tier_parity {
    use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

    fn t1(html: &str, keep: &[&str]) -> String {
        let opts = ConversionOptions {
            tier_strategy: TierStrategy::ForceTier1,
            keep_inline_images_in: keep.iter().map(|s| s.to_string()).collect(),
            ..ConversionOptions::default()
        };
        convert(html, Some(opts)).expect("tier-1 conversion must succeed").content.unwrap_or_default()
    }

    fn t2(html: &str, keep: &[&str]) -> String {
        let opts = ConversionOptions {
            tier_strategy: TierStrategy::Tier2Only,
            keep_inline_images_in: keep.iter().map(|s| s.to_string()).collect(),
            ..ConversionOptions::default()
        };
        convert(html, Some(opts)).expect("tier-2 conversion must succeed").content.unwrap_or_default()
    }

    // 6a. Default (empty keep list) — paragraph image
    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_empty_keep_list_paragraph_image() {
        let html = "<p><img src=\"x.png\" alt=\"A\"></p>";
        assert_eq!(t1(html, &[]), t2(html, &[]), "empty keep list must be byte-identical");
    }

    // 6b. Image inside matching heading ancestor
    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_image_in_matching_heading() {
        let html = "<h1><img src=\"x.png\" alt=\"A\"></h1>";
        assert_eq!(
            t1(html, &["h1"]),
            t2(html, &["h1"]),
            "image in h1 with keep=[h1] must be byte-identical"
        );
    }

    // 6c. Image in paragraph (no heading ancestor) with non-empty keep list
    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_image_in_paragraph_with_keep_list() {
        let html = "<p><img src=\"x.png\" alt=\"A\"></p>";
        assert_eq!(
            t1(html, &["h1"]),
            t2(html, &["h1"]),
            "image in paragraph with keep=[h1] must be byte-identical"
        );
    }

    // 6d. Image in h1/h2 keep list
    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_image_in_h1_with_h1_h2_keep_list() {
        let html = "<h1><img src=\"x.png\" alt=\"Logo\"></h1>";
        assert_eq!(
            t1(html, &["h1", "h2"]),
            t2(html, &["h1", "h2"]),
            "image in h1 with keep=[h1,h2] must be byte-identical"
        );
    }

    // 6e. Deep nesting with heading ancestor
    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_deeply_nested_heading_image() {
        let html = "<h1><span><strong><img src=\"x.png\" alt=\"A\"></strong></span></h1>";
        assert_eq!(
            t1(html, &["h1"]),
            t2(html, &["h1"]),
            "deeply nested image in h1 must be byte-identical"
        );
    }

    // 6f. Image in non-matching heading strips to alt
    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_image_in_non_matching_heading_strips_to_alt() {
        let html = "<h2><img src=\"x.png\" alt=\"A\"></h2>";
        assert_eq!(
            t1(html, &["h1"]),
            t2(html, &["h1"]),
            "image in h2 with keep=[h1] must be byte-identical (alt-only)"
        );
    }
}
