//! Tier-1 alt/title entity handling: the two tiers must agree byte for byte.
//!
//! Both tiers now fully decode attribute character references (issue #494), so the answer no
//! longer depends on whether html5ever repaired the document: `&#x22;`, `&quot;` and a literal
//! `"` all converge on `"`, which is then escaped for its Markdown context. The custom-element
//! cases below stay valuable precisely because they force the repair path -- they are what
//! proves the two routes still converge.
//!
//! These assertions only ever compared Tier 1 against Tier 2, so they survived that change
//! unaltered; only the names and this note, which described the old half-decoding contract,
//! needed correcting. `with_custom_element_title_escaped` is the one that caught Tier-1
//! emitting an unescaped `""t""` once decoding made a raw quote reachable.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn assert_matches(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

// ~keep ── No repair path: the document parses cleanly. ────────────────────────────

#[test]
fn plain_named_quote_matches_tier2() {
    assert_matches(r#"<p><img src="/x.png" alt="hello &quot;world&quot;"></p>"#);
}

#[test]
fn plain_amp_matches_tier2() {
    assert_matches(r#"<p><img src="/x.png" alt="A &amp; B"></p>"#);
}

#[test]
fn plain_hex_entity_matches_tier2() {
    assert_matches(r#"<p><img src="/x.png" alt="hello &#x22;w&#x22;"></p>"#);
}

// ~keep ── Repair path: a custom element sends T2 through the html5ever roundtrip. ──

#[test]
fn with_custom_element_hex_entity_matches_tier2() {
    assert_matches(r#"<my-component>x</my-component><p><img src="/x.png" alt="hello &#x22;w&#x22;"></p>"#);
}

#[test]
fn with_custom_element_amp_matches_tier2() {
    assert_matches(r#"<my-component>x</my-component><p><img src="/x.png" alt="A &amp; B"></p>"#);
}

#[test]
fn with_custom_element_title_escaped() {
    assert_matches(r#"<my-component>x</my-component><p><img src="/x.png" alt="a" title="&#x22;t&#x22;"></p>"#);
}
