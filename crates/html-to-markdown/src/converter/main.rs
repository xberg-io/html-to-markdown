//! Main conversion pipeline for HTML to Markdown.
//!
//! This module implements the core conversion functions and the recursive tree walker
//! that transforms HTML DOM nodes into Markdown output.

#![allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::trivially_copy_pass_by_ref,
    clippy::items_after_statements
)]

use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::converter::dom_context::DomContext;
use crate::converter::main_helpers::{
    extract_head_metadata, format_metadata_frontmatter, has_custom_element_tags, repair_with_html5ever,
    trim_line_end_whitespace, trim_trailing_whitespace,
};
use crate::converter::plain_text::extract_plain_text;
use crate::converter::preprocessing_helpers::{has_inline_block_misnest, should_drop_for_preprocessing};
use crate::converter::utility::caching::build_dom_context;
use crate::converter::utility::content::normalized_tag_name;
use crate::converter::utility::preprocessing::{preprocess_html, strip_hidden_elements, strip_script_and_style_tags};
use crate::converter::utility::serialization::serialize_tag_to_html;
use crate::options::OutputFormat;

use crate::converter::handlers::{handle_blockquote, handle_code, handle_graphic, handle_img, handle_link, handle_pre};
use crate::error::Result;
use crate::options::ConversionOptions;

use crate::converter::context::{Context, InlineCollectorHandle};
use crate::types::structure_collector::StructureCollectorHandle;

/// Converts HTML to Markdown using the provided conversion options.
///
/// This is the main entry point for HTML to Markdown conversion.
pub fn convert_html(html: &str, options: &ConversionOptions) -> Result<String> {
    convert_html_impl(html, options, None, None, None, None).map(|(md, _)| md)
}

/// Converts HTML to Markdown with a custom visitor for callbacks during traversal.
///
/// This variant allows passing a visitor that will receive callbacks for each node
/// during the tree walk, enabling custom processing or analysis.
#[cfg(feature = "visitor")]
pub fn convert_html_with_visitor(
    html: &str,
    options: &ConversionOptions,
    visitor: Option<crate::visitor::VisitorHandle>,
) -> Result<String> {
    convert_html_impl(html, options, None, None, visitor, None).map(|(md, _)| md)
}

