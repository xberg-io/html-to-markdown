//! Single-pass byte scanner that cleans HTML and emits signals
//! consumed by the tier-1/tier-2 router (added in M2).

use std::borrow::Cow;
use std::ops::Range;
use std::str;

/// Signals captured during the single prescan pass.
#[derive(Debug, Default, Clone)]
pub struct PrescanReport {
    /// Byte range of the contents of `<head>…</head>` (between the tags) in the
    /// **cleaned** buffer, or `None`.
    pub head_range: Option<Range<usize>>,
    /// Any tag-open whose name contains `-` (custom-elements heuristic).
    pub had_custom_elements: bool,
    /// Any occurrence of `<![CDATA[`.
    pub had_cdata: bool,
    /// Any `<` that the prescan escaped via the invalid-tag branch.
    pub had_unescaped_lt: bool,
    /// Saw two consecutive opens of `<li>`, `<p>`, `<tr>`, `<td>`, `<dt>`, `<dd>`,
    /// or `<option>` without an intervening matching close.
    pub had_optional_close_rule_trigger: bool,
    /// Saw `<table>` followed directly by `<tr>` without an intervening
    /// `<tbody>`/`<thead>`/`<tfoot>`.
    pub has_no_tbody_table: bool,
    /// Saw `<script>` or `<style>` in the source.
    pub has_script_or_style: bool,
    /// SVG depth ever exceeded zero.
    pub has_svg: bool,
    /// Count of every `<…` that begins a tag (open OR close, void OR not).
    pub total_tags: u32,
    /// Increment on non-void non-self-closing tag open, decrement on close.
    /// Tracks the running maximum as a saturating `u16`.
    pub max_estimated_depth: u16,
}

// Tags whose optional-close rule fires when a matching open follows without
// an intervening close: the last-seen open for each tag is tracked.
const OPTIONAL_CLOSE_TAGS: &[&[u8]] = &[b"li", b"p", b"tr", b"td", b"dt", b"dd", b"option"];

// Void elements — do not contribute to depth tracking.
const VOID_TAGS: &[&[u8]] = &[
    b"area", b"base", b"br", b"col", b"embed", b"hr", b"img", b"input", b"link", b"meta", b"param", b"source",
    b"track", b"wbr",
];

// Tags that are stripped of their content by the prescan.
const STRIP_CONTENT_TAGS: [&[u8]; 2] = [b"script", b"style"];

const SVG_TAG: &[u8] = b"svg";
const HEAD_TAG: &[u8] = b"head";
const TABLE_TAG: &[u8] = b"table";
const TR_TAG: &[u8] = b"tr";
const TBODY_LIKE: &[&[u8]] = &[b"tbody", b"thead", b"tfoot"];
const CDATA_START: &[u8] = b"<![CDATA[";
const DOCTYPE: &[u8] = b"doctype";
const EMPTY_COMMENT: &[u8] = b"<!---->";
const SELF_CLOSING: [(&[u8], &str); 3] = [(b"<br/>", "<br>"), (b"<hr/>", "<hr>"), (b"<img/>", "<img>")];

