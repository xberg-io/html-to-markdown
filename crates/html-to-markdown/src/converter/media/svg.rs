//! SVG and `MathML` element handling with serialization and base64 encoding.

use crate::converter::main_helpers::{effective_max_depth, tag_name_eq};
use crate::converter::utility::content::normalized_tag_name;
use crate::converter::utility::escaping::escape_link_label;
use crate::converter::utility::serialization::escape_html_attribute_value;
use crate::converter::utility::svg_attrs::canonical_svg_attr;
use crate::options::conversion::NATIVE_STACK_SAFE_DEPTH;
// ~keep reason: BTreeMap is only used when the inline-images feature is active.
#[allow(unused_imports)]
use std::collections::BTreeMap;
use tl::{NodeHandle, Parser};

#[cfg(feature = "inline-images")]
use crate::inline_images::{InlineImageBuild, InlineImageCollector, InlineImageFormat, InlineImageSource};

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

    let image = collector.build_image(InlineImageBuild {
        data,
        format: InlineImageFormat::Svg,
        filename: filename_candidate,
        description,
        dimensions: None,
        source: InlineImageSource::SvgElement,
        attributes,
    });

    collector.push_image(index, image);
}

/// Serialize an element to HTML string (for SVG and Math elements).
///
/// Attributes are sorted by name to guarantee deterministic output across
/// process invocations (the underlying parser stores them in a `HashMap`).
///
/// Depth-guarded by [`NATIVE_STACK_SAFE_DEPTH`]. Callers that already track the
/// element's depth in the wider DOM walk (`handle_svg`, `handle_math`) should call
/// [`serialize_element_at_depth`] instead, so the same recursion budget the caller is
/// already spending against `effective_max_depth` is honored here too.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_element(node_handle: &NodeHandle, parser: &Parser) -> String {
    serialize_element_at_depth(node_handle, parser, 0, NATIVE_STACK_SAFE_DEPTH)
}

/// Serialize a node to HTML string.
///
/// See [`serialize_element`] for the depth-guard contract.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_node(node_handle: &NodeHandle, parser: &Parser) -> String {
    serialize_node_at_depth(node_handle, parser, 0, NATIVE_STACK_SAFE_DEPTH)
}

