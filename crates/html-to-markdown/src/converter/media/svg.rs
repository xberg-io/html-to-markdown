//! SVG and MathML element handling with serialization and base64 encoding.

use crate::converter::main_helpers::tag_name_eq;
use crate::converter::utility::content::normalized_tag_name;
#[allow(unused_imports)]
use std::collections::BTreeMap;
use tl::{NodeHandle, Parser};

#[cfg(feature = "inline-images")]
use crate::inline_images::{InlineImageCollector, InlineImageFormat, InlineImageSource};

#[cfg(feature = "inline-images")]
type InlineCollectorHandle = std::rc::Rc<std::cell::RefCell<InlineImageCollector>>;

/// Handle inline SVG elements with size limits and base64 encoding.
///
/// # Features
/// - SVG serialization to HTML string
/// - Size validation with configurable limits
/// - Base64 encoding for data URI
/// - Metadata extraction (aria-label, title, dimensions)
#[cfg(feature = "inline-images")]
#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::option_if_let_else)]
pub fn handle_inline_svg(
    collector_ref: &InlineCollectorHandle,
    node_handle: &NodeHandle,
    parser: &Parser,
    title_opt: Option<String>,
    attributes: BTreeMap<String, String>,
) {
    let max_size = {
        let borrow = collector_ref.borrow();
        if !borrow.capture_svg() {
            return;
        }
        borrow.max_decoded_size()
    };

    if max_size == 0 {
        let mut collector = collector_ref.borrow_mut();
        let index = collector.next_index();
        collector.warn_skip(index, "max SVG payload size is zero");
        return;
    }

    let mut collector = collector_ref.borrow_mut();
    let index = collector.next_index();

    let serialized = serialize_element(node_handle, parser);
    if serialized.is_empty() {
        collector.warn_skip(index, "unable to serialize SVG element");
        return;
    }

    let data = serialized.into_bytes();
    if data.len() as u64 > max_size {
        collector.warn_skip(
            index,
            format!(
                "serialized SVG payload ({} bytes) exceeds configured max ({})",
                data.len(),
                max_size
            ),
        );
        return;
    }

    let description = attributes
        .get("aria-label")
        .and_then(|value| non_empty_trimmed(value))
        .or_else(|| title_opt.as_deref().and_then(non_empty_trimmed));

    let filename_candidate = attributes
        .get("data-filename")
        .cloned()
        .or_else(|| attributes.get("filename").cloned())
        .or_else(|| attributes.get("data-name").cloned());

    let image = collector.build_image(
        data,
        InlineImageFormat::Svg,
        filename_candidate,
        description,
        None,
        InlineImageSource::SvgElement,
        attributes,
    );

    collector.push_image(index, image);
}

/// Serialize an element to HTML string (for SVG and Math elements).
///
/// Attributes are sorted by name to guarantee deterministic output across
/// process invocations (the underlying parser stores them in a `HashMap`).
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_element(node_handle: &NodeHandle, parser: &Parser) -> String {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let tag_name = normalized_tag_name(tag.name().as_utf8_str());
        let mut html = String::with_capacity(256);
        html.push('<');
        html.push_str(&tag_name);

        let mut attrs: Vec<_> = tag.attributes().iter().collect();
        attrs.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
        for (key, value_opt) in attrs {
            html.push(' ');
            html.push_str(&key);
            if let Some(value) = value_opt {
                html.push_str("=\"");
                html.push_str(&value);
                html.push('"');
            }
        }

        let has_children = !tag.children().top().is_empty();
        if has_children {
            html.push('>');
            let children = tag.children();
            {
                for child_handle in children.top().iter() {
                    html.push_str(&serialize_node(child_handle, parser));
                }
            }
            html.push_str("</");
            html.push_str(&tag_name);
            html.push('>');
        } else {
            html.push_str(" />");
        }
        return html;
    }
    String::new()
}

/// Serialize a node to HTML string.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_node(node_handle: &NodeHandle, parser: &Parser) -> String {
    if let Some(node) = node_handle.get(parser) {
        match node {
            tl::Node::Raw(bytes) => bytes.as_utf8_str().to_string(),
            tl::Node::Tag(_) => serialize_element(node_handle, parser),
            _ => String::new(),
        }
    } else {
        String::new()
    }
}

/// Extract non-empty trimmed string or return None.
#[cfg(feature = "inline-images")]
fn non_empty_trimmed(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Handle SVG element conversion to Markdown.
///
/// Extracts title from child elements, handles inline image collection,
/// and outputs either the title text (in inline mode) or a base64-encoded image.
#[allow(clippy::too_many_arguments)]
pub fn handle_svg(
    node_handle: &NodeHandle,
    tag: &tl::HTMLTag,
    parser: &Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    _depth: usize,
    dom_ctx: &super::DomContext,
) {
    use crate::converter::utility::content::get_text_content;

    let mut title = String::from("SVG Image");
    let children = tag.children();
    for child_handle in children.top().iter() {
        if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
            if tag_name_eq(child_tag.name().as_utf8_str(), "title") {
                title = get_text_content(child_handle, parser, dom_ctx).trim().to_string();
                break;
            }
        }
    }

    #[cfg(feature = "inline-images")]
    if let Some(ref collector_ref) = ctx.inline_collector {
        let title_opt = if title == "SVG Image" {
            None
        } else {
            Some(title.clone())
        };
        let mut attributes_map = BTreeMap::new();
        for (key, value_opt) in tag.attributes().iter() {
            let key_str = key.to_string();
            let keep = key_str == "width"
                || key_str == "height"
                || key_str == "filename"
                || key_str == "aria-label"
                || key_str.starts_with("data-");
            if keep {
                let value = value_opt.map(|value| value.to_string()).unwrap_or_default();
                attributes_map.insert(key_str, value);
            }
        }
        handle_inline_svg(collector_ref, node_handle, parser, title_opt, attributes_map);
    }

    if options.skip_images {
        return;
    }

    if ctx.convert_as_inline {
        output.push_str(&title);
    } else {
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let svg_html = serialize_element(node_handle, parser);
        let base64_svg = STANDARD.encode(svg_html.as_bytes());

        output.push_str("![");
        output.push_str(&title);
        output.push_str("](data:image/svg+xml;base64,");
        output.push_str(&base64_svg);
        output.push(')');
    }
}

/// Handle MathML element conversion to Markdown.
///
/// Serializes MathML to HTML comment and outputs text content with escaping.
#[allow(clippy::too_many_arguments)]
pub fn handle_math(
    node_handle: &NodeHandle,
    tag: &tl::HTMLTag,
    parser: &Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    _depth: usize,
    dom_ctx: &super::DomContext,
) {
    use crate::converter::utility::content::get_text_content;
    use crate::text;

    let text_content = get_text_content(node_handle, parser, dom_ctx).trim().to_string();

    if text_content.is_empty() {
        return;
    }

    let math_html = serialize_element(node_handle, parser);

    let escaped_text = text::escape(
        &text_content,
        options.escape_misc,
        options.escape_asterisks,
        options.escape_underscores,
        options.escape_ascii,
    );

    let is_display_block = tag
        .attributes()
        .get("display")
        .flatten()
        .is_some_and(|v| v.as_utf8_str() == "block");

    if is_display_block && !ctx.in_paragraph && !ctx.convert_as_inline {
        output.push_str("\n\n");
    }

    output.push_str("<!-- MathML: ");
    output.push_str(&math_html);
    output.push_str(" --> ");
    output.push_str(&escaped_text);

    if is_display_block && !ctx.in_paragraph && !ctx.convert_as_inline {
        output.push_str("\n\n");
    }
}
