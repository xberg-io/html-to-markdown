//! Handler for semantic inline elements (mark, del, s, ins, u, small, sub, sup, var, dfn, abbr, span).
//!
//! Converts HTML semantic tags to Markdown formatting with support for:
//! - Highlight/mark element with configurable styles (==, ::, ^^, <mark>)
//! - Strikethrough (del, s tags) with ~~ syntax
//! - Underline/inserted text (ins, u tags) with == syntax
//! - Small text (passes through without formatting)
//! - Subscript and superscript with configurable symbols
//! - Variable (var) and definition (dfn) text with italic formatting
//! - Abbreviation (abbr) text with optional title attribute
//! - Span element with special handling for OCR words and whitespace
//! - Visitor callbacks for custom processing (feature-gated)

mod marks;
mod typography;

use crate::options::ConversionOptions;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handler for semantic inline elements: mark, del, s, ins, u, small, sub, sup, var, dfn, abbr, span.
///
/// Processes semantic content based on tag and options:
/// - Mark: configurable highlight style (==, ::, ^^, <mark>, **bold, none)
/// - Del/S: strikethrough with ~~ and visitor callback support
/// - Ins: underline with == and visitor callback support
/// - U: underline with visitor callback support
/// - Small: pass through without formatting
/// - Sub/Sup: wrap with configurable symbols
/// - Var: wrap with italic symbol (`strong_em_symbol`)
/// - Dfn: wrap with italic symbol (`strong_em_symbol`)
/// - Abbr: pass through content with optional title in parentheses
/// - Span: pass through content with special handling for OCR words and whitespace
///
/// # Note
/// This function references helper functions and `walk_node` from converter.rs
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
    match tag_name {
        "mark" => {
            marks::handle_mark(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "del" | "s" | "strike" => {
            marks::handle_strikethrough(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "ins" => {
            marks::handle_inserted(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "u" => {
            marks::handle_underline(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "small" => {
            typography::handle_small(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "sub" => {
            typography::handle_subscript(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "sup" => {
            typography::handle_superscript(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "var" => {
            typography::handle_variable(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "dfn" => {
            typography::handle_definition(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "abbr" => {
            typography::handle_abbreviation(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "span" => {
            typography::handle_span(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        _ => {}
    }
}
