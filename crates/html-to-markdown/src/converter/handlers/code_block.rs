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
use crate::converter::inline::wrapped::emit_code_span;
use crate::converter::main::walk_node;
use crate::converter::text::dedent_code_block;
use crate::options::ConversionOptions;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node;
#[cfg(feature = "visitor")]
use std::borrow::Cow;

/// Minimum length of a Markdown code fence (```` ``` ```` or `~~~`) per CommonMark.
const MIN_FENCE_LENGTH: usize = 3;

/// Compute the length of the longest consecutive run of `marker` in `content`.
fn longest_consecutive_run(content: &str, marker: char) -> usize {
    content
        .chars()
        .fold((0usize, 0usize), |(max, current), c| {
            if c == marker {
                let next = current + 1;
                (max.max(next), next)
            } else {
                (max, 0)
            }
        })
        .0
}

/// Smallest backtick-run length (starting at 1) that does not occur as a run inside `content`.
///
/// CommonMark closes an inline code span at the next backtick string of the *same* length as
/// the opening delimiter (6.1) — a longer or shorter run never matches. So the delimiter only
/// needs to avoid colliding with a run length that actually appears in `content`; it does not
/// need to exceed the longest run (unlike a fenced block, whose closing rule matches on *any*
/// run at least as long as the fence). Picking `longest_run + 1` unconditionally over-escapes:
/// content `` `` `` (a single length-2 run, no length-1 run) is valid with a single backtick.
fn min_safe_code_span_delimiter_length(content: &str) -> usize {
    let mut run_lengths = std::collections::HashSet::new();
    let mut current = 0usize;
    for c in content.chars() {
        if c == '`' {
            current += 1;
        } else {
            if current > 0 {
                run_lengths.insert(current);
            }
            current = 0;
        }
    }
    if current > 0 {
        run_lengths.insert(current);
    }

    let mut candidate = 1usize;
    while run_lengths.contains(&candidate) {
        candidate += 1;
    }
    candidate
}

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

        // ~keep issue #481: an all-whitespace body is NOT an empty one. `<code> </code>` is a
        // ~keep code span whose content is a space, and CommonMark spells that exactly --
        // ~keep `format_inline_code`'s `all_spaces` branch pads with delimiter spaces so the
        // ~keep span survives the spec's own stripping rule (spec example 138). Testing
        // ~keep `trim()` here dropped the element outright and joined the words either side.
        if !content.is_empty() {
            #[cfg(feature = "visitor")]
            let code_output = if let Some(ref visitor_handle) = ctx.visitor {
                use crate::visitor::{NodeContext, NodeType, VisitResult};

                let node_id = node_handle.get_inner();
                let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
                let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

                let node_ctx = NodeContext::with_lazy_attributes(
                    NodeType::Code,
                    Cow::Borrowed("code"),
                    tag,
                    depth,
                    index_in_parent,
                    parent_tag.map(Cow::Borrowed),
                    true,
                );

                let visit_result = {
                    let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
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
                emit_inline_code(trimmed, output, node_handle, parser, dom_ctx);
            }

            #[cfg(not(feature = "visitor"))]
            {
                emit_inline_code(trimmed, output, node_handle, parser, dom_ctx);
            }
        }
    }
}

