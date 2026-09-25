//! URL-encoding and destination-rendering helpers for Markdown links.
//!
//! Split out of `link.rs` to stay under the 1000-line quality gate; these
//! percent-encoding/title-escaping concerns evolve independently of the
//! DOM-walking handler that remains there.

use crate::options::ConversionOptions;
use std::borrow::Cow;

/// Check whether `href` begins with a syntactically valid RFC 3986 URI scheme.
///
/// A scheme matches `ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )` followed by `:`.
/// Bare paths and filenames (e.g. `foobar.png`) fail this check and must be rendered
/// as `[text](href)` rather than as autolinks per GFM §6.5.
#[must_use]
pub fn has_uri_scheme(href: &str) -> bool {
    let mut bytes = href.bytes();
    match bytes.next() {
        Some(b) if b.is_ascii_alphabetic() => {}
        _ => return false,
    }
    for b in bytes {
        match b {
            b':' => return true,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'-' | b'.' => {}
            _ => return false,
        }
    }
    false
}

/// Percent-encode a URL destination.
///
/// Encodes every character that is not an RFC 3986 unreserved character (`A-Z`, `a-z`, `0-9`,
/// `-`, `_`, `.`, `~`) or a forward slash (`/`). This produces a destination that all
/// Markdown parsers handle correctly even when the original URL contains `<`, `>`, spaces,
/// or parentheses.
#[must_use]
pub fn percent_encode_url(url: &str) -> String {
    let mut encoded = String::with_capacity(url.len() * 2);
    for byte in url.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                encoded.push(byte as char);
            }
            other => {
                encoded.push('%');
                let hi = char::from_digit(u32::from(other >> 4), 16)
                    .unwrap_or('0')
                    .to_ascii_uppercase();
                let lo = char::from_digit(u32::from(other & 0x0f), 16)
                    .unwrap_or('0')
                    .to_ascii_uppercase();
                encoded.push(hi);
                encoded.push(lo);
            }
        }
    }
    encoded
}

/// Percent-encode only the non-ASCII bytes of a URL destination, leaving every ASCII byte --
/// including reserved/punctuation characters such as `:`, `?`, `&`, `=`, `(`, `)`, and `\` --
/// untouched.
///
/// RFC 3986 defines a URI's grammar as ASCII-only; a raw byte >= 0x80 is not valid URI syntax
/// under any encoding style, `Angle` included. This is also what every CommonMark-compliant
/// HTML renderer does when serializing a link destination into an `href` attribute: the
/// spec's own worked examples encode it this way (`[foo](/f\u{f6}\u{f6})` renders as
/// `<a href="/f%C3%B6%C3%B6">` per `commonmark-spec`'s example for entity references), and
/// `cmark`, `commonmark.js`, and `comrak` all implement the same `houdini_escape_href`-style
/// safe-character set. Encoding it here is therefore standards-driven, not an accommodation
/// for one downstream renderer -- unlike [`percent_encode_url`], it never touches an ASCII
/// byte, so it cannot interact with the balanced-parens / unbalanced-parens escaping below or
/// with the documented backslash tradeoff in [`append_url_destination`].
#[must_use]
pub(super) fn percent_encode_non_ascii(url: &str) -> std::borrow::Cow<'_, str> {
    if url.is_ascii() {
        return std::borrow::Cow::Borrowed(url);
    }
    let mut encoded = String::with_capacity(url.len() + 8);
    for byte in url.bytes() {
        if byte.is_ascii() {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            let hi = char::from_digit(u32::from(byte >> 4), 16)
                .unwrap_or('0')
                .to_ascii_uppercase();
            let lo = char::from_digit(u32::from(byte & 0x0f), 16)
                .unwrap_or('0')
                .to_ascii_uppercase();
            encoded.push(hi);
            encoded.push(lo);
        }
    }
    std::borrow::Cow::Owned(encoded)
}

/// Check whether every `)` in `href` is matched by a preceding `(`, and every `(` is closed.
///
/// A raw (non-bracketed) Markdown link destination may contain parentheses only if they form a
/// properly nested, balanced pair — a plain count of `(` versus `)` is not sufficient, since e.g.
/// `")("` has equal counts but is not balanced (CommonMark 6.3). Unbalanced parentheses must be
/// backslash-escaped or the destination must be wrapped in angle brackets.
#[must_use]
pub(super) fn parens_are_balanced(href: &str) -> bool {
    let mut depth: i32 = 0;
    for c in href.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}

