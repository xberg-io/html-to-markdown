use std::borrow::Cow;

/// Outcome of scanning one `<...>` tag starting at `tag_start` (the index of `<`), for
/// `normalize_unclosed_list_items`.
enum ListTagScan {
    /// A named tag was fully parsed. `name_start..name_end` bounds the tag name (captured
    /// before any attributes); `idx` is the position just past its `>`, or `bytes.len()` if the
    /// tag is unterminated after the name.
    Tag {
        is_close: bool,
        name_start: usize,
        name_end: usize,
        idx: usize,
    },
    /// The tag had no name (e.g. `<>` or `</>`); `idx` is where scanning should resume.
    Empty { idx: usize },
    /// The input ended before the tag could be parsed; scanning must stop entirely.
    Truncated,
}

/// Advance `*idx`/`*in_comment` past a `<!--`/`-->` boundary, or past the current byte if
/// already inside a comment. Returns `true` when the caller's scan loop should `continue`
/// immediately (the position was consumed as comment text or a comment boundary), `false` when
/// `bytes[*idx]` is unrelated to comment tracking and `*idx` is unchanged.
///
/// Extracted from `normalize_unclosed_list_items`'s main scan loop — identical comment
/// state-machine, unchanged.
const fn advance_past_comment(bytes: &[u8], idx: &mut usize, len: usize, in_comment: &mut bool) -> bool {
    let b = bytes[*idx];

    if *in_comment {
        if b == b'-' && *idx + 2 < len && bytes[*idx + 1] == b'-' && bytes[*idx + 2] == b'>' {
            *in_comment = false;
            *idx += 3;
        } else {
            *idx += 1;
        }
        return true;
    }

    if b == b'<' && *idx + 3 < len && bytes[*idx + 1] == b'!' && bytes[*idx + 2] == b'-' && bytes[*idx + 3] == b'-' {
        *in_comment = true;
        *idx += 4;
        return true;
    }

    false
}

/// Scan one tag for `normalize_unclosed_list_items`, starting at `tag_start` (the index of `<`).
///
/// Extracted from `normalize_unclosed_list_items`'s main scan loop — identical open/close-slash,
/// whitespace-skip, name-scan, and quote-aware attribute scan, unchanged.
const fn scan_list_tag(bytes: &[u8], tag_start: usize, len: usize) -> ListTagScan {
    let mut idx = tag_start + 1;
    if idx >= len {
        return ListTagScan::Truncated;
    }

    let is_close = bytes[idx] == b'/';
    if is_close {
        idx += 1;
        if idx >= len {
            return ListTagScan::Truncated;
        }
    }

    while idx < len && bytes[idx].is_ascii_whitespace() {
        idx += 1;
    }

    let name_start = idx;
    while idx < len {
        let ch = bytes[idx];
        if ch == b'>' || ch == b'/' || ch.is_ascii_whitespace() {
            break;
        }
        idx += 1;
    }
    let name_end = idx;
    if name_end == name_start {
        return ListTagScan::Empty { idx };
    }

    let mut in_single_quote = false;
    let mut in_double_quote = false;
    while idx < len {
        match bytes[idx] {
            b'\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                idx += 1;
            }
            b'"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                idx += 1;
            }
            b'>' if !in_single_quote && !in_double_quote => {
                idx += 1;
                break;
            }
            _ => {
                idx += 1;
            }
        }
    }

    ListTagScan::Tag {
        is_close,
        name_start,
        name_end,
        idx,
    }
}

/// Whether `name_bytes` names an HTML element whose body is raw/verbatim text — `<pre>`,
/// `<code>`, `<script>`, or `<style>` — inside which list-item auto-closing must not run.
///
/// Extracted from `normalize_unclosed_list_items`'s main scan loop — identical tag-name checks,
/// unchanged.
fn is_verbatim_tag_name(name_bytes: &[u8]) -> bool {
    name_bytes.eq_ignore_ascii_case(b"pre")
        || name_bytes.eq_ignore_ascii_case(b"code")
        || name_bytes.eq_ignore_ascii_case(b"script")
        || name_bytes.eq_ignore_ascii_case(b"style")
}

