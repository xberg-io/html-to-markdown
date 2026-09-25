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

use crate::converter::utility::content::{collect_link_label_text, normalize_link_label};
use crate::converter::utility::escaping::escape_link_label;
use crate::converter::utility::preprocessing::sanitize_markdown_url;
use crate::options::ConversionOptions;
use std::borrow::Cow;
#[cfg(feature = "metadata")]
use std::collections::BTreeMap;
use tl::{NodeHandle, Parser};

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
    use crate::converter::block::heading::{heading_allows_inline_images, push_heading};
    use crate::converter::utility::content::normalized_tag_name;
    // ~keep reason: serialize_node is only used when the visitor feature is active.
    #[allow(unused_imports)]
    use crate::converter::utility::serialization::serialize_node;
    use crate::converter::{find_single_heading_child, get_text_content, walk_node};

    let Some(node) = node_handle.get(parser) else {
        return;
    };

    let tl::Node::Tag(tag) = node else {
        return;
    };

    let href_attr = tag.attributes().get("href").flatten().map(|v| {
        let decoded = crate::text::decode_html_entities(&v.as_utf8_str());
        let resolved = ctx.resolve_url(&decoded).unwrap_or(decoded);
        sanitize_markdown_url(&resolved).into_owned()
    });
    // ~keep Still a Cow borrowed from the tag's attribute bytes in the common case
    // ~keep (Tier-2 hot-spot pass III): `decoded_attribute` returns early when the value
    // ~keep holds no `&`, so a title without entities allocates nothing. The note that
    // ~keep stood here claimed decoding was "not needed" and skipped it, which emitted
    // ~keep `title="A&amp;B"` as the literal text `A&amp;B` (issue #494).
    let title = crate::converter::utility::attributes::decoded_attribute(tag, "title");

    if let Some(href) = href_attr {
        if ctx.in_link {
            let children = tag.children();
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
            return;
        }

        let owned_children: Vec<tl::NodeHandle>;
        let children: &[tl::NodeHandle] = if let Some(c) = dom_ctx.children_of(node_handle.get_inner()) {
            c.as_slice()
        } else {
            owned_children = tag.children().top().iter().copied().collect();
            owned_children.as_slice()
        };
        let (inline_label, _block_nodes, saw_block) = collect_link_label_text(children, parser, dom_ctx);

        // ~keep Without block descendants the sweep above already visited exactly the nodes
        // ~keep `get_text_content` would and decoded them the same way, so its text is reused
        // ~keep rather than walking the `<a>` subtree a second time.
        let text_source: Cow<'_, str> = if saw_block {
            Cow::Owned(get_text_content(node_handle, parser, dom_ctx))
        } else {
            Cow::Borrowed(inline_label.as_str())
        };
        let normalized_text = crate::text::normalize_whitespace_cow(text_source.as_ref());
        let raw_text = normalized_text.trim();

        // ~keep Check if this should be rendered as an autolink.
        // ~keep GFM requires an absolute URI with a scheme (e.g. `https://…`, `mailto:…`);
        // ~keep bare paths or filenames must use the full `[text](href)` form.
        let is_autolink = options.autolinks
            && !options.default_title
            && !href.is_empty()
            && has_uri_scheme(href.as_str())
            && (raw_text == href || (href.starts_with("mailto:") && raw_text == &href[7..]));

        if is_autolink {
            output.push('<');
            if href.starts_with("mailto:") && raw_text == &href[7..] {
                output.push_str(raw_text);
            } else {
                output.push_str(&href);
            }
            output.push('>');
            return;
        }

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
                            raw_text,
                            options,
                            ctx.reference_collector.as_ref(),
                        );
                        push_heading(output, ctx, options, heading_level, link_buffer.as_str());
                        return;
                    }
                }
            }
        }

        let mut label = if saw_block {
            let mut content = String::new();
            let link_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                convert_as_inline: true,
                in_link: true,
                ..ctx.clone()
            };
            for child_handle in children {
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
            for child_handle in children {
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

        // ~keep `raw_text` is already the whole-subtree text when `saw_block`, so this single
        // ~keep fallback covers both the block and inline cases.
        if label.is_empty() && !raw_text.is_empty() {
            label = normalize_link_label(raw_text);
        }

        if label.is_empty() && !href.is_empty() && !children.is_empty() {
            label.clone_from(&href);
        }

        let escaped_label = escape_link_label(&label);

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
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
                VisitResult::Continue => append_markdown_link(
                    output,
                    &escaped_label,
                    href.as_str(),
                    title.as_deref(),
                    label.as_str(),
                    options,
                    ctx.reference_collector.as_ref(),
                ),
                VisitResult::Custom(custom) => output.push_str(&custom),
                VisitResult::Skip => {}
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                }
                VisitResult::PreserveHtml => output.push_str(&serialize_node(node_handle, parser)),
            }
        } else {
            append_markdown_link(
                output,
                &escaped_label,
                href.as_str(),
                title.as_deref(),
                label.as_str(),
                options,
                ctx.reference_collector.as_ref(),
            );
        }

        #[cfg(not(feature = "visitor"))]
        append_markdown_link(
            output,
            &escaped_label,
            href.as_str(),
            title.as_deref(),
            label.as_str(),
            options,
            ctx.reference_collector.as_ref(),
        );

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
                collector.borrow_mut().add_link(
                    href.clone(),
                    label,
                    title.as_deref().map(str::to_string),
                    rel_attr,
                    attributes_map,
                );
            }
        }
    } else {
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    }
}

