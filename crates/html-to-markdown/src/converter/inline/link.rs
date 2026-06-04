//! Handler for link elements (a, anchor).
//!
//! Converts HTML anchor tags to Markdown links with support for:
//! - Standard Markdown link syntax `[label](href "title")`
//! - Autolinks for simple URLs like `<https://example.com>`
//! - Link label escaping for special Markdown characters
//! - Heading-in-link special handling (wraps link around heading)
//! - Visitor callbacks for custom link processing
//! - Metadata collection for links (links, URLs, titles, rel attributes)
//! - Block-level content within links (via inline context)

use crate::converter::utility::content::{collect_link_label_text, escape_link_label, normalize_link_label};
use crate::converter::utility::preprocessing::sanitize_markdown_url;
use crate::options::ConversionOptions;
#[cfg(feature = "visitor")]
use std::borrow::Cow;
#[cfg(any(feature = "metadata", feature = "visitor"))]
use std::collections::BTreeMap;
use tl::{NodeHandle, Parser};

// Type aliases for Context and DomContext to avoid circular imports
// These are imported from converter.rs and should be made accessible
type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handler for anchor/link elements: `<a>`.
///
/// Processes anchor tags to generate Markdown links:
/// - Detects autolinks (link text matches href)
/// - Extracts and normalizes link labels
/// - Handles nested headings within links
/// - Escapes special characters in labels
/// - Collects metadata when feature is enabled
/// - Supports visitor callbacks for custom processing
///
/// # Link Label Extraction
/// For links with block-level content, extracts text separately.
/// Collapses newlines and normalizes whitespace per Markdown spec.
///
/// # Autolinks
/// When `autolinks` option is enabled, detects links where the text equals
/// the href (e.g., `<a href="https://example.com">https://example.com</a>`)
/// and outputs as `<https://example.com>` instead.
///
/// # Note
/// This function references helper functions from converter.rs
/// which must be accessible (pub(crate)) for this module to work correctly.
pub fn handle(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    // Import helper functions from parent converter module
    use crate::converter::block::heading::{heading_allows_inline_images, push_heading};
    use crate::converter::utility::content::normalized_tag_name;
    #[allow(unused_imports)]
    use crate::converter::utility::serialization::serialize_node;
    use crate::converter::{find_single_heading_child, get_text_content, walk_node};

    let Some(node) = node_handle.get(parser) else {
        return;
    };

    let tl::Node::Tag(tag) = node else {
        return;
    };

    // Extract href and title attributes
    let href_attr = tag.attributes().get("href").flatten().map(|v| {
        let decoded = crate::text::decode_html_entities(&v.as_utf8_str());
        sanitize_markdown_url(&decoded).into_owned()
    });
    let title = tag
        .attributes()
        .get("title")
        .flatten()
        .map(|v| v.as_utf8_str().to_string());

    if let Some(href) = href_attr {
        let raw_text = crate::text::normalize_whitespace(&get_text_content(node_handle, parser, dom_ctx))
            .trim()
            .to_string();

        // If we're already inside a link, just render the text content, don't create a nested link
        if ctx.in_link {
            let children = tag.children();
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
            return;
        }

        // Check if this should be rendered as an autolink.
        // GFM requires an absolute URI with a scheme (e.g. `https://…`, `mailto:…`);
        // bare paths or filenames must use the full `[text](href)` form.
        let is_autolink = options.autolinks
            && !options.default_title
            && !href.is_empty()
            && has_uri_scheme(href.as_str())
            && (raw_text == href || (href.starts_with("mailto:") && raw_text == href[7..]));

        if is_autolink {
            output.push('<');
            if href.starts_with("mailto:") && raw_text == href[7..] {
                output.push_str(&raw_text);
            } else {
                output.push_str(&href);
            }
            output.push('>');
            return;
        }

        // Check if link contains a single heading child element
        if let Some((heading_level, heading_handle)) = find_single_heading_child(*node_handle, parser) {
            if let Some(heading_node) = heading_handle.get(parser) {
                if let tl::Node::Tag(heading_tag) = heading_node {
                    let heading_name = normalized_tag_name(heading_tag.name().as_utf8_str()).into_owned();
                    let mut heading_text = String::new();
                    let heading_ctx = Context {
                        in_heading: true,
                        convert_as_inline: true,
                        heading_allow_inline_images: heading_allows_inline_images(
                            &heading_name,
                            &ctx.keep_inline_images_in,
                        ),
                        ..ctx.clone()
                    };
                    walk_node(
                        &heading_handle,
                        parser,
                        &mut heading_text,
                        options,
                        &heading_ctx,
                        depth + 1,
                        dom_ctx,
                    );
                    let trimmed_heading = heading_text.trim();
                    if !trimmed_heading.is_empty() {
                        let escaped_label = escape_link_label(trimmed_heading);
                        let mut link_buffer = String::new();
                        append_markdown_link(
                            &mut link_buffer,
                            &escaped_label,
                            href.as_str(),
                            title.as_deref(),
                            raw_text.as_str(),
                            options,
                            ctx.reference_collector.as_ref(),
                        );
                        push_heading(output, ctx, options, heading_level, link_buffer.as_str());
                        return;
                    }
                }
            }
        }

        // Collect link label from children
        let children: Vec<_> = tag.children().top().iter().copied().collect();
        let (inline_label, _block_nodes, saw_block) = collect_link_label_text(&children, parser, dom_ctx);
        let mut label = if saw_block {
            let mut content = String::new();
            let link_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                convert_as_inline: true,
                in_link: true,
                ..ctx.clone()
            };
            for child_handle in &children {
                let mut child_buf = String::new();
                walk_node(
                    child_handle,
                    parser,
                    &mut child_buf,
                    options,
                    &link_ctx,
                    depth + 1,
                    dom_ctx,
                );
                if !child_buf.trim().is_empty()
                    && !content.is_empty()
                    && !content.chars().last().is_none_or(char::is_whitespace)
                    && !child_buf.chars().next().is_none_or(char::is_whitespace)
                {
                    content.push(' ');
                }
                content.push_str(&child_buf);
            }
            if content.trim().is_empty() {
                normalize_link_label(&inline_label)
            } else {
                normalize_link_label(&content)
            }
        } else {
            let mut content = String::new();
            let link_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                in_link: true,
                ..ctx.clone()
            };
            for child_handle in &children {
                walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    &link_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
            normalize_link_label(&content)
        };

        // Apply fallback label strategies
        if label.is_empty() && saw_block {
            let fallback = crate::text::normalize_whitespace(&get_text_content(node_handle, parser, dom_ctx));
            label = normalize_link_label(&fallback);
        }

        if label.is_empty() && !raw_text.is_empty() {
            label = normalize_link_label(&raw_text);
        }

        if label.is_empty() && !href.is_empty() && !children.is_empty() {
            label = href.clone();
        }

        // Truncate label if it exceeds maximum length
        let escaped_label = escape_link_label(&label);

        // Handle visitor callbacks if feature is enabled
        #[cfg(feature = "visitor")]
        let link_output = if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Link,
                Cow::Borrowed("a"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_link(&node_ctx, &href, &label, title.as_deref())
            };
            match visit_result {
                VisitResult::Continue => {
                    let mut buf = String::new();
                    append_markdown_link(
                        &mut buf,
                        &escaped_label,
                        href.as_str(),
                        title.as_deref(),
                        label.as_str(),
                        options,
                        ctx.reference_collector.as_ref(),
                    );
                    Some(buf)
                }
                VisitResult::Custom(custom) => Some(custom),
                VisitResult::Skip => None,
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    None
                }
                VisitResult::PreserveHtml => Some(serialize_node(node_handle, parser)),
            }
        } else {
            let mut buf = String::new();
            append_markdown_link(
                &mut buf,
                &escaped_label,
                href.as_str(),
                title.as_deref(),
                label.as_str(),
                options,
                ctx.reference_collector.as_ref(),
            );
            Some(buf)
        };

        #[cfg(not(feature = "visitor"))]
        let link_output = {
            let mut buf = String::new();
            append_markdown_link(
                &mut buf,
                &escaped_label,
                href.as_str(),
                title.as_deref(),
                label.as_str(),
                options,
                ctx.reference_collector.as_ref(),
            );
            Some(buf)
        };

        if let Some(link_text) = link_output {
            output.push_str(&link_text);
        }

        // Collect metadata if feature is enabled
        #[cfg(feature = "metadata")]
        if ctx.metadata_wants_links {
            if let Some(ref collector) = ctx.metadata_collector {
                let rel_attr = tag
                    .attributes()
                    .get("rel")
                    .flatten()
                    .map(|v| v.as_utf8_str().to_string());
                let mut attributes_map = BTreeMap::new();
                for (key, value_opt) in tag.attributes().iter() {
                    let key_str = key.to_string();
                    if key_str == "href" {
                        continue;
                    }

                    let value = value_opt.map(|v| v.to_string()).unwrap_or_default();
                    attributes_map.insert(key_str, value);
                }
                collector
                    .borrow_mut()
                    .add_link(href.clone(), label, title.clone(), rel_attr, attributes_map);
            }
        }
    } else {
        // No href: just process children as inline content
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    }
}

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
            // RFC 3986 unreserved characters plus slash
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
/// - When `default_title` option is true and raw_text equals href, adds href as title
///
/// # Arguments
/// * `output` - Output buffer to append the link to
/// * `label` - The link text (already escaped)
/// * `href` - The URL/destination
/// * `title` - Optional link title attribute
/// * `raw_text` - Original unprocessed text (for default_title option)
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

    if href.is_empty() {
        output.push_str("<>");
    } else if options.url_escape_style == crate::options::validation::UrlEscapeStyle::Percent {
        let encoded = percent_encode_url(href);
        output.push_str(&encoded);
    } else if href.contains(' ') || href.contains('\n') {
        output.push('<');
        output.push_str(href);
        output.push('>');
    } else {
        let open_count = href.chars().filter(|&c| c == '(').count();
        let close_count = href.chars().filter(|&c| c == ')').count();

        if open_count == close_count {
            output.push_str(href);
        } else {
            let escaped_href = href.replace('(', "\\(").replace(')', "\\)");
            output.push_str(&escaped_href);
        }
    }

    if let Some(title_text) = title {
        output.push_str(" \"");
        if title_text.contains('"') {
            let escaped_title = title_text.replace('"', "\\\"");
            output.push_str(&escaped_title);
        } else {
            output.push_str(title_text);
        }
        output.push('"');
    } else if options.default_title && raw_text == href {
        output.push_str(" \"");
        if href.contains('"') {
            let escaped_href = href.replace('"', "\\\"");
            output.push_str(&escaped_href);
        } else {
            output.push_str(href);
        }
        output.push('"');
    }

    output.push(')');
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::validation::UrlEscapeStyle;

    fn opts_with_style(style: UrlEscapeStyle) -> ConversionOptions {
        ConversionOptions::builder().url_escape_style(style).build()
    }

    // ── has_uri_scheme ───────────────────────────────────────────────────────

    #[test]
    fn has_uri_scheme_accepts_http() {
        assert!(has_uri_scheme("http://example.com"));
        assert!(has_uri_scheme("https://example.com/path"));
    }

    #[test]
    fn has_uri_scheme_accepts_mailto() {
        assert!(has_uri_scheme("mailto:a@b.com"));
    }

    #[test]
    fn has_uri_scheme_accepts_uncommon_schemes() {
        assert!(has_uri_scheme("ftp://host"));
        assert!(has_uri_scheme("ssh://host"));
        assert!(has_uri_scheme("data:text/plain,foo"));
        assert!(has_uri_scheme("file:///etc/hosts"));
    }

    #[test]
    fn has_uri_scheme_rejects_bare_paths() {
        assert!(!has_uri_scheme("foobar.png"));
        assert!(!has_uri_scheme("/relative/path"));
        assert!(!has_uri_scheme("../up.html"));
        assert!(!has_uri_scheme("#fragment"));
    }

    #[test]
    fn has_uri_scheme_rejects_leading_digit_or_punct() {
        assert!(!has_uri_scheme("9scheme:foo"));
        assert!(!has_uri_scheme(":no-scheme"));
        assert!(!has_uri_scheme(""));
    }

    #[test]
    fn issue_397_filename_with_extension_is_not_autolinked() {
        // <a href="foobar.png">foobar.png</a> must NOT become <foobar.png> — there's no scheme.
        // The autolink check `is_autolink` is wired with `has_uri_scheme`; verify the helper.
        assert!(!has_uri_scheme("foobar.png"));
    }

    // ── percent_encode_url ───────────────────────────────────────────────────

    #[test]
    fn percent_encode_url_leaves_unreserved_chars_unchanged() {
        // Note: ':' and '/' are outside the strict unreserved set, but '/' is explicitly allowed
        let result = percent_encode_url("/path-to_file.html~");
        assert_eq!(result, "/path-to_file.html~");
    }

    #[test]
    fn percent_encode_url_encodes_spaces() {
        assert_eq!(percent_encode_url("/file (1).pdf"), "/file%20%281%29.pdf");
    }

    #[test]
    fn percent_encode_url_encodes_angle_brackets() {
        assert_eq!(percent_encode_url("/file <draft>.pdf"), "/file%20%3Cdraft%3E.pdf");
    }

    #[test]
    fn percent_encode_url_full_issue_example() {
        // Reproduces the exact example from GitHub issue #392
        assert_eq!(
            percent_encode_url("/file (1) <draft>.pdf"),
            "/file%20%281%29%20%3Cdraft%3E.pdf"
        );
    }

    // ── append_markdown_link with Angle style (default) ──────────────────────

    #[test]
    fn append_markdown_link_angle_plain_url_unchanged() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "/file.pdf", None, "text", &options, None);
        assert_eq!(out, "[text](/file.pdf)");
    }

    #[test]
    fn append_markdown_link_angle_wraps_space_in_angle_brackets() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "file", "/file (1).pdf", None, "file", &options, None);
        assert_eq!(out, "[file](</file (1).pdf>)");
    }

    // ── append_markdown_link with Percent style ───────────────────────────────

    #[test]
    fn append_markdown_link_percent_encodes_spaces() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Percent);
        append_markdown_link(&mut out, "file", "/file (1).pdf", None, "file", &options, None);
        assert_eq!(out, "[file](/file%20%281%29.pdf)");
    }

    #[test]
    fn append_markdown_link_percent_encodes_angle_brackets() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Percent);
        append_markdown_link(&mut out, "file", "/file <draft>.pdf", None, "file", &options, None);
        assert_eq!(out, "[file](/file%20%3Cdraft%3E.pdf)");
    }

    #[test]
    fn append_markdown_link_percent_full_issue_example() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Percent);
        append_markdown_link(&mut out, "file", "/file (1) <draft>.pdf", None, "file", &options, None);
        assert_eq!(out, "[file](/file%20%281%29%20%3Cdraft%3E.pdf)");
    }

    #[test]
    fn append_markdown_link_percent_preserves_title() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Percent);
        append_markdown_link(
            &mut out,
            "link",
            "/path with spaces",
            Some("My Title"),
            "link",
            &options,
            None,
        );
        assert_eq!(out, "[link](/path%20with%20spaces \"My Title\")");
    }
}
