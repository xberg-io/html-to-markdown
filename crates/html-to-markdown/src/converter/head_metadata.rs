//! Metadata extraction by re-parsing the prescan's head_range slice.
//!
//! The prescan walks the input once and captures the byte range of the
//! `<head>…</head>` content (between the tags) in the **cleaned** buffer.
//! `extract_frontmatter` re-parses just the head slice (small) using
//! `tl::parse` and applies the same extraction rules as the Tier-2 path,
//! so both paths produce byte-identical YAML frontmatter output.
//!
//! Heads are typically small (< 50 KB), so the second parse is cheap.

use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::converter::prescan::PrescanReport;
use crate::options::ConversionOptions;
use crate::text;

/// Attempt to extract YAML frontmatter from the document head.
///
/// Returns `Some(frontmatter_string)` when `options.extract_metadata` is
/// true AND the prescan captured a head range AND at least one metadata
/// field was found.  Returns `None` otherwise — callers should prepend the
/// returned string to the body only when `Some` is returned.
pub fn extract_frontmatter(html: &str, report: &PrescanReport, options: &ConversionOptions) -> Option<String> {
    if !options.extract_metadata {
        return None;
    }

    let head_range = report.head_range.as_ref()?;

    // Guard: the head range must be within the html slice bounds.
    if head_range.end > html.len() {
        return None;
    }

    let head_content = &html[head_range.clone()];
    if head_content.is_empty() {
        return None;
    }

    // Wrap the extracted head content in a minimal HTML document so that
    // `tl::parse` has the correct context.  The wrapper tags are never
    // accessed during extraction — we only walk the direct children of
    // the synthesised `<head>` tag.
    let wrapped = format!("<html><head>{head_content}</head></html>");

    // SAFETY: we are re-parsing a small (head-only) slice of the same
    // already-cleaned HTML.  `tl::parse` is infallible for well-formed
    // UTF-8; any parse errors are silently ignored (empty metadata).
    let dom = match tl::parse(&wrapped, tl::ParserOptions::default()) {
        Ok(d) => d,
        Err(_) => return None,
    };

    let parser = dom.parser();
    let metadata = extract_metadata_from_dom(&dom, parser);

    if metadata.is_empty() {
        return None;
    }

    Some(format_metadata_frontmatter(&metadata))
}

// ── Internal helpers (byte-equivalent to the Tier-2 implementations) ─────────

