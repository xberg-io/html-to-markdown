//! Integration tests for Tier-1 `keep_inline_images_in` support.
//!
//! Verifies that Tier-1 handles `keep_inline_images_in` natively and produces
//! output byte-identical to Tier-2 for the same inputs.
//!
//! Tier-2 semantics (confirmed by reading `handlers/image.rs`'s `keep_as_markdown` /
//! `should_use_alt_text`, and `handlers/link.rs`'s `link_allow_inline_images`, issue #492):
//!
//! - `keep_inline_images_in` empty → emit `![alt](src)` unconditionally
//!   (outside headings this is always the case; inside headings Tier-2 sets
//!   `convert_as_inline = true`, so images strip to alt-only unless overridden
//!   by `keep_inline_images_in`).
//! - `keep_inline_images_in` non-empty and `<img>` has a heading ancestor whose
//!   lowercased tag name is in the list → emit `![alt](src)`.
//! - `keep_inline_images_in` non-empty and `<img>` has an `<a>` ancestor whose
//!   lowercased tag name ("a") is in the list → emit `![alt](src)`, REGARDLESS of
//!   any heading further out, matching or not (issue #492: heading is no longer
//!   the only ancestor rule -- a matching link overrides a non-matching heading).
//! - `keep_inline_images_in` non-empty, no matching link ancestor, and `<img>` is
//!   in a heading whose tag is NOT in the list → emit alt-text only.
//! - `<img>` outside any heading and outside any matching-tag link: always emit
//!   `![alt](src)` regardless of `keep_inline_images_in` (Tier-2 only gates on
//!   `ctx.in_heading` / `ctx.link_allow_inline_images`).

#[cfg(feature = "inline-images")]
use html_to_markdown_rs::{ConversionOptions, convert};

/// Convert using `Auto` tier selection (exercising Tier-1 for simple inputs).
#[cfg(feature = "inline-images")]
fn auto(html: &str, keep: &[&str]) -> String {
    let opts = ConversionOptions {
        keep_inline_images_in: keep.iter().map(ToString::to_string).collect(),
        ..ConversionOptions::default()
    };
    convert(html, Some(opts))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

#[test]
#[cfg(feature = "inline-images")]
fn default_empty_list_preserves_image_in_paragraph() {
    let html = "<p><img src=\"x.png\" alt=\"A\"></p>";
    let result = auto(html, &[]);
    assert!(
        result.contains("![A](x.png)"),
        "expected markdown image in output, got: {result:?}"
    );
}

#[test]
#[cfg(feature = "inline-images")]
fn image_inside_matching_heading_ancestor_preserved() {
    let html = "<h1><img src=\"x.png\" alt=\"A\"></h1>";
    let result = auto(html, &["h1"]);
    assert!(
        result.contains("![A](x.png)"),
        "expected markdown image in h1 output, got: {result:?}"
    );
}

#[test]
#[cfg(feature = "inline-images")]
fn image_in_heading_without_match_strips_to_alt() {
    let html = "<h2><img src=\"x.png\" alt=\"A\"></h2>";
    let result = auto(html, &["h1"]);
    assert!(
        !result.contains("!["),
        "expected no markdown image syntax, got: {result:?}"
    );
    assert!(result.contains('A'), "expected alt text in output, got: {result:?}");
}

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

#[test]
#[cfg(feature = "inline-images")]
fn image_in_deeply_nested_heading_preserved() {
    let html = "<h1><span><strong><img src=\"x.png\" alt=\"A\"></strong></span></h1>";
    let result = auto(html, &["h1"]);
    assert!(
        result.contains("![A](x.png)"),
        "expected markdown image in deeply-nested h1, got: {result:?}"
    );
}

// ~keep ── 6. Byte-equality with Tier-2 ─────────────────────────────────────────────

// ~keep Every test in this module additionally requires `inline-images`, so the
// ~keep module must carry that gate too — with only `testkit` enabled the `t1`/`t2`
// ~keep helpers would compile with no callers and trip `-D dead-code`.
#[cfg(all(feature = "testkit", feature = "inline-images"))]
mod tier_parity {
    use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

    fn t1(html: &str, keep: &[&str]) -> String {
        let opts = ConversionOptions {
            tier_strategy: TierStrategy::Tier1,
            keep_inline_images_in: keep.iter().map(ToString::to_string).collect(),
            ..ConversionOptions::default()
        };
        convert(html, Some(opts))
            .expect("tier-1 conversion must succeed")
            .content
            .unwrap_or_default()
    }

    fn t2(html: &str, keep: &[&str]) -> String {
        let opts = ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            keep_inline_images_in: keep.iter().map(ToString::to_string).collect(),
            ..ConversionOptions::default()
        };
        convert(html, Some(opts))
            .expect("tier-2 conversion must succeed")
            .content
            .unwrap_or_default()
    }

    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_empty_keep_list_paragraph_image() {
        let html = "<p><img src=\"x.png\" alt=\"A\"></p>";
        assert_eq!(t1(html, &[]), t2(html, &[]), "empty keep list must be byte-identical");
    }

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

    // ~keep ── Issue #492: <a> ancestor in keep_inline_images_in ────────────────────────

    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_image_in_link_with_a_in_keep_list() {
        let html = "<a href=\"x\"><img src=\"i.png\" alt=\"A\"></a>";
        assert_eq!(
            t1(html, &["a"]),
            t2(html, &["a"]),
            "image in a link with keep=[a] must be byte-identical"
        );
    }

    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_image_in_link_with_a_absent_from_keep_list() {
        // ~keep Baseline: no heading, no matching link -- both tiers fall through to the
        // ~keep unconditional "no restriction" default.
        let html = "<a href=\"x\"><img src=\"i.png\" alt=\"A\"></a>";
        assert_eq!(
            t1(html, &["h1"]),
            t2(html, &["h1"]),
            "image in a link with keep=[h1] (a absent) must be byte-identical"
        );
    }

    #[test]
    #[cfg(feature = "inline-images")]
    fn parity_image_in_link_inside_non_matching_heading_with_a_in_keep_list() {
        // ~keep THE divergence guard: this is the one shape #492's Tier-1 fix exists for.
        // ~keep `<h1>` opens OUTSIDE the `<a>`, so Tier-1's block-tag-inside-a-link bail
        // ~keep (scanner.rs:1111-1128) does not fire here -- unlike `<a><h1>...`, which
        // ~keep would bail to Tier-2 before `keep_inline_image_for_ancestors` ever runs.
        // ~keep Before the `TagKind::Link` arm, Tier-1 stopped at the first `Heading` frame
        // ~keep and never saw the closer, matching `Link` frame at all, so "h1" not being in
        // ~keep the keep list would have made Tier-1 strip to alt-only while Tier-2 (via
        // ~keep `link_allow_inline_images`) kept the markdown image -- a real divergence this
        // ~keep test exists to catch.
        let html = "<h1><a href=\"x\"><img src=\"i.png\" alt=\"A\"></a></h1>";
        assert_eq!(
            t1(html, &["a"]),
            t2(html, &["a"]),
            "image in a link inside a non-matching h1, with keep=[a], must be byte-identical"
        );
    }
}
