// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #470: adjacent `<p>` elements inside a table cell were joined
//! with zero bytes between them, merging words and running two emphasis spans together into
//! invalid Markdown (`**Alice Example***Customer Service*`).
//!
//! The cell is reached through the *layout*-table path: `border="0"` plus a spanning cell
//! makes `looks_like_layout` true (issue #500 dropped the reporter's original trigger, ragged
//! row lengths, since a headerless table with ragged rows is ordinary tabular data far more
//! often than it is layout), and a header-less, caption-less layout table renders each row as
//! a list item. `append_layout_cell_text` built
//! its cell context with `convert_as_inline: true` and nothing else, so `<p>`'s
//! table-continuation branch never fired — `br_in_tables` was not even consulted — while
//! `convert_as_inline` simultaneously suppressed the ordinary block separator. Neither
//! separator was emitted, so the paragraphs abutted.
//!
//! The fix marks the context as a layout cell, which routes `<p>` and `<div>` continuations
//! through the settled cell rule from issues #453/#454 (`emit_table_cell_break`): a literal
//! `<br>` under `br_in_tables`, otherwise a single space. Not a regression — this predates
//! 3.8.3, matching the report.

use html_to_markdown_rs::{ConversionOptions, NewlineStyle, convert};

fn reporter_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        br_in_tables: true,
        bullets: "*+-".to_string(),
        compact_tables: true,
        keep_inline_images_in: vec!["a".to_string(), "td".to_string(), "th".to_string()],
        newline_style: NewlineStyle::Spaces,
        ..ConversionOptions::default()
    }
}

const REPORTED_HTML: &str = r#"<table border="0">
  <tr>
    <td><p><b>Alice Example</b></p><p><i>Customer Service</i></p><p><i>Example Group</i></p></td>
    <td colspan="1">Logo</td>
  </tr>
  <tr>
    <td><p>Example</p><p>Contact details</p></td>
    <td>Other</td>
  </tr>
  <tr><td>A</td><td>B</td><td>C</td></tr>
</table>"#;

fn content(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options)).unwrap().content.unwrap_or_default()
}

/// The reported reproduction: no two paragraphs may abut.
#[test]
fn should_separate_adjacent_paragraphs_in_a_layout_cell() {
    let out = content(REPORTED_HTML, reporter_options());
    assert!(
        !out.contains("**Alice Example***Customer Service*"),
        "emphasis spans still run together: {out:?}"
    );
    assert!(
        !out.contains("ExampleContact details"),
        "paragraph text still merged: {out:?}"
    );
    for word in [
        "Alice Example",
        "Customer Service",
        "Example Group",
        "Contact details",
        "Logo",
    ] {
        assert!(out.contains(word), "{word:?} missing from output: {out:?}");
    }
}

/// `br_in_tables` selects how the boundary is represented, exactly as it does for `<br>`,
/// `<div>` and list items inside a cell (issues #453/#454).
#[test]
fn should_honour_br_in_tables_for_the_layout_cell_boundary() {
    let with_br = content(REPORTED_HTML, reporter_options());
    assert!(
        with_br.contains("**Alice Example**<br>*Customer Service*"),
        "br_in_tables must emit a literal <br>: {with_br:?}"
    );

    let without_br = content(
        REPORTED_HTML,
        ConversionOptions {
            br_in_tables: false,
            ..reporter_options()
        },
    );
    assert!(
        without_br.contains("**Alice Example** *Customer Service*"),
        "without br_in_tables the boundary must collapse to one space: {without_br:?}"
    );
    assert!(
        !without_br.contains("<br>"),
        "no <br> may be emitted when br_in_tables is off: {without_br:?}"
    );
}

/// A `<div>` continuation follows the same settled rule as `<p>`.
#[test]
fn should_separate_adjacent_divs_in_a_layout_cell() {
    let html = r#"<table border="0"><tr><td><div>A</div><div>B</div></td><td colspan="1">x</td></tr></table>"#;
    let out = content(html, reporter_options());
    assert!(!out.contains("AB"), "divs still merged: {out:?}");
}

/// The layout row is a list item, not a table row, so its text must not pick up table-cell
/// pipe/emphasis escaping along with the continuation rule.
#[test]
fn should_not_escape_pipes_or_emphasis_markers_in_a_layout_row() {
    let html = r#"<table border="0"><tr><td><p>a|b</p><p>c*d</p></td><td colspan="1">x</td></tr></table>"#;
    let out = content(html, reporter_options());
    assert!(!out.contains(r"\|"), "pipe was escaped in a layout row: {out:?}");
    assert!(!out.contains(r"\*"), "asterisk was escaped in a layout row: {out:?}");
}

/// A layout row renders as a list item, so its buffer — unlike a real cell's — can already end
/// in a newline, and separating there opened the next line with a stray leading space.
///
/// The fixture is the link-heavy layout table lifted from the gh-190 benchmark corpus, which is
/// what caught this: reduced synthetic markup did not reproduce it, so the real markup is pinned
/// verbatim. Real table cells are unaffected — their buffer never holds a newline by
/// construction — so the guard is inert for them.
#[test]
fn should_not_open_a_continuation_line_with_a_stray_space() {
    let html = include_str!("fixtures/regressions/issue_470_layout_cell_stray_space.html");
    // ~keep Default options: under `br_in_tables` the boundary is a `<br>`, not the space this
    // ~keep pins, so the reporter's option set cannot reproduce it.
    let out = content(html, ConversionOptions::default());
    for line in out.lines() {
        assert!(
            !line.starts_with(' '),
            "a rendered line opens with a stray space: {out:?}"
        );
    }
    assert!(out.contains("Actual is not normal"), "fixture content missing: {out:?}");
}

/// A `<br>` inside a layout cell follows the same settled cell rule as a `<div>`/`<p>`
/// continuation. It used to emit a hard-break marker, which put a bare newline inside a list
/// item's line and ended the item -- surfaced by the gh-121 Hacker News golden once issue #500
/// stopped dropping the spacer image that used to precede the break.
#[test]
fn should_fold_a_br_inside_a_layout_cell_to_the_cell_boundary() {
    let html = r#"<table border="0"><tr><td>x<br>y<div>z</div></td><td colspan="1">w</td></tr></table>"#;
    let out = content(html, ConversionOptions::default());
    assert_eq!(out, "- x y z w\n", "actual: {out:?}");
    let out = content(html, reporter_options());
    assert_eq!(out, "* x<br>y<br>z w\n", "actual: {out:?}");
}

/// A nested table that renders to nothing (a bgcolor spacer bar) must not split the layout
/// row's list item around itself -- the second half of the gh-121 footer shape.
#[test]
fn should_not_split_a_layout_row_around_a_blank_nested_table() {
    let html =
        r#"<table border="0"><tr><td>q<table><tr><td></td></tr></table><br>x</td><td colspan="1">w</td></tr></table>"#;
    let out = content(html, ConversionOptions::default());
    assert_eq!(out, "- q x w\n", "actual: {out:?}");
}
