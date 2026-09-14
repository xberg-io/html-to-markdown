// ~keep The inner attribute below is a crate-level Rust attribute, not a shell shebang.
#![allow(missing_docs)]

//! A `<br>` inside a code context must not put a `newline_style` marker INSIDE a code span's
//! backticks or a fenced block's content.
//!
//! A code span (or fenced block) reproduces its content literally, so a `newline_style` marker
//! is not syntax there -- it is content. `CommonMark` treats a line ending inside a code span
//! as a space and gives it no hard-break meaning at all
//! (<https://spec.commonmark.org/spec#code-spans>), and a fenced block would carry the marker
//! into the user's code verbatim.
//!
//! The fix (issue #487) is not to suppress the marker: it is to move it. A `<br>` inside a code
//! SPAN splits the span in two -- one backtick pair per side -- and emits the marker BETWEEN
//! them, outside both backtick pairs, where it is syntax rather than span content. This mirrors
//! how `<b>`/`<i>` already turn an internal `<br>` into a hard break between two delimiter
//! pairs (see `br_in_inline_test.rs`). A `<br>` inside a fenced code BLOCK is unaffected: block
//! content keeps every line ending verbatim, same as before.
//!
//! A `<br>` inside a code span that also sits inside a table cell or a heading is different
//! again: neither context can carry a hard break at all (same rule `line_break.rs`'s
//! `in_table_cell` branch already applies to ordinary text), so there it folds to a single
//! space instead of splitting -- see `issue_455_code_in_table_cell_test.rs` and
//! `br_inside_code_spans_context_folding_test.rs`.

use html_to_markdown_rs::{ConversionOptions, NewlineStyle};

fn convert(html: &str, style: NewlineStyle) -> String {
    html_to_markdown_rs::convert(
        html,
        Some(ConversionOptions {
            newline_style: style,
            ..Default::default()
        }),
    )
    .expect("conversion should succeed")
    .content
    .unwrap_or_default()
}

/// Asserts that no `\` appears between a `` ` `` and its matching closing `` ` `` -- i.e.
/// inside a code span's content -- anywhere in `out`. Assumes (true of every fixture this test
/// exercises) that no span's own content contains a literal backtick, so a plain alternating
/// split on `` ` `` correctly tracks "inside a span" vs "outside one".
fn assert_no_backslash_inside_a_code_spans_backticks(out: &str, html: &str) {
    let mut inside_span = false;
    for part in out.split('`') {
        if inside_span {
            assert!(
                !part.contains('\\'),
                "backslash marker leaked inside a code span's backticks for {html}: {out:?}"
            );
        }
        inside_span = !inside_span;
    }
}

#[test]
fn should_not_put_a_backslash_marker_inside_a_code_spans_backticks() {
    // ~keep Narrowed from "no `\` anywhere in the document" (the pre-#487 assertion, which
    // ~keep pinned marker SUPPRESSION) to "no `\` inside a span's own backticks" (the #487
    // ~keep design, which moves the marker outside the span instead): a `\` between two
    // ~keep split spans, outside both backtick pairs, is now correct and expected.
    for html in [
        "<p><code>A<br>B</code></p>",
        "<code>A<br>B</code>",
        "<p><code>A<br>B</code>C</p>",
        "<p><kbd>A<br>B</kbd></p>",
        "<p><samp>A<br>B</samp></p>",
    ] {
        let out = convert(html, NewlineStyle::Backslash);
        assert_no_backslash_inside_a_code_spans_backticks(&out, html);
    }
}

#[test]
fn should_split_into_two_backslash_joined_spans_under_backslash_style() {
    // ~keep The positive counterpart to the narrowed test above: the marker is not merely
    // ~keep absent from inside the backticks, it is present, correctly, between two spans.
    assert_eq!(
        convert("<p><code>A<br>B</code></p>", NewlineStyle::Backslash),
        "`A`\\\n`B`\n"
    );
    assert_eq!(
        convert("<p><kbd>A<br>B</kbd></p>", NewlineStyle::Backslash),
        "`A`\\\n`B`\n"
    );
    assert_eq!(
        convert("<p><samp>A<br>B</samp></p>", NewlineStyle::Backslash),
        "`A`\\\n`B`\n"
    );
}

#[test]
fn should_not_put_a_marker_inside_a_fenced_code_block() {
    // ~keep The worst of the set: the fence reproduces its content verbatim, so a marker
    // ~keep here is not invisible under either style -- it is a character in the user's code.
    assert_eq!(
        convert("<pre><code>A<br>B</code></pre>", NewlineStyle::Backslash),
        "```\nA\nB\n```\n"
    );
    assert_eq!(
        convert("<pre><code>A<br>B</code></pre>", NewlineStyle::Spaces),
        "```\nA\nB\n```\n"
    );
}

#[test]
fn should_emit_the_same_code_span_content_under_both_newline_styles_with_a_different_separator() {
    // ~keep `newline_style` selects hard-break *syntax*, and a split code span's segments now
    // ~keep carry that syntax BETWEEN them (issue #487) rather than never seeing it at all --
    // ~keep so, unlike before, the two styles no longer have to agree byte for byte. What must
    // ~keep still agree is the CODE CONTENT of each segment: strip each style's own separator
    // ~keep first, then compare what is left.
    for html in [
        "<p><code>A<br>B</code></p>",
        "<p><code>A<br><br>B</code></p>",
        "<blockquote><p><code>A<br>B</code></p></blockquote>",
    ] {
        let spaces_out = convert(html, NewlineStyle::Spaces);
        let backslash_out = convert(html, NewlineStyle::Backslash);
        let spaces_segments: Vec<&str> = spaces_out.split("  \n").collect();
        let backslash_segments: Vec<&str> = backslash_out.split("\\\n").collect();
        assert_eq!(
            spaces_segments, backslash_segments,
            "segment content disagrees between newline styles for {html}: spaces={spaces_out:?} backslash={backslash_out:?}"
        );
    }

    // ~keep A fenced block's content has no separator to strip -- both styles must still
    // ~keep agree byte for byte, exactly as before (`should_not_put_a_marker_inside_a_fenced_code_block`
    // ~keep already pins the exact value; this only re-confirms the two styles match).
    let html = "<pre><code>A<br>B</code></pre>";
    assert_eq!(
        convert(html, NewlineStyle::Backslash),
        convert(html, NewlineStyle::Spaces)
    );
}

#[test]
fn should_still_emit_a_real_break_between_two_code_spans() {
    // ~keep The counterweight: the <br> is BETWEEN spans, not inside one, so it is an
    // ~keep ordinary hard break and must keep its marker.
    let out = convert("<p><code>a</code><br><code>b</code></p>", NewlineStyle::Backslash);
    assert_eq!(out, "`a`\\\n`b`\n");
}
