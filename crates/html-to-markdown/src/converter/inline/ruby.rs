//! Handler for ruby annotation inline elements (ruby, rb, rt, rp, rtc).
//!
//! Converts HTML ruby annotation elements to Markdown format with support for:
//! - Ruby base text elements (<ruby>, <rb>)
//! - Ruby text annotations (<rt>) for phonetic guidance (common in CJK)
//! - Ruby parentheses (<rp>) for fallback presentation in browsers without ruby support
//! - Ruby text container (<rtc>) for secondary annotations or separate ruby text grouping
//! - Interleaved rendering mode: rb/rt pairs rendered inline (rb1(rt1)rb2(rt2))
//! - Grouped rendering mode: all rb text followed by rt annotations in parentheses
//! - Proper handling of CJK (Chinese/Japanese/Korean) text with multiple annotations
//! - Visitor callbacks for custom ruby processing
//! - Whitespace normalization and trimming

use crate::options::ConversionOptions;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handles ruby annotation elements: ruby, rb, rt, rp, rtc.
///
/// Ruby annotations are used in East Asian typography to show pronunciation guides
/// or provide alternate text. The handler supports two rendering modes:
///
/// # Rendering Modes
///
/// **Interleaved mode** (when rb and rt elements are alternated without rtc):
/// - Renders ruby text inline with base text: `base(annotation)base(annotation)`
/// - Example: `<ruby><rb>漢</rb><rt>かん</rt></ruby>` → `漢(かん)`
///
/// **Grouped mode** (when rtc is present or rb/rt are not interleaved):
/// - Renders all base text first, then all annotations in parentheses: `base(annotation1annotation2)`
/// - Handles multiple rt elements and rtc (ruby text container) grouping
/// - Example: `<ruby><rb>東</rb><rb>京</rb><rt>とう</rt><rt>きょう</rt></ruby>` → `東京(とうきょう)`
///
/// # Element Handling
///
/// - `<ruby>`: Main container, detects layout and delegates to appropriate rendering mode
/// - `<rb>`: Base text; content is extracted and used in output
/// - `<rt>`: Annotation text; wrapped in parentheses in standalone contexts
/// - `<rp>`: Ruby parentheses (fallback for browsers without ruby support); skipped in most contexts
/// - `<rtc>`: Ruby text container for grouped annotations; content extracted after rt annotations
///
/// # Note
/// This function references `walk_node` and `normalized_tag_name` from converter.rs,
/// which must be accessible (pub(crate)) for this module to work correctly.
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
    use crate::converter::{normalized_tag_name, walk_node};

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    match tag_name {
        "ruby" => {
            let ruby_ctx = ctx.inline_buffer(output, false);

            let tag_sequence: Vec<String> = tag
                .children()
                .top()
                .iter()
                .filter_map(|child_handle| {
                    if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                        let tag_name = normalized_tag_name(child_tag.name().as_utf8_str());
                        if matches!(tag_name.as_ref(), "rb" | "rt" | "rtc") {
                            Some(tag_name.into_owned())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            let has_rtc = tag_sequence.iter().any(|tag| tag == "rtc");

            let is_interleaved = tag_sequence.windows(2).any(|w| w[0] == "rb" && w[1] == "rt");

            if is_interleaved && !has_rtc {
                let mut current_base = String::new();
                let children = tag.children();
                {
                    for child_handle in children.top().iter() {
                        if let Some(node) = child_handle.get(parser) {
                            match node {
                                tl::Node::Tag(child_tag) => {
                                    let tag_name = normalized_tag_name(child_tag.name().as_utf8_str());
                                    if tag_name == "rt" {
                                        let mut annotation = String::new();
                                        walk_node(
                                            child_handle,
                                            parser,
                                            &mut annotation,
                                            options,
                                            &ruby_ctx,
                                            depth + 1,
                                            dom_ctx,
                                        );
                                        if !current_base.is_empty() {
                                            output.push_str(current_base.trim());
                                            current_base.clear();
                                        }
                                        output.push_str(annotation.trim());
                                    } else if tag_name == "rb" {
                                        if !current_base.is_empty() {
                                            output.push_str(current_base.trim());
                                            current_base.clear();
                                        }
                                        walk_node(
                                            child_handle,
                                            parser,
                                            &mut current_base,
                                            options,
                                            &ruby_ctx,
                                            depth + 1,
                                            dom_ctx,
                                        );
                                    } else if tag_name != "rp" {
                                        walk_node(
                                            child_handle,
                                            parser,
                                            &mut current_base,
                                            options,
                                            &ruby_ctx,
                                            depth + 1,
                                            dom_ctx,
                                        );
                                    }
                                }
                                tl::Node::Raw(_) => {
                                    walk_node(
                                        child_handle,
                                        parser,
                                        &mut current_base,
                                        options,
                                        &ruby_ctx,
                                        depth + 1,
                                        dom_ctx,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
                if !current_base.is_empty() {
                    output.push_str(current_base.trim());
                }
            } else {
                let mut base_text = String::new();
                let mut rt_annotations = Vec::new();
                let mut rtc_content = String::new();

                let children = tag.children();
                {
                    for child_handle in children.top().iter() {
                        if let Some(node) = child_handle.get(parser) {
                            match node {
                                tl::Node::Tag(child_tag) => {
                                    let tag_name = normalized_tag_name(child_tag.name().as_utf8_str());
                                    if tag_name == "rt" {
                                        let mut annotation = String::new();
                                        walk_node(
                                            child_handle,
                                            parser,
                                            &mut annotation,
                                            options,
                                            &ruby_ctx,
                                            depth + 1,
                                            dom_ctx,
                                        );
                                        rt_annotations.push(annotation);
                                    } else if tag_name == "rtc" {
                                        walk_node(
                                            child_handle,
                                            parser,
                                            &mut rtc_content,
                                            options,
                                            &ruby_ctx,
                                            depth + 1,
                                            dom_ctx,
                                        );
                                    } else if tag_name != "rp" {
                                        walk_node(
                                            child_handle,
                                            parser,
                                            &mut base_text,
                                            options,
                                            &ruby_ctx,
                                            depth + 1,
                                            dom_ctx,
                                        );
                                    }
                                }
                                tl::Node::Raw(_) => {
                                    walk_node(
                                        child_handle,
                                        parser,
                                        &mut base_text,
                                        options,
                                        &ruby_ctx,
                                        depth + 1,
                                        dom_ctx,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }

                let trimmed_base = base_text.trim();
                output.push_str(trimmed_base);

                if !rt_annotations.is_empty() {
                    let rt_text = rt_annotations.iter().map(|s| s.trim()).collect::<Vec<_>>().join("");
                    if !rt_text.is_empty() {
                        if has_rtc && !rtc_content.trim().is_empty() && rt_annotations.len() > 1 {
                            output.push('(');
                            output.push_str(&rt_text);
                            output.push(')');
                        } else {
                            output.push_str(&rt_text);
                        }
                    }
                }

                if !rtc_content.trim().is_empty() {
                    output.push_str(rtc_content.trim());
                }
            }
        }

        "rb" => {
            let mut text = String::new();
            let children = tag.children();
            let text_ctx = ctx.inline_buffer(output, false);
            {
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, &mut text, options, &text_ctx, depth + 1, dom_ctx);
                }
            }
            output.push_str(text.trim());
        }

        "rt" => {
            let mut text = String::new();
            let children = tag.children();
            let text_ctx = ctx.inline_buffer(output, true);
            {
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, &mut text, options, &text_ctx, depth + 1, dom_ctx);
                }
            }
            let trimmed = text.trim();

            if output.ends_with('(') {
                output.push_str(trimmed);
            } else {
                output.push('(');
                output.push_str(trimmed);
                output.push(')');
            }
        }

        "rp" => {
            // ~keep Ruby parenthesis element (fallback for non-ruby-supporting browsers)
            // ~keep In Markdown output, generally skip these as annotations are in parentheses
            let mut content = String::new();
            let children = tag.children();
            let content_ctx = ctx.inline_buffer(output, false);
            {
                for child_handle in children.top().iter() {
                    walk_node(
                        child_handle,
                        parser,
                        &mut content,
                        options,
                        &content_ctx,
                        depth + 1,
                        dom_ctx,
                    );
                }
            }
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                output.push_str(trimmed);
            }
        }

        "rtc" => {
            let children = tag.children();
            {
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
        }

        _ => {
            // ~keep Fallback for unknown ruby-related tags: process children normally
            let children = tag.children();
            {
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
        }
    }
}
