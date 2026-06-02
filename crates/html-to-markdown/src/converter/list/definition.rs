//! Definition list handling (dl, dt, dd elements).
//!
//! Processes definition lists with:
//! - Definition terms (dt)
//! - Definition descriptions (dd)
//! - Plain block formatting (no Pandoc colon syntax)

use crate::options::ConversionOptions;
use std::borrow::Cow;
use tl;

// Type aliases for Context and DomContext to avoid circular imports
type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle definition list element (<dl>).
///
/// Groups dt/dd pairs and formats them with proper Markdown separation.
pub fn handle_dl(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let tag = match node_handle.get(parser) {
        Some(tl::Node::Tag(t)) => t,
        _ => return,
    };

    if ctx.convert_as_inline {
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                use crate::converter::walk_node;
                walk_node(child_handle, parser, output, options, ctx, depth, dom_ctx);
            }
        }
        return;
    }

    let mut content = String::new();
    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            crate::converter::walk_node(child_handle, parser, &mut content, options, ctx, depth, dom_ctx);
        }
    }

    let trimmed = content.trim();
    if !trimmed.is_empty() {
        if !output.is_empty() && !output.ends_with("\n\n") {
            output.push_str("\n\n");
        }
        output.push_str(trimmed);
        output.push_str("\n\n");
    }
}

/// Handle definition term element (<dt>).
///
/// Outputs the term text followed by a newline.
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_dt(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let tag = match node_handle.get(parser) {
        Some(tl::Node::Tag(t)) => t,
        _ => return,
    };

    let mut content = String::with_capacity(64);
    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            crate::converter::walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
        }
    }
    let trimmed = content.trim().to_owned();
    if trimmed.is_empty() {
        return;
    }

    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::converter::utility::content::collect_tag_attributes;
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let attributes = collect_tag_attributes(tag);
        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
        let node_ctx = NodeContext {
            node_type: NodeType::DefinitionTerm,
            tag_name: Cow::Borrowed("dt"),
            attributes: Cow::Owned(attributes),
            depth,
            index_in_parent,
            parent_tag: parent_tag.map(Cow::Owned),
            is_inline: false,
        };
        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_definition_term(&node_ctx, &trimmed)
        };
        match visit_result {
            VisitResult::Continue => {}
            VisitResult::Skip => return,
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
                if !ctx.convert_as_inline && !custom.ends_with('\n') {
                    output.push('\n');
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

    if ctx.convert_as_inline {
        output.push_str(&trimmed);
    } else {
        output.push_str(&trimmed);
        output.push('\n');
    }
}

/// Handle definition description element (<dd>).
///
/// Outputs the description as a plain block.
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_dd(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let tag = match node_handle.get(parser) {
        Some(tl::Node::Tag(t)) => t,
        _ => return,
    };

    let mut content = String::with_capacity(128);
    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            crate::converter::walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
        }
    }

    let trimmed = content.trim().to_owned();
    if trimmed.is_empty() {
        return;
    }

    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::converter::utility::content::collect_tag_attributes;
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let attributes = collect_tag_attributes(tag);
        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
        let node_ctx = NodeContext {
            node_type: NodeType::DefinitionDescription,
            tag_name: Cow::Borrowed("dd"),
            attributes: Cow::Owned(attributes),
            depth,
            index_in_parent,
            parent_tag: parent_tag.map(Cow::Owned),
            is_inline: false,
        };
        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_definition_description(&node_ctx, &trimmed)
        };
        match visit_result {
            VisitResult::Continue => {}
            VisitResult::Skip => return,
            VisitResult::Custom(custom) => {
                if ctx.convert_as_inline {
                    output.push_str(&custom);
                } else {
                    output.push_str(&custom);
                    if !custom.ends_with('\n') {
                        output.push_str("\n\n");
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

    if ctx.convert_as_inline {
        output.push_str(&trimmed);
    } else {
        output.push_str(&trimmed);
        output.push_str("\n\n");
    }
}
