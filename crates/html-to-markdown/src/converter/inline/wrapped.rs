//! Shared emission for inline elements that wrap their trimmed content in a delimiter pair.
//!
//! Every such element has the same two edge cases, and both were originally fixed for
//! `<strong>`/`<em>` alone (issues #483 and #481) while the other handlers kept their own
//! byte-for-byte copy of the naive emission and kept the bugs:
//!
//! - A delimiter built from a repeated character (`*`, `**`, `~~`, `==`) that opens
//!   immediately after the matching close of a preceding sibling forms ONE longer
//!   delimiter run on reparse, not two spans.
//! - A body that is non-empty but entirely whitespace stands for a word separator, and a
//!   handler that only tests `!trimmed.is_empty()` drops it, joining the words either side.

use tl::{NodeHandle, Parser};

type DomContext = crate::converter::DomContext;

/// Tag names that render with the single `strong_em_symbol` italic delimiter.
///
/// `<var>` and `<dfn>` wrap their content in exactly the same one-character pair `<em>`/`<i>`
/// do, so a `<var>` opening right after an `<em>` closes forms one longer delimiter run just
/// as two `<em>`s would -- the corruption issue #483 describes does not care which tag
/// emitted each half of the run. ~keep
pub const EMPHASIS_SIBLING_TAGS: [&str; 4] = ["em", "i", "var", "dfn"];

/// Tag names that render with the doubled `strong_em_symbol` bold delimiter.
pub const STRONG_SIBLING_TAGS: [&str; 2] = ["strong", "b"];

/// The delimiter pair an inline element wraps its trimmed content in, and what it may merge
/// that pair with.
///
/// `open`/`close` are the exact strings emitted around the content. Pass `("", "")` for an
/// element that renders with no marker of its own, such as a `<strong>` nested inside another
/// `<strong>`.
///
/// `merge_symbol` is the single character `open` is built from, used to detect and merge a
/// `CommonMark` adjacency with the immediately preceding sibling's close marker (issue #483,
/// see [`merge_adjacent_emphasis`](crate::converter::merge_adjacent_emphasis)) instead of
/// opening a second, textually-adjacent delimiter run. Use `None` for a delimiter that cannot
/// form a run at all -- Djot's `{-`/`-}` and `{+`/`+}` pairs, `<mark>`'s HTML style, or an
/// arbitrary caller-configured `sub_symbol`. A `Some` whose character does not make up the
/// whole of `open` is treated the same way, so a caller cannot accidentally pop bytes that are
/// not a delimiter run.
///
/// `sibling_tag_names` lists the HTML tag names (e.g. `["strong", "b"]`) that make this
/// element's immediately preceding DOM sibling a genuine candidate for that merge. Buffer
/// content alone is ambiguous: ordinary prose can coincidentally end in a literal `*`/`**`
/// (CommonMark spec example 442, `<p>*<em>foo</em></p>` -> `**foo*`, NOT a merge into `*foo*`)
/// that is indistinguishable, byte-for-byte, from a just-emitted close marker. Requiring the
/// DOM to actually show a matching sibling element resolves the ambiguity in favor of "no
/// merge" whenever the preceding content is plain text. ~keep
#[derive(Clone, Copy)]
pub struct InlineDelimiters<'a> {
    pub open: &'a str,
    pub close: &'a str,
    pub merge_symbol: Option<char>,
    pub sibling_tag_names: &'a [&'a str],
}