/// Internal implementation of HTML to Markdown conversion.
///
/// Returns `(markdown, Option<DocumentStructure>)`.  The structure is populated when
/// `options.include_document_structure == true` and a `structure_collector` handle is provided.
#[cfg_attr(
    any(not(feature = "inline-images"), not(feature = "metadata"), not(feature = "visitor")),
    allow(unused_variables)
)]
#[allow(clippy::too_many_lines)]
pub(crate) fn convert_html_impl(
    html: &str,
    options: &ConversionOptions,
    inline_collector: Option<InlineCollectorHandle>,
    #[cfg(feature = "metadata")] metadata_collector: Option<crate::metadata::MetadataCollectorHandle>,
    #[cfg(not(feature = "metadata"))] _metadata_collector: Option<()>,
    #[cfg(feature = "visitor")] visitor: Option<crate::visitor::VisitorHandle>,
    #[cfg(not(feature = "visitor"))] _visitor: Option<()>,
    structure_collector: Option<StructureCollectorHandle>,
) -> Result<(String, Option<crate::types::DocumentStructure>)> {
    // Strip script and style tags completely to prevent parser confusion from HTML-like content
    // inside script/style elements. This preserves JSON-LD for metadata extraction.
    let stripped = strip_script_and_style_tags(html);
    // Strip elements with the `hidden` attribute before parsing.
    let stripped = strip_hidden_elements(&stripped);
    let mut preprocessed = preprocess_html(&stripped).into_owned();
    let mut preprocessed_len = preprocessed.len();

    if has_custom_element_tags(&preprocessed) {
        if let Some(repaired_html) = repair_with_html5ever(&preprocessed) {
            let repaired = preprocess_html(&repaired_html).into_owned();
            preprocessed = repaired;
            preprocessed_len = preprocessed.len();
        }
    }
    let parser_options = tl::ParserOptions::default();
    let mut dom = loop {
        if let Ok(dom) = tl::parse(&preprocessed, parser_options) {
            break dom;
        }
        if let Some(repaired_html) = repair_with_html5ever(&preprocessed) {
            preprocessed = preprocess_html(&repaired_html).into_owned();
            preprocessed_len = preprocessed.len();
            continue;
        }
        return Err(crate::error::ConversionError::ParseError(
            "Failed to parse HTML".to_string(),
        ));
    };
    let mut parser = dom.parser();
    let mut output = String::with_capacity(preprocessed_len.saturating_add(preprocessed_len / 4));

    let mut dom_ctx = build_dom_context(&dom, parser, preprocessed_len);

    // Check for inline-block misnesting and repair if needed
    if has_inline_block_misnest(&dom_ctx, parser) {
        if let Some(repaired_html) = repair_with_html5ever(&preprocessed) {
            // Drop dom to release borrow on preprocessed
            drop(dom);
            preprocessed = preprocess_html(&repaired_html).into_owned();
            preprocessed_len = preprocessed.len();
            // Re-parse with repaired HTML
            dom = tl::parse(&preprocessed, parser_options)
                .map_err(|_| crate::error::ConversionError::ParseError("Failed to parse repaired HTML".to_string()))?;
            parser = dom.parser();
            dom_ctx = build_dom_context(&dom, parser, preprocessed_len);
            output = String::with_capacity(preprocessed_len.saturating_add(preprocessed_len / 4));
        }
    }

    // Plain text output: run the full pipeline (for metadata + visitor callbacks),
    // then return plain text instead of markdown.
    let is_plain_text = options.output_format == OutputFormat::Plain;

    let wants_frontmatter = options.extract_metadata && !options.convert_as_inline;
    #[cfg(feature = "metadata")]
    let wants_document = metadata_collector
        .as_ref()
        .is_some_and(|collector| collector.borrow().wants_document());
    #[cfg(not(feature = "metadata"))]
    let wants_document = false;

    if wants_frontmatter || wants_document {
        let mut head_metadata: Option<BTreeMap<String, String>> = None;
        #[cfg(feature = "metadata")]
        let mut document_lang: Option<String> = None;
        #[cfg(feature = "metadata")]
        let mut document_dir: Option<String> = None;

        for child_handle in dom.children() {
            if head_metadata.is_none() {
                let metadata = extract_head_metadata(child_handle, parser, options);
                if !metadata.is_empty() {
                    head_metadata = Some(metadata);
                }
            }

            #[cfg(feature = "metadata")]
            if wants_document {
                if let Some(tl::Node::Tag(tag)) = child_handle.get(parser) {
                    let tag_name = tag.name().as_utf8_str();
                    if tag_name == "html" || tag_name == "body" {
                        if document_lang.is_none() {
                            if let Some(Some(lang_bytes)) = tag.attributes().get("lang") {
                                document_lang = Some(lang_bytes.as_utf8_str().to_string());
                            }
                        }
                        if document_dir.is_none() {
                            if let Some(Some(dir_bytes)) = tag.attributes().get("dir") {
                                document_dir = Some(dir_bytes.as_utf8_str().to_string());
                            }
                        }
                    }
                }
            }
        }

        if wants_frontmatter {
            if let Some(metadata) = head_metadata.as_ref() {
                if !metadata.is_empty() {
                    let metadata_frontmatter = format_metadata_frontmatter(metadata);
                    output.push_str(&metadata_frontmatter);
                }
            }
        }

        #[cfg(feature = "metadata")]
        if wants_document {
            if let Some(ref collector) = metadata_collector {
                if let Some(metadata) = head_metadata {
                    if !metadata.is_empty() {
                        collector.borrow_mut().set_head_metadata(metadata);
                    }
                }
                if let Some(lang) = document_lang {
                    collector.borrow_mut().set_language(lang);
                }
                if let Some(dir) = document_dir {
                    collector.borrow_mut().set_text_direction(dir);
                }
            }
        }
    }

    let reference_collector = if options.link_style == crate::options::LinkStyle::Reference {
        Some(std::rc::Rc::new(std::cell::RefCell::new(
            crate::converter::reference_collector::ReferenceCollector::new(),
        )))
    } else {
        None
    };

    #[cfg(all(feature = "metadata", feature = "visitor"))]
    let ctx = Context::new(
        options,
        inline_collector,
        metadata_collector,
        visitor,
        structure_collector.as_ref().map(std::rc::Rc::clone),
        reference_collector.as_ref().map(std::rc::Rc::clone),
    );
    #[cfg(all(feature = "metadata", not(feature = "visitor")))]
    let ctx = Context::new(
        options,
        inline_collector,
        metadata_collector,
        _visitor,
        structure_collector.as_ref().map(std::rc::Rc::clone),
        reference_collector.as_ref().map(std::rc::Rc::clone),
    );
    #[cfg(all(not(feature = "metadata"), feature = "visitor"))]
    let ctx = Context::new(
        options,
        inline_collector,
        _metadata_collector,
        visitor,
        structure_collector.as_ref().map(std::rc::Rc::clone),
        reference_collector.as_ref().map(std::rc::Rc::clone),
    );
    #[cfg(all(not(feature = "metadata"), not(feature = "visitor")))]
    let ctx = Context::new(
        options,
        inline_collector,
        _metadata_collector,
        _visitor,
        structure_collector.as_ref().map(std::rc::Rc::clone),
        reference_collector.as_ref().map(std::rc::Rc::clone),
    );

    for child_handle in dom.children() {
        walk_node(child_handle, parser, &mut output, options, &ctx, 0, &dom_ctx);
    }

    if ctx.depth_exceeded.get() {
        return Err(crate::error::ConversionError::InvalidInput(format!(
            "DOM tree depth exceeds max_depth ({})",
            options.max_depth.unwrap_or(0)
        )));
    }

    #[cfg(feature = "visitor")]
    if let Some(err) = ctx.visitor_error.borrow().as_ref() {
        return Err(crate::error::ConversionError::Visitor(err.clone()));
    }

    // Drop ctx before unwrapping the structure collector Rc — ctx holds a cloned Rc
    // reference to the same collector, and Rc::try_unwrap requires exactly one reference.
    drop(ctx);

    // Append reference-style link definitions if any were collected
    if let Some(rc) = reference_collector {
        if let Ok(collector) = std::rc::Rc::try_unwrap(rc) {
            let ref_section = collector.into_inner().finish();
            if !ref_section.is_empty() {
                let trimmed_len = output.trim_end_matches('\n').len();
                output.truncate(trimmed_len);
                output.push_str("\n\n");
                output.push_str(&ref_section);
            }
        }
    }

    // If plain text was requested, discard the markdown output and return plain text.
    // The full pipeline was still run above so that metadata + visitor callbacks fire.
    if is_plain_text {
        let plain = extract_plain_text(&dom, parser, options)?;
        let document =
            structure_collector.and_then(|sc| std::rc::Rc::try_unwrap(sc).ok().map(|cell| cell.into_inner().finish()));
        return Ok((plain, document));
    }

    trim_line_end_whitespace(&mut output);
    let trimmed = output.trim_end_matches('\n');
    let markdown = if trimmed.is_empty() {
        String::new()
    } else {
        format!("{trimmed}\n")
    };

    // Finish the structure collector if present.
    let document =
        structure_collector.and_then(|sc| std::rc::Rc::try_unwrap(sc).ok().map(|cell| cell.into_inner().finish()));

    Ok((markdown, document))
}
// has_more_than_one_char moved to main_helpers
// is_inline_element available from utility::content