/// Run the prescan over `html`, returning the cleaned buffer and signals.
///
/// `Cow::Borrowed` is returned when no transformation was needed.
///
/// # Panics
///
/// Panics if a tag-name byte sequence encountered during script/style stripping
/// is not valid UTF-8 (this cannot happen in practice because it is always a
/// sub-slice of the valid UTF-8 input `html`).
pub fn run(html: &str) -> (Cow<'_, str>, PrescanReport) {
    let bytes = html.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return (Cow::Borrowed(html), PrescanReport::default());
    }

    let mut report = PrescanReport::default();

    let mut idx = 0usize;
    let mut last = 0usize;
    let mut output: Option<String> = None;

    let mut svg_depth = 0usize;

    // Head-range tracking: byte index in the *output* buffer after `<head…>` closes.
    let mut head_open_end: Option<usize> = None;

    // Optional-close rule: track which of the OPTIONAL_CLOSE_TAGS was last opened.
    // Index 0..OPTIONAL_CLOSE_TAGS.len() stores whether tag[i] has an open without a close.
    let mut opt_close_open = [false; 7]; // one bool per OPTIONAL_CLOSE_TAGS entry

    // Table no-tbody detection: two-level state machine.
    // 0 = not inside table, 1 = inside table (looking for tbody/thead/tfoot/tr),
    // 2 = already saw tbody (no signal needed).
    let mut table_state: u8 = 0;

    // Depth tracking for max_estimated_depth.
    let mut current_depth: u16 = 0;

    while idx < len {
        if bytes[idx] != b'<' {
            idx += 1;
            continue;
        }

        // ── `<![CDATA[` detection (signal only; cleaning falls through) ─────────
        // The `<` in `<![CDATA[` will be processed by the is_valid_tag check below
        // (it is NOT a valid tag: `!` followed by `[` fails the validity test), so
        // it gets escaped to `&lt;` — exactly what the original preprocess_html did.
        // We only set the signal here without `continue`.
        if bytes[idx..].starts_with(CDATA_START) {
            report.had_cdata = true;
            // Fall through to is_valid_tag / escape logic below.
        }

        // ── Empty-comment normalisation: `<!---->` → `<!-- -->` ───────────────
        if bytes[idx..].starts_with(EMPTY_COMMENT) {
            let out = output.get_or_insert_with(|| String::with_capacity(html.len()));
            // flush output position accounting for bytes emitted into `output`
            out.push_str(&html[last..idx]);
            out.push_str("<!-- -->");
            idx += EMPTY_COMMENT.len();
            last = idx;
            continue;
        }

        // ── Self-closing normalisation: `<br/>` → `<br>` etc. ────────────────
        {
            let mut replaced = false;
            for (pattern, replacement) in &SELF_CLOSING {
                if bytes[idx..].starts_with(pattern) {
                    let out = output.get_or_insert_with(|| String::with_capacity(html.len()));
                    out.push_str(&html[last..idx]);
                    out.push_str(replacement);
                    idx += pattern.len();
                    last = idx;
                    replaced = true;
                    break;
                }
            }
            if replaced {
                // Self-closing tags: count as a tag, but not open+close depth contributors.
                report.total_tags += 1;
                continue;
            }
        }

        // ── SVG open / close ──────────────────────────────────────────────────
        if matches_tag_start(bytes, idx + 1, SVG_TAG) {
            if let Some(open_end) = find_tag_end(bytes, idx + 1 + SVG_TAG.len()) {
                svg_depth += 1;
                report.has_svg = true;
                report.total_tags += 1;
                current_depth = current_depth.saturating_add(1);
                if current_depth > report.max_estimated_depth {
                    report.max_estimated_depth = current_depth;
                }
                idx = open_end;
                continue;
            }
        } else if matches_end_tag_start(bytes, idx + 1, SVG_TAG) {
            if let Some(close_end) = find_tag_end(bytes, idx + 2 + SVG_TAG.len()) {
                if svg_depth > 0 {
                    svg_depth = svg_depth.saturating_sub(1);
                }
                report.total_tags += 1;
                current_depth = current_depth.saturating_sub(1);
                idx = close_end;
                continue;
            }
        }

        // ── Operations only outside SVG ───────────────────────────────────────
        if svg_depth == 0 {
            // ── `<script>` / `<style>` content stripping ──────────────────────
            let mut handled = false;
            for tag in &STRIP_CONTENT_TAGS {
                if matches_tag_start(bytes, idx + 1, tag) {
                    if let Some(open_end) = find_tag_end(bytes, idx + 1 + tag.len()) {
                        report.has_script_or_style = true;
                        report.total_tags += 1;
                        let remove_end = find_closing_tag(bytes, open_end, tag).unwrap_or(len);
                        if remove_end < len {
                            report.total_tags += 1; // the closing tag
                        }
                        let out = output.get_or_insert_with(|| String::with_capacity(html.len()));
                        out.push_str(&html[last..idx]);
                        out.push_str(&html[idx..open_end]);
                        out.push_str("</");
                        out.push_str(str::from_utf8(tag).unwrap());
                        out.push('>');
                        last = remove_end;
                        idx = remove_end;
                        handled = true;
                        break;
                    }
                }
            }

            if handled {
                continue;
            }

            // ── DOCTYPE stripping ─────────────────────────────────────────────
            if idx + 2 < len && bytes[idx + 1] == b'!' {
                let mut cursor = idx + 2;
                while cursor < len && bytes[cursor].is_ascii_whitespace() {
                    cursor += 1;
                }
                if cursor + DOCTYPE.len() <= len && bytes[cursor..cursor + DOCTYPE.len()].eq_ignore_ascii_case(DOCTYPE)
                {
                    if let Some(end) = find_tag_end(bytes, cursor + DOCTYPE.len()) {
                        let out = output.get_or_insert_with(|| String::with_capacity(html.len()));
                        out.push_str(&html[last..idx]);
                        last = end;
                        idx = end;
                        continue;
                    }
                }
            }

            // ── Signal: `<head>` / `</head>` ─────────────────────────────────
            if matches_tag_start(bytes, idx + 1, HEAD_TAG) {
                if let Some(open_end) = find_tag_end(bytes, idx + 1 + HEAD_TAG.len()) {
                    report.total_tags += 1;
                    current_depth = current_depth.saturating_add(1);
                    if current_depth > report.max_estimated_depth {
                        report.max_estimated_depth = current_depth;
                    }
                    // Record output position after the `<head…>` close-bracket.
                    // We need to compute the offset in the *output* buffer.
                    let flushed_so_far = if let Some(ref out) = output {
                        out.len() + (open_end - last)
                    } else {
                        open_end
                    };
                    head_open_end = Some(flushed_so_far);
                    idx = open_end;
                    continue;
                }
            } else if matches_end_tag_start(bytes, idx + 1, HEAD_TAG) {
                if let Some(close_end) = find_tag_end(bytes, idx + 2 + HEAD_TAG.len()) {
                    report.total_tags += 1;
                    current_depth = current_depth.saturating_sub(1);
                    if let Some(start) = head_open_end.take() {
                        // The `</head>` tag itself starts at the current output position.
                        let flushed_so_far = if let Some(ref out) = output {
                            out.len() + (idx - last)
                        } else {
                            idx
                        };
                        report.head_range = Some(start..flushed_so_far);
                    }
                    idx = close_end;
                    continue;
                }
            }

            // ── Signal: `<table>` / `<tbody>`‥ / `<tr>` ─────────────────────
            if matches_tag_start(bytes, idx + 1, TABLE_TAG) {
                if let Some(open_end) = find_tag_end(bytes, idx + 1 + TABLE_TAG.len()) {
                    report.total_tags += 1;
                    current_depth = current_depth.saturating_add(1);
                    if current_depth > report.max_estimated_depth {
                        report.max_estimated_depth = current_depth;
                    }
                    table_state = 1; // inside table, awaiting tbody or tr
                    idx = open_end;
                    continue;
                }
            } else if table_state == 1 {
                // Check for tbody/thead/tfoot (means table has proper sectioning)
                let mut saw_tbody = false;
                for &body_tag in TBODY_LIKE {
                    if matches_tag_start(bytes, idx + 1, body_tag) {
                        saw_tbody = true;
                        break;
                    }
                }
                if saw_tbody {
                    table_state = 2; // has proper sectioning
                } else if matches_tag_start(bytes, idx + 1, TR_TAG) {
                    // `<tr>` directly inside `<table>` — no tbody
                    report.has_no_tbody_table = true;
                    table_state = 0; // signal fired; stop tracking this table
                }
            }

            // ── Signal: custom elements (tag name contains `-`) ───────────────
            // Only fires for open tags, not close tags.
            {
                let tag_start = idx + 1;
                if tag_start < len && (bytes[tag_start].is_ascii_alphabetic()) {
                    // Find the end of the tag name.
                    let name_end = {
                        let mut e = tag_start;
                        while e < len && (bytes[e].is_ascii_alphanumeric() || bytes[e] == b'-' || bytes[e] == b'_') {
                            e += 1;
                        }
                        e
                    };
                    let tag_name = &bytes[tag_start..name_end];
                    if tag_name.contains(&b'-') {
                        report.had_custom_elements = true;
                    }
                }
            }
        }

        // ── Validity check (applies at all depths) ────────────────────────────
        let is_valid_tag = if idx + 1 < len {
            match bytes[idx + 1] {
                b'!' => {
                    idx + 2 < len
                        && (bytes[idx + 2] == b'-'
                            || bytes[idx + 2].is_ascii_alphabetic()
                            || bytes[idx + 2].is_ascii_uppercase())
                }
                b'/' => idx + 2 < len && (bytes[idx + 2].is_ascii_alphabetic() || bytes[idx + 2].is_ascii_uppercase()),
                b'?' => true,
                c if c.is_ascii_alphabetic() || c.is_ascii_uppercase() => true,
                _ => false,
            }
        } else {
            false
        };

        if !is_valid_tag {
            report.had_unescaped_lt = true;
            let out = output.get_or_insert_with(|| String::with_capacity(html.len() + 4));
            out.push_str(&html[last..idx]);
            out.push_str("&lt;");
            idx += 1;
            last = idx;
            continue;
        }

        // ── Tag counting + depth + optional-close signals ─────────────────────
        // We reach here for all valid `<…` that weren't handled by special cases above.
        {
            let is_close = idx + 1 < len && bytes[idx + 1] == b'/';
            let name_start = if is_close { idx + 2 } else { idx + 1 };

            // Find end of tag name.
            let name_end = {
                let mut e = name_start;
                while e < len
                    && (bytes[e].is_ascii_alphanumeric() || bytes[e] == b'-' || bytes[e] == b'_' || bytes[e] == b':')
                {
                    e += 1;
                }
                e
            };

            if name_start < name_end {
                report.total_tags += 1;

                let tag_name = &bytes[name_start..name_end];

                // ── Optional-close rule ──────────────────────────────────────
                if let Some(tag_idx) = OPTIONAL_CLOSE_TAGS
                    .iter()
                    .position(|t| tag_name.len() == t.len() && tag_name.eq_ignore_ascii_case(t))
                {
                    if is_close {
                        opt_close_open[tag_idx] = false;
                    } else {
                        if opt_close_open[tag_idx] {
                            // Consecutive open without close
                            report.had_optional_close_rule_trigger = true;
                        }
                        opt_close_open[tag_idx] = true;
                    }
                }

                // ── Depth tracking ────────────────────────────────────────────
                let is_void = VOID_TAGS
                    .iter()
                    .any(|t| tag_name.len() == t.len() && tag_name.eq_ignore_ascii_case(t));

                if !is_void {
                    if is_close {
                        current_depth = current_depth.saturating_sub(1);
                    } else {
                        current_depth = current_depth.saturating_add(1);
                        if current_depth > report.max_estimated_depth {
                            report.max_estimated_depth = current_depth;
                        }
                    }
                }
            }
        }

        idx += 1;
    }

    // If `<head>` was opened but `</head>` was never seen, record to EOF.
    if let Some(start) = head_open_end.take() {
        let end = if let Some(ref out) = output {
            out.len() + (len - last)
        } else {
            len
        };
        report.head_range = Some(start..end);
    }

    let cow = if let Some(mut out) = output {
        if last < len {
            out.push_str(&html[last..]);
        }
        Cow::Owned(out)
    } else {
        Cow::Borrowed(html)
    };

    (cow, report)
}

