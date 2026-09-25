//! Graphic element handler for HTML to Markdown conversion.
//!
//! Handles `<graphic>` elements including:
//! - Alternative source attributes (url, href, xlink:href, src)
//! - Fallback alt text from filename attribute
//! - Metadata collection for graphic extraction
//! - Visitor callback integration

use std::borrow::Cow;
#[cfg(feature = "metadata")]
use std::collections::BTreeMap;

use crate::converter::Context;
use crate::converter::dom_context::DomContext;
use crate::converter::inline::link::{append_url_destination, escape_markdown_title};
use crate::converter::utility::content::escape_link_label;
use crate::converter::utility::preprocessing::sanitize_markdown_url;
use crate::options::ConversionOptions;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node;

#[cfg(feature = "metadata")]
type GraphicMetadataPayload = (BTreeMap<String, String>, Option<u32>, Option<u32>);

/// Handle a `<graphic>` element and convert to Markdown.
///
/// This handler processes graphic elements including:
/// - Extracting source from url, href, xlink:href, or src attributes
/// - Using alt attribute, with fallback to filename
/// - Collecting metadata when the metadata feature is enabled
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Generating appropriate markdown output
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_graphic(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let src = ["url", "href", "xlink:href", "src"]
        .into_iter()
        .find_map(|name| crate::converter::utility::attributes::decoded_attribute(tag, name))
        .map_or(Cow::Borrowed(""), |s| {
            let resolved = ctx.resolve_url(&s);
            Cow::Owned(sanitize_markdown_url(resolved.as_deref().unwrap_or(&s)).into_owned())
        });

    // ~keep Use "alt" attribute, fallback to "filename"
    let alt = crate::converter::utility::attributes::decoded_attribute(tag, "alt")
        .or_else(|| crate::converter::utility::attributes::decoded_attribute(tag, "filename"))
        .unwrap_or(Cow::Borrowed(""));

    // ~keep An empty `title=""` carries no information, and `[t](u "")` / `![a](i "")` is
    // ~keep noise that no Markdown serializer round-trips: re-rendering the output drops
    // ~keep the empty title, so the second pass no longer matches the first. Treat it as
    // ~keep absent, which is what it means.
    let title = crate::converter::utility::attributes::decoded_attribute(tag, "title").filter(|v| !v.is_empty());

    #[cfg(feature = "metadata")]
    #[allow(clippy::useless_let_if_seq)]
    let mut metadata_payload: Option<GraphicMetadataPayload> = None;
    #[cfg(feature = "metadata")]
    if ctx.metadata_wants_images {
        let mut attributes_map = BTreeMap::new();
        let mut width: Option<u32> = None;
        let mut height: Option<u32> = None;
        for (key, value_opt) in tag.attributes().iter() {
            let key_str = key.to_string();
            if key_str == "url" || key_str == "href" || key_str == "xlink:href" || key_str == "src" {
                continue;
            }
            let value = value_opt.map(|v| v.to_string()).unwrap_or_default();
            if key_str == "width" {
                if let Ok(parsed) = value.parse::<u32>() {
                    width = Some(parsed);
                }
            } else if key_str == "height" {
                if let Ok(parsed) = value.parse::<u32>() {
                    height = Some(parsed);
                }
            }
            attributes_map.insert(key_str, value);
        }
        metadata_payload = Some((attributes_map, width, height));
    }

    // ~keep #492: byte-identical twin of `handlers/image.rs`'s `keep_as_markdown` -- see
    // ~keep that file's comment for why `|| ctx.link_allow_inline_images` is purely additive
    // ~keep and why the heading pattern's negative clause is deliberately not mirrored here.
    // ~keep Kept in lockstep so `<graphic>` and `<img>` never disagree inside the same anchor.
    let keep_as_markdown = (ctx.in_heading && ctx.heading_allow_inline_images)
        || ctx.cell_allow_inline_images
        || ctx.link_allow_inline_images;

    let should_use_alt_text =
        !keep_as_markdown && (ctx.convert_as_inline || (ctx.in_heading && !ctx.heading_allow_inline_images));

    #[cfg(feature = "visitor")]
    let graphic_output = if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Image,
            Cow::Borrowed("graphic"),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_image(&node_ctx, &src, &alt, title.as_deref())
        };
        match visit_result {
            VisitResult::Continue => Some(format_graphic_markdown(
                &src,
                &alt,
                title.as_deref(),
                should_use_alt_text,
                options.link_style,
                options.url_escape_style,
                ctx.reference_collector.as_ref(),
            )),
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
        Some(format_graphic_markdown(
            &src,
            &alt,
            title.as_deref(),
            should_use_alt_text,
            options.link_style,
            options.url_escape_style,
            ctx.reference_collector.as_ref(),
        ))
    };

    #[cfg(not(feature = "visitor"))]
    let graphic_output = Some(format_graphic_markdown(
        &src,
        &alt,
        title.as_deref(),
        should_use_alt_text,
        options.link_style,
        options.url_escape_style,
        ctx.reference_collector.as_ref(),
    ));

    if !options.skip_images {
        if let Some(graphic_text) = graphic_output {
            output.push_str(&graphic_text);
        }
    }

    #[cfg(feature = "metadata")]
    if ctx.metadata_wants_images {
        if let Some(ref collector) = ctx.metadata_collector {
            if let Some((attributes_map, width, height)) = metadata_payload {
                if !src.is_empty() {
                    let dimensions = match (width, height) {
                        (Some(w), Some(h)) => Some(crate::metadata::ImageDimensions { width: w, height: h }),
                        _ => None,
                    };
                    collector.borrow_mut().add_image(
                        src.to_string(),
                        if alt.is_empty() { None } else { Some(alt.to_string()) },
                        title.as_deref().map(std::string::ToString::to_string),
                        dimensions,
                        attributes_map,
                    );
                }
            }
        }
    }
}

