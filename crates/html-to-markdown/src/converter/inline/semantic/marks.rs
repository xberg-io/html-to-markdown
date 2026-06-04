//! Handlers for mark/highlight and strikethrough/underline elements.
//!
//! Contains:
//! - Mark (highlight) element with configurable styles
//! - Strikethrough (del, s tags) with ~~ syntax
//! - Inserted/underlined text (ins, u tags) with == syntax

use crate::options::{ConversionOptions, OutputFormat};
#[cfg(feature = "visitor")]
use std::borrow::Cow;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle mark (highlight) element with configurable styles.
///
/// Supports multiple highlight styles:
/// - DoubleEqual: `==highlighted==`
/// - Html: `<mark>highlighted</mark>`
/// - Bold: `**highlighted**`
/// - None: just pass through content
pub fn handle_mark(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    #[allow(unused_imports)]
    use crate::converter::{get_text_content, serialize_node, walk_node};

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    #[cfg(feature = "visitor")]
    let mark_output = if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let text_content = get_text_content(node_handle, parser, dom_ctx);

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Mark,
            tag.name().as_utf8_str(),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_mark(&node_ctx, &text_content)
        };
        match visit_result {
            VisitResult::Continue => None,
            VisitResult::Custom(custom) => Some(custom),
            VisitResult::Skip => Some(String::new()),
            VisitResult::PreserveHtml => Some(serialize_node(node_handle, parser)),
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                None
            }
        }
    } else {
        None
    };

    #[cfg(feature = "visitor")]
    if let Some(custom_output) = mark_output {
        output.push_str(&custom_output);
        return;
    }

    if ctx.convert_as_inline {
        // In inline conversion context, just pass through children
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    } else {
        use crate::options::HighlightStyle;
        match options.highlight_style {
            HighlightStyle::DoubleEqual => {
                if options.output_format == OutputFormat::Djot {
                    output.push_str("{=");
                } else {
                    output.push_str("==");
                }
                let children = tag.children();
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
                if options.output_format == OutputFormat::Djot {
                    output.push_str("=}");
                } else {
                    output.push_str("==");
                }
            }
            HighlightStyle::Html => {
                output.push_str("<mark>");
                let children = tag.children();
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
                output.push_str("</mark>");
            }
            HighlightStyle::Bold => {
                let mut symbol = String::with_capacity(2);
                symbol.push(options.strong_em_symbol);
                symbol.push(options.strong_em_symbol);
                output.push_str(&symbol);
                let bold_ctx = Context {
                    in_strong: true,
                    ..ctx.clone()
                };
                let children = tag.children();
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, &bold_ctx, depth + 1, dom_ctx);
                }
                output.push_str(&symbol);
            }
            HighlightStyle::None => {
                let children = tag.children();
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
        }
    }
}

/// Handle strikethrough element (del, s tags).
///
/// Converts to `~~content~~` syntax. Suppresses formatting in code context.
/// Supports visitor callbacks when the visitor feature is enabled.
#[allow(unused_variables)]
pub fn handle_strikethrough(
    tag_name: &str,
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    use crate::converter::{append_inline_suffix, chomp_inline, walk_node};

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    if ctx.in_code {
        // Suppress strikethrough in code context, just process children
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    } else {
        let mut content = String::with_capacity(32);
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
        }

        #[cfg(feature = "visitor")]
        let strikethrough_output = if let Some(ref visitor_handle) = ctx.visitor {
            use crate::converter::get_text_content;
            use crate::visitor::{NodeContext, NodeType, VisitResult};
            let text_content = get_text_content(node_handle, parser, dom_ctx);

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Strikethrough,
                Cow::Borrowed(tag_name),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_strikethrough(&node_ctx, &text_content)
            };
            match visit_result {
                VisitResult::Continue => None,
                VisitResult::Custom(custom) => Some(custom),
                VisitResult::Skip => Some(String::new()),
                VisitResult::PreserveHtml => {
                    use crate::converter::serialize_node;
                    Some(serialize_node(node_handle, parser))
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    None
                }
            }
        } else {
            None
        };

        #[cfg(feature = "visitor")]
        if let Some(custom_output) = strikethrough_output {
            output.push_str(&custom_output);
        } else {
            let (prefix, suffix, trimmed) = chomp_inline(&content);
            if !content.trim().is_empty() {
                output.push_str(prefix);
                if options.output_format == OutputFormat::Djot {
                    output.push_str("{-");
                } else {
                    output.push_str("~~");
                }
                output.push_str(trimmed);
                if options.output_format == OutputFormat::Djot {
                    output.push_str("-}");
                } else {
                    output.push_str("~~");
                }
                append_inline_suffix(output, suffix, !trimmed.is_empty(), node_handle, parser, dom_ctx);
            } else if !content.is_empty() {
                output.push_str(prefix);
                append_inline_suffix(output, suffix, false, node_handle, parser, dom_ctx);
            }
        }

        #[cfg(not(feature = "visitor"))]
        {
            let (prefix, suffix, trimmed) = chomp_inline(&content);
            if !content.trim().is_empty() {
                output.push_str(prefix);
                if options.output_format == OutputFormat::Djot {
                    output.push_str("{-");
                } else {
                    output.push_str("~~");
                }
                output.push_str(trimmed);
                if options.output_format == OutputFormat::Djot {
                    output.push_str("-}");
                } else {
                    output.push_str("~~");
                }
                append_inline_suffix(output, suffix, !trimmed.is_empty(), node_handle, parser, dom_ctx);
            } else if !content.is_empty() {
                output.push_str(prefix);
                append_inline_suffix(output, suffix, false, node_handle, parser, dom_ctx);
            }
        }
    }
}

