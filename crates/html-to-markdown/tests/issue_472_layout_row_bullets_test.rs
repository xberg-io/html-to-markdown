// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #472: a layout table renders each row as a list item, but the
//! renderer hardcoded `- ` and never read `options.bullets`, so setting `bullets` had no effect
//! on the markers users actually saw. Surfaced by the #470 reporter, who passed `bullets="*+-"`
//! and still got `-`.
//!
//! The marker is chosen the way the list handlers choose theirs — cycling through `bullets` by
//! nesting depth — so a layout row nested inside a list picks the next marker rather than
//! repeating its parent's.
//!
//! Tier-1 bails on layout tables (`close_table` in `tier1/scanner.rs`), so this path is Tier-2
//! only and needs no parity mirror.

use html_to_markdown_rs::{ConversionOptions, convert};

/// A short, link-dense table with no `<th>`/`<caption>` is what makes the converter treat a
/// table as layout and render its rows as list items (`row_count <= 2 && link_count >= 3`;
/// issue #500 dropped ragged row lengths alone as a layout trigger, since a headerless table
/// with ragged rows is ordinary tabular data far more often than it is layout).
const LAYOUT_TABLE: &str = "<table><tr><td>alpha</td><td>beta</td></tr><tr><td><a href=\"/1\">a</a></td><td><a href=\"/2\">b</a></td><td><a href=\"/3\">c</a></td></tr></table>";

fn content(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options)).unwrap().content.unwrap_or_default()
}

fn with_bullets(bullets: &str) -> ConversionOptions {
    ConversionOptions {
        bullets: bullets.to_string(),
        ..ConversionOptions::default()
    }
}

/// The reported case: the first configured bullet is used at top level.
#[test]
fn should_use_the_configured_bullet_for_a_layout_row() {
    let out = content(LAYOUT_TABLE, with_bullets("*+-"));
    for line in out.lines().filter(|l| !l.trim().is_empty()) {
        assert!(
            line.starts_with("* "),
            "layout row ignored options.bullets (expected `* `): {out:?}"
        );
    }
}

/// The default is unchanged, so nobody who never set `bullets` sees a different marker.
#[test]
fn should_keep_the_default_bullet_when_bullets_is_not_customised() {
    let out = content(LAYOUT_TABLE, ConversionOptions::default());
    for line in out.lines().filter(|l| !l.trim().is_empty()) {
        assert!(line.starts_with("- "), "default marker changed: {out:?}");
    }
}

/// Every configured bullet set is honoured, not just the reported one.
#[test]
fn should_honour_each_configured_bullet_set() {
    for (bullets, expected) in [("-*+", "- "), ("*+-", "* "), ("+-*", "+ ")] {
        let out = content(LAYOUT_TABLE, with_bullets(bullets));
        let first = out.lines().find(|l| !l.trim().is_empty()).unwrap_or_default();
        assert!(
            first.starts_with(expected),
            "bullets={bullets:?} expected {expected:?}: {out:?}"
        );
    }
}

/// A layout table inside a list is one level deeper, so it takes the next marker in the cycle
/// rather than repeating its parent's — same rule the list handlers follow.
#[test]
fn should_advance_the_bullet_cycle_for_a_layout_row_nested_in_a_list() {
    let html = format!("<ul><li>parent{LAYOUT_TABLE}</li></ul>");
    let out = content(html.as_str(), with_bullets("*+-"));
    assert!(out.contains("* parent"), "parent item lost its marker: {out:?}");
    assert!(
        out.contains("+ alpha"),
        "nested layout row should take the second bullet: {out:?}"
    );
}

/// An empty `bullets` string must not panic or index out of bounds.
#[test]
fn should_fall_back_when_bullets_is_empty() {
    let out = content(LAYOUT_TABLE, with_bullets(""));
    assert!(!out.trim().is_empty(), "empty bullets produced no output: {out:?}");
}