/// Walk the DOM and extract metadata from the `<head>` element's children.
///
/// Produces a `BTreeMap<String, String>` with the same keys/values as
/// `crate::converter::extract_metadata`.  Byte-equivalence is the contract.
fn extract_metadata_from_dom(dom: &tl::VDom, parser: &tl::Parser) -> BTreeMap<String, String> {
    let mut metadata = BTreeMap::new();

    // Find the <head> node by walking the DOM children.
    let head_handle = dom
        .children()
        .iter()
        .find_map(|child_handle| find_head_node(child_handle, parser));

    let head_handle = match head_handle {
        Some(h) => h,
        None => return metadata,
    };

    if let Some(head_node) = head_handle.get(parser) {
        if let tl::Node::Tag(head_tag) = head_node {
            let children = head_tag.children();
            for child_handle in children.top().iter() {
                if let Some(child_node) = child_handle.get(parser) {
                    if let tl::Node::Tag(child_tag) = child_node {
                        let raw_name = child_tag.name().as_utf8_str();
                        let tag_name = normalize_tag_name(raw_name);

                        match tag_name.as_ref() {
                            "title" => extract_title(child_tag, parser, &mut metadata),
                            "base" => extract_base(child_tag, &mut metadata),
                            "meta" => extract_meta(child_tag, &mut metadata),
                            "link" => extract_link(child_tag, &mut metadata),
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    metadata
}

/// Recursively find the first `<head>` tag in the DOM.
fn find_head_node(node_handle: &tl::NodeHandle, parser: &tl::Parser) -> Option<tl::NodeHandle> {
    if let Some(node) = node_handle.get(parser) {
        if let tl::Node::Tag(tag) = node {
            if normalize_tag_name(tag.name().as_utf8_str()) == "head" {
                return Some(*node_handle);
            }
            let children = tag.children();
            for child_handle in children.top().iter() {
                if let Some(result) = find_head_node(child_handle, parser) {
                    return Some(result);
                }
            }
        }
    }
    None
}

fn extract_title(tag: &tl::HTMLTag, parser: &tl::Parser, metadata: &mut BTreeMap<String, String>) {
    let children = tag.children();
    if let Some(first_child) = children.top().iter().next() {
        if let Some(text_node) = first_child.get(parser) {
            if let tl::Node::Raw(bytes) = text_node {
                let title = text::normalize_whitespace(&bytes.as_utf8_str()).trim().to_string();
                if !title.is_empty() {
                    metadata.insert("title".to_string(), title);
                }
            }
        }
    }
}

fn extract_base(tag: &tl::HTMLTag, metadata: &mut BTreeMap<String, String>) {
    if let Some(href_attr) = tag.attributes().get("href") {
        if let Some(href_bytes) = href_attr {
            let href = href_bytes.as_utf8_str().to_string();
            if !href.is_empty() {
                metadata.insert("base-href".to_string(), href);
            }
        }
    }
}

fn extract_meta(tag: &tl::HTMLTag, metadata: &mut BTreeMap<String, String>) {
    let mut name_attr: Option<String> = None;
    let mut property_attr: Option<String> = None;
    let mut http_equiv_attr: Option<String> = None;
    let mut content_attr: Option<String> = None;

    if let Some(attr) = tag.attributes().get("name") {
        if let Some(bytes) = attr {
            name_attr = Some(bytes.as_utf8_str().to_string());
        }
    }
    if let Some(attr) = tag.attributes().get("property") {
        if let Some(bytes) = attr {
            property_attr = Some(bytes.as_utf8_str().to_string());
        }
    }
    if let Some(attr) = tag.attributes().get("http-equiv") {
        if let Some(bytes) = attr {
            http_equiv_attr = Some(bytes.as_utf8_str().to_string());
        }
    }
    if let Some(attr) = tag.attributes().get("content") {
        if let Some(bytes) = attr {
            content_attr = Some(bytes.as_utf8_str().to_string());
        }
    }

    if let Some(content) = content_attr {
        if let Some(name) = name_attr {
            let key = format!("meta-{}", name.to_lowercase());
            metadata.insert(key, content);
        } else if let Some(property) = property_attr {
            let key = format!("meta-{}", property.to_lowercase().replace(':', "-"));
            metadata.insert(key, content);
        } else if let Some(http_equiv) = http_equiv_attr {
            let key = format!("meta-{}", http_equiv.to_lowercase());
            metadata.insert(key, content);
        }
    }
}

fn extract_link(tag: &tl::HTMLTag, metadata: &mut BTreeMap<String, String>) {
    let mut rel_attr: Option<String> = None;
    let mut href_attr: Option<String> = None;

    if let Some(attr) = tag.attributes().get("rel") {
        if let Some(bytes) = attr {
            rel_attr = Some(bytes.as_utf8_str().to_string());
        }
    }
    if let Some(attr) = tag.attributes().get("href") {
        if let Some(bytes) = attr {
            href_attr = Some(bytes.as_utf8_str().to_string());
        }
    }

    if let (Some(rel), Some(href)) = (rel_attr, href_attr) {
        let rel_lower = rel.to_lowercase();
        match rel_lower.as_str() {
            "canonical" => {
                metadata.insert("canonical".to_string(), href);
            }
            "author" | "license" | "alternate" => {
                metadata.insert(format!("link-{rel_lower}"), href);
            }
            _ => {}
        }
    }
}

/// Format a metadata map as YAML frontmatter.
///
/// Byte-equivalent to `crate::converter::format_metadata_frontmatter`.
fn format_metadata_frontmatter(metadata: &BTreeMap<String, String>) -> String {
    debug_assert!(!metadata.is_empty());

    let mut lines = vec!["---".to_string()];
    for (key, value) in metadata {
        let needs_quotes = value.contains(':') || value.contains('#') || value.contains('[') || value.contains(']');
        if needs_quotes {
            let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
            lines.push(format!("{key}: \"{escaped}\""));
        } else {
            lines.push(format!("{key}: {value}"));
        }
    }
    lines.push("---".to_string());

    lines.join("\n") + "\n\n"
}

/// ASCII-lowercase a `Cow<str>` tag name without allocation when already lower.
fn normalize_tag_name(raw: Cow<'_, str>) -> Cow<'_, str> {
    if raw.as_bytes().iter().any(u8::is_ascii_uppercase) {
        let mut owned = raw.into_owned();
        owned.make_ascii_lowercase();
        Cow::Owned(owned)
    } else {
        raw
    }
}
