// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #505: a whitespace-only newline nested two levels deep inside
//! plain inline wrappers (with no next sibling of its own at either level) was dropped, joining
//! the words on either side -- `<i>Alpha</i><span><span>\n</span></span>Beta` rendered
//! `*Alpha*Beta`.
//!
//! Issue #430 taught `newline_span_needs_separating_space` to look one level up: if a
//! newline-only text node's immediate parent is inline-like and THAT parent has a following
//! inline sibling, the newline still separates words. But it only checked the immediate
//! parent's own next sibling. `<span><span>\n</span></span>Beta` nests the newline inside the
//! INNER span, whose parent (the outer span) has no next sibling of its own at all -- the real
//! next-sibling question belongs to the outer span, one level further up. The fix climbs
//! through consecutive inline-like ancestors that themselves have no next sibling, stopping the
//! moment a level is conclusive (a block boundary, or a real blocking/qualifying sibling).
//!
//! This shape reproduces on 3.14.1 too (not a 3.14.2 regression like #504); it is a gap #430
//! left one level short of, not a new corruption.

use html_to_markdown_rs::{ConversionOptions, convert};

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

#[test]
fn should_keep_a_separator_for_a_newline_two_levels_deep_in_plain_spans() {
    let out = content("<p><i>Alpha</i><span><span>\n</span></span>Beta</p>");
    assert_eq!(out, "*Alpha* Beta\n", "actual: {out:?}");
}

#[test]
fn should_keep_a_separator_for_a_newline_three_levels_deep_in_plain_spans() {
    let out = content("<p><i>Alpha</i><span><span><span>\n</span></span></span>Beta</p>");
    assert_eq!(out, "*Alpha* Beta\n", "actual: {out:?}");
}

#[test]
fn should_still_emit_nothing_when_the_outer_span_also_has_no_next_sibling() {
    // ~keep The climb must stop at a genuine block boundary rather than treating "no sibling
    // ~keep anywhere in the ancestor chain" as "keep going forever" -- there is no next word to
    // ~keep separate from here.
    let out = content("<p>Alpha<span><span>\n</span></span></p>");
    assert_eq!(out, "Alpha\n", "actual: {out:?}");
}

#[test]
fn should_not_add_a_separator_when_the_outer_spans_next_sibling_is_a_block() {
    // ~keep The outer span's own next sibling exists but is not inline content -- a block
    // ~keep boundary blocks the climb outright rather than looking further.
    let out = content("<div><span><span>\n</span></span><p>Beta</p></div>");
    assert_eq!(out, "Beta\n", "actual: {out:?}");
}

#[test]
fn should_not_double_a_separator_the_previous_sibling_already_emitted() {
    let out = content("<p>Alpha <span><span>\n</span></span>Beta</p>");
    assert_eq!(out, "Alpha Beta\n", "actual: {out:?}");
}