// ── Private helpers (mirrors of the ones in converter.rs) ──────────────────

fn matches_tag_start(bytes: &[u8], mut start: usize, tag: &[u8]) -> bool {
    if start >= bytes.len() || start + tag.len() > bytes.len() {
        return false;
    }
    if !bytes[start..start + tag.len()].eq_ignore_ascii_case(tag) {
        return false;
    }
    start += tag.len();
    matches!(
        bytes.get(start),
        Some(b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r') | None
    )
}

fn matches_end_tag_start(bytes: &[u8], start: usize, tag: &[u8]) -> bool {
    if start >= bytes.len() || bytes[start] != b'/' {
        return false;
    }
    matches_tag_start(bytes, start + 1, tag)
}

fn find_tag_end(bytes: &[u8], mut idx: usize) -> Option<usize> {
    let len = bytes.len();
    let mut in_quote: Option<u8> = None;
    while idx < len {
        match bytes[idx] {
            b'"' | b'\'' => {
                if let Some(current) = in_quote {
                    if current == bytes[idx] {
                        in_quote = None;
                    }
                } else {
                    in_quote = Some(bytes[idx]);
                }
            }
            b'>' if in_quote.is_none() => return Some(idx + 1),
            _ => {}
        }
        idx += 1;
    }
    None
}

fn find_closing_tag(bytes: &[u8], mut idx: usize, tag: &[u8]) -> Option<usize> {
    let len = bytes.len();
    let mut depth = 1usize;
    while idx < len {
        if bytes[idx] == b'<' {
            if matches_tag_start(bytes, idx + 1, tag) {
                if let Some(next) = find_tag_end(bytes, idx + 1 + tag.len()) {
                    depth += 1;
                    idx = next;
                    continue;
                }
            } else if matches_end_tag_start(bytes, idx + 1, tag) {
                if let Some(close) = find_tag_end(bytes, idx + 2 + tag.len()) {
                    depth -= 1;
                    if depth == 0 {
                        return Some(close);
                    }
                    idx = close;
                    continue;
                }
            }
        }
        idx += 1;
    }
    None
}