/// The closing tag text for an auto-closed list-item element name (`"li"`, `"dt"`, or `"dd"`).
///
/// Extracted from `normalize_unclosed_list_items` — identical lookup, unchanged.
fn list_item_close_tag(item: &'static str) -> &'static str {
    match item {
        "li" => "</li>",
        "dt" => "</dt>",
        "dd" => "</dd>",
        _ => unreachable!(),
    }
}

/// Append `&input[*last_flush..pos]` followed by `close_tag` to the lazily-allocated `*output`,
/// then advance `*last_flush` to `pos`.
///
/// Extracted from `normalize_unclosed_list_items`'s local `emit_close_before!` macro as a plain
/// function — identical operations and order, unchanged.
fn emit_close_before(
    input: &str,
    len: usize,
    last_flush: &mut usize,
    output: &mut Option<String>,
    pos: usize,
    close_tag: &str,
) {
    let out = output.get_or_insert_with(|| String::with_capacity(len + 64));
    out.push_str(&input[*last_flush..pos]);
    out.push_str(close_tag);
    *last_flush = pos;
}

/// Update the open-list-item state machine for one non-verbatim tag and emit an auto-close for
/// a still-open `<li>`/`<dt>`/`<dd>` where HTML allows omitting the end tag.
///
/// Extracted from `normalize_unclosed_list_items`'s main scan loop — identical container/
/// list-item bookkeeping and close-tag emission, unchanged.
struct ListItemTag<'a> {
    input: &'a str,
    len: usize,
    tag_start: usize,
    is_close: bool,
    name_bytes: &'a [u8],
}

impl<'a> ListItemTag<'a> {
    const fn new(input: &'a str, tag_start: usize, is_close: bool, name_bytes: &'a [u8]) -> Self {
        Self {
            input,
            len: input.len(),
            tag_start,
            is_close,
            name_bytes,
        }
    }
}

struct ListItemState<'a> {
    open_item: &'a mut Option<&'static str>,
    list_stack: &'a mut Vec<Option<&'static str>>,
    last_flush: &'a mut usize,
    output: &'a mut Option<String>,
}

impl<'a> ListItemState<'a> {
    const fn new(
        open_item: &'a mut Option<&'static str>,
        list_stack: &'a mut Vec<Option<&'static str>>,
        last_flush: &'a mut usize,
        output: &'a mut Option<String>,
    ) -> Self {
        Self {
            open_item,
            list_stack,
            last_flush,
            output,
        }
    }
}

fn apply_list_item_tag(tag: ListItemTag<'_>, state: ListItemState<'_>) {
    let is_list_container = tag.name_bytes.eq_ignore_ascii_case(b"ul")
        || tag.name_bytes.eq_ignore_ascii_case(b"ol")
        || tag.name_bytes.eq_ignore_ascii_case(b"dl");

    let is_li = tag.name_bytes.eq_ignore_ascii_case(b"li");
    let is_def_term = tag.name_bytes.eq_ignore_ascii_case(b"dt");
    let is_def_desc = tag.name_bytes.eq_ignore_ascii_case(b"dd");
    let is_list_item = is_li || is_def_term || is_def_desc;

    if tag.is_close {
        if is_list_container {
            if let Some(item) = state.open_item.take() {
                emit_close_before(
                    tag.input,
                    tag.len,
                    state.last_flush,
                    state.output,
                    tag.tag_start,
                    list_item_close_tag(item),
                );
            }
            *state.open_item = state.list_stack.pop().unwrap_or(None);
        } else if is_list_item {
            *state.open_item = None;
        }
        return;
    }

    if is_list_container {
        state.list_stack.push(state.open_item.take());
    } else if is_list_item {
        let item_name: &'static str = if is_li {
            "li"
        } else if is_def_term {
            "dt"
        } else {
            "dd"
        };

        if let Some(prev_item) = state.open_item.replace(item_name) {
            emit_close_before(
                tag.input,
                tag.len,
                state.last_flush,
                state.output,
                tag.tag_start,
                list_item_close_tag(prev_item),
            );
        }
    }
}

