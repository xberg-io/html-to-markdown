#![allow(missing_docs)]

//! Regression tests for two Tier-2 emphasis bugs:
//!
//! - Issue #483: sibling `<i>`/`<em>`/`<strong>`/`<b>` elements each independently wrapped
//!   their own content in `*…*`/`**…**`, producing a byte-adjacent delimiter run (`*A**B*`)
//!   that `CommonMark` reparses as nested/misplaced emphasis instead of the two flat spans
//!   the source expressed. Fixed by `merge_adjacent_emphasis`
//!   (`converter/utility/content.rs`), called from `converter/inline/emphasis.rs`.
//! - Issue #481: a whitespace-only inline element (e.g. `<i> </i>`) either duplicated its
//!   single space (`chomp_inline` counted the same run as both prefix and suffix) or lost it
//!   entirely (a `<p>`-specific "insignificant leading block whitespace" check misfired on a
//!   nested inline wrapper's fresh, empty scratch buffer). Fixed by `chomp_inline`'s
//!   whitespace-only collapse, `emphasis.rs`'s `!output.ends_with(' ')` guard, and
//!   `text_node.rs`'s `Context::block_output_ptr` buffer-identity guard.

use html_to_markdown_rs::convert;
use html_to_markdown_rs::options::ConversionOptions;

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn content_with(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

// ~keep ── Issue #483: adjacent emphasis forms an unintended CommonMark delimiter run ──────

#[test]
fn should_merge_three_adjacent_emphasis_elements_into_one_run() {
    assert_eq!(content("<i>A</i><i>B</i><i>C</i>").trim(), "*ABC*");
}

#[test]
fn should_merge_adjacent_emphasis_elements_whose_content_is_punctuation() {
    assert_eq!(content("<em>A</em><em>(</em><em>x</em>").trim(), "*A(x*");
}

#[test]
fn should_merge_two_adjacent_strong_elements_into_one_run() {
    assert_eq!(content("<strong>A</strong><strong>B</strong>").trim(), "**AB**");
}

#[test]
fn should_not_merge_emphasis_elements_separated_by_a_real_space() {
    // ~keep Control: a genuine space between the two elements means `output` does not end
    // ~keep with the matching close marker when the second opens, so no merge must occur.
    assert_eq!(content("<i>A</i> <i>B</i>").trim(), "*A* *B*");
}

#[test]
fn should_not_merge_across_different_emphasis_kinds() {
    // ~keep `<em>` closing into `<strong>` opening is not the SAME delimiter run
    // ~keep (verified against the comrak oracle: `*A***B**` parses as
    // ~keep `<em>A</em><strong>B</strong>`, i.e. correctly, so no merge is needed or wanted
    // ~keep here) -- this is the "also investigate" shape from issue #483, left unmerged.
    assert_eq!(content("<i>A</i><b>B</b>").trim(), "*A***B**");
}

#[test]
fn should_still_produce_triple_delimiters_for_nested_strong_and_emphasis() {
    assert_eq!(content("<strong><em>both</em></strong>").trim(), "***both***");
    assert_eq!(content("<strong><em>x</em></strong>").trim(), "***x***");
}

#[test]
fn should_merge_adjacent_emphasis_under_the_underscore_symbol_variant() {
    let options = ConversionOptions {
        strong_em_symbol: '_',
        ..ConversionOptions::default()
    };
    assert_eq!(content_with("<em>A</em><em>B</em>", options).trim(), "_AB_");
}

#[test]
fn should_merge_adjacent_strong_under_the_underscore_symbol_variant() {
    let options = ConversionOptions {
        strong_em_symbol: '_',
        ..ConversionOptions::default()
    };
    assert_eq!(
        content_with("<strong>A</strong><strong>B</strong>", options).trim(),
        "__AB__"
    );
}

#[test]
fn should_not_merge_when_the_preceding_run_is_already_longer_than_the_marker() {
    // ~keep The first `<em>`'s content itself ends in a literal, unescaped `*`, so its
    // ~keep close marker lands right after another `*` -- an actual 2-run, not "exactly 1
    // ~keep matching close marker with nothing symbol-like before it". Merging would
    // ~keep incorrectly eat into that literal asterisk.
    assert_eq!(content(r"<em>a*</em><em>b</em>").trim(), "*a***b*");
}

#[test]
fn should_not_merge_across_an_escaped_trailing_backslash() {
    // ~keep The first `<em>`'s content is a literal trailing backslash, which the
    // ~keep text-node escaper doubles (issue #458) -- so the byte immediately before the
    // ~keep close marker's `*` is itself a backslash. `merge_adjacent_emphasis` must
    // ~keep refuse to merge across it (its explicit `preceding == '\\'` guard) rather than
    // ~keep treating that backslash as escaping the close marker itself.
    assert_eq!(content(r"<em>a\</em><em>b</em>").trim(), r"*a\\**b*");
}

// ~keep ── Issue #481: a whitespace-only inline element duplicates or loses its space ──────

#[test]
fn should_collapse_a_whitespace_only_emphasis_body_to_one_space_between_bare_text() {
    assert_eq!(content("A<i> </i>B").trim(), "A B");
}

#[test]
fn should_collapse_a_whitespace_only_emphasis_body_when_a_real_space_already_precedes_it() {
    assert_eq!(content("A <i> </i>B").trim(), "A B");
}

#[test]
fn should_preserve_the_single_space_from_a_whitespace_only_emphasis_body_inside_a_paragraph() {
    assert_eq!(content("<p>A<i> </i>B</p>").trim(), "A B");
}

#[test]
fn should_separate_whitespace_only_strong_siblings_with_a_single_space() {
    assert_eq!(
        content("<p><strong>w</strong><strong> </strong><strong>w</strong></p>").trim(),
        "**w** **w**"
    );
}