/// Escape a literal `\` in a raw (non-bracketed) link destination when the byte that will
/// immediately follow it is ASCII punctuation.
///
/// A raw `\` followed by ASCII punctuation is a CommonMark backslash escape (6.1): a
/// compliant parser consumes the `\` and keeps only the punctuation character, silently
/// dropping a byte that was never meant as an escape. For a `\` that is not the last
/// character of `dest`, that next byte is another character still inside `dest` -- this is
/// the CommonMark spec fixpoint example 631 defect: `href="\*"` emitted verbatim as a bare
/// destination `\*` reparses as destination `*`, permanently losing the backslash. Doubling
/// the `\` (`\\*`) makes it reparse back to the original two bytes.
///
/// For a `\` that *is* the last character of `dest`, the next byte is whatever
/// `append_url_destination`'s caller emits right after the destination closes: a literal
/// space before an opening title quote, or the closing `)` when no title follows --
/// `title_follows` tells this function which. A space is not ASCII punctuation, so a
/// trailing `\` before a title is left alone (audit #24's
/// `append_markdown_link_escapes_a_trailing_backslash_in_default_title_...` pins exactly
/// this case). `)` *is* ASCII punctuation, so a trailing `\` with no title must be doubled or
/// `\)` reparses as an escaped paren: the destination never closes, and the whole link
/// degrades to literal bracket text (`href="x\"` with no title emits `(x\)`, which reparses
/// as plain text `[t](x)`, losing both the destination and the link itself -- worse than
/// example 631's single dropped byte).
#[must_use]
pub(super) fn escape_ambiguous_destination_backslashes(dest: &str, title_follows: bool) -> std::borrow::Cow<'_, str> {
    if !dest.contains('\\') {
        return std::borrow::Cow::Borrowed(dest);
    }
    let chars: Vec<char> = dest.chars().collect();
    let last_index = chars.len() - 1;
    let mut escaped = String::with_capacity(dest.len() + 4);
    for (index, &c) in chars.iter().enumerate() {
        if c == '\\' {
            let next_is_punctuation = if index == last_index {
                !title_follows
            } else {
                chars.get(index + 1).is_some_and(char::is_ascii_punctuation)
            };
            if next_is_punctuation {
                escaped.push('\\');
            }
            escaped.push('\\');
        } else {
            escaped.push(c);
        }
    }
    std::borrow::Cow::Owned(escaped)
}

/// Find the end of an entity-shaped reference (`#?[A-Za-z0-9]{1,32};`) starting right after
/// the `&` at `amp_index`, returning the byte offset just past the terminating `;`.
///
/// Matches the syntax of both a named reference (`plus`) and a numeric one (decimal `#43` or
/// hex `#x2B`) -- `[A-Za-z0-9]` covers hex digits and the `x` marker generically, so one scan
/// serves both. Returns `None` when `amp_index` is not followed by this shape at all (no
/// digit/letter run, or no terminating `;`), which is the common case for a bare `&`.
pub(super) fn entity_reference_end(bytes: &[u8], amp_index: usize) -> Option<usize> {
    const MAX_REFERENCE_NAME_LEN: usize = 32;

    let mut index = amp_index + 1;
    if bytes.get(index) == Some(&b'#') {
        index += 1;
    }
    let name_start = index;
    while index < bytes.len() && bytes[index].is_ascii_alphanumeric() && index - name_start < MAX_REFERENCE_NAME_LEN {
        index += 1;
    }
    if index == name_start {
        return None;
    }
    (bytes.get(index) == Some(&b';')).then_some(index + 1)
}