/// Handle inserted/underlined text (ins tag).
///
/// Converts to `==content==` syntax. Supports visitor callbacks when enabled.
pub fn handle_inserted(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    use crate::converter::{append_inline_suffix, chomp_inline, walk_node};

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    let mut content = String::with_capacity(32);
    let children = tag.children();
    for child_handle in children.top().iter() {
        walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
    }

    #[cfg(feature = "visitor")]
    let underline_output = if let Some(ref visitor_handle) = ctx.visitor {
        use crate::converter::get_text_content;
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let text_content = get_text_content(node_handle, parser, dom_ctx);

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Underline,
            Cow::Borrowed("ins"),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_underline(&node_ctx, &text_content)
        };
        match visit_result {
            VisitResult::Continue => None,
            VisitResult::Custom(custom) => Some(custom),
            VisitResult::Skip => Some(String::new()),
            VisitResult::PreserveHtml => {
                use crate::converter::serialize_node;
                Some(serialize_node(node_handle, parser))
            }
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                None
            }
        }
    } else {
        None
    };

    #[cfg(feature = "visitor")]
    if let Some(custom_output) = underline_output {
        output.push_str(&custom_output);
    } else {
        let (prefix, suffix, trimmed) = chomp_inline(&content);
        if !trimmed.is_empty() {
            output.push_str(prefix);
            if options.output_format == OutputFormat::Djot {
                output.push_str("{+");
            } else {
                output.push_str("==");
            }
            output.push_str(trimmed);
            if options.output_format == OutputFormat::Djot {
                output.push_str("+}");
            } else {
                output.push_str("==");
            }
            append_inline_suffix(output, suffix, !trimmed.is_empty(), node_handle, parser, dom_ctx);
        }
    }

    #[cfg(not(feature = "visitor"))]
    {
        let (prefix, suffix, trimmed) = chomp_inline(&content);
        if !trimmed.is_empty() {
            output.push_str(prefix);
            if options.output_format == OutputFormat::Djot {
                output.push_str("{+");
            } else {
                output.push_str("==");
            }
            output.push_str(trimmed);
            if options.output_format == OutputFormat::Djot {
                output.push_str("+}");
            } else {
                output.push_str("==");
            }
            append_inline_suffix(output, suffix, !trimmed.is_empty(), node_handle, parser, dom_ctx);
        }
    }
}

/// Handle underline element (u tag).
///
/// Just passes through content (HTML doesn't have native underline in Markdown).
/// Supports visitor callbacks when enabled, which can provide custom formatting.
pub fn handle_underline(
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

    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::converter::get_text_content;
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let text_content = get_text_content(node_handle, parser, dom_ctx);

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Underline,
            Cow::Borrowed("u"),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_underline(&node_ctx, &text_content)
        };
        match visit_result {
            VisitResult::Continue => {
                let children = tag.children();
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
            }
            VisitResult::Skip => {}
            VisitResult::PreserveHtml => {
                use crate::converter::serialize_node;
                output.push_str(&serialize_node(node_handle, parser));
            }
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                let children = tag.children();
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
        }
    } else {
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    }

    #[cfg(not(feature = "visitor"))]
    {
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    }
}

#[cfg(all(test, feature = "visitor"))]
mod tests {
    use crate::convert;
    use crate::options::ConversionOptions;
    use crate::visitor::{HtmlVisitor, NodeContext, VisitResult};
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    struct MarkSkipVisitor;

    impl HtmlVisitor for MarkSkipVisitor {
        fn visit_mark(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::Skip
        }
    }

    #[derive(Debug)]
    struct MarkCustomVisitor;

    impl HtmlVisitor for MarkCustomVisitor {
        fn visit_mark(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::Custom("REPLACED".to_string())
        }
    }

    #[derive(Debug)]
    struct MarkPreserveVisitor;

    impl HtmlVisitor for MarkPreserveVisitor {
        fn visit_mark(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::PreserveHtml
        }
    }

    fn make_visitor<V: HtmlVisitor + 'static>(v: V) -> ConversionOptions {
        ConversionOptions {
            visitor: Some(Arc::new(Mutex::new(v))),
            ..ConversionOptions::default()
        }
    }

    #[test]
    fn test_visitor_mark_skip() {
        let html = "<p>before <mark>highlighted</mark> after</p>";
        let result = convert(html, Some(make_visitor(MarkSkipVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            !content.contains("highlighted"),
            "mark content should be absent: {}",
            content
        );
        assert!(
            content.contains("before"),
            "surrounding text should be present: {}",
            content
        );
    }

    #[test]
    fn test_visitor_mark_custom() {
        let html = "<p>before <mark>highlighted</mark> after</p>";
        let result = convert(html, Some(make_visitor(MarkCustomVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            content.contains("REPLACED"),
            "custom output should be present: {}",
            content
        );
    }

    #[test]
    fn test_visitor_mark_preserve_html() {
        let html = "<p>before <mark>highlighted</mark> after</p>";
        let result = convert(html, Some(make_visitor(MarkPreserveVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            content.contains("<mark>highlighted</mark>"),
            "original html should be preserved: {}",
            content
        );
    }
}
