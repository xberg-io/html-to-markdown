//! Graphic element handler for HTML to Markdown conversion.
//!
//! Handles `<graphic>` elements including:
//! - Alternative source attributes (url, href, xlink:href, src)
//! - Fallback alt text from filename attribute
//! - Metadata collection for graphic extraction
//! - Visitor callback integration

use std::borrow::Cow;
#[cfg(any(feature = "metadata", feature = "visitor"))]
use std::collections::BTreeMap;

use crate::converter::Context;
use crate::converter::dom_context::DomContext;
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
    // Check source attributes in order: url, href, xlink:href, src
    let src = tag
        .attributes()
        .get("url")
        .flatten()
        .or_else(|| tag.attributes().get("href").flatten())
        .or_else(|| tag.attributes().get("xlink:href").flatten())
        .or_else(|| tag.attributes().get("src").flatten())
        .map_or(Cow::Borrowed(""), |v| v.as_utf8_str());

    // Use "alt" attribute, fallback to "filename"
    let alt = tag
        .attributes()
        .get("alt")
        .flatten()
        .map(|v| v.as_utf8_str())
        .or_else(|| tag.attributes().get("filename").flatten().map(|v| v.as_utf8_str()))
        .unwrap_or(Cow::Borrowed(""));

    let title = tag.attributes().get("title").flatten().map(|v| v.as_utf8_str());

    // Collect metadata payload if metadata feature is enabled
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

    let keep_as_markdown = ctx.in_heading && ctx.heading_allow_inline_images;

    let should_use_alt_text =
        !keep_as_markdown && (ctx.convert_as_inline || (ctx.in_heading && !ctx.heading_allow_inline_images));

    // Generate graphic output with visitor integration
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
        ctx.reference_collector.as_ref(),
    ));

    if !options.skip_images {
        if let Some(graphic_text) = graphic_output {
            output.push_str(&graphic_text);
        }
    }

    // Add graphic to metadata collector
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
    reference_collector: Option<&crate::converter::reference_collector::ReferenceCollectorHandle>,
) -> String {
    if use_alt_only {
        return alt.to_string();
    }
    if link_style == crate::options::validation::LinkStyle::Reference {
        if let Some(collector) = reference_collector {
            let ref_num = collector.borrow_mut().get_or_insert(src, title);
            let mut buf = String::with_capacity(alt.len() + 10);
            buf.push_str("![");
            buf.push_str(alt);
            buf.push_str("][");
            buf.push_str(&ref_num.to_string());
            buf.push(']');
            return buf;
        }
    }
    let mut buf = String::with_capacity(src.len() + alt.len() + 10);
    buf.push_str("![");
    buf.push_str(alt);
    buf.push_str("](");
    buf.push_str(src);
    if let Some(title_text) = title {
        buf.push_str(" \"");
        buf.push_str(title_text);
        buf.push('"');
    }
    buf.push(')');
    buf
}
