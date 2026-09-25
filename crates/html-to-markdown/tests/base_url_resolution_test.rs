//! Coverage for `ConversionOptions::base_url`, which resolves relative `href`/`src`
//! destinations against a caller-supplied base URL.
//!
//! The Tier-1 (byte scanner) and Tier-2 (DOM walk) conversion paths implement URL
//! resolution independently (`converter/tier1/scanner.rs` and the `converter/handlers/*`
//! / `converter/inline/link.rs` / `converter/media/embedded.rs` call sites), both
//! reading the SAME pre-computed `Option<Rc<url::Url>>` that `convert_api.rs` builds once
//! per conversion (see `converter::url_resolve::compute_effective_base`). `assert_tier1_matches_tier2`
//! below is the parity check that would catch a resolve-only-one-tier regression: temporarily
//! reverting either of `converter/tier1/scanner.rs`'s two `state.resolve_url(...)` call sites
//! and rerunning this file fails 7 of these 13 tests with an explicit tier1-vs-tier2 diff
//! (verified manually; a "make it fail on purpose" run cannot itself be committed as a
//! passing test).

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn convert_with(html: &str, base_url: Option<&str>, tier_strategy: TierStrategy) -> String {
    let options = ConversionOptions {
        base_url: base_url.map(str::to_string),
        tier_strategy,
        // ~keep Metadata/frontmatter extraction is unrelated to URL resolution and has its
        // ~keep own pre-existing Tier-1/Tier-2 blank-line divergence when a `<base>` tag is
        // ~keep present (reproduced independently of `base_url`, on unmodified `main`) --
        // ~keep disabled here so these tests isolate `base_url` resolution specifically.
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

/// Runs `html` through both tiers (forcing each explicitly via the testkit-only
/// `TierStrategy::Tier1`/`Tier2`, not the auto-routing default) with the same
/// `base_url`, and asserts the two tiers produced byte-identical output.
fn assert_tier1_matches_tier2(html: &str, base_url: &str) -> String {
    let tier1_output = convert_with(html, Some(base_url), TierStrategy::Tier1);
    let tier2_output = convert_with(html, Some(base_url), TierStrategy::Tier2);
    assert_eq!(
        tier1_output, tier2_output,
        "tier1 diverged from tier2 for base_url resolution\ninput: {html:?}\nbase_url: {base_url:?}\ntier1: {tier1_output:?}\ntier2: {tier2_output:?}"
    );
    tier1_output
}

#[test]
fn should_leave_output_byte_identical_when_base_url_is_unset() {
    let html = r#"<a href="rel/child.html">child</a> <img src="pic.png" alt="a">"#;
    let without_base = convert_with(html, None, TierStrategy::Auto);
    assert_eq!(without_base, "[child](rel/child.html) ![a](pic.png)\n");
}

#[test]
fn should_resolve_relative_link_against_base_url_on_both_tiers() {
    let html = r#"<a href="rel/child.html">child</a>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/blog/index.html");
    assert_eq!(out, "[child](https://example.com/blog/rel/child.html)\n");
}

#[test]
fn should_resolve_absolute_path_link_against_base_url_on_both_tiers() {
    let html = r#"<a href="/about">about</a>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/blog/index.html");
    assert_eq!(out, "[about](https://example.com/about)\n");
}

#[test]
fn should_resolve_relative_image_src_against_base_url_on_both_tiers() {
    let html = r#"<img src="images/pic.png" alt="a photo">"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/blog/index.html");
    assert_eq!(out, "![a photo](https://example.com/blog/images/pic.png)\n");
}

#[test]
fn should_resolve_protocol_relative_url_against_base_scheme_on_both_tiers() {
    let html = r#"<a href="//cdn.example.com/x">x</a>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/");
    assert_eq!(out, "[x](https://cdn.example.com/x)\n");
}

#[test]
fn should_resolve_fragment_only_href_against_full_page_url_on_both_tiers() {
    let html = r##"<a href="#section">jump</a>"##;
    let out = assert_tier1_matches_tier2(html, "https://example.com/blog/post.html");
    assert_eq!(out, "[jump](https://example.com/blog/post.html#section)\n");
}

#[test]
fn should_leave_already_absolute_url_unchanged_on_both_tiers() {
    let html = r#"<a href="https://other.example/x">x</a>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/");
    assert_eq!(out, "[x](https://other.example/x)\n");
}

#[test]
fn should_leave_mailto_href_unchanged_on_both_tiers() {
    let html = r#"<a href="mailto:foo@bar.com">mail</a>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/");
    assert_eq!(out, "[mail](mailto:foo@bar.com)\n");
}

#[test]
fn should_leave_empty_href_unchanged_on_both_tiers() {
    let html = r#"<a href="">empty</a>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/");
    assert_eq!(out, "[empty](<>)\n");
}

#[test]
fn should_leave_malformed_relative_reference_unchanged_without_panicking_on_both_tiers() {
    let html = r#"<a href="http://[not-a-valid-host">bad</a>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/");
    assert_eq!(out, "[bad](http://[not-a-valid-host)\n");
}

#[test]
fn should_leave_output_unchanged_when_base_url_itself_is_malformed() {
    let html = r#"<a href="rel/child.html">child</a>"#;
    let out = assert_tier1_matches_tier2(html, "not a url");
    assert_eq!(out, "[child](rel/child.html)\n");
}

#[test]
fn should_resolve_document_base_href_relative_to_caller_base_url_on_both_tiers() {
    let html = r#"<html><head><base href="/assets/"></head><body><a href="child.html">c</a></body></html>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/blog/post.html");
    assert_eq!(out, "[c](https://example.com/assets/child.html)\n");
}

#[test]
fn should_let_absolute_document_base_href_override_caller_base_url_on_both_tiers() {
    let html = r#"<html><head><base href="https://cdn.example.com/"></head><body><a href="x.png">x</a></body></html>"#;
    let out = assert_tier1_matches_tier2(html, "https://example.com/blog/post.html");
    assert_eq!(out, "[x](https://cdn.example.com/x.png)\n");
}