/// Format a graphic as Markdown syntax.
///
/// If `use_alt_only` is true, returns just the alt text.
/// Otherwise returns the full `![alt](src "title")` syntax.
fn format_graphic_markdown(
    src: &str,
    alt: &str,
    title: Option<&str>,
    use_alt_only: bool,
    link_style: crate::options::validation::LinkStyle,
    url_escape_style: crate::options::validation::UrlEscapeStyle,
    reference_collector: Option<&crate::converter::reference_collector::ReferenceCollectorHandle>,
) -> String {
    if use_alt_only {
        return alt.to_string();
    }
    let escaped_alt = escape_link_label(alt);
    if link_style == crate::options::validation::LinkStyle::Reference {
        if let Some(collector) = reference_collector {
            let ref_num = collector.borrow_mut().get_or_insert(src, title);
            let mut buf = String::with_capacity(escaped_alt.len() + 10);
            buf.push_str("![");
            buf.push_str(&escaped_alt);
            buf.push_str("][");
            buf.push_str(&ref_num.to_string());
            buf.push(']');
            return buf;
        }
    }
    let mut buf = String::with_capacity(src.len() + escaped_alt.len() + 10);
    buf.push_str("![");
    buf.push_str(&escaped_alt);
    buf.push_str("](");
    append_url_destination(&mut buf, src, url_escape_style, title.is_some());
    if let Some(title_text) = title {
        buf.push_str(" \"");
        buf.push_str(&escape_markdown_title(title_text));
        buf.push('"');
    }
    buf.push(')');
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::validation::{LinkStyle, UrlEscapeStyle};

    #[test]
    fn should_escape_alt_text_that_would_close_the_graphic_and_open_a_new_link() {
        // ~keep audit #24 finding 6: `<graphic alt>` had the same unescaped-label bug as `<img alt>`
        // (finding 1) — an inert `]`/`(` in alt must not manufacture a second, live link.
        let result = format_graphic_markdown(
            "x.svg",
            "a](https://evil.example/payload)",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![a\\](https://evil.example/payload)](x.svg)");
    }

    #[test]
    fn should_escape_a_quote_in_the_title_that_would_open_a_real_link_after_it() {
        // ~keep audit #24 finding 4: `<graphic title>` had zero quote escaping.
        let result = format_graphic_markdown(
            "a.svg",
            "diagram",
            Some("x\" [click](https://evil.example)"),
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![diagram](a.svg \"x\\\" [click](https://evil.example)\")");
    }

    #[test]
    fn should_reject_out_of_order_parens_in_src() {
        // ~keep audit #24 finding 6/7: `<graphic src>` previously had no paren handling at all
        // (worse than `<img src>`'s naive count check).
        let result = format_graphic_markdown(
            "a)(b.svg",
            "alt",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![alt](a\\)\\(b.svg)");
    }

    #[test]
    fn should_sanitize_a_markdown_like_src_the_same_way_img_does() {
        // ~keep audit #24 finding 6: `<graphic src>` never went through `sanitize_markdown_url`,
        // unlike `<img src>` (image.rs). Verified end-to-end since sanitization happens at
        // attribute-extraction time in `handle_graphic`, before `format_graphic_markdown`.
        let html = r#"<p><graphic url="[p](https://example.com/real)" alt="pic" /></p>"#;
        let result = crate::convert(html, None).unwrap();
        let content = result.content.unwrap_or_default();
        assert_eq!(content, "![pic](https://example.com/real)\n");
    }
}
