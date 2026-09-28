// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for the "is the output currently sitting at a bare list marker?"
//! idiom implemented as a plain two-byte suffix check (`ends_with("* ")` / `("- ")` /
//! `(". ")`, sometimes also missing `"+ "`, the third bullet in the default `"-*+"`
//! cycle). A closing `<strong>`/`<em>` immediately followed by a migrated trailing space
//! (e.g. `"**bold** "`) ends in the exact same two bytes as a real bare `"* "` bullet,
//! so the suffix check cannot tell them apart -- it must instead check whether the
//! WHOLE current line decomposes into nothing but marker tokens
//! (`list::utils::line_is_bare_list_marker`). This file covers the occurrences of that
//! idiom in `handlers/blockquote.rs`, `handlers/code_block.rs`, `block/div.rs`, and
//! Tier-1's `open_paragraph` (`tier1/scanner.rs`) -- the sibling to the list-flattening
//! defect already covered by `tier1_list_paragraph_test.rs`'s
//! `nested_list_after_strong_with_trailing_space_matches` and friends.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn default_convert(html: &str) -> String {
    convert(html, None).unwrap().content.unwrap_or_default()
}

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

fn assert_tiers_match(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

/// `handlers/blockquote.rs`'s `is_list_continuation`: a blockquote following a list
/// item's own `<strong>` text (not the item's first content) was misread as sitting
/// right after the bullet -- since `"**bold** "` ends in the same two bytes as a real
/// `"* "` -- and skipped the continuation indent on its first quoted line. An
/// unindented `"> "` line falls out of the item, and the list, on reparse (`CommonMark`
/// spec example 263).
#[test]
fn blockquote_after_strong_gets_continuation_indent() {
    let html = "<ul><li><strong>bold</strong> <blockquote>quoted text here</blockquote></li></ul>";
    assert_eq!(default_convert(html), "- **bold**\n  > quoted text here\n");
}

/// `handlers/code_block.rs`'s `format_code_block_in_list_item`'s `is_continuation`: a
/// fenced code block following a list item's own `<strong>` text was misread as the
/// item's first content and its opening fence got glued onto the same physical line as
/// the preceding inline text (`"Lead **bold** ```"`) instead of starting its own line --
/// which is not a valid fence opener in `CommonMark` (a fence must be the first thing on
/// its line), so the whole construct would parse back as a plain paragraph instead of a
/// code block.
#[test]
fn code_block_after_strong_starts_its_own_line() {
    let html = "<ul><li>Lead <strong>bold</strong> <pre>code line 1\ncode line 2</pre></li></ul>";
    assert_eq!(
        default_convert(html),
        "- Lead **bold**\n\n  ```\n  code line 1\n  code line 2\n  ```\n"
    );
}

/// Same handler, different half of the idiom: the suffix check only tested `"* "` and
/// `"- "`, omitting `"+ "` -- the third bullet in the default `"-*+"` cycle used by every
/// third nesting level. A fenced code block that is the sole, first content of a
/// third-level list item was misread as a continuation of its own (nonexistent)
/// preceding content, getting an extra unwanted blank-line separator and a doubled
/// indent on its first line instead of sitting directly after the `"+"` marker like the
/// first- and second-level cases do.
#[test]
fn code_block_as_first_content_of_third_level_item_has_no_extra_indent() {
    let html = "<ul><li><ul><li><ul><li><pre>code line 1\ncode line 2</pre></li></ul></li></ul></li></ul>";
    assert_eq!(
        default_convert(html),
        "- * + ```\n      code line 1\n      code line 2\n      ```\n"
    );
}

/// `block/div.rs`'s `is_list_continuation`: the false positive here is the most severe
/// of the four, because neither branch of the surrounding `if`/`else if` chain fires
/// when it's wrong -- the div's content was appended directly after the preceding
/// `<strong>` text with NO separator of any kind, silently merging block-level content
/// into the middle of an inline run.
#[test]
fn div_after_strong_gets_its_own_indented_line() {
    // ~keep The div is its own paragraph in the item, after a blank line (issue #583).
    let html = "<ul><li>Lead <strong>bold</strong> <div>divcontent</div></li></ul>";
    assert_eq!(default_convert(html), "- Lead **bold**\n\n  divcontent\n");
}

/// Same handler, the missing-`"+ "` half of the idiom, mirroring the code-block case
/// above: a `<div>` as the sole, first content of a third-level list item.
#[test]
fn div_as_first_content_of_third_level_item_has_no_extra_indent() {
    let html = "<ul><li><ul><li><ul><li><div>divcontent</div></li></ul></li></ul></li></ul>";
    assert_eq!(default_convert(html), "- * + divcontent\n");
}

/// Tier-1's `open_paragraph` applied its "does this paragraph sit right after a bullet?"
/// suffix check unconditionally, without first checking whether a list item is even open
/// -- unlike Tier-2's `paragraph.rs`, which only asks that question when
/// `ctx.in_list_item` is true. Plain top-level text ending in a real `"- "` (no list
/// anywhere in the document) followed by a `<p>` was misread as "first content of a list
/// item" and silently lost its `"\n\n"` block separator, gluing the two blocks onto one
/// line. This diverged from Tier-2, which always separates top-level paragraphs
/// regardless of what the preceding text happens to end in.
#[test]
fn tier1_paragraph_after_toplevel_dash_text_gets_blank_line() {
    let html = "Score is 5 - <p>New para</p>";
    assert_tiers_match(html);
    assert_eq!(tier2(html), "Score is 5 -\n\nNew para\n");
}

/// Same Tier-1 gate, exercised via the `<strong>`/migrated-trailing-space ambiguity
/// rather than a literal bullet: `"**bold** "` ends in the same two bytes as `"* "`.
/// With no list item open, this must still get the ordinary top-level blank-line
/// separator before the following `<p>`.
#[test]
fn tier1_paragraph_after_toplevel_strong_gets_blank_line() {
    let html = "<strong>bold</strong> <p>New para</p>";
    assert_tiers_match(html);
    assert_eq!(tier2(html), "**bold**\n\nNew para\n");
}
