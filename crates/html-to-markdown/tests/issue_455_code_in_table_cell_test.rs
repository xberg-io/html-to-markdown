// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

use html_to_markdown_rs::{ConversionOptions, WhitespaceMode};

fn convert(html: &str, opts: Option<ConversionOptions>) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

/// Regression test for issue #455: a literal newline inside a `<code>` span in a table
/// cell must not survive into the rendered row and split it across physical lines, with
/// `br_in_tables: false`.
#[test]
fn should_fold_newline_in_code_span_inside_table_cell_when_br_in_tables_is_false() {
    let html = "<table><tr><td><code>a\nb</code></td></tr></table>";
    let options = ConversionOptions {
        br_in_tables: false,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result}"
    );
}

/// Same as above but with `br_in_tables: true` — the code-span newline must still fold to
/// a space, and must never become a literal `<br>` since `<br>` is not valid inside a code
/// span regardless of `br_in_tables`.
#[test]
fn should_fold_newline_in_code_span_inside_table_cell_when_br_in_tables_is_true() {
    let html = "<table><tr><td><code>a\nb</code></td></tr></table>";
    let options = ConversionOptions {
        br_in_tables: true,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result}");
    assert!(
        !result.contains("<br>"),
        "a code span must never contain a literal <br>: {result}"
    );
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result}"
    );
}

/// The HTML5 preprocessing step normalizes a raw `\r\n`/`\r` in the source markup to `\n`
/// before this crate's parser ever sees it, so a literal carriage return can only reach the
/// converter via a decoded numeric character reference (`&#13;`). That decoded `\r` inside a
/// code span in a cell must still fold to a single space.
#[test]
fn should_fold_decoded_carriage_return_in_code_span_inside_table_cell() {
    let html = "<table><tr><td><code>a&#13;b</code></td></tr></table>";
    let options = ConversionOptions {
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result}"
    );
}

/// A decoded `\r\n` pair (`&#13;&#10;`) inside a code span in a cell must fold to a single
/// space, not two.
#[test]
fn should_fold_decoded_crlf_in_code_span_inside_table_cell_to_a_single_space() {
    let html = "<table><tr><td><code>a&#13;&#10;b</code></td></tr></table>";
    let options = ConversionOptions {
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result}"
    );
}

/// A code span with multiple consecutive whitespace characters (no newline) inside a table
/// cell must preserve them verbatim: only line breaks are structurally forbidden in a GFM
/// cell, plain repeated spaces are not, and code spans preserve whitespace exactly.
#[test]
fn should_preserve_multiple_consecutive_spaces_in_code_span_inside_table_cell() {
    let html = "<table><tr><td><code>a    b</code></td></tr></table>";
    let options = ConversionOptions {
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a    b` |\n| --- |\n", "actual: {result}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result}"
    );
}

/// The newline fold inside a table-cell code span must apply regardless of `whitespace_mode`:
/// a raw newline is a structural GFM-cell constraint, not a stylistic normalization choice, so
/// `WhitespaceMode::Strict` must not let it leak through.
#[test]
fn should_fold_newline_in_code_span_inside_table_cell_under_strict_whitespace_mode() {
    let html = "<table><tr><td><code>a\nb</code></td></tr></table>";
    let options = ConversionOptions {
        whitespace_mode: WhitespaceMode::Strict,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result}"
    );
}

/// A multi-line code span outside any table cell must fold its literal newline to a single
/// space, exactly like a table cell's own fold above -- just outside a cell too.
///
/// ~keep Superseded by issue #487: a raw line ending inside a code SPAN has no hard-break
/// ~keep meaning at all under `CommonMark` (the code-spans section of the spec), so it always
/// ~keep renders as a space on reparse -- preserving it verbatim (the pre-#487 assertion this
/// ~keep replaced) was never round-trip stable. This document has no `<br>` at all, so there
/// ~keep is nothing to split: `text_node.rs`'s `in_code && !in_code_block` branch folds the
/// ~keep literal newline before it ever reaches `handlers::code_block::emit_inline_code`'s
/// ~keep segment-split logic, so a single, unsplit span is what comes out.
#[test]
fn should_fold_newline_to_a_space_in_code_span_outside_table_cell() {
    let html = "<p>see <code>a\nb</code> here</p>";
    let result = convert(html, None).unwrap();
    assert_eq!(result, "see `a b` here\n", "actual: {result:?}");
}

/// A multi-line code span outside any table cell must also fold a decoded `\r` the same way
/// (the HTML5 preprocessing step already normalizes any raw source `\r`/`\r\n` to `\n`, so a
/// literal carriage return can only reach the converter via `&#13;`).
///
/// ~keep Superseded by issue #487, same reasoning as the sibling test above: a raw `\r` is
/// ~keep still a line ending, so it folds too, not just `\n`.
#[test]
fn should_fold_decoded_carriage_return_to_a_space_in_code_span_outside_table_cell() {
    let html = "<p>see <code>a&#13;b</code> here</p>";
    let result = convert(html, None).unwrap();
    assert_eq!(result, "see `a b` here\n", "actual: {result:?}");
}

/// `<kbd>` and `<samp>` share the same verbatim, in-code text-node path as `<code>` (both set
/// `ctx.in_code` while walking their children), so a newline inside either must fold the same
/// way inside a table cell.
#[test]
fn should_fold_newline_in_kbd_inside_table_cell_when_br_in_tables_is_true() {
    let html = "<table><tr><td><kbd>a\nb</kbd></td></tr></table>";
    let options = ConversionOptions {
        br_in_tables: true,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result}"
    );
}
