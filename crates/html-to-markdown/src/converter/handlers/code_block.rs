//! Code and pre element handlers for HTML to Markdown conversion.
//!
//! Handles `<code>` and `<pre>` elements including:
//! - Inline code with backtick formatting
//! - Code block formatting (indented or fenced)
//! - Language detection from class attributes
//! - Whitespace normalization and dedenting
//! - Visitor callback integration

use crate::converter::Context;
use crate::converter::dom_context::DomContext;
use crate::converter::main::walk_node;
use crate::converter::text::dedent_code_block;
use crate::options::ConversionOptions;

#[cfg(feature = "visitor")]
#[cfg(feature = "visitor")]
use crate::converter::utility::content::collect_tag_attributes;
#[cfg(feature = "visitor")]
use std::collections::BTreeMap;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node;

/// Handle an inline `<code>` element and convert to Markdown.
///
/// This handler processes inline code elements including:
/// - Extracting code content and applying backtick delimiters
/// - Handling backticks in content by using multiple delimiters
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Generating appropriate markdown output with proper escaping
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_code(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let code_ctx = Context {
        in_code: true,
        ..ctx.clone()
    };

    if ctx.in_code {
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, &code_ctx, depth + 1, dom_ctx);
            }
        }
    } else {
        let mut content = String::with_capacity(32);
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    &code_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
        }

        let trimmed = &content;

        if !content.trim().is_empty() {
            #[cfg(feature = "visitor")]
            let code_output = if let Some(ref visitor_handle) = ctx.visitor {
                use crate::visitor::{NodeContext, NodeType, VisitResult};

                let attributes: BTreeMap<String, String> = collect_tag_attributes(tag);

                let node_id = node_handle.get_inner();
                let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
                let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

                let node_ctx = NodeContext {
                    node_type: NodeType::Code,
                    tag_name: "code".to_string(),
                    attributes,
                    depth,
                    index_in_parent,
                    parent_tag,
                    is_inline: true,
                };

                let visit_result = {
                    let mut visitor = visitor_handle.borrow_mut();
                    visitor.visit_code_inline(&node_ctx, trimmed)
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
            if let Some(custom_output) = code_output {
                output.push_str(&custom_output);
            } else {
                format_inline_code(trimmed, output);
            }

            #[cfg(not(feature = "visitor"))]
            {
                format_inline_code(trimmed, output);
            }
        }
    }
}

