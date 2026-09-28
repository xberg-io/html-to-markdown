//! Metadata extraction by re-parsing the prescan's `head_range` slice.
//!
//! The prescan walks the input once and captures the byte range of the
//! `<head>…</head>` content (between the tags) in the **cleaned** buffer.
//! `extract_frontmatter` re-parses just the head slice (small) using
//! `tl::parse` and applies the same extraction rules as the Tier-2 path,
//! so both paths produce byte-identical YAML frontmatter output.
//!
//! Heads are typically small (< 50 KB), so the second parse is cheap.

use std::collections::BTreeMap;
use std::ops::Range;

use crate::converter::main_helpers::{extract_head_metadata, format_metadata_frontmatter};
use crate::options::ConversionOptions;

/// Attempt to extract YAML frontmatter from the document head.
///
/// `head_range` is the byte range of `<head>…</head>` content within `html`.
/// Originally captured by `prescan::run`; since Phase C the Tier-1 scanner
/// computes it directly during its single pass.
///
/// Returns `Some(frontmatter_string)` when `options.extract_metadata` is
/// true AND `head_range` is `Some` AND at least one metadata field was
/// found.  Returns `None` otherwise — callers should prepend the returned
/// string to the body only when `Some` is returned.
pub fn extract_frontmatter(
    html: &str,
    head_range: Option<&Range<usize>>,
    options: &ConversionOptions,
) -> Option<String> {
    if !options.extract_metadata {
        return None;
    }

    let head_range = head_range?;

    if head_range.end > html.len() {
        return None;
    }

    let head_content = &html[head_range.clone()];
    if head_content.is_empty() {
        return None;
    }

    // ~keep Wrap the extracted head content in a minimal HTML document so that
    // ~keep `tl::parse` has the correct context.  The wrapper tags are never
    // ~keep accessed during extraction — we only walk the direct children of
    // ~keep the synthesised `<head>` tag.
    let wrapped = format!("<html><head>{head_content}</head></html>");

    // ~keep SAFETY: we are re-parsing a small (head-only) slice of the same
    // ~keep already-cleaned HTML.  `tl::parse` is infallible for well-formed
    // ~keep UTF-8; any parse errors are silently ignored (empty metadata).
    let dom = match tl::parse(&wrapped, tl::ParserOptions::default()) {
        Ok(d) => d,
        Err(_) => return None,
    };

    let parser = dom.parser();
    // ~keep Delegate to the canonical Tier-2 extractor so the two tiers agree on
    // ~keep key names / casing / value normalisation byte-for-byte.  Walk the
    // ~keep synthesised wrapper's children to find the `<head>` node first.
    let mut metadata = BTreeMap::new();
    for child_handle in dom.children() {
        let m = extract_head_metadata(child_handle, parser, options);
        if !m.is_empty() {
            metadata = m;
            break;
        }
    }

    if metadata.is_empty() {
        return None;
    }

    Some(format_metadata_frontmatter(&metadata))
}
