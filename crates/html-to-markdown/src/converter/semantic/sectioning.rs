//! Handlers for HTML5 sectioning elements.
//!
//! Processes semantic sectioning elements:
//! - `<article>` - Independent, self-contained content
//! - `<section>` - Generic grouping of thematic content
//! - `<nav>` - Navigation links (typically rendered inline or in sidebars)
//! - `<aside>` - Peripheral content (sidebars, callouts)
//! - `<header>` - Introductory content (page headers)
//! - `<footer>` - End content (page footers)
//! - `<main>` - Primary content area
//!
//! All these elements are treated as block-level containers.
//! Their content is extracted and formatted with proper spacing.

/// Handles sectioning elements (article, section, nav, aside, header, footer, main).
///
/// Sectioning elements are rendered as block-level containers. When in inline
/// conversion mode, their content is rendered inline without block spacing.
/// Otherwise, content is wrapped with proper blank lines to separate from other blocks.
///
/// # Behavior
///
/// - **Inline mode**: Children are processed inline; block spacing is skipped
/// - **Block mode**: Content is collected, trimmed, and formatted with blank lines
/// - **Empty content**: Empty sections are skipped entirely
///
/// # Implementation Note
///
/// Sectioning elements act as transparent containers—their presence doesn't
/// add any special formatting beyond structural grouping.
pub fn handle(
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

        let mut content = String::with_capacity(256);
        let children = tag.children();
        {
            let section_ctx = crate::converter::list::utils::nested_block_context(output, ctx, options);
            for child_handle in children.top().iter() {
                super::walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    &section_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
        }

        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since this sectioning element's
        // ~keep content is simply finished here — so this closes its own trailing run the
        // ~keep same way `paragraph.rs` closes its own (issue #464 follow-up). `content` is
        // ~keep pushed to `output` raw below (not `.trim()`-ed), so the strip has to run here.
        crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer(
            &mut content,
            options.newline_style,
        );

        if content.trim().is_empty() {
            return;
        }

        // ~keep Inside a list item the section starts at the item's content column (issue #583).
        if ctx.in_list_item && ctx.blockquote_depth == 0 && !ctx.in_table_cell && !output.is_empty() {
            crate::converter::list::utils::start_block_in_list_item(output, ctx, options);
        } else if !output.is_empty() && !output.ends_with("\n\n") {
            output.push_str("\n\n");
        }

        output.push_str(&content);

        if content.ends_with('\n') && !content.ends_with("\n\n") {
            output.push('\n');
        } else if !content.ends_with('\n') {
            output.push_str("\n\n");
        }
    }
}