/// Escape a literal `&` that starts a byte sequence CommonMark would decode as an entity or
/// numeric character reference (`&plus;`, `&#43;`, `&#x2B;`, ...), so a destination or title
/// string that already went through [`crate::text::decode_html_entities`] round-trips
/// byte-for-byte through a CommonMark parser instead of being decoded a second time.
///
/// Only escapes references [`html_escape::decode_html_entities`] -- the same decoder used by
/// `crate::text::decode_html_entities` -- actually changes: `&foo;` where `foo` is not a
/// recognized reference name is left alone, matching the existing pinned behavior for
/// `?a&b` (no terminating `;`, never a candidate at all).
#[must_use]
pub fn escape_entity_ampersands(text: &str) -> Cow<'_, str> {
    if !text.contains('&') {
        return Cow::Borrowed(text);
    }

    let bytes = text.as_bytes();
    let mut result = String::with_capacity(text.len());
    let mut copied_up_to = 0;
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'&' {
            if let Some(reference_end) = entity_reference_end(bytes, index) {
                let candidate = &text[index..reference_end];
                if html_escape::decode_html_entities(candidate) != candidate {
                    result.push_str(&text[copied_up_to..index]);
                    result.push_str("&amp;");
                    result.push_str(&text[index + 1..reference_end]);
                    copied_up_to = reference_end;
                    index = reference_end;
                    continue;
                }
            }
        }
        index += 1;
    }

    if copied_up_to == 0 {
        return Cow::Borrowed(text);
    }
    result.push_str(&text[copied_up_to..]);
    Cow::Owned(result)
}

