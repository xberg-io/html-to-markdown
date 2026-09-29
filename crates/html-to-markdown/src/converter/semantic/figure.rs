//! Handlers for HTML5 figure elements.
//!
//! Processes figure-related semantic elements:
//! - `<figure>` - Self-contained illustration, diagram, photo, code listing, etc.
//! - `<figcaption>` - Caption or legend for a figure
//!
//! Figure elements group an image (or other media) with its associated caption.
//! The Markdown output preserves this relationship through content organization.

#[cfg(feature = "visitor")]
use std::borrow::Cow;

/// Handles the `<figure>` element.
///
/// A figure element contains content (typically images) and optionally a figcaption.
/// The handler collects all content and cleans up extra line breaks.
///
/// # Behavior
///
/// - **Inline mode**: Children are processed inline without block spacing
/// - **Block mode**: Content is collected, line breaks normalized, and wrapped with blank lines
/// - **Image normalization**: Removes extra spaces before `![` to improve Markdown formatting
///
/// # Implementation Details
///
/// The handler performs the following on the collected content:
/// 1. Normalizes newline + image sequences: `\n![` → `![`
/// 2. Normalizes space + image sequences: ` ![` → `![`
/// 3. Trims the final content and wraps it with blank lines
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_figure(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        if ctx.convert_as_inline {
            let children = tag.children();
            {
                for child_handle in children.top().iter() {
                    super::walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
            return;
        }

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Figure,
                Cow::Borrowed("figure"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                false,
            );
            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_figure_start(&node_ctx)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Skip => return,
                VisitResult::Custom(custom) => {
                    let start_pos = output.len();
                    if !output.is_empty() && !output.ends_with("\n\n") {
                        output.push_str("\n\n");
                    }
                    output.push_str(&custom);
                    let safe_start =
                        crate::converter::utility::content::floor_char_boundary(output, start_pos.min(output.len()));
                    let figure_output = output[safe_start..].to_owned();
                    let end_result = {
                        let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                        visitor.visit_figure_end(&node_ctx, &figure_output)
                    };
                    match end_result {
                        VisitResult::Continue => {
                            if !output.ends_with('\n') {
                                output.push_str("\n\n");
                            } else if !output.ends_with("\n\n") {
                                output.push('\n');
                            }
                        }
                        VisitResult::Custom(end_custom) => {
                            output.truncate(safe_start);
                            output.push_str(&end_custom);
                        }
                        VisitResult::Skip => {
                            output.truncate(safe_start);
                        }
                        VisitResult::PreserveHtml => {
                            use crate::converter::utility::serialization::serialize_node;
                            output.truncate(safe_start);
                            output.push_str(&serialize_node(node_handle, parser));
                        }
                        VisitResult::Error(err) => {
                            if ctx.visitor_error.borrow().is_none() {
                                *ctx.visitor_error.borrow_mut() = Some(err);
                            }
                        }
                    }
                    return;
                }
                VisitResult::PreserveHtml => {
                    use crate::converter::utility::serialization::serialize_node;
                    output.push_str(&serialize_node(node_handle, parser));
                    return;
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    return;
                }
            }
        }

        let figure_ctx = crate::converter::list::utils::nested_block_context(output, ctx, options);
        let in_list_item = crate::converter::list::utils::container_starts_in_list_item(output, ctx);
        crate::converter::list::utils::start_container_block(output, ctx, options);
        let figure_start = output.len();

        let mut figure_content = String::new();
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                super::walk_node(
                    child_handle,
                    parser,
                    &mut figure_content,
                    options,
                    &figure_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
        }

        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since the figure's content is
        // ~keep simply finished here — so this closes its own trailing run the same way
        // ~keep `paragraph.rs` closes its own (issue #464 follow-up). `trim_matches` below
        // ~keep does not treat `\` as trimmable, so it cannot clean this up on its own.
        crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer(
            &mut figure_content,
            options.newline_style,
        );

        // ~keep In a list item the lines of the figure start at the item's content column, and
        // ~keep the space before an image is part of that indent.
        if !in_list_item {
            figure_content = figure_content.replace("\n![", "![");
            figure_content = figure_content.replace(" ![", "![");
        }

        let trimmed = figure_content.trim_matches(|c| c == '\n' || c == ' ' || c == '\t');
        if !trimmed.is_empty() {
            output.push_str(trimmed);
            if !output.ends_with('\n') {
                output.push('\n');
            }
            if !output.ends_with("\n\n") {
                output.push('\n');
            }
        }

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Figure,
                Cow::Borrowed("figure"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                false,
            );
            let safe_start =
                crate::converter::utility::content::floor_char_boundary(output, figure_start.min(output.len()));
            let figure_output = output[safe_start..].to_owned();
            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_figure_end(&node_ctx, &figure_output)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Skip => {
                    output.truncate(safe_start);
                }
                VisitResult::Custom(custom) => {
                    output.truncate(safe_start);
                    output.push_str(&custom);
                }
                VisitResult::PreserveHtml => {
                    use crate::converter::utility::serialization::serialize_node;
                    output.truncate(safe_start);
                    output.push_str(&serialize_node(node_handle, parser));
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                }
            }
        }
    }
}