/// Render an inline code span, then emit it through the adjacent-span merge.
fn emit_inline_code(
    content: &str,
    output: &mut String,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) {
    let mut span = String::with_capacity(content.len() + 2);
    format_inline_code(content, &mut span);
    emit_code_span(&span, content, output, node_handle, parser, dom_ctx);
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

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Pre,
                Cow::Borrowed("pre"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                false,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
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
        // ~keep CommonMark strips one space from each end of a code span ONLY when the
        // ~keep content is not entirely spaces, so an all-spaces body needs no padding --
        // ~keep and padding it changes what the span contains (`` ` ` `` is one space,
        // ~keep `` `   ` `` is three). Spec example 138 is exactly this case.
        starts_with_backtick || ends_with_backtick || (starts_with_space && ends_with_space && contains_backtick)
    };

    let (num_backticks, needs_spaces) = if contains_backtick {
        (min_safe_code_span_delimiter_length(content), needs_delimiter_spaces)
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
    if ctx.in_table_cell {
        // ~keep Neither code-block style can exist inside a GFM pipe cell: the fence (or the
        // ~keep 4-space indent) is line-structured and both styles bracket the block with blank
        // ~keep lines, so the row would be split across physical lines (issue #456). Emit the
        // ~keep content inline with the block syntax dropped — the same degradation Tier-1's
        // ~keep `close_pre` already performs, and consistent with headings and list items
        // ~keep shedding their markers in a cell. Line breaks fold to a space rather than to
        // ~keep `<br>` so that the two tiers stay byte-equal.
        output.push_str(crate::text::fold_cell_line_breaks_verbatim_cow(content.trim_matches('\n')).as_ref());
        return;
    }

    if ctx.in_list_item {
        format_code_block_in_list_item(content, language, output, options, ctx);
        return;
    }

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

            let fence_char = if options.code_block_style == crate::options::CodeBlockStyle::Backticks {
                '`'
            } else {
                '~'
            };
            // ~keep the fence must be strictly longer than the longest run of the fence
            // ~keep character inside the content, otherwise the fence terminates early
            // ~keep and corrupts the rest of the document (CommonMark 4.5).
            let fence_length = (longest_consecutive_run(content, fence_char) + 1).max(MIN_FENCE_LENGTH);
            let fence: String = std::iter::repeat_n(fence_char, fence_length).collect();

            output.push_str(&fence);
            if let Some(lang) = language {
                output.push_str(lang);
            } else if !options.code_language.is_empty() {
                output.push_str(&options.code_language);
            }
            output.push('\n');
            output.push_str(content.trim_end_matches('\n'));
            output.push('\n');
            output.push_str(&fence);
            output.push_str("\n\n");
        }
    }
}

/// Format a code block that is a child of a list item.
///
/// ~keep A fenced (or indented) code block spans several physical lines, but the
/// ~keep only call site that indented list continuation content
/// ~keep (`block/paragraph.rs::add_list_continuation_indent`) indented a single
/// ~keep leading position, not every line a block emits. CommonMark's list
/// ~keep container match is per physical line: a non-blank line that is not
/// ~keep indented to `list_indent_columns` is not part of the item, so an
/// ~keep unindented closing fence (or any interior content line) drops the rest
/// ~keep of the block, and the item itself, out of the list on re-parse
/// ~keep (CommonMark spec examples 263, 273, 274, 318, 324). Render into a
/// ~keep scratch buffer first so every line can be indented uniformly, then only
/// ~keep skip the indent on the very first line when this block sits directly
/// ~keep after the marker text (i.e. it is the item's first content, not a
/// ~keep continuation) — that line already starts at the right column.
fn format_code_block_in_list_item(
    content: &str,
    language: Option<&str>,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
) {
    let mut rendered = String::new();
    let plain_ctx = Context {
        in_list_item: false,
        ..ctx.clone()
    };
    format_code_block(content, language, &mut rendered, options, &plain_ctx);

    // ~keep A plain suffix check like `output.ends_with("* ")` also matches the closing
    // ~keep "**"/"*" of `<strong>`/`<em>` immediately followed by a migrated trailing
    // ~keep space, indistinguishable from a real bare bullet by suffix alone -- and, being
    // ~keep hardcoded to `-`/`*`, never matched the third bullet `+` at all. Both false
    // ~keep positive (fake marker) and false negative (real `+ ` marker) misclassified
    // ~keep this fenced block relative to the marker, either gluing the opening fence onto
    // ~keep the previous inline line (breaking the fence syntax) or doubly indenting the
    // ~keep first line. See `list::utils::line_is_bare_list_marker`'s doc comment.
    let is_continuation = !ctx.convert_as_inline
        && !output.is_empty()
        && !crate::converter::list::utils::line_is_bare_list_marker(output);

    if is_continuation {
        crate::converter::trim_trailing_whitespace(output);
        if !output.ends_with("\n\n") {
            if output.ends_with('\n') {
                output.push('\n');
            } else {
                output.push_str("\n\n");
            }
        }
    }

    let indent =
        crate::converter::list::utils::continuation_indent_string(ctx.list_depth, ctx.list_indent_columns, options)
            .unwrap_or_default();

    for (index, segment) in rendered.split_inclusive('\n').enumerate() {
        let line = segment.strip_suffix('\n').unwrap_or(segment);
        if line.is_empty() || (index == 0 && !is_continuation) {
            output.push_str(segment);
        } else {
            output.push_str(&indent);
            output.push_str(segment);
        }
    }
}