/// Serialize an element to HTML string, stopping descent once `depth` reaches `max_depth`.
///
/// Mutually recursive with [`serialize_node_at_depth`] over `tag.children()`. Follows the
/// same convention as the main DOM walker (`walk_node` in `converter/main.rs`): the depth
/// counter increments on every descent into a child, and reaching the limit stops further
/// descent rather than erroring, so a pathologically nested `<svg>`/`<math>` subtree cannot
/// overflow the stack (audit #23). The element's own opening tag and attributes are always
/// emitted; only its descendants are dropped once the budget is exhausted.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_element_at_depth(node_handle: &NodeHandle, parser: &Parser, depth: usize, max_depth: usize) -> String {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let tag_name = normalized_tag_name(tag.name().as_utf8_str());
        let mut html = String::with_capacity(256);
        html.push('<');
        html.push_str(&tag_name);

        let mut attrs: Vec<_> = tag.attributes().iter().collect();
        attrs.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
        for (key, value_opt) in attrs {
            html.push(' ');
            // ~keep Restore camelCase for SVG/MathML attributes whose canonical
            // ~keep WHATWG spelling is mixed-case.  tl lowercases all attribute
            // ~keep names when it re-parses a wrapped fragment (Tier-1 path via
            // ~keep emit_svg_from_slice), but preserves case on a full-document
            // ~keep parse (Tier-2 path).  Applying the lookup in both paths is
            // ~keep safe: Tier-2 already has the correct spelling so the lookup
            // ~keep returns None and the original key is used unchanged.
            let key_str = key.as_ref();
            let canonical = canonical_svg_attr(key_str);
            html.push_str(canonical.unwrap_or(key_str));
            if let Some(value) = value_opt {
                // ~keep Treat empty value identically to a bare attribute.  When tl
                // ~keep re-parses a wrapped SVG slice (Tier-1's emit_svg_from_slice)
                // ~keep it yields `None` for `attr=""` while a single full-document
                // ~keep parse (Tier-2) yields `Some("")`.  Both forms are
                // ~keep HTML5-equivalent; normalise here so both tiers produce
                // ~keep byte-identical output.
                if !value.is_empty() {
                    html.push_str("=\"");
                    html.push_str(&escape_html_attribute_value(&value));
                    html.push('"');
                }
            }
        }

        let has_children = !tag.children().top().is_empty();
        if has_children {
            html.push('>');
            if depth >= max_depth {
                tracing::warn!(
                    target: "html_to_markdown::convert",
                    max_depth,
                    tag = %tag_name,
                    "SVG/MathML serialization reached the effective depth limit; descendants were skipped"
                );
            } else {
                let children = tag.children();
                for child_handle in children.top().iter() {
                    html.push_str(&serialize_node_at_depth(child_handle, parser, depth + 1, max_depth));
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

/// Serialize a node to HTML string, stopping descent once `depth` reaches `max_depth`.
///
/// See [`serialize_element_at_depth`] for the depth-guard contract.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_node_at_depth(node_handle: &NodeHandle, parser: &Parser, depth: usize, max_depth: usize) -> String {
    if let Some(node) = node_handle.get(parser) {
        match node {
            tl::Node::Raw(bytes) => bytes.as_utf8_str().to_string(),
            tl::Node::Tag(_) => serialize_element_at_depth(node_handle, parser, depth, max_depth),
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
    depth: usize,
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

        let svg_html = serialize_element_at_depth(node_handle, parser, depth, effective_max_depth(options));
        let base64_svg = STANDARD.encode(svg_html.as_bytes());

        output.push_str("![");
        output.push_str(&escape_link_label(&title));
        output.push_str("](data:image/svg+xml;base64,");
        output.push_str(&base64_svg);
        output.push(')');
    }
}

/// Handle `MathML` element conversion to Markdown.
///
/// Serializes `MathML` to HTML comment and outputs text content with escaping.
#[allow(clippy::too_many_arguments)]
pub fn handle_math(
    node_handle: &NodeHandle,
    tag: &tl::HTMLTag,
    parser: &Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    use crate::converter::utility::content::get_text_content;
    use crate::text;

    let text_content = get_text_content(node_handle, parser, dom_ctx).trim().to_string();

    if text_content.is_empty() {
        return;
    }

    let math_html = serialize_element_at_depth(node_handle, parser, depth, effective_max_depth(options));

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

#[cfg(test)]
mod tests {
    #[test]
    fn should_escape_svg_title_caption_that_would_open_a_new_image_link() {
        // ~keep audit #24 finding 5: the `<svg><title>` used as the data-URI image caption was
        // pushed unescaped, so an inert `]`/`(` in it could manufacture a second, live image
        // pointing at an attacker-controlled URL — same mechanism as finding 1 (`<img alt>`).
        let html = "<svg><title>a](https://evil.example/payload)</title></svg>";
        let result = crate::convert(html, None).unwrap();
        let content = result.content.unwrap_or_default();
        assert_eq!(
            content,
            "![a\\](https://evil.example/payload)](data:image/svg+xml;base64,\
             PHN2Zz48dGl0bGU+YV0oaHR0cHM6Ly9ldmlsLmV4YW1wbGUvcGF5bG9hZCk8L3RpdGxlPjwvc3ZnPg==)\n"
        );
    }
}

#[cfg(test)]
mod attribute_escaping_tests {
    use super::serialize_element;

    #[test]
    fn should_escape_a_quote_inside_a_reconstructed_attribute_value() {
        // ~keep audit #24 finding 3 (media/svg.rs duplicate): the source HTML's `"` is valid,
        // inert content inside a single-quoted attribute. Reconstructing it into a
        // double-quoted attribute without escaping manufactures a real `onclick` that never
        // existed as an attribute in the original document.
        let html = r#"<foo title='x" onclick="alert(1)" y=' data-safe="1">"#;
        let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let node_handle = dom
            .children()
            .iter()
            .find(|handle| matches!(handle.get(parser), Some(tl::Node::Tag(_))))
            .expect("tag node");
        let result = serialize_element(node_handle, parser);
        assert_eq!(
            result,
            "<foo data-safe=\"1\" title=\"x&quot; onclick=&quot;alert(1)&quot; y=\" />"
        );
    }
}
