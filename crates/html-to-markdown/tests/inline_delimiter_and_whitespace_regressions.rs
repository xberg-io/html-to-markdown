#![allow(missing_docs)]

//! Regression tests for two whole-family Tier-2 inline defects that the issue #481 and
//! #483 fixes repaired for `<em>`/`<strong>` only, plus one tag that was never wired into
//! the Tier-2 walker at all.
//!
//! - **`<strike>` was invisible to Tier-2.** `converter/main.rs`'s inline dispatch arm,
//!   `dispatch_inline_handler` and `inline/semantic/mod.rs` all listed `del`/`s` without it,
//!   so `<strike>` fell through to the generic child walk and its strikethrough was dropped.
//!   Tier-1 (`tier1/tags.rs`), `types/structure_builder.rs` and `roundtrip_fixpoint.rs`'s
//!   module doc all treat `strike` as a strikethrough synonym, so this was also a
//!   Tier-1/Tier-2 divergence.
//! - **Issue #483's delimiter-run adjacency applies to every repeated-character delimiter**,
//!   not just `*`/`**`: `~~` (`del`/`s`/`strike`), `==` (`ins`, `mark`), the
//!   `strong_em_symbol` reused by `var`/`dfn`, and a code span's backtick. Verified against
//!   the comrak oracle: `~~A~~~~B~~` reparses as ONE strikethrough whose text is `A~~~~B`,
//!   and two adjacent single-backtick code spans reparse as ONE code span whose text
//!   carries the two literal backticks that used to separate them -- both corrupt.
//! - **Issue #481's whitespace-only body applies to every inline handler.** A body that is
//!   non-empty but all whitespace stands for a word separator; handlers that tested
//!   `!trimmed.is_empty()` with no `else` branch dropped it, joining the words either side.

use html_to_markdown_rs::convert;
use html_to_markdown_rs::options::ConversionOptions;

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
        .trim()
        .to_owned()
}

fn content_with(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
        .trim()
        .to_owned()
}

// ~keep ── <strike> is a strikethrough synonym, like <del> and <s> ────────────────────────

#[test]
fn should_render_strike_as_strikethrough_like_del_and_s() {
    assert_eq!(content("<p><strike>gone</strike></p>"), "~~gone~~");
    assert_eq!(content("<p><del>gone</del></p>"), "~~gone~~");
    assert_eq!(content("<p><s>gone</s></p>"), "~~gone~~");
}

#[test]
fn should_suppress_strike_markers_inside_a_code_span() {
    // ~keep Mirrors <del>/<s>: handle_strikethrough emits no `~~` under `in_code`.
    assert_eq!(content("<p><code>a<strike>b</strike></code></p>"), "`ab`");
}

#[test]
fn should_render_strike_as_djot_when_the_output_format_is_djot() {
    let options = ConversionOptions {
        output_format: html_to_markdown_rs::OutputFormat::Djot,
        ..ConversionOptions::default()
    };
    assert_eq!(content_with("<p><strike>gone</strike></p>", options), "{-gone-}");
}
