// ~keep The inner attribute below is a crate-level Rust attribute, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #487's split-code-span design (see `br_inside_code_spans.rs`
//! for the core split/join behaviour): the contexts where a `<br>` inside a code span does
//! NOT split, plus the segment-collapse and per-segment-fencing edge cases the split
//! introduces.
//!
//! - A code span inside a table cell or a heading cannot carry a hard break at all (neither
//!   context can, independent of code): `line_break.rs` folds the `<br>` to a single space
//!   directly, before it ever reaches the segment-split logic, so the result stays ONE span.
//! - A code span inside a link label is the opposite: `CommonMark` explicitly permits a hard
//!   break inside link text (spec examples 642/643), and this crate already preserves one
//!   there (issue #432's `normalize_link_label`) rather than folding it away. So a `<br>`
//!   inside a code span nested in a link label DOES split, same as everywhere else outside a
//!   table cell or heading.
//! - `<code>A<br><br>B</code>` (adjacent `<br>`) and a leading/trailing `<br>` each produce an
//!   empty segment, which is dropped rather than rendered as a dangling, empty backtick pair.
//! - A backtick inside only one segment must widen only that segment's fence, independently
//!   of its sibling segment.

use html_to_markdown_rs::{ConversionOptions, NewlineStyle};

fn convert(html: &str) -> String {
    html_to_markdown_rs::convert(html, None)
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn convert_backslash(html: &str) -> String {
    html_to_markdown_rs::convert(
        html,
        Some(ConversionOptions {
            newline_style: NewlineStyle::Backslash,
            ..Default::default()
        }),
    )
    .expect("conversion should succeed")
    .content
    .unwrap_or_default()
}

#[test]
fn should_fold_a_br_inside_a_code_span_inside_a_table_cell_to_a_space_instead_of_splitting() {
    let html = "<table><tr><td><code>A<br>B</code></td></tr></table>";
    let result = convert(html);
    assert_eq!(result, "| `A B` |\n| ----- |\n", "actual: {result:?}");
    assert_eq!(
        result.lines().count(),
        2,
        "cell must stay on one physical line: {result:?}"
    );
}

#[test]
fn should_fold_a_br_inside_a_kbd_inside_a_table_cell_to_a_space_instead_of_splitting() {
    let html = "<table><tr><td><kbd>A<br>B</kbd></td></tr></table>";
    let result = convert(html);
    assert_eq!(result, "| `A B` |\n| ----- |\n", "actual: {result:?}");
}

#[test]
fn should_fold_a_br_inside_a_code_span_inside_a_heading_to_a_space_instead_of_splitting() {
    let html = "<h2><code>A<br>B</code></h2>";
    let result = convert(html);
    assert_eq!(result, "## `A B`\n", "actual: {result:?}");
}

/// ~keep Deviation from the literal spec wording "table cell / heading / link label ...
/// ~keep folds to a single space, NOT a split": a link label is NOT like a heading or a
/// ~keep table cell here. `CommonMark` spec examples 642/643 make a hard break inside link
/// ~keep text legal, and this crate already preserves one there on purpose (issue #432,
/// ~keep `normalize_link_label` in `converter/utility/content.rs`) rather than collapsing it
/// ~keep -- collapsing it would be a regression, not a fix. So a `<br>` inside a code span
/// ~keep nested in a link label DOES split, exactly as it would outside the link.
#[test]
fn should_still_split_a_br_inside_a_code_span_inside_a_link_label() {
    let html = r#"<p><a href="https://example.com/"><code>A<br>B</code></a></p>"#;
    let result = convert(html);
    assert_eq!(result, "[`A`  \n`B`](https://example.com/)\n", "actual: {result:?}");
}

#[test]
fn should_collapse_the_empty_middle_segment_from_two_adjacent_br_tags() {
    let html = "<code>A<br><br>B</code>";
    let result = convert(html);
    assert_eq!(result, "`A`  \n`B`\n", "actual: {result:?}");
    assert!(
        !result.contains("``  \n"),
        "must not emit a dangling empty span: {result:?}"
    );
}

#[test]
fn should_collapse_a_leading_br_with_nothing_before_it() {
    let html = "<code><br>A</code>";
    let result = convert(html);
    assert_eq!(result, "`A`\n", "actual: {result:?}");
}

#[test]
fn should_collapse_a_trailing_br_with_nothing_after_it() {
    let html = "<code>A<br></code>";
    let result = convert(html);
    assert_eq!(result, "`A`\n", "actual: {result:?}");
}

#[test]
fn should_widen_only_the_segment_that_contains_a_backtick() {
    // ~keep The first segment ("a`b") contains a backtick and needs a doubled fence; the
    // ~keep second ("c") does not and must stay single-backtick -- computed independently,
    // ~keep not from the widest fence across both segments.
    let html = "<code>a`b<br>c</code>";
    let result = convert(html);
    assert_eq!(result, "``a`b``  \n`c`\n", "actual: {result:?}");
}

#[test]
fn should_widen_only_the_segment_that_contains_a_backtick_under_backslash_style() {
    let html = "<code>a`b<br>c</code>";
    let result = convert_backslash(html);
    assert_eq!(result, "``a`b``\\\n`c`\n", "actual: {result:?}");
}
