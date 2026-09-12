//! Handlers for typography and text semantic elements.
//!
//! Contains:
//! - Small text (pass through)
//! - Subscript and superscript with configurable symbols
//! - Variable (var) and definition (dfn) text with italic formatting
//! - Abbreviation (abbr) with optional title
//! - Span element with special OCR handling

use crate::converter::inline::wrapped::{EMPHASIS_SIBLING_TAGS, InlineDelimiters, emit_wrapped_inline};
use crate::options::{ConversionOptions, OutputFormat};
#[cfg(feature = "visitor")]
use std::borrow::Cow;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle small element.
///
/// Small text has no direct Markdown equivalent, so just pass through content.
pub fn handle_small(
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
    for child_handle in children.top().iter() {
        walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
    }
}

/// Resolve a `<sub>`/`<sup>` wrapping pair for the current output format and configured symbol.
///
/// An HTML-ish symbol such as `<sub>` closes with its matching end tag rather than a repeat of
/// itself; every other symbol, including the empty default, closes with the symbol as given. ~keep
fn resolve_script_delimiters(options: &ConversionOptions, symbol: &str, djot_marker: char) -> (String, String) {
    if options.output_format == OutputFormat::Djot {
        let marker = djot_marker.to_string();
        return (marker.clone(), marker);
    }
    if symbol.starts_with('<') && !symbol.starts_with("</") {
        return (symbol.to_owned(), symbol.replace('<', "</"));
    }
    (symbol.to_owned(), symbol.to_owned())
}

/// Handle subscript element (sub tag).
///
/// Wraps content with configurable subscript symbol from options.
pub fn handle_subscript(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    // ~keep reason: serialize_node is only used when the visitor feature is active;
    // ~keep other imports depend on feature-gated code paths.
    #[allow(unused_imports)]
    use crate::converter::{append_inline_suffix, chomp_inline, get_text_content, serialize_node, walk_node};

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

    if ctx.in_code {
        output.push_str(&content);
        return;
    }

    #[cfg(feature = "visitor")]
    let sub_output = if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let text_content = get_text_content(node_handle, parser, dom_ctx);

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Subscript,
            tag.name().as_utf8_str(),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_subscript(&node_ctx, &text_content)
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
    if let Some(custom_output) = sub_output {
        output.push_str(&custom_output);
        return;
    }

    let (open, close) = resolve_script_delimiters(options, &options.sub_symbol, '~');
    emit_wrapped_inline(
        output,
        &content,
        &InlineDelimiters {
            open: &open,
            close: &close,
            merge_symbol: None,
            sibling_tag_names: &[],
        },
        node_handle,
        parser,
        dom_ctx,
    );
}

/// Handle superscript element (sup tag).
///
/// Wraps content with configurable superscript symbol from options.
pub fn handle_superscript(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    // ~keep reason: serialize_node is only used when the visitor feature is active;
    // ~keep other imports depend on feature-gated code paths.
    #[allow(unused_imports)]
    use crate::converter::{append_inline_suffix, chomp_inline, get_text_content, serialize_node, walk_node};

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

    if ctx.in_code {
        output.push_str(&content);
        return;
    }

    #[cfg(feature = "visitor")]
    let sup_output = if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let text_content = get_text_content(node_handle, parser, dom_ctx);

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Superscript,
            tag.name().as_utf8_str(),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_superscript(&node_ctx, &text_content)
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
    if let Some(custom_output) = sup_output {
        output.push_str(&custom_output);
        return;
    }

    let (open, close) = resolve_script_delimiters(options, &options.sup_symbol, '^');
    emit_wrapped_inline(
        output,
        &content,
        &InlineDelimiters {
            open: &open,
            close: &close,
            merge_symbol: None,
            sibling_tag_names: &[],
        },
        node_handle,
        parser,
        dom_ctx,
    );
}

/// Handle variable element (var tag).
///
/// Wraps content with italic symbol (`strong_em_symbol` from options).
pub fn handle_variable(
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

    let mut content = String::with_capacity(32);
    let children = tag.children();
    for child_handle in children.top().iter() {
        walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
    }

    let marker = options.strong_em_symbol.to_string();
    emit_wrapped_inline(
        output,
        &content,
        &InlineDelimiters {
            open: &marker,
            close: &marker,
            merge_symbol: Some(options.strong_em_symbol),
            sibling_tag_names: &EMPHASIS_SIBLING_TAGS,
        },
        node_handle,
        parser,
        dom_ctx,
    );
}

/// Handle definition element (dfn tag).
///
/// Wraps content with italic symbol (`strong_em_symbol` from options).
pub fn handle_definition(
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

    let mut content = String::with_capacity(32);
    let children = tag.children();
    for child_handle in children.top().iter() {
        walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
    }

    let marker = options.strong_em_symbol.to_string();
    emit_wrapped_inline(
        output,
        &content,
        &InlineDelimiters {
            open: &marker,
            close: &marker,
            merge_symbol: Some(options.strong_em_symbol),
            sibling_tag_names: &EMPHASIS_SIBLING_TAGS,
        },
        node_handle,
        parser,
        dom_ctx,
    );
}

