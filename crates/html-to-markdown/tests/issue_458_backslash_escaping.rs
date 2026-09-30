// ~keep The inner attribute below is a crate-level Rust attribute, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #458: a source backslash in prose was emitted bare, so a
//! `CommonMark` parser reading the Markdown back consumed it as an escape trigger and
//! the character vanished. Escaping it is unconditional — not gated behind
//! `escape_misc`/`escape_ascii` — mirroring `escape_markdown_title` in
//! `converter/inline/link.rs`, which has always escaped backslashes for the same
//! reason. ~keep

use html_to_markdown_rs::ConversionOptions;

fn convert(html: &str, options: ConversionOptions) -> String {
    html_to_markdown_rs::convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn default_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        autolinks: false,
        ..Default::default()
    }
}

// ~keep ── CommonMark example 15 shape: backslash before ASCII punctuation ────────

#[test]
fn should_escape_backslash_before_punctuation_under_default_options() {
    assert_eq!(convert(r"<p>3\*4</p>", default_options()).trim(), r"3\\*4");
}

#[test]
fn should_escape_backslash_before_punctuation_in_the_tag_free_fast_path() {
    // ~keep `convert_api.rs`'s `fast_text_only` skips HTML parsing entirely when the
    // ~keep input contains no `<`, and only reached `text::escape` when one of the
    // ~keep optional escape_* flags was set — bypassing the unconditional rule.
    assert_eq!(convert(r"3\*4", default_options()).trim(), r"3\\*4");
}

#[test]
fn should_escape_backslash_before_brackets() {
    assert_eq!(convert(r"<p>a\[b\]c</p>", default_options()).trim(), r"a\\[b\\]c");
}

// ~keep ── CommonMark example 13 shape: backslash before non-punctuation ──────────

#[test]
fn should_leave_backslash_before_non_punctuation_bare() {
    assert_eq!(convert(r"<p>a\3b</p>", default_options()).trim(), r"a\3b");
}

#[test]
fn should_leave_windows_paths_untouched() {
    assert_eq!(
        convert(r"<p>C:\Users\Alice</p>", default_options()).trim(),
        r"C:\Users\Alice"
    );
}

// ~keep ── Run boundaries ──────────────────────────────────────────────────────────

#[test]
fn should_escape_backslash_at_end_of_paragraph() {
    assert_eq!(convert(r"<p>abc\</p>", default_options()).trim(), r"abc\\");
}

#[test]
fn should_escape_backslash_before_a_source_line_ending() {
    // ~keep Left bare, this is CommonMark's hard-line-break syntax on re-parse.
    assert_eq!(convert("<p>abc\\\ndef</p>", default_options()).trim(), "abc\\\\\ndef");
}

// ~keep ── Verbatim contexts are untouched ────────────────────────────────────────

#[test]
fn should_not_escape_backslash_inside_a_code_span() {
    let markdown = convert(r"<p>Use <code>a\*b</code> here.</p>", default_options());
    assert!(
        markdown.contains(r"`a\*b`"),
        "code span content must stay verbatim, got: {markdown}"
    );
}

#[test]
fn should_not_escape_backslash_inside_a_code_block() {
    let markdown = convert("<pre><code>a\\*b\nc\\d</code></pre>", default_options());
    assert!(
        markdown.contains("a\\*b") && markdown.contains("c\\d"),
        "code block content must stay verbatim, got: {markdown}"
    );
}

#[test]
fn should_not_double_escape_a_backslash_in_a_link_title() {
    // ~keep `escape_markdown_title` already escapes the title's backslash; the prose
    // ~keep rule must not run over the same bytes a second time.
    let html = r#"<p><a href="https://example.com" title="a\b">link</a></p>"#;
    assert_eq!(
        convert(html, default_options()).trim(),
        r#"[link](https://example.com "a\\b")"#
    );
}

// ~keep ── Table cells share the rule ─────────────────────────────────────────────

#[test]
fn should_escape_backslash_in_table_cell_text() {
    let html = r"
        <table>
            <thead><tr><th>Expr</th></tr></thead>
            <tbody><tr><td>3\*4</td></tr></tbody>
        </table>
    ";
    let markdown = convert(html, default_options());
    // ~keep A cell follows `escape_asterisks` like a paragraph (issue #638), so with the
    // ~keep default options only the backslash rule applies: the `\` is doubled and the
    // ~keep `*` stays as it is.
    assert!(
        markdown.contains(r"3\\*4") && !markdown.contains(r"3\\\*4"),
        "literal backslash in a table cell should be escaped, got: {markdown}"
    );
}

// ~keep ── `chomp_inline` hazard: an escaped trailing backslash must not be mistaken
// ~keep    for this crate's own `\\\n` hard-break marker. ────────────────────────────

#[test]
fn should_not_read_a_trailing_escaped_backslash_as_a_hard_break() {
    // ~keep `chomp_inline` (converter/utility/content.rs) sniffs a trailing `\` + `\n`
    // ~keep on emphasis content to find a hard break placed just inside the closing
    // ~keep `**`. Nothing follows here, so there is no trailing newline to find.
    assert_eq!(
        convert("<p><strong>abc\\</strong></p>", default_options()).trim(),
        r"**abc\\**"
    );
}

#[test]
fn should_drop_a_trailing_hard_break_after_an_escaped_backslash() {
    // ~keep The `<br>` is the last child of `<strong>`, which is itself the last child
    // ~keep of `<p>` — so the break is trailing at the block level. Issue #464: a
    // ~keep trailing `<br>` has no next line to break to and must not render at all, so
    // ~keep the `\` + `\n` marker (NewlineStyle::Backslash) that `chomp_inline` placed
    // ~keep after the escaped source backslash is stripped by the paragraph handler's
    // ~keep end-of-block pass, leaving only the escaped backslash behind.
    let options = ConversionOptions {
        newline_style: html_to_markdown_rs::options::NewlineStyle::Backslash,
        ..default_options()
    };
    assert_eq!(convert("<p><strong>abc\\<br></strong></p>", options), "**abc\\\\**\n");
}

#[test]
fn should_keep_both_an_escaped_backslash_and_a_real_hard_break() {
    // ~keep Same shape as above, but the `<br>` is no longer trailing at the block level
    // ~keep (`more` follows `</strong>` inside the same `<p>`), so the hard break is
    // ~keep real and must survive. `chomp_inline` must still strip exactly the `<br>`'s
    // ~keep two bytes and move the marker after `<strong>`'s closing `**`.
    let options = ConversionOptions {
        newline_style: html_to_markdown_rs::options::NewlineStyle::Backslash,
        ..default_options()
    };
    assert_eq!(
        convert("<p><strong>abc\\<br></strong>more</p>", options),
        "**abc\\\\**\\\nmore\n"
    );
}
