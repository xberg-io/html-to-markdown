#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Guards the html5ever repair path against a dependency-level data-loss bug.
//!
//! html5ever 0.40.0's serializer dropped the leading `0xC2` byte of a two-byte UTF-8
//! sequence, which covers the U+0080..U+00BF block -- `§`, `©`, `°`, `·`, `\u{a0}`.
//! `repair_with_html5ever` serializes the repaired tree back to a string for `tl` to
//! re-parse, so a mangled byte there corrupts the re-parse and every node after the
//! damaged text is silently lost. Measured on 0.40.0: the whole trailing `<p>` of
//! `<table><tr><th>Before</th></tr><tr></table><p>Footer § 9</p>` disappeared, while the
//! byte-identical ASCII document converted correctly.
//!
//! That ASCII/non-ASCII split is why it needs its own test: every other repair-path
//! regression test in this suite uses ASCII fixtures and passes on the broken release.
//! Verified by building against `=0.40.0` and watching these assertions fail, then
//! against 0.40.1 and watching them pass.

use html_to_markdown_rs::{ConversionOptions, convert};

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

/// Every character here encodes as `0xC2 0x??`, the exact range 0.40.0 truncated.
const C2_CHARS: [(&str, char); 4] = [
    ("section sign", '\u{a7}'),
    ("copyright sign", '\u{a9}'),
    ("degree sign", '\u{b0}'),
    ("middle dot", '\u{b7}'),
];

#[test]
fn repair_path_keeps_content_after_a_c2_range_character() {
    for (name, ch) in C2_CHARS {
        let html = format!("<table><tr><th>Before</th></tr><tr></table><p>Footer {ch} 9</p>");
        let out = content(&html);
        assert_eq!(
            out,
            format!("| Before |\n| ------ |\n\nFooter {ch} 9\n"),
            "{name} (U+{:04X}) on the repair path; actual: {out:?}",
            ch as u32
        );
    }
}

#[test]
fn repair_path_keeps_a_c2_range_character_inside_the_table() {
    for (name, ch) in C2_CHARS {
        let html = format!("<table><tr><th>{ch} 12</th></tr><tr></table><p>Visible footer</p>");
        let out = content(&html);
        assert!(
            out.contains(&format!("{ch} 12")) && out.contains("Visible footer"),
            "{name} (U+{:04X}) inside a repaired table; actual: {out:?}",
            ch as u32
        );
    }
}

#[test]
fn control_ascii_repair_path_is_unaffected() {
    // ~keep The control that made the bug invisible: this exact shape passes on the broken
    // ~keep release, so an ASCII-only suite is not evidence the repair path is sound.
    let out = content("<table><tr><th>Before</th></tr><tr></table><p>Visible footer</p>");
    assert_eq!(out, "| Before |\n| ------ |\n\nVisible footer\n", "actual: {out:?}");
}