mod link_url;
pub use link_url::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::validation::UrlEscapeStyle;

    fn opts_with_style(style: UrlEscapeStyle) -> ConversionOptions {
        ConversionOptions::builder().url_escape_style(style).build()
    }

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
        assert!(!has_uri_scheme("foobar.png"));
    }

    #[test]
    fn percent_encode_url_leaves_unreserved_chars_unchanged() {
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
        assert_eq!(
            percent_encode_url("/file (1) <draft>.pdf"),
            "/file%20%281%29%20%3Cdraft%3E.pdf"
        );
    }

    #[test]
    fn percent_encode_non_ascii_leaves_ascii_only_input_unchanged() {
        assert_eq!(
            percent_encode_non_ascii("/file (1) <draft>.pdf"),
            "/file (1) <draft>.pdf"
        );
    }

    #[test]
    fn percent_encode_non_ascii_encodes_only_non_ascii_bytes() {
        assert_eq!(percent_encode_non_ascii("/f\u{f6}\u{f6}.html"), "/f%C3%B6%C3%B6.html");
    }

    #[test]
    fn append_markdown_link_angle_plain_url_unchanged() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "/file.pdf", None, "text", &options, None);
        assert_eq!(out, "[text](/file.pdf)");
    }

    // ~keep Regression for the CommonMark spec fixpoint gap: a raw non-ASCII byte in a
    // ~keep destination is never valid URI syntax (RFC 3986), and every compliant HTML
    // ~keep renderer percent-encodes it on render (confirmed against the spec's own worked
    // ~keep example for entity references, `/f\u{f6}\u{f6}` -> `/f%C3%B6%C3%B6`), so the
    // ~keep default `Angle` style must emit it pre-encoded to reach a round-trip fixpoint.
    #[test]
    fn append_markdown_link_angle_percent_encodes_non_ascii() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "öö.html", "öö.html", None, "öö.html", &options, None);
        assert_eq!(out, "[öö.html](%C3%B6%C3%B6.html)");
    }

    // ~keep Non-ASCII pre-encoding must not disturb ASCII reserved characters the `Angle`
    // ~keep style deliberately leaves alone (unlike `Percent`, which would also mangle this
    // ~keep query string): only bytes >= 0x80 are touched.
    #[test]
    fn append_markdown_link_angle_percent_encodes_non_ascii_but_not_ascii_reserved_chars() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            "text",
            "https://example.com/söq?a=1&b=2",
            None,
            "text",
            &options,
            None,
        );
        assert_eq!(out, "[text](https://example.com/s%C3%B6q?a=1&b=2)");
    }

    // ~keep Regression: a same-document fragment must byte-match the target element's
    // ~keep `id`, which this converter does not control, and real-world generators
    // ~keep commonly leave non-ASCII characters raw there (verified against a
    // ~keep GitHub-rendered fixture, `gh-143-links-wordwrap.html`'s
    // ~keep `#walk-up-\u{fe0e}-and-walk-down-\u{fe0e}`) -- pre-encoding only the href while
    // ~keep the id it must match stays raw would break the anchor instead of normalizing it.
    #[test]
    fn append_markdown_link_angle_does_not_percent_encode_a_fragment_destination() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "#walk-up-\u{fe0e}", None, "text", &options, None);
        assert_eq!(out, "[text](#walk-up-\u{fe0e})");
    }

    #[test]
    fn append_markdown_link_angle_wraps_space_in_angle_brackets() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "file", "/file (1).pdf", None, "file", &options, None);
        assert_eq!(out, "[file](</file (1).pdf>)");
    }

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
    fn parens_are_balanced_accepts_nested_parens() {
        assert!(parens_are_balanced("wiki/Rust_(programming_language)"));
        assert!(parens_are_balanced("no/parens/here"));
    }

    #[test]
    fn parens_are_balanced_rejects_equal_counts_out_of_order() {
        // ~keep equal open/close counts are not sufficient for balance: a `)` before its `(`
        // ~keep is the exact naive-count bug this check replaces.
        assert!(!parens_are_balanced("a)b(c"));
    }

    #[test]
    fn parens_are_balanced_rejects_unmatched_open_or_close() {
        assert!(!parens_are_balanced("a(b"));
        assert!(!parens_are_balanced("a)b"));
    }

    #[test]
    fn append_markdown_link_angle_leaves_balanced_parens_unescaped_when_href_has_parens() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            "Rust",
            "https://en.wikipedia.org/wiki/Rust_(programming_language)",
            None,
            "Rust",
            &options,
            None,
        );
        assert_eq!(out, "[Rust](https://en.wikipedia.org/wiki/Rust_(programming_language))");
    }

    #[test]
    fn append_markdown_link_angle_escapes_out_of_order_parens_when_href_has_parens() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            "link",
            "http://example.com/a)(b",
            None,
            "link",
            &options,
            None,
        );
        assert_eq!(out, "[link](http://example.com/a\\)\\(b)");
    }

    #[test]
    fn append_markdown_link_angle_escapes_gt_inside_wrap_when_href_has_space_and_gt() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "/my file >.pdf", None, "text", &options, None);
        assert_eq!(out, "[text](</my file \\>.pdf>)");
    }

    #[test]
    fn append_markdown_link_angle_produces_empty_angle_brackets_when_href_is_empty() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "", None, "text", &options, None);
        assert_eq!(out, "[text](<>)");
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

    // ~keep Issue #498: `decoded_attribute` already decoded `&amp;plus;` to `&plus;` before this
    // ~keep function ever sees it, so these unit tests exercise it with already-decoded input --
    // ~keep exactly what a destination/title looks like by the time it reaches `append_url_destination`
    // ~keep or `escape_markdown_title`.
    #[test]
    fn escape_entity_ampersands_escapes_a_named_reference() {
        assert_eq!(escape_entity_ampersands("?&plus;"), "?&amp;plus;");
    }

    #[test]
    fn escape_entity_ampersands_escapes_a_decimal_numeric_reference() {
        assert_eq!(escape_entity_ampersands("?&#43;"), "?&amp;#43;");
    }

    #[test]
    fn escape_entity_ampersands_escapes_a_hex_numeric_reference() {
        assert_eq!(escape_entity_ampersands("?&#x2B;"), "?&amp;#x2B;");
    }

    #[test]
    fn escape_entity_ampersands_leaves_a_bare_ampersand_unchanged() {
        // ~keep `&b` has no terminating `;`, so it is never a reference candidate -- pins the
        // ~keep existing `?a=1&b=2` behavior (link.rs's `append_markdown_link_angle_...` tests).
        match escape_entity_ampersands("?a&b") {
            Cow::Borrowed(unchanged) => assert_eq!(unchanged, "?a&b"),
            Cow::Owned(owned) => panic!("expected no allocation, got {owned:?}"),
        }
    }

    #[test]
    fn escape_entity_ampersands_leaves_an_unrecognized_named_reference_unchanged() {
        assert_eq!(escape_entity_ampersands("&foo;"), "&foo;");
    }

    #[test]
    fn escape_entity_ampersands_escapes_amp_itself_since_it_is_a_valid_reference() {
        assert_eq!(escape_entity_ampersands("&amp;"), "&amp;amp;");
    }

    #[test]
    fn escape_markdown_title_escapes_backslash_before_quote_so_the_closing_quote_is_not_swallowed() {
        // ~keep audit #24 finding 8: a title ending in a literal `\` must not let a following `\"`
        // (backslash escaping the delimiter's quote) read as an escaped quote instead of the
        // closing delimiter.
        assert_eq!(escape_markdown_title("foo\\"), "foo\\\\");
        assert_eq!(escape_markdown_title("say \"hi\"\\"), "say \\\"hi\\\"\\\\");
    }

    #[test]
    fn append_markdown_link_escapes_a_trailing_backslash_in_title_so_the_closing_quote_is_not_swallowed() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "/url", Some("foo\\"), "text", &options, None);
        assert_eq!(out, "[text](/url \"foo\\\\\")");
    }

    #[test]
    fn append_markdown_link_escapes_a_trailing_backslash_in_default_title_so_the_closing_quote_is_not_swallowed() {
        let mut out = String::new();
        let mut options = opts_with_style(UrlEscapeStyle::Angle);
        options.default_title = true;
        append_markdown_link(&mut out, "text", "http://a\\", None, "http://a\\", &options, None);
        assert_eq!(out, "[text](http://a\\ \"http://a\\\\\")");
    }

    #[test]
    fn append_markdown_link_escapes_a_backslash_inside_the_angle_bracket_wrap_so_it_cannot_unescape_a_delimiter() {
        // ~keep audit #24 finding 8: inside an angle-bracket-wrapped destination, an unescaped `\`
        // immediately before an escaped `<`/`>` merges with it into a single `\\` escape pair,
        // un-escaping the delimiter and terminating the destination early.
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "/my file\\>.pdf", None, "text", &options, None);
        assert_eq!(out, "[text](</my file\\\\\\>.pdf>)");
    }

    // ~keep Regression for CommonMark spec fixpoint example 631: a `\` immediately followed by
    // ~keep ASCII punctuation in a *balanced* (non-angle-bracket) destination must be doubled,
    // ~keep or a compliant reparse consumes it as a backslash escape and silently drops the
    // ~keep byte -- `href="\*"` emitted verbatim as `\*` reparses as destination `*`.
    #[test]
    fn append_markdown_link_escapes_a_backslash_before_punctuation_in_a_balanced_destination() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "\\*", None, "text", &options, None);
        assert_eq!(out, "[text](\\\\*)");
    }

    // ~keep A `\` that is the *last* character of a balanced destination is left unescaped:
    // ~keep this call cannot see what byte `output` gains next (title quote or closing `)`), and
    // ~keep escaping it unconditionally was the audit #24 regression pinned by
    // ~keep `append_markdown_link_escapes_a_trailing_backslash_in_default_title_...` above.
    // ~keep Regression: a trailing `\` in a balanced destination with no title next escapes
    // ~keep the closing `)` on reparse (`\)` is a CommonMark backslash escape) unless doubled.
    // ~keep Left unescaped, `href="x\"` with no title emitted `(x\)` reparses as `\[t\](x)` --
    // ~keep the destination never closes and the whole link degrades to literal bracket text,
    // ~keep which is worse than example 631's single dropped byte. `title_follows` is `false`
    // ~keep here because the very next byte this call emits is the closing `)`.
    #[test]
    fn append_markdown_link_escapes_a_trailing_backslash_in_a_balanced_destination_with_no_title() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "/path\\", None, "text", &options, None);
        assert_eq!(out, "[text](/path\\\\)");
    }

    // ~keep A trailing `\` in a balanced destination is left unescaped when a title follows:
    // ~keep the next byte this call emits is a literal space, and `\ ` is not a CommonMark
    // ~keep backslash escape (only ASCII punctuation may be escaped), so the `\` is already
    // ~keep safe. Escaping it here would be the audit #24 regression pinned by
    // ~keep `append_markdown_link_escapes_a_trailing_backslash_in_default_title_...` above.
    #[test]
    fn append_markdown_link_leaves_a_trailing_backslash_in_a_balanced_destination_unescaped_when_a_title_follows() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(&mut out, "text", "/path\\", Some("t"), "text", &options, None);
        assert_eq!(out, "[text](/path\\ \"t\")");
    }
}