/// Close implicitly-terminated list-item elements that `tl` would otherwise
/// absorb as deep children, causing stack overflows on large documents.
///
/// The HTML5 parsing spec (§13.2.6.4.7 "in body" insertion mode) states that
/// an open `<li>`, `<dt>`, or `<dd>` tag is *implicitly closed* when:
///
/// - Another `<li>` / `<dt>` / `<dd>` open tag is encountered, or
/// - The closing `</ul>`, `</ol>`, or `</dl>` tag is reached.
///
/// The `tl` parser is not a full HTML5 parser; it does not apply implicit
/// closure rules.  When it encounters `<li>content<li>more` it treats the
/// second `<li>` as a child of the first, producing a linear chain of depth
/// equal to the number of items.  A list with 400 unclosed `<li>` items
/// causes `walk_node` to recurse 400 levels deep; with 467 such lists in a
/// single document (curl.se/changes.html) the cumulative stack depth triggers
/// an OS-level stack overflow.
///
/// This function rewrites the source HTML in one linear pass before `tl` sees
/// it, inserting the missing `</li>`, `</dt>`, and `</dd>` close tags exactly
/// where the HTML5 spec says they belong.
///
/// # Scope
///
/// Only the three list-item element types are handled here; `<p>` and other
/// auto-closing block elements are intentionally left to the existing
/// `has_inline_block_misnest` → `repair_with_html5ever` path.
/// Whether `bytes` holds any tag that could open or close a list item, which is the only
/// reason to run the rewrite scan at all.
///
/// Extracted from `normalize_unclosed_list_items`' early-exit guard — identical window scan,
/// unchanged.
fn contains_list_item_tag(bytes: &[u8], len: usize) -> bool {
    len >= 4
        && bytes
            .windows(3)
            .any(|w| w.eq_ignore_ascii_case(b"<li") || w.eq_ignore_ascii_case(b"<dt") || w.eq_ignore_ascii_case(b"<dd"))
}

/// Track `<pre>`/`<code>` nesting depth, whose contents are verbatim and must never be rewritten.
///
/// Extracted from `normalize_unclosed_list_items`' scan loop — identical saturating arithmetic,
/// unchanged.
const fn update_verbatim_depth(depth: &mut usize, is_close: bool) {
    *depth = if is_close { depth.saturating_sub(1) } else { *depth + 1 };
}

pub fn normalize_unclosed_list_items(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    if !contains_list_item_tag(bytes, len) {
        return Cow::Borrowed(input);
    }

    let mut open_item: Option<&'static str> = None;
    let mut list_stack: Vec<Option<&'static str>> = Vec::new();
    let mut in_pre_or_code: usize = 0;
    let mut in_comment = false;
    let mut idx = 0usize;
    let mut last_flush = 0usize;
    let mut output: Option<String> = None;

    while idx < len {
        if advance_past_comment(bytes, &mut idx, len, &mut in_comment) {
            continue;
        }

        if bytes[idx] != b'<' {
            idx += 1;
            continue;
        }

        let tag_start = idx;
        let (is_close, name_start, name_end) = match scan_list_tag(bytes, tag_start, len) {
            ListTagScan::Truncated => break,
            ListTagScan::Empty { idx: new_idx } => {
                idx = new_idx;
                continue;
            }
            ListTagScan::Tag {
                is_close,
                name_start,
                name_end,
                idx: new_idx,
            } => {
                idx = new_idx;
                (is_close, name_start, name_end)
            }
        };
        let name_bytes = &bytes[name_start..name_end];

        if is_verbatim_tag_name(name_bytes) {
            update_verbatim_depth(&mut in_pre_or_code, is_close);
            continue;
        }
        if in_pre_or_code > 0 {
            continue;
        }

        apply_list_item_tag(
            ListItemTag::new(input, tag_start, is_close, name_bytes),
            ListItemState::new(&mut open_item, &mut list_stack, &mut last_flush, &mut output),
        );
    }

    if let Some(item) = open_item.take() {
        emit_close_before(input, len, &mut last_flush, &mut output, len, list_item_close_tag(item));
    }

    match output {
        Some(mut out) => {
            if last_flush < len {
                out.push_str(&input[last_flush..]);
            }
            Cow::Owned(out)
        }
        None => Cow::Borrowed(input),
    }
}
