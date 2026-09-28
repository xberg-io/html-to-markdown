//! Handler for metadata and script elements (head, script, style, math).
//!
//! Converts various metadata-related elements:
//! - **head**: Document metadata container; processes script[type="application/ld+json"]
//! - **script**: Script elements; extracts JSON-LD structured data when appropriate
//! - **style**: CSS stylesheet elements; skipped in conversion
//! - **math**: `MathML` elements with serialization and HTML comments for preservation

use crate::converter::media::svg::serialize_element;
use crate::options::ConversionOptions;
#[cfg(feature = "metadata")]
use crate::text::decode_attribute_value_cow;
use crate::text::escape;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handles metadata elements: head, script, style, math.
///
/// Processes various metadata-related elements:
/// - head: Scans for structured data in script[type="application/ld+json"]
/// - script: Extracts JSON-LD for structured data collection
/// - style: Skipped (CSS not relevant in markdown)
/// - math: Preserves `MathML` as HTML comments with text content
pub fn handle(
    tag_name: &str,
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    match tag_name {
        "head" => {
            handle_head(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "script" => {
            handle_script(node_handle, parser, output, options, ctx);
        }
        "style" => {}
        "math" => {
            handle_math(node_handle, parser, output, options, ctx, dom_ctx);
        }
        _ => {}
    }
}

/// Handle head element.
///
/// Head elements contain metadata. We process them to extract structured data from
/// nested script[type="application/ld+json"] elements if metadata collection is enabled.
fn handle_head(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    use crate::converter::walk_node;

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    let children = tag.children();
    let has_body_like = children.top().iter().any(|child_handle| {
        if let Some(child_name) = dom_ctx.tag_name_for(*child_handle, parser) {
            matches!(
                child_name.as_ref(),
                "body" | "main" | "article" | "section" | "div" | "p"
            )
        } else {
            false
        }
    });

    #[cfg(feature = "metadata")]
    if ctx.metadata_wants_structured_data {
        if let Some(ref collector) = ctx.metadata_collector {
            for child_handle in children.top().iter() {
                if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                    let child_name = dom_ctx
                        .tag_name_for(*child_handle, parser)
                        .unwrap_or_else(|| crate::converter::normalized_tag_name(child_tag.name().as_utf8_str()));
                    if child_name.as_ref() == "script" {
                        if let Some(type_attr) = child_tag.attributes().get("type").flatten() {
                            let type_value = type_attr.as_utf8_str();
                            let type_value = type_value.as_ref();
                            let type_value = type_value.split(';').next().unwrap_or(type_value);
                            if type_value.trim().eq_ignore_ascii_case("application/ld+json") {
                                let json = child_tag.inner_text(parser);
                                let json = json.trim();
                                if !json.is_empty() {
                                    let json = decode_attribute_value_cow(json).into_owned();
                                    if !json.is_empty() {
                                        collector.borrow_mut().add_json_ld(json);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if has_body_like {
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    }
}

/// Handle script element.
///
/// Script elements are processed to extract JSON-LD structured data when
/// the type is "application/ld+json" and metadata collection is enabled.
#[cfg_attr(not(feature = "metadata"), allow(unused_variables))]
fn handle_script(
    node_handle: &NodeHandle,
    parser: &Parser,
    _output: &mut String,
    _options: &ConversionOptions,
    ctx: &Context,
) {
    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    #[cfg(feature = "metadata")]
    if let Some(type_attr) = tag.attributes().get("type").flatten() {
        let type_value = type_attr.as_utf8_str();
        let type_value = type_value.as_ref();
        let type_value = type_value.split(';').next().unwrap_or(type_value);
        if type_value.trim().eq_ignore_ascii_case("application/ld+json") && ctx.metadata_wants_structured_data {
            if let Some(ref collector) = ctx.metadata_collector {
                let json = tag.inner_text(parser);
                let json = json.trim();
                if !json.is_empty() {
                    let json = decode_attribute_value_cow(json).into_owned();
                    if !json.is_empty() {
                        collector.borrow_mut().add_json_ld(json);
                    }
                }
            }
        }
    }
}

/// Handle math element.
///
/// `MathML` elements are serialized to HTML and wrapped in a comment to preserve them.
/// The text content of the element is also output as plain text.
fn handle_math(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    dom_ctx: &DomContext,
) {
    let text_content = crate::converter::get_text_content(node_handle, parser, dom_ctx)
        .trim()
        .to_string();

    if text_content.is_empty() {
        return;
    }

    let math_html = serialize_element(node_handle, parser);

    let escaped_text = escape(
        &text_content,
        options.escape_misc,
        options.escape_asterisks,
        options.escape_underscores,
        options.escape_ascii,
    );

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

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