/// Escape a Markdown title's backslashes and double quotes for interpolation into a
/// double-quoted title `"..."`.
///
/// Backslashes are escaped *before* quotes: a title ending in a literal `\` would otherwise
/// make the following delimiter's `\"` read as an escaped quote instead of the closing
/// delimiter, letting the title (and the destination that follows) run into whatever content
/// comes next in the document.
///
/// [`escape_entity_ampersands`] runs first so the two transforms compose: an entity-shaped `&`
/// becomes `&amp;` (no backslash or quote involved), and any literal `\`/`"` already in the
/// title is still escaped afterward.
#[must_use]
pub fn escape_markdown_title(text: &str) -> std::borrow::Cow<'_, str> {
    let text = escape_entity_ampersands(text);
    if !text.contains('\\') && !text.contains('"') {
        return text;
    }
    std::borrow::Cow::Owned(text.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Wrap `dest` in `<...>`, escaping the three bytes that would otherwise terminate the wrap
/// early or merge with an already-escaped character: `\`, `<`, `>` (CommonMark 6.3). A literal
/// line ending is not permitted inside `<...>` at all -- not even escaped -- so it is folded to
/// a space rather than passed through. Split out of [`append_url_destination`] to keep that
/// function under the line-count quality gate.
fn push_angle_wrapped_destination(output: &mut String, dest: &str) {
    output.push('<');
    for c in dest.chars() {
        match c {
            '\\' => output.push_str("\\\\"),
            '<' => output.push_str("\\<"),
            '>' => output.push_str("\\>"),
            '\n' => output.push(' '),
            other => output.push(other),
        }
    }
    output.push('>');
}

/// Append a Markdown link destination (the `(...)` portion, without the enclosing parens) to
/// `output`, honoring `url_escape_style`.
///
/// Shared by [`append_markdown_link`] (for `<a href>`) and the image/graphic handlers, so a
/// destination gets the same treatment — empty-destination handling, percent-encoding,
/// space-triggered angle-bracket wrapping with backslash-safe `<`/`>` escaping inside it, and
/// paren-balance escaping — no matter which element produced it.
///
/// `title_follows` must be `true` exactly when the caller is about to emit a title
/// (` "..."`) after this destination, and `false` when the closing `)` comes next -- see
/// [`escape_ambiguous_destination_backslashes`] for why the balanced-parens arm needs it.
pub fn append_url_destination(
    output: &mut String,
    dest: &str,
    url_escape_style: crate::options::validation::UrlEscapeStyle,
    title_follows: bool,
) {
    if dest.is_empty() {
        output.push_str("<>");
        return;
    }
    if url_escape_style == crate::options::validation::UrlEscapeStyle::Percent {
        let encoded = percent_encode_url(dest);
        output.push_str(&encoded);
        return;
    }

    // ~keep Applied up front, before the space/newline and paren-balance branches below: a
    // ~keep raw non-ASCII byte is never valid URI syntax (RFC 3986), so encoding it is correct
    // ~keep regardless of which branch handles the rest of `dest`, and it changes nothing
    // ~keep else -- see `percent_encode_non_ascii`'s doc comment for why this is
    // ~keep standards-driven rather than a downstream-renderer accommodation, and for why it
    // ~keep cannot interact with the backslash tradeoff in the arms below (it never touches an
    // ~keep ASCII byte, so it commutes with every other transform in this function).
    //
    // ~keep Skipped for a same-document fragment (`#...`): a fragment must byte-match the
    // ~keep target element's `id`, which the id-generating tool controls, not this converter
    // ~keep -- real-world generators (verified against a GitHub-rendered fixture,
    // ~keep `gh-143-links-wordwrap.html`'s `#walk-up-\u{fe0e}-and-walk-down-\u{fe0e}`) commonly
    // ~keep leave non-ASCII characters raw in anchor hrefs. Encoding only the href here, while
    // ~keep the id it must match stays raw, breaks the link rather than normalizing it.
    let dest = if dest.starts_with('#') {
        std::borrow::Cow::Borrowed(dest)
    } else {
        percent_encode_non_ascii(dest)
    };
    let dest = dest.as_ref();

    // ~keep Applied before the space/paren branches below so the two compose: this only ever
    // ~keep turns an `&` into `&amp;`, which introduces no space and no paren, so it cannot
    // ~keep change which of the three branches below fires -- see `escape_entity_ampersands`'s
    // ~keep doc comment for why an entity-shaped `&` must not reach a CommonMark parser raw.
    let dest = escape_entity_ampersands(dest);
    let dest = dest.as_ref();

    if dest.contains(' ') || dest.contains('\n') {
        push_angle_wrapped_destination(output, dest);
    } else if parens_are_balanced(dest) {
        // ~keep See `escape_ambiguous_destination_backslashes` for the full rationale: a `\`
        // ~keep followed by ASCII punctuation -- inside `dest`, or (via `title_follows`) the
        // ~keep `)`/space this call emits right after -- is doubled so a compliant reparse
        // ~keep cannot consume it as a CommonMark backslash escape. Audit #24's original
        // ~keep "this call does not know the next byte" premise no longer holds: the caller
        // ~keep always knows whether a title follows before it calls this function.
        output.push_str(&escape_ambiguous_destination_backslashes(dest, title_follows));
    } else {
        // ~keep A literal backslash immediately preceding the `)`/`(` this arm is about to
        // ~keep escape would otherwise merge with that escape and un-escape it on reparse, so
        // ~keep it must be doubled first -- this arm, unlike the balanced-parens one above,
        // ~keep controls every byte that follows each backslash in `dest` and can decide
        // ~keep safely.
        let dest = dest.replace('\\', "\\\\");
        let escaped_dest = dest.replace('(', "\\(").replace(')', "\\)");
        output.push_str(&escaped_dest);
    }
}

/// Format and append a Markdown link to the output string.
///
/// Generates the link syntax: `[label](href "title")`
/// Handles special cases:
/// - Empty href renders as `[label]()`
/// - With `UrlEscapeStyle::Angle` (default): hrefs with spaces/newlines get wrapped in angle
///   brackets: `[label](<URL with spaces>)`
/// - With `UrlEscapeStyle::Percent`: every non-unreserved character is percent-encoded
/// - Unbalanced parentheses in href get escaped when using `Angle` style
/// - Titles are wrapped in quotes and quotes inside are escaped
/// - When `default_title` option is true and `raw_text` equals href, adds href as title
///
/// # Arguments
/// * `output` - Output buffer to append the link to
/// * `label` - The link text (already escaped)
/// * `href` - The URL/destination
/// * `title` - Optional link title attribute
/// * `raw_text` - Original unprocessed text (for `default_title` option)
/// * `options` - Conversion options
pub fn append_markdown_link(
    output: &mut String,
    label: &str,
    href: &str,
    title: Option<&str>,
    raw_text: &str,
    options: &ConversionOptions,
    reference_collector: Option<&crate::converter::reference_collector::ReferenceCollectorHandle>,
) {
    if options.link_style == crate::options::validation::LinkStyle::Reference && !href.is_empty() {
        if let Some(collector) = reference_collector {
            let ref_num = collector.borrow_mut().get_or_insert(href, title);
            output.push('[');
            output.push_str(label);
            output.push_str("][");
            output.push_str(&ref_num.to_string());
            output.push(']');
            return;
        }
    }

    output.push('[');
    output.push_str(label);
    output.push_str("](");

    let title_follows = title.is_some() || (options.default_title && raw_text == href);
    append_url_destination(output, href, options.url_escape_style, title_follows);

    if let Some(title_text) = title {
        output.push_str(" \"");
        output.push_str(&escape_markdown_title(title_text));
        output.push('"');
    } else if options.default_title && raw_text == href {
        output.push_str(" \"");
        output.push_str(&escape_markdown_title(href));
        output.push('"');
    }

    output.push(')');
}