/// Handle abbreviation element (abbr tag).
///
/// Passes through content and optionally appends title attribute in parentheses.
pub fn handle_abbreviation(
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

    let (prefix, suffix, trimmed) = chomp_inline(&content);

    if trimmed.is_empty() {
        // ~keep issue #481: `<abbr>` renders no delimiters of its own, so a whitespace-only
        // ~keep body left nothing behind at all and joined the words either side of it.
        if !content.is_empty() && !output.ends_with(' ') {
            output.push_str(prefix);
        }
        return;
    }

    output.push_str(prefix);
    output.push_str(trimmed);

    if let Some(title) = tag.attributes().get("title").flatten().map(|v| v.as_utf8_str()) {
        let trimmed_title = title.trim();
        if !trimmed_title.is_empty() {
            output.push_str(" (");
            output.push_str(trimmed_title);
            output.push(')');
        }
    }
    append_inline_suffix(output, suffix, true, node_handle, parser, dom_ctx);
}

/// Handle span element.
///
/// Processes span elements with special handling for:
/// - OCR words (elements with class "`ocrx_word")`: adds space before if needed
/// - Whitespace normalization in normalized mode: removes single newlines
/// - Otherwise passes through content normally
pub fn handle_span(
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

    let is_hocr_word = tag.attributes().iter().any(|(name, value)| {
        name.as_ref() == "class" && value.as_ref().is_some_and(|v| v.as_ref().contains("ocrx_word"))
    });

    if is_hocr_word
        && !output.is_empty()
        && !output.ends_with(' ')
        && !output.ends_with('\t')
        && !output.ends_with('\n')
    {
        output.push(' ');
    }

    if !ctx.in_code
        && options.whitespace_mode == crate::options::WhitespaceMode::Normalized
        && output.ends_with('\n')
        && !output.ends_with("\n\n")
        // ~keep issue #432: never strip an intentional Markdown hard break.
        && !output.ends_with("  \n")
        && !output.ends_with("\\\n")
        // ~keep issue #431: never cross a block boundary — a table row ends with
        // ~keep "|\n"; popping it would glue this span onto the delimiter row.
        && !output.ends_with("|\n")
    {
        output.pop();
    }

    let children = tag.children();
    {
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
    struct SubSkipVisitor;

    impl HtmlVisitor for SubSkipVisitor {
        fn visit_subscript(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::Skip
        }
    }

    #[derive(Debug)]
    struct SubCustomVisitor;

    impl HtmlVisitor for SubCustomVisitor {
        fn visit_subscript(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::Custom("REPLACED".to_string())
        }
    }

    #[derive(Debug)]
    struct SubPreserveVisitor;

    impl HtmlVisitor for SubPreserveVisitor {
        fn visit_subscript(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::PreserveHtml
        }
    }

    #[derive(Debug)]
    struct SupSkipVisitor;

    impl HtmlVisitor for SupSkipVisitor {
        fn visit_superscript(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::Skip
        }
    }

    #[derive(Debug)]
    struct SupCustomVisitor;

    impl HtmlVisitor for SupCustomVisitor {
        fn visit_superscript(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::Custom("REPLACED".to_string())
        }
    }

    #[derive(Debug)]
    struct SupPreserveVisitor;

    impl HtmlVisitor for SupPreserveVisitor {
        fn visit_superscript(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
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
    fn test_visitor_subscript_skip() {
        let html = "<p>H<sub>2</sub>O</p>";
        let result = convert(html, Some(make_visitor(SubSkipVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(!content.contains('2'), "sub content should be absent: {content}");
        assert!(content.contains('H'), "surrounding text should be present: {content}");
    }

    #[test]
    fn test_visitor_subscript_custom() {
        let html = "<p>H<sub>2</sub>O</p>";
        let result = convert(html, Some(make_visitor(SubCustomVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            content.contains("REPLACED"),
            "custom output should be present: {content}"
        );
    }

    #[test]
    fn test_visitor_subscript_preserve_html() {
        let html = "<p>H<sub>2</sub>O</p>";
        let result = convert(html, Some(make_visitor(SubPreserveVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            content.contains("<sub>2</sub>"),
            "original html should be preserved: {content}"
        );
    }

    #[test]
    fn test_visitor_superscript_skip() {
        let html = "<p>E=mc<sup>2</sup></p>";
        let result = convert(html, Some(make_visitor(SupSkipVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(!content.contains('2'), "sup content should be absent: {content}");
    }

    #[test]
    fn test_visitor_superscript_custom() {
        let html = "<p>E=mc<sup>2</sup></p>";
        let result = convert(html, Some(make_visitor(SupCustomVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            content.contains("REPLACED"),
            "custom output should be present: {content}"
        );
    }

    #[test]
    fn test_visitor_superscript_preserve_html() {
        let html = "<p>E=mc<sup>2</sup></p>";
        let result = convert(html, Some(make_visitor(SupPreserveVisitor))).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            content.contains("<sup>2</sup>"),
            "original html should be preserved: {content}"
        );
    }
}
