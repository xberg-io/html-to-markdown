// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

use html_to_markdown_rs::{CodeBlockStyle, ConversionOptions, WhitespaceMode};

fn convert(html: &str, opts: Option<ConversionOptions>) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

fn cell_options(br_in_tables: bool) -> ConversionOptions {
    ConversionOptions {
        br_in_tables,
        compact_tables: true,
        ..Default::default()
    }
}

/// Regression test for issue #456: the reported repro. A fenced code block cannot exist inside
/// a GFM pipe cell — the fence markers are line-structured — so `<pre>` in a cell renders as
/// its content inline with the block syntax dropped. Previously the fence newlines split the
/// row into four physical lines.
#[test]
fn should_drop_code_fence_when_pre_is_inside_table_cell_and_br_in_tables_is_true() {
    let html = "<table><tr><td><pre>a\nb</pre></td></tr></table>";
    let result = convert(html, Some(cell_options(true))).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result:?}");
    assert!(
        !result.contains("```"),
        "no code fence may appear in a cell: {result:?}"
    );
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result:?}"
    );
}

/// With `br_in_tables: false` the whole-cell backstop used to mop the fence newlines into
/// spaces, yielding the degraded-but-valid `` | ``` a b ``` | ``. The fence markers are now
/// dropped outright rather than surviving as literal text.
#[test]
fn should_drop_code_fence_when_pre_is_inside_table_cell_and_br_in_tables_is_false() {
    let html = "<table><tr><td><pre>a\nb</pre></td></tr></table>";
    let result = convert(html, Some(cell_options(false))).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result:?}");
    assert!(
        !result.contains("```"),
        "no code fence may appear in a cell: {result:?}"
    );
}

/// `<pre><code>` is the common documentation form and routes through the same handler, so it
/// must degrade identically.
#[test]
fn should_drop_code_fence_when_pre_wraps_a_code_element_inside_table_cell() {
    let html = "<table><tr><td><pre><code>a\nb</code></pre></td></tr></table>";
    let result = convert(html, Some(cell_options(true))).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result:?}");
}

/// The `Indented` code-block style corrupts the row too — it has no fence, but it brackets the
/// block with blank lines and indents each line. The issue only showed the `Backticks` case;
/// this pins the other style.
#[test]
fn should_drop_indented_code_block_when_pre_is_inside_table_cell() {
    let html = "<table><tr><td><pre>a\nb</pre>tail</td></tr></table>";
    let options = ConversionOptions {
        code_block_style: CodeBlockStyle::Indented,
        br_in_tables: true,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b`<br>tail |\n| --- |\n", "actual: {result:?}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result:?}"
    );
}

/// The `Tildes` fence style must be dropped in a cell as well.
#[test]
fn should_drop_tilde_fence_when_pre_is_inside_table_cell() {
    let html = "<table><tr><td><pre>a\nb</pre></td></tr></table>";
    let options = ConversionOptions {
        code_block_style: CodeBlockStyle::Tildes,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result:?}");
    assert!(
        !result.contains("~~~"),
        "no tilde fence may appear in a cell: {result:?}"
    );
}

/// A fence's language info string is part of the dropped block syntax: with no fence to carry
/// it, emitting `rust` would leak the class name into the cell as stray text.
#[test]
fn should_not_emit_language_info_string_when_pre_is_inside_table_cell() {
    let html = "<table><tr><td><pre class=\"language-rust\">a\nb</pre></td></tr></table>";
    let result = convert(html, Some(cell_options(true))).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result:?}");
    assert!(
        !result.contains("rust"),
        "language must not leak into the cell: {result:?}"
    );
}

/// The content becomes a code span in the cell, so a backtick in it widens the span's
/// delimiters, as for inline code.
#[test]
fn should_widen_the_code_span_for_backticks_in_pre_content_inside_table_cell() {
    let html = "<table><tr><td><pre>a`b</pre></td></tr></table>";
    let result = convert(html, Some(cell_options(true))).unwrap();
    assert_eq!(result, "| ``a`b`` |\n| --- |\n", "actual: {result:?}");
}

/// The fold must hold under `Strict` too: `whitespace_mode` does not make a raw newline legal
/// between two pipes.
#[test]
fn should_drop_code_fence_when_pre_is_inside_table_cell_under_strict_whitespace_mode() {
    let options = ConversionOptions {
        br_in_tables: true,
        whitespace_mode: WhitespaceMode::Strict,
        compact_tables: true,
        ..Default::default()
    };
    let html = "<table><tr><td><pre>a\nb</pre></td></tr></table>";
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| `a b` |\n| --- |\n", "actual: {result:?}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result:?}"
    );
}

/// A `<pre>` outside any table cell must still render as a normal fenced code block, keeping
/// its newlines and language — the regression guard against the in-cell degradation leaking
/// into the ordinary code-block path.
#[test]
fn should_keep_fenced_code_block_when_pre_is_outside_table_cell() {
    let result = convert("<div><pre>a\nb</pre></div>", Some(cell_options(true))).unwrap();
    assert_eq!(result, "```\na\nb\n```\n", "actual: {result:?}");

    let with_language = convert(
        "<div><pre class=\"language-rust\">a\nb</pre></div>",
        Some(cell_options(true)),
    )
    .unwrap();
    assert_eq!(with_language, "```rust\na\nb\n```\n", "actual: {with_language:?}");
}
