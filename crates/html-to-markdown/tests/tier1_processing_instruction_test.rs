//! Tier-1 processing-instruction bail (Phase X reworked in Phase Q.4).
//!
//! `<?...?>` and `<?>` are HTML processing instructions / malformed PI
//! markers.  Tier-2 handles them inconsistently depending on whether
//! html5ever-repair ran (it rewrites bogus comments) and how tl chooses
//! to parse the run.  Tier-1 cannot mirror both paths cheaply, so it
//! bails to the Tier-2 fallback whose output is the authoritative
//! shape.

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

#[test]
fn empty_pi_falls_back_to_tier2() {
    assert_matches("<p>a<?>b</p>");
}

#[test]
fn xml_pi_falls_back_to_tier2() {
    assert_matches(r#"<p>before<?xml version="1.0"?>after</p>"#);
}

#[test]
fn pi_outside_paragraph_falls_back_to_tier2() {
    assert_matches("<p>x</p><?> <section><p>y</p></section>");
}