/// Handle a `<pre>` element and convert to Markdown.
///
/// This handler processes code block elements including:
/// - Extracting language information from class attributes
/// - Processing whitespace and dedenting code content
/// - Supporting multiple code block styles (indented, backticks, tildes)
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Generating appropriate markdown output
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_pre(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let code_ctx = Context {
        in_code: true,
        ..ctx.clone()
    };

    #[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
    let language: Option<String> = {
        let mut lang: Option<String> = None;

        // First, try to extract language from <pre> tag's class attribute
        if let Some(class_attr) = tag.attributes().get("class") {
            if let Some(class_bytes) = class_attr {
                let class_str = class_bytes.as_utf8_str();
                for cls in class_str.split_whitespace() {
                    if let Some(stripped) = cls.strip_prefix("language-") {
                        lang = Some(String::from(stripped));
                        break;
                    } else if let Some(stripped) = cls.strip_prefix("lang-") {
                        lang = Some(String::from(stripped));
                        break;
                    }
                }
            }
        }

        // If not found on <pre>, try to extract from nested <code> tag's class attribute
        if lang.is_none() {
            let children = tag.children();
            for child_handle in children.top().iter() {
                if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                    if child_tag.name() == "code" {
                        if let Some(class_attr) = child_tag.attributes().get("class") {
                            if let Some(class_bytes) = class_attr {
                                let class_str = class_bytes.as_utf8_str();
                                for cls in class_str.split_whitespace() {
                                    if let Some(stripped) = cls.strip_prefix("language-") {
                                        lang = Some(String::from(stripped));
                                        break;
                                    } else if let Some(stripped) = cls.strip_prefix("lang-") {
                                        lang = Some(String::from(stripped));
                                        break;
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        lang
    };

    let mut content = String::with_capacity(256);
    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            walk_node(
                child_handle,
                parser,
                &mut content,
                options,
                &code_ctx,
                depth + 1,
                dom_ctx,
            );
        }
    }

    if !content.is_empty() {
        let leading_newlines = content.chars().take_while(|&c| c == '\n').count();
        let trailing_newlines = content.chars().rev().take_while(|&c| c == '\n').count();
        let core = content.trim_matches('\n');
        let is_whitespace_only = core.trim().is_empty();

        let processed_content = if options.whitespace_mode == crate::options::WhitespaceMode::Strict {
            content
        } else {
            // Always dedent code blocks to remove common leading whitespace
            let mut core_text = dedent_code_block(core);

            if is_whitespace_only {
                let mut rebuilt = String::new();
                for _ in 0..leading_newlines {
                    rebuilt.push('\n');
                }
                rebuilt.push_str(&core_text);
                for _ in 0..trailing_newlines {
                    rebuilt.push('\n');
                }
                rebuilt
            } else {
                for _ in 0..trailing_newlines {
                    core_text.push('\n');
                }
                core_text
            }
        };

        #[cfg(feature = "visitor")]
        let code_block_output = if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let attributes: BTreeMap<String, String> = collect_tag_attributes(tag);

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext {
                node_type: NodeType::Pre,
                tag_name: "pre".to_string(),
                attributes,
                depth,
                index_in_parent,
                parent_tag,
                is_inline: false,
            };

            let visit_result = {
                let mut visitor = visitor_handle.borrow_mut();
                visitor.visit_code_block(&node_ctx, language.as_deref(), &processed_content)
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
        if let Some(custom_output) = code_block_output {
            output.push_str(&custom_output);
        } else {
            format_code_block(&processed_content, language.as_deref(), output, options, ctx);
        }

        #[cfg(not(feature = "visitor"))]
        {
            format_code_block(&processed_content, language.as_deref(), output, options, ctx);
        }

        if let Some(ref sc) = ctx.structure_collector {
            sc.borrow_mut().push_code(&processed_content, language.as_deref());
        }
    }
}

/// Format inline code with appropriate backtick delimiters.
///
/// Handles:
/// - Single backticks for normal content
/// - Double backticks when content contains backticks
/// - Space padding when needed to avoid backtick adjacency
fn format_inline_code(content: &str, output: &mut String) {
    let contains_backtick = content.contains('`');

    let needs_delimiter_spaces = {
        let first_char = content.chars().next();
        let last_char = content.chars().last();
        let starts_with_space = first_char == Some(' ');
        let ends_with_space = last_char == Some(' ');
        let starts_with_backtick = first_char == Some('`');
        let ends_with_backtick = last_char == Some('`');
        let all_spaces = content.chars().all(|c| c == ' ');

        all_spaces
            || starts_with_backtick
            || ends_with_backtick
            || (starts_with_space && ends_with_space && contains_backtick)
    };

    let (num_backticks, needs_spaces) = if contains_backtick {
        let max_consecutive = content
            .chars()
            .fold((0, 0), |(max, current), c| {
                if c == '`' {
                    let new_current = current + 1;
                    (max.max(new_current), new_current)
                } else {
                    (max, 0)
                }
            })
            .0;
        let num = if max_consecutive == 1 { 2 } else { 1 };
        (num, needs_delimiter_spaces)
    } else {
        (1, needs_delimiter_spaces)
    };

    for _ in 0..num_backticks {
        output.push('`');
    }
    if needs_spaces {
        output.push(' ');
    }
    output.push_str(content);
    if needs_spaces {
        output.push(' ');
    }
    for _ in 0..num_backticks {
        output.push('`');
    }
}

/// Format a code block with the specified style and language.
///
/// Supports:
/// - Indented style (4-space indentation)
/// - Fenced style with backticks (```language)
/// - Fenced style with tildes (~~~language)
fn format_code_block(
    content: &str,
    language: Option<&str>,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
) {
    match options.code_block_style {
        crate::options::CodeBlockStyle::Indented => {
            if !ctx.convert_as_inline && !output.is_empty() && !output.ends_with("\n\n") {
                if output.ends_with('\n') {
                    output.push('\n');
                } else {
                    output.push_str("\n\n");
                }
            }

            let indented = content
                .lines()
                .map(|line| {
                    if line.is_empty() {
                        String::new()
                    } else {
                        format!("    {line}")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            output.push_str(&indented);

            output.push_str("\n\n");
        }
        crate::options::CodeBlockStyle::Backticks | crate::options::CodeBlockStyle::Tildes => {
            if !ctx.convert_as_inline && !output.is_empty() && !output.ends_with("\n\n") {
                if output.ends_with('\n') {
                    output.push('\n');
                } else {
                    output.push_str("\n\n");
                }
            }

            let fence = if options.code_block_style == crate::options::CodeBlockStyle::Backticks {
                "```"
            } else {
                "~~~"
            };

            output.push_str(fence);
            if let Some(lang) = language {
                output.push_str(lang);
            } else if !options.code_language.is_empty() {
                output.push_str(&options.code_language);
            }
            output.push('\n');
            output.push_str(content);
            output.push('\n');
            output.push_str(fence);
            output.push('\n');
        }
    }
}