/// Handles the `<figcaption>` element.
///
/// A figcaption element contains text that describes or supplements the figure.
/// It is rendered as emphasized (italic) text to distinguish it from regular content.
///
/// # Behavior
///
/// - Content is collected and trimmed
/// - Non-empty content is wrapped in `*text*` (emphasis) markers
/// - Proper spacing is maintained around the caption
///
/// # Implementation Details
///
/// The handler:
/// 1. Collects and processes all children
/// 2. Checks for existing output and adds spacing as needed
/// 3. Wraps content in emphasis markers: `*caption*`
/// 4. Ensures proper blank-line spacing after the caption
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_figcaption(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let mut text = String::new();
        let children = tag.children();
        {
            let caption_ctx = super::Context {
                text_in_markers: true,
                ..ctx.clone()
            };
            for child_handle in children.top().iter() {
                super::walk_node(
                    child_handle,
                    parser,
                    &mut text,
                    options,
                    &caption_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
        }

        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since the figcaption's content
        // ~keep is simply finished here — so this closes its own trailing run the same way
        // ~keep `paragraph.rs` closes its own (issue #464 follow-up). Left uncaught, the
        // ~keep stray `\` below ends up between the text and the closing `*`, escaping the
        // ~keep emphasis delimiter instead of merely surviving as a visible character.
        crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer(
            &mut text,
            options.newline_style,
        );

        let text = text.trim().to_owned();
        if text.is_empty() {
            return;
        }

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Figcaption,
                Cow::Borrowed("figcaption"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                false,
            );
            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_figcaption(&node_ctx, &text)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Skip => return,
                VisitResult::Custom(custom) => {
                    if !output.is_empty() {
                        if output.ends_with("```\n") {
                            output.push('\n');
                        } else {
                            while output.ends_with(' ') || output.ends_with('\t') {
                                output.pop();
                            }
                            if output.ends_with('\n') && !output.ends_with("\n\n") {
                                output.push('\n');
                            } else if !output.ends_with('\n') {
                                output.push_str("\n\n");
                            }
                        }
                    }
                    output.push_str(&custom);
                    if !custom.ends_with('\n') {
                        output.push_str("\n\n");
                    }
                    return;
                }
                VisitResult::PreserveHtml => {
                    use crate::converter::utility::serialization::serialize_node;
                    output.push_str(&serialize_node(node_handle, parser));
                    return;
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    return;
                }
            }
        }

        if crate::converter::list::utils::container_starts_in_list_item(output, ctx) {
            crate::converter::list::utils::start_block_in_list_item(output, ctx, options);
        } else if !output.is_empty() {
            if output.ends_with("```\n") {
                output.push('\n');
            } else {
                while output.ends_with(' ') || output.ends_with('\t') {
                    output.pop();
                }
                if output.ends_with('\n') && !output.ends_with("\n\n") {
                    output.push('\n');
                } else if !output.ends_with('\n') {
                    output.push_str("\n\n");
                }
            }
        }

        output.push('*');
        output.push_str(&text);
        output.push_str("*\n\n");
    }
}

/// Dispatcher for figure-related elements.
///
/// Routes `<figure>` and `<figcaption>` elements to their respective handlers.
pub fn handle(
    tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    match tag_name {
        "figure" => handle_figure(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "figcaption" => handle_figcaption(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn figure_caption_separated_from_image() {
        let html = r#"<figure><img src="photo.jpg" alt="Photo"><figcaption>A nice photo</figcaption></figure>"#;
        let result = crate::convert(html, None).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            content.contains("![Photo](photo.jpg)"),
            "image should be present: {content}"
        );
        assert!(content.contains("A nice photo"), "caption should be present: {content}");
        let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
        let img_line = lines.iter().position(|l| l.contains("![")).unwrap_or(999);
        let cap_line = lines.iter().position(|l| l.contains("A nice photo")).unwrap_or(999);
        assert!(
            cap_line > img_line,
            "caption should be on a separate line after image, lines: {lines:?}"
        );
    }
}