/// Emit `content` wrapped in `delimiters`, handling delimiter-run adjacency with a preceding
/// sibling and a body that is non-empty but entirely whitespace.
pub fn emit_wrapped_inline(
    output: &mut String,
    content: &str,
    delimiters: &InlineDelimiters<'_>,
    node_handle: &NodeHandle,
    parser: &Parser,
    dom_ctx: &DomContext,
) {
    let InlineDelimiters {
        open,
        close,
        merge_symbol,
        sibling_tag_names,
    } = *delimiters;
    use crate::converter::utility::siblings::get_previous_sibling_tag;
    use crate::converter::{append_inline_suffix, chomp_inline, merge_adjacent_emphasis};

    let (prefix, suffix, trimmed) = chomp_inline(content);
    if content.trim().is_empty() {
        if content.is_empty() {
            return;
        }
        // ~keep issue #481: a whitespace-only body (e.g. `<i> </i>`) must contribute at
        // ~keep most one space -- `chomp_inline` above already collapsed prefix/suffix to
        // ~keep a single representation, but the buffer can already end with a real space
        // ~keep from a preceding sibling (e.g. `A <i> </i>B`), in which case even that one
        // ~keep copy must be suppressed. Mirrors `text_node.rs`'s `!output.ends_with(' ')`
        // ~keep guards, which every handler but `text_node` was originally missing.
        if !output.ends_with(' ') {
            output.push_str(prefix);
        }
        append_inline_suffix(output, suffix, false, node_handle, parser, dom_ctx);
        return;
    }

    output.push_str(prefix);
    let sibling_is_matching_tag =
        get_previous_sibling_tag(node_handle, parser, dom_ctx).is_some_and(|name| sibling_tag_names.contains(&name));
    let merged = prefix.is_empty()
        && sibling_is_matching_tag
        && merge_symbol.is_some_and(|symbol| {
            open.chars().all(|c| c == symbol) && merge_adjacent_emphasis(output, symbol, open.chars().count())
        });
    if !merged {
        output.push_str(open);
    }
    output.push_str(trimmed);
    output.push_str(close);
    append_inline_suffix(output, suffix, !trimmed.is_empty(), node_handle, parser, dom_ctx);
}

/// Emit an already-rendered inline code `span`, merging it into an immediately preceding
/// sibling code span rather than letting the two backticks sit adjacent.
///
/// `CommonMark` reads a code span's closing backtick immediately followed by the next span's
/// opening backtick as interior text, not as two spans: `` `A` `` and `` `B` `` written
/// back-to-back reparse as ONE span whose content carries both literal backticks -- issue
/// #483's delimiter-run adjacency in its backtick form. There is no `CommonMark` spelling for
/// two genuinely adjacent code spans, so the faithful choice is the merged span: it preserves
/// every character of the content, which the corrupted reparse does not.
///
/// Only a span that rendered as a plain single-backtick pair can merge. One that needed a
/// longer fence or delimiter spaces (because its own content contains backticks or edge
/// spaces) is emitted untouched rather than half-rewritten. ~keep
pub fn emit_code_span(
    span: &str,
    content: &str,
    output: &mut String,
    node_handle: &NodeHandle,
    parser: &Parser,
    dom_ctx: &DomContext,
) {
    use crate::converter::merge_adjacent_emphasis;
    use crate::converter::utility::siblings::get_previous_sibling_tag;

    let is_plain_single_backtick_pair =
        span.len() == content.len() + 2 && span.starts_with('`') && span.ends_with('`') && !content.contains('`');
    let previous_is_code_sibling = get_previous_sibling_tag(node_handle, parser, dom_ctx)
        .is_some_and(|name| matches!(name, "code" | "kbd" | "samp"));

    if is_plain_single_backtick_pair && previous_is_code_sibling && merge_adjacent_emphasis(output, '`', 1) {
        output.push_str(&span[1..]);
    } else {
        output.push_str(span);
    }
}

/// Push the single space a non-empty, all-whitespace inline body stands for.
///
/// Suppressed when the buffer already ends with a space, mirroring `text_node.rs`'s
/// `!output.ends_with(' ')` guards. ~keep
pub fn emit_whitespace_only_inline_body(content: &str, output: &mut String) {
    use crate::converter::chomp_inline;

    let (prefix, _, _) = chomp_inline(content);
    if !output.ends_with(' ') {
        output.push_str(prefix);
    }
}
