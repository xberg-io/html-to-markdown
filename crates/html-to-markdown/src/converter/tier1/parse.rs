//! Byte-level helpers for the Tier-1 HTML scanner.
//!
//! All functions operate on raw byte slices and return byte-index ranges.
//! Attribute values are returned as byte ranges without decoding — the
//! caller is responsible for entity-decoding when needed.

use std::ops::Range;

/// Find the end of an ASCII tag name starting at `start`.
/// Returns the index of the first byte NOT part of the tag name
/// (whitespace, `/`, `>`).
#[inline]
pub fn scan_tag_name(bytes: &[u8], start: usize) -> usize {
    let mut end = start;
    while end < bytes.len() && is_tag_name_continue(bytes[end]) {
        end += 1;
    }
    end
}

/// True if byte `b` can start an HTML tag name (ASCII letter).
#[inline]
pub fn is_tag_name_start(b: u8) -> bool {
    b.is_ascii_alphabetic()
}

/// True if byte `b` can continue an HTML tag name (alpha-num, `-`, `_`).
#[inline]
pub fn is_tag_name_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}

/// Skip ASCII whitespace, returning the new position.
#[inline]
pub fn skip_ws(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

/// Find the closing `>` of a tag starting at `start` (the position after the tag name and attributes).
///
/// Returns `(close_bracket_idx, is_self_closing)` where `is_self_closing` is true if the tag ended
/// with `/>`.  Returns `None` if no `>` was found before the end of input.
pub fn find_tag_close(bytes: &[u8], start: usize) -> Option<(usize, bool)> {
    let mut pos = start;
    let len = bytes.len();
    let mut in_quote: Option<u8> = None;

    while pos < len {
        match bytes[pos] {
            b'"' | b'\'' => {
                if let Some(q) = in_quote {
                    if q == bytes[pos] {
                        in_quote = None;
                    }
                } else {
                    in_quote = Some(bytes[pos]);
                }
            }
            b'>' if in_quote.is_none() => {
                let self_closing = pos > 0 && bytes[pos - 1] == b'/';
                return Some((pos, self_closing));
            }
            _ => {}
        }
        pos += 1;
    }
    None
}

/// Parse a single attribute starting at `pos` (after skipping any preceding
/// whitespace).  Returns `(key_range, value_range, new_pos)`.
/// `value_range` is `None` for boolean attributes.
///
/// Returns `None` if no attribute could be parsed (e.g. at `>` or `/>` or EOF).
pub fn scan_attribute(bytes: &[u8], pos: usize) -> Option<(Range<usize>, Option<Range<usize>>, usize)> {
    let pos = skip_ws(bytes, pos);
    if pos >= bytes.len() {
        return None;
    }
    // Stop at tag terminators
    if bytes[pos] == b'>' || (bytes[pos] == b'/' && bytes.get(pos + 1) == Some(&b'>')) {
        return None;
    }

    // Key: scan until `=`, whitespace, `/`, or `>`
    let key_start = pos;
    let mut key_end = pos;
    while key_end < bytes.len() {
        match bytes[key_end] {
            b'=' | b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r' => break,
            _ => key_end += 1,
        }
    }
    if key_end == key_start {
        // Unexpected char — skip one byte to avoid infinite loop
        return None;
    }

    let after_key = skip_ws(bytes, key_end);

    if after_key >= bytes.len() || bytes[after_key] != b'=' {
        // Boolean attribute (no `=`)
        return Some((key_start..key_end, None, after_key));
    }

    // Skip `=`
    let after_eq = skip_ws(bytes, after_key + 1);
    if after_eq >= bytes.len() {
        return None;
    }

    // Value: quoted or unquoted
    let (value_range, new_pos) = if let q @ (b'"' | b'\'') = bytes[after_eq] {
        let val_start = after_eq + 1;
        let mut val_end = val_start;
        while val_end < bytes.len() && bytes[val_end] != q {
            val_end += 1;
        }
        (val_start..val_end, val_end + 1)
    } else {
        // Unquoted value: ends at whitespace or `>`
        let val_start = after_eq;
        let mut val_end = val_start;
        while val_end < bytes.len() && !matches!(bytes[val_end], b' ' | b'\t' | b'\n' | b'\r' | b'>' | b'/') {
            val_end += 1;
        }
        (val_start..val_end, val_end)
    };

    Some((key_start..key_end, Some(value_range), new_pos))
}

/// Collect all attributes between `start` and `end` (exclusive) in the tag.
///
/// Returns a vec of `(key_bytes, Option<value_bytes>)` where slices point
/// into `bytes`.
pub fn collect_attrs(bytes: &[u8], start: usize, end: usize) -> Vec<(&[u8], Option<&[u8]>)> {
    let mut attrs = Vec::new();
    let mut pos = skip_ws(bytes, start);
    while pos < end {
        match scan_attribute(bytes, pos) {
            Some((key_range, val_range, new_pos)) => {
                if new_pos <= pos {
                    // Avoid infinite loop
                    break;
                }
                let key = &bytes[key_range.start..key_range.end.min(end)];
                let val = val_range.map(|r| &bytes[r.start..r.end.min(end)]);
                attrs.push((key, val));
                pos = new_pos;
            }
            None => break,
        }
    }
    attrs
}
