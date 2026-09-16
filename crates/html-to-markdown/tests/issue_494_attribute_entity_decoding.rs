#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #494: character references in attribute values reached the
//! Markdown output undecoded.
//!
//! `<a href>` was the single attribute site that called `decode_html_entities`; every other
//! user-visible attribute passed the raw bytes straight through, so `title="A&amp;B"` rendered
//! as the literal text `A&amp;B`. Twelve sites were affected, found by probing every attribute
//! that reaches output rather than by reading the ones that looked likely.
//!
//! The Markdown-level escaping was never the problem and is unchanged: a single-quoted
//! `title='say "hi"'` already emitted `"say \"hi\""` correctly. The decoded character simply
//! never arrived. These tests therefore assert decode-then-existing-escape, and
//! `entity_and_literal_forms_agree` pins that equivalence directly.

use html_to_markdown_rs::{ConversionOptions, convert};

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

/// `(label, html, expected)` for every site the probe found leaking.
const LEAK_CASES: &[(&str, &str, &str)] = &[
    ("a/title", r#"<a href="/x" title="A&amp;B">L</a>"#, "[L](/x \"A&B\")\n"),
    ("img/alt", r#"<img src="i.png" alt="A&amp;B">"#, "![A&B](i.png)\n"),
    (
        "img/title",
        r#"<img src="i.png" alt="z" title="A&amp;B">"#,
        "![z](i.png \"A&B\")\n",
    ),
    (
        "img/src",
        r#"<img src="i.png?a=A&amp;B" alt="z">"#,
        "![z](i.png?a=A&B)\n",
    ),
    ("abbr/title", r#"<abbr title="A&amp;B">x</abbr>"#, "x (A&B)\n"),
    (
        "code/class",
        r#"<pre><code class="language-A&amp;B">x</code></pre>"#,
        "```A&B\nx\n```\n",
    ),
];

#[test]
fn attribute_character_references_are_decoded() {
    for (label, html, expected) in LEAK_CASES {
        let out = content(html);
        assert_eq!(out, *expected, "{label}; actual: {out:?}");
    }
}

#[test]
fn media_src_character_references_are_decoded() {
    for (label, html) in [
        ("audio", r#"<audio src="a.mp3?x=A&amp;B"></audio>"#),
        ("video", r#"<video src="v.mp4?x=A&amp;B"></video>"#),
        ("iframe", r#"<iframe src="f.html?x=A&amp;B"></iframe>"#),
        ("source", r#"<video><source src="s.mp4?x=A&amp;B"></video>"#),
    ] {
        let out = content(html);
        assert!(
            !out.contains("&amp;"),
            "{label} leaked a raw entity into the output; actual: {out:?}"
        );
        assert!(out.contains("A&B"), "{label} lost the decoded value; actual: {out:?}");
    }
}

#[test]
fn blockquote_cite_character_references_are_decoded() {
    let out = content(r#"<blockquote cite="/c?a=A&amp;B">x</blockquote>"#);
    assert!(!out.contains("&amp;"), "actual: {out:?}");
    assert!(out.contains("A&B"), "actual: {out:?}");
}

#[test]
fn an_entity_encoded_quote_in_a_title_is_escaped_not_passed_through() {
    // ~keep The entity form and the literal form must produce identical output. The literal
    // ~keep form already escaped correctly before this fix; only the entity form was broken,
    // ~keep which is why decoding is the whole change and the escaper is untouched.
    let entity = content(r#"<a href="/x" title="say &#34;hi&#34;">L</a>"#);
    let literal = content(r#"<a href='/x' title='say "hi"'>L</a>"#);
    assert_eq!(entity, literal, "entity form: {entity:?} literal form: {literal:?}");
    assert_eq!(entity, "[L](/x \"say \\\"hi\\\"\")\n", "actual: {entity:?}");
}

#[test]
fn entity_and_literal_forms_agree() {
    for (label, entity_html, literal_html) in [
        (
            "a/title apostrophe",
            r#"<a href="/x" title="it&#39;s">L</a>"#,
            r#"<a href="/x" title="it's">L</a>"#,
        ),
        (
            "img/alt bracket",
            r#"<img src="i.png" alt="a&#93;b">"#,
            r#"<img src="i.png" alt="a]b">"#,
        ),
        (
            "abbr/title ampersand",
            r#"<abbr title="A&amp;B">x</abbr>"#,
            r#"<abbr title="A&B">x</abbr>"#,
        ),
    ] {
        let entity = content(entity_html);
        let literal = content(literal_html);
        assert_eq!(entity, literal, "{label}: {entity:?} vs {literal:?}");
    }
}

// ~keep ── Controls: the two sites that were already correct must stay correct ──────────

#[test]
fn control_href_and_text_content_are_unchanged() {
    let href = content(r#"<a href="/x?p=A&amp;B">L</a>"#);
    assert_eq!(href, "[L](/x?p=A&B)\n", "actual: {href:?}");

    let text = content("<p>A&amp;B</p>");
    assert_eq!(text, "A&B\n", "actual: {text:?}");
}

#[test]
fn control_an_attribute_without_entities_is_untouched() {
    let out = content(r#"<a href="/x" title="plain title">L</a>"#);
    assert_eq!(out, "[L](/x \"plain title\")\n", "actual: {out:?}");
}

// ~keep ── Tier-1 parity ──────────────────────────────────────────────────────────────
//
// ~keep The fast scanner reads attributes from raw bytes on its own path, so it can
// ~keep reintroduce the defect independently. These call `tier1::run` directly rather than
// ~keep going through `convert`, so a bail surfaces as a test failure instead of silently
// ~keep falling back to Tier 2 and reporting a pass Tier 1 never earned.

use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{TierStrategy, prescan};

fn tier1_run(html: &str) -> Result<String, BailReason> {
    let (cleaned, report) = prescan::run(html);
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..ConversionOptions::default()
    };
    tier1::run(cleaned.as_ref(), &report, &options)
}

#[test]
fn tier1_decodes_attribute_entities_identically_to_tier2() {
    let mut checked = 0_usize;
    let mut bailed = Vec::new();
    for (label, html, _) in LEAK_CASES {
        match tier1_run(html) {
            Ok(tier1_output) => {
                assert_eq!(tier1_output, content(html), "{label}: tier1 vs tier2");
                assert!(
                    !tier1_output.contains("&amp;"),
                    "{label}: tier1 leaked a raw entity: {tier1_output:?}"
                );
                checked += 1;
            }
            // ~keep A bail is a legitimate Tier-1 answer (the router falls back to Tier 2),
            // ~keep but record it so this test can never silently check nothing.
            Err(reason) => bailed.push((label, reason)),
        }
    }
    assert!(
        checked > 0,
        "every case bailed, so this test verified nothing: {bailed:?}"
    );
    assert_eq!(
        checked + bailed.len(),
        LEAK_CASES.len(),
        "case accounting is wrong: {checked} checked, {} bailed, {} total",
        bailed.len(),
        LEAK_CASES.len()
    );
}

#[test]
fn tier1_escapes_a_decoded_quote_in_a_title_like_tier2() {
    // ~keep Decoding surfaced a latent Tier-1 escaping gap: before #494 neither tier could
    // ~keep put a raw `"` in a title, so nothing exercised the escape. Cover both the image
    // ~keep and the link emit paths, since they build the title string separately.
    for (label, html) in [
        ("img", r#"<img src="/x.png" alt="a" title="&#x22;t&#x22;">"#),
        ("a", r#"<a href="/x" title="&#x22;t&#x22;">L</a>"#),
    ] {
        let tier2_output = content(html);
        assert!(
            tier2_output.contains("\\\""),
            "{label}: tier2 should escape the decoded quote; actual: {tier2_output:?}"
        );
        match tier1_run(html) {
            Ok(tier1_output) => assert_eq!(tier1_output, tier2_output, "{label}: tier1 vs tier2"),
            Err(reason) => panic!("{label}: tier1 bailed ({reason:?}), so this case checked nothing"),
        }
    }
}

/// Characters that decoding newly makes reachable inside an attribute value, each of which
/// means something in the Markdown context the value lands in.
const NEWLY_REACHABLE: &[(&str, &str)] = &[
    ("quote", "&#x22;"),
    ("bracket_close", "&#93;"),
    ("bracket_open", "&#91;"),
    ("paren_close", "&#41;"),
    ("paren_open", "&#40;"),
    ("backslash", "&#92;"),
    ("backtick", "&#96;"),
    ("space", "&#32;"),
    ("newline", "&#10;"),
    ("lt", "&lt;"),
    ("gt", "&gt;"),
];

#[test]
fn tier1_matches_tier2_for_every_newly_reachable_character() {
    // ~keep A sweep rather than a case list: decoding turned each of these from "unreachable
    // ~keep inside an attribute" into "ordinary content", and every one of them is syntax in
    // ~keep the Markdown position it lands in. Two Tier-1 escaping gaps were already found
    // ~keep this way (image title, link title), both latent long before #494 because no input
    // ~keep could reach them. The sweep is what shows there is not a third.
    let mut checked = 0_usize;
    let mut bailed = 0_usize;
    let mut divergences = Vec::new();

    for (char_name, entity) in NEWLY_REACHABLE {
        for (site, template) in [
            ("a/title", format!(r#"<a href="/x" title="p{entity}q">L</a>"#)),
            ("img/alt", format!(r#"<img src="/x.png" alt="p{entity}q">"#)),
            ("img/title", format!(r#"<img src="/x.png" alt="a" title="p{entity}q">"#)),
            ("img/src", format!(r#"<img src="/x{entity}y.png" alt="a">"#)),
            ("abbr/title", format!(r#"<abbr title="p{entity}q">x</abbr>"#)),
        ] {
            let tier2_output = content(&template);
            match tier1_run(&template) {
                Ok(tier1_output) => {
                    checked += 1;
                    if tier1_output != tier2_output {
                        divergences.push(format!(
                            "{site} with {char_name}: tier1 {tier1_output:?} != tier2 {tier2_output:?}"
                        ));
                    }
                }
                Err(_) => bailed += 1,
            }
        }
    }

    let total = NEWLY_REACHABLE.len() * 5;
    assert_eq!(checked + bailed, total, "case accounting is wrong");
    assert!(checked > 0, "every case bailed, so this test verified nothing");
    assert!(
        divergences.is_empty(),
        "{} of {checked} checked cases diverged:\n{}",
        divergences.len(),
        divergences.join("\n")
    );
}