/// Recursively walk DOM nodes and convert to Markdown.
#[allow(clippy::only_used_in_recursion)]
#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn walk_node(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let Some(node) = node_handle.get(parser) else { return };

    if let Some(max) = options.max_depth {
        if depth > max {
            ctx.depth_exceeded.set(true);
            return;
        }
    }

    match node {
        tl::Node::Raw(bytes) => {
            let raw = bytes.as_utf8_str();
            crate::converter::text_node::process_text_node(
                raw.as_ref(),
                node_handle,
                parser,
                output,
                options,
                ctx,
                depth,
                dom_ctx,
            );
        }

        tl::Node::Tag(tag) => {
            let tag_name = match dom_ctx.tag_info(node_handle.get_inner(), parser) {
                Some(info) => Cow::Borrowed(info.name.as_str()),
                None => normalized_tag_name(tag.name().as_utf8_str()),
            };

            #[cfg(feature = "visitor")]
            if let Some(ref visitor_handle) = ctx.visitor {
                use crate::converter::visitor_hooks::{VisitAction, handle_visitor_element_start};

                let action = handle_visitor_element_start(
                    visitor_handle,
                    tag_name.as_ref(),
                    node_handle,
                    tag,
                    parser,
                    output,
                    ctx,
                    depth,
                    dom_ctx,
                );

                match action {
                    VisitAction::Continue => {}
                    VisitAction::Skip => return,
                    VisitAction::Custom => return,
                    VisitAction::Error => return,
                }
            }

            if should_drop_for_preprocessing(node_handle, tag_name.as_ref(), tag, parser, dom_ctx, options) {
                trim_trailing_whitespace(output);
                return;
            }

            if ctx.strip_tags.contains(tag_name.as_ref()) {
                let children = tag.children();
                {
                    for child_handle in children.top().iter() {
                        walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                    }
                }
                return;
            }

            if ctx.preserve_tags.contains(tag_name.as_ref()) {
                let html = serialize_tag_to_html(node_handle, parser);
                output.push_str(&html);
                return;
            }

            #[cfg(feature = "metadata")]
            if matches!(tag_name.as_ref(), "html" | "head" | "body") && ctx.metadata_wants_document {
                if let Some(ref collector) = ctx.metadata_collector {
                    let mut c = collector.borrow_mut();

                    if let Some(lang) = tag.attributes().get("lang").flatten() {
                        c.set_language(lang.as_utf8_str().to_string());
                    }

                    if let Some(dir) = tag.attributes().get("dir").flatten() {
                        c.set_text_direction(dir.as_utf8_str().to_string());
                    }
                }
            }

            #[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
            let element_output_start = output.len();

            match tag_name.as_ref() {
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    crate::converter::block::heading::handle(
                        &tag_name,
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                "p" => {
                    crate::converter::block::paragraph::handle(
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                // All inline elements routed to inline dispatcher
                "strong" | "b" | "em" | "i" | "mark" | "del" | "s" | "ins" | "u" | "small" | "sub" | "sup" | "kbd"
                | "samp" | "var" | "dfn" | "abbr" | "ruby" | "rb" | "rt" | "rp" | "rtc" | "span" => {
                    crate::converter::inline::dispatch_inline_handler(
                        &tag_name,
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                "a" => handle_link(node_handle, tag, parser, output, options, ctx, depth, dom_ctx),
                "img" => handle_img(node_handle, tag, parser, output, options, ctx, depth, dom_ctx),
                "graphic" => handle_graphic(node_handle, tag, parser, output, options, ctx, depth, dom_ctx),
                "code" => handle_code(node_handle, tag, parser, output, options, ctx, depth, dom_ctx),
                "pre" => handle_pre(node_handle, tag, parser, output, options, ctx, depth, dom_ctx),
                "blockquote" => handle_blockquote(node_handle, tag, parser, output, options, ctx, depth, dom_ctx),

                "time" | "data" => {
                    crate::converter::block::container::handle_passthrough(
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                // Noop elements that produce no output
                "wbr" | "thead" | "tbody" | "tfoot" | "tr" | "th" | "td" | "source" => {
                    crate::converter::block::container::handle_noop(
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                "br" => crate::converter::block::line_break::handle(
                    node_handle,
                    parser,
                    output,
                    options,
                    ctx,
                    depth,
                    dom_ctx,
                ),
                "hr" => crate::converter::block::horizontal_rule::handle(
                    node_handle,
                    parser,
                    output,
                    options,
                    ctx,
                    depth,
                    dom_ctx,
                ),
                "div" => {
                    crate::converter::block::div::handle(node_handle, parser, output, options, ctx, depth, dom_ctx);
                }
                "caption" => crate::converter::block::table::handle_caption(
                    node_handle,
                    parser,
                    output,
                    options,
                    ctx,
                    depth,
                    dom_ctx,
                ),
                "table" => crate::converter::block::table::handle_table_with_context(
                    node_handle,
                    parser,
                    output,
                    options,
                    ctx,
                    dom_ctx,
                    depth,
                ),

                // List elements routed to list dispatcher
                "ul" | "ol" | "li" | "dl" | "dt" | "dd" => {
                    crate::converter::list::dispatch_list_handler(
                        &tag_name,
                        node_handle,
                        tag,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                // Quote element routed to semantic dispatcher
                "q" => {
                    crate::converter::semantic::dispatch_semantic_handler(
                        &tag_name,
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                // Figure elements routed to semantic dispatcher
                "figure" | "figcaption" => {
                    crate::converter::semantic::dispatch_semantic_handler(
                        &tag_name,
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                // Semantic interactive elements routed to semantic dispatcher
                "details" | "summary" | "dialog" | "menu" => {
                    crate::converter::semantic::dispatch_semantic_handler(
                        &tag_name,
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                // Media elements routed to media dispatcher
                "audio" | "video" | "picture" | "iframe" | "svg" | "math" => {
                    crate::converter::media::dispatch_media_handler(
                        &tag_name,
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                // Form elements routed to form dispatcher
                "form" | "fieldset" | "legend" | "label" | "input" | "textarea" | "select" | "option" | "optgroup"
                | "button" | "progress" | "meter" | "output" | "datalist" => {
                    crate::converter::form::dispatch_form_handler(
                        &tag_name,
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                // Metadata elements routed to metadata handler
                "head" | "script" | "style" => {
                    crate::converter::metadata::handle(
                        &tag_name,
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                "body" | "html" => {
                    crate::converter::block::container::handle_structural_container(
                        node_handle,
                        parser,
                        output,
                        options,
                        ctx,
                        depth,
                        dom_ctx,
                    );
                }

                _ => {
                    crate::converter::block::unknown::handle(node_handle, parser, output, options, ctx, depth, dom_ctx);
                }
            }

            #[cfg(feature = "visitor")]
            if let Some(ref visitor_handle) = ctx.visitor {
                use crate::converter::visitor_hooks::handle_visitor_element_end;

                handle_visitor_element_end(
                    visitor_handle,
                    tag_name.as_ref(),
                    node_handle,
                    tag,
                    parser,
                    output,
                    element_output_start,
                    ctx,
                    depth,
                    dom_ctx,
                );
            }
        }

        tl::Node::Comment(_) => {}
    }
}
