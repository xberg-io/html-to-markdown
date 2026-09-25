//! Link element handler for HTML to Markdown conversion.
//!
//! Handles `<a>` elements including:
//! - Basic link markdown output `[text](href "title")`
//! - Autolinks when text matches href
//! - Links containing heading elements
//! - Complex link content with mixed block/inline elements
//! - Visitor callback integration
//! - Link metadata collection

#[cfg(feature = "metadata")]
use std::collections::BTreeMap;

use crate::converter::Context;
use crate::converter::block::heading::{find_single_heading_child, heading_allows_inline_images, push_heading};
use crate::converter::dom_context::DomContext;
use crate::converter::inline::link::{append_markdown_link, has_uri_scheme};
use crate::converter::main::walk_node;
use crate::converter::utility::content::{
    collect_link_label_text, get_text_content, node_is_block_level, normalize_link_label, normalized_tag_name,
};
use crate::converter::utility::escaping::escape_link_label;
use crate::options::ConversionOptions;
use crate::text;
use std::borrow::Cow;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node;

/// Handle an `<a>` (link) element and convert to Markdown.
///
/// This handler processes link elements including:
/// - Extracting href and title attributes
/// - Detecting autolinks (where text equals href)
/// - Handling links that contain heading elements
/// - Processing complex link content (mixed block/inline)
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Collecting link metadata when the metadata feature is enabled
/// - Generating appropriate markdown link output
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_link(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let href_attr = tag
        .attributes()
        .get("href")
        .flatten()
        .map(|v| text::decode_html_entities(&v.as_utf8_str()))
        .map(|href| ctx.resolve_url(&href).unwrap_or(href));
    // ~keep An empty `title=""` carries no information, and `[t](u "")` / `![a](i "")` is
    // ~keep noise that no Markdown serializer round-trips: re-rendering the output drops
    // ~keep the empty title, so the second pass no longer matches the first. Treat it as
    // ~keep absent, which is what it means.
    let title = crate::converter::utility::attributes::decoded_attribute(tag, "title").filter(|v| !v.is_empty());

    if let Some(href) = href_attr {
        let owned_children: Vec<tl::NodeHandle>;
        let children: &[tl::NodeHandle] = if let Some(c) = dom_ctx.children_of(node_handle.get_inner()) {
            c.as_slice()
        } else {
            owned_children = tag.children().top().iter().copied().collect();
            owned_children.as_slice()
        };
        let (inline_label, _block_nodes, saw_block) = collect_link_label_text(children, parser, dom_ctx);
        // ~keep #492: the only two prior consumers of `keep_inline_images_in` were headings
        // ~keep and layout cells (#433's precedent); `<a>` had none. Computed once here and
        // ~keep carried unchanged into both the block-label and inline-label `Context`
        // ~keep literals below, so the option means the same thing regardless of whether the
        // ~keep anchor happens to contain a block child.
        let link_allow_inline_images = ctx.keep_inline_images_in.contains("a");

        // ~keep Without block descendants the sweep above already visited exactly the nodes
        // ~keep `get_text_content` would and decoded them the same way, so its text is reused
        // ~keep rather than walking the `<a>` subtree a second time.
        let text_source: Cow<'_, str> = if saw_block {
            Cow::Owned(get_text_content(node_handle, parser, dom_ctx))
        } else {
            Cow::Borrowed(inline_label.as_str())
        };
        let normalized_text = text::normalize_whitespace_cow(text_source.as_ref());
        let raw_text = normalized_text.trim();

        // ~keep #490: partition the anchor's DIRECT children (not `collect_link_label_text`'s
        // ~keep topmost block *descendants*, which can sit several inline wrappers deep) into
        // ~keep inline and block using the identical `node_is_block_level` test, so every byte
        // ~keep of the anchor's content lands in exactly one of the two halves -- this is also
        // ~keep what catches a table buried under a block wrapper (`<a><div><table>...`).
        let (inline_children, deferred) = partition_link_children(children, parser, dom_ctx);
        let emit_blocks_separately = should_defer_table_blocks(ctx, &deferred, parser, dom_ctx);

        // ~keep GFM requires an absolute URI with a scheme (e.g. `https://…`, `mailto:…`);
        // ~keep bare paths or filenames must use the full `[text](href)` form (issue #397).
        // ~keep `!emit_blocks_separately` (#490): `raw_text` is whole-subtree text including
        // ~keep the deferred table's own cell text, so without this guard a table whose text
        // ~keep happened to equal the href would autolink on the TABLE's text and silently
        // ~keep drop the table itself.
        let is_autolink = options.autolinks
            && !options.default_title
            && !emit_blocks_separately
            && !href.is_empty()
            && has_uri_scheme(href.as_str())
            && (raw_text == href || (href.starts_with("mailto:") && raw_text == &href[7..]));

        if is_autolink {
            output.push('<');
            if href.starts_with("mailto:") && raw_text == &href[7..] {
                output.push_str(raw_text);
            } else {
                output.push_str(&href);
            }
            output.push('>');
            return;
        }

        if let Some((heading_level, heading_handle)) = find_single_heading_child(*node_handle, parser) {
            if let Some(heading_node) = heading_handle.get(parser) {
                if let tl::Node::Tag(heading_tag) = heading_node {
                    let heading_name = normalized_tag_name(heading_tag.name().as_utf8_str()).into_owned();
                    let mut heading_text = String::new();
                    let heading_ctx = Context {
                        in_heading: true,
                        convert_as_inline: true,
                        heading_allow_inline_images: heading_allows_inline_images(
                            &heading_name,
                            &ctx.keep_inline_images_in,
                        ),
                        ..ctx.clone()
                    };
                    walk_node(
                        &heading_handle,
                        parser,
                        &mut heading_text,
                        options,
                        &heading_ctx,
                        depth + 1,
                        dom_ctx,
                    );
                    let trimmed_heading = heading_text.trim();
                    if !trimmed_heading.is_empty() {
                        let escaped_label = escape_link_label(trimmed_heading);
                        let mut link_buffer = String::new();
                        append_markdown_link(
                            &mut link_buffer,
                            &escaped_label,
                            href.as_str(),
                            title.as_deref(),
                            raw_text,
                            options,
                            ctx.reference_collector.as_ref(),
                        );
                        push_heading(output, ctx, options, heading_level, link_buffer.as_str());
                        return;
                    }
                }
            }
        }

        let mut label = if emit_blocks_separately {
            // ~keep #490: only the DIRECT inline children feed the label -- the deferred
            // ~keep block children (which is what triggered this branch) are walked
            // ~keep separately, after the link, near the end of this function. Walk them
            // ~keep (do NOT reuse the text-only `inline_label`), or an `<img>` among the
            // ~keep inline children would render as nothing instead of `![alt](src)`.
            let mut content = String::new();
            let link_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                in_link: true,
                link_allow_inline_images,
                ..ctx.clone()
            };
            for child_handle in &inline_children {
                walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    &link_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
            normalize_link_label(&content)
        } else if saw_block {
            let mut content = String::new();
            let link_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                in_link: true,
                convert_as_inline: true,
                link_allow_inline_images,
                ..ctx.clone()
            };
            for child_handle in children {
                let mut child_buf = String::new();
                walk_node(
                    child_handle,
                    parser,
                    &mut child_buf,
                    options,
                    &link_ctx,
                    depth + 1,
                    dom_ctx,
                );
                if !child_buf.trim().is_empty()
                    && !content.is_empty()
                    && !content.chars().last().is_none_or(char::is_whitespace)
                    && !child_buf.chars().next().is_none_or(char::is_whitespace)
                {
                    content.push(' ');
                }
                content.push_str(&child_buf);
            }
            if content.trim().is_empty() {
                normalize_link_label(&inline_label)
            } else {
                normalize_link_label(&content)
            }
        } else {
            let mut content = String::new();
            let link_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                in_link: true,
                link_allow_inline_images,
                ..ctx.clone()
            };
            for child_handle in children {
                walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    &link_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
            normalize_link_label(&content)
        };

        // ~keep `raw_text` is already the whole-subtree text when `saw_block`, so this single
        // ~keep fallback covers both the block and inline cases. Suppressed when
        // ~keep `emit_blocks_separately` (#490): `raw_text` there is whole-subtree text
        // ~keep INCLUDING the deferred table's own cell text, so using it here would
        // ~keep duplicate the table's text into the label. Suppressing it instead lets the
        // ~keep href fallback immediately below fire.
        if !emit_blocks_separately && label.is_empty() && !raw_text.is_empty() {
            label = normalize_link_label(raw_text);
        }

        if label.is_empty() && !href.is_empty() && !children.is_empty() {
            // ~keep The href is raw attribute text that never passed through a text node's
            // ~keep normal escaping, unlike every other label source above (heading text,
            // ~keep inline content, `raw_text`) which was already escaped while it was
            // ~keep walked. Escaping it here the same way keeps this fallback consistent with
            // ~keep those paths -- without it, a `*`/`_`/literal backslash surviving unescaped
            // ~keep into the label round-trips into structure (emphasis, or a silently
            // ~keep swallowed backslash) once it is re-parsed.
            label = text::escape(
                &href,
                options.escape_misc,
                options.escape_asterisks,
                options.escape_underscores,
                options.escape_ascii,
            )
            .into_owned();
        }

        if label == "^" && href.starts_with('#') {
            label = "↑".to_string();
        }

        let escaped_label = escape_link_label(&label);

        // ~keep #490: whether the deferred block children (if any) should still be walked
        // ~keep after the link markdown below. `false` only for `Skip` (the caller asked for
        // ~keep nothing) and `PreserveHtml` (the serialized anchor already contains the
        // ~keep table) -- every other outcome, including the no-visitor default, still wrote
        // ~keep the link's own markdown/custom text and expects its deferred blocks to follow.
        #[cfg(feature = "visitor")]
        let mut should_emit_deferred_blocks = true;
        #[cfg(not(feature = "visitor"))]
        let should_emit_deferred_blocks = true;

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Link,
                Cow::Borrowed("a"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_link(&node_ctx, &href, &label, title.as_deref())
            };
            match visit_result {
                VisitResult::Continue => append_markdown_link(
                    output,
                    &escaped_label,
                    href.as_str(),
                    title.as_deref(),
                    label.as_str(),
                    options,
                    ctx.reference_collector.as_ref(),
                ),
                VisitResult::Custom(custom) => output.push_str(&custom),
                VisitResult::Skip => should_emit_deferred_blocks = false,
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                }
                VisitResult::PreserveHtml => {
                    output.push_str(&serialize_node(node_handle, parser));
                    should_emit_deferred_blocks = false;
                }
            }
        } else {
            append_markdown_link(
                output,
                &escaped_label,
                href.as_str(),
                title.as_deref(),
                label.as_str(),
                options,
                ctx.reference_collector.as_ref(),
            );
        }

        #[cfg(not(feature = "visitor"))]
        append_markdown_link(
            output,
            &escaped_label,
            href.as_str(),
            title.as_deref(),
            label.as_str(),
            options,
            ctx.reference_collector.as_ref(),
        );

        #[cfg(feature = "metadata")]
        if ctx.metadata_wants_links {
            if let Some(ref collector) = ctx.metadata_collector {
                let rel_attr = tag
                    .attributes()
                    .get("rel")
                    .flatten()
                    .map(|v| v.as_utf8_str().to_string());
                let mut attributes_map = BTreeMap::new();
                for (key, value_opt) in tag.attributes().iter() {
                    let key_str = key.to_string();
                    if key_str == "href" {
                        continue;
                    }

                    let value = value_opt.map(|v| v.to_string()).unwrap_or_default();
                    attributes_map.insert(key_str, value);
                }
                collector.borrow_mut().add_link(
                    href.clone(),
                    label,
                    title.as_deref().map(str::to_string),
                    rel_attr,
                    attributes_map,
                );
            }
        }

        // ~keep #490: walk the deferred block children (the wrapped `<table>`, or its block
        // ~keep ancestor) with `ctx` UNCHANGED -- not `link_ctx` -- so it renders as a normal
        // ~keep block (a real GFM table) rather than being forced inline. Skipped when the
        // ~keep visitor already produced or suppressed all output for this link (see
        // ~keep `should_emit_deferred_blocks`'s doc comment above).
        if emit_blocks_separately && should_emit_deferred_blocks {
            for child_handle in &deferred {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
    } else {
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
    }
}

/// Partition an anchor's DIRECT children into (inline, block) using `node_is_block_level`
/// (issue #490).
///
/// ~keep Deliberately direct children only, not `collect_link_label_text`'s topmost block
/// ~keep *descendants* (which can sit several inline wrappers deep) -- every byte of the
/// ~keep anchor's content must land in exactly one of the two halves, which is also what
/// ~keep catches a table buried under a block wrapper (`<a><div><table>...`).
fn partition_link_children(
    children: &[tl::NodeHandle],
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) -> (Vec<tl::NodeHandle>, Vec<tl::NodeHandle>) {
    children
        .iter()
        .copied()
        .partition(|child| !node_is_block_level(child, parser, dom_ctx))
}

/// Decide whether an anchor's deferred (direct block) children should be rendered as
/// separate blocks after the link, instead of being walked into the inline label
/// (issue #490).
///
/// ~keep A wrapped `<table>` crushed a whole GFM table into the link label: the label would
/// ~keep contain literal `|`/`\n` that either get escaped into noise or, unescaped, corrupt
/// ~keep the OUTER row on reparse. Emitting the table as a separate block after the link is
/// ~keep the only shape that round-trips. Gated on an actual `<table>` inside a deferred
/// ~keep subtree so the common case (a `<p>`/`<div>`-only anchor) pays nothing and behaves
/// ~keep exactly as before. `!ctx.convert_as_inline` and `!ctx.in_heading` are load-bearing,
/// ~keep not polish: a heading's `normalize_heading_text` folds `\n` to spaces, so a block
/// ~keep table inside a heading is no better than the crushed-label bug; an inline context (a
/// ~keep data cell's own label, or this link nested inside an outer link's label) has nowhere
/// ~keep to put a deferred block at all.
///
/// ~keep A layout cell is the exception among inline contexts (issue #503): it converts as
/// ~keep inline only because its row becomes one list item, and that item's text already
/// ~keep holds a bare nested table. Refusing there sent the table through the label, where
/// ~keep `escape_link_label` turned every inner link into `\[One\](/one)` text.
fn should_defer_table_blocks(
    ctx: &Context,
    deferred: &[tl::NodeHandle],
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) -> bool {
    (!ctx.convert_as_inline || ctx.in_layout_cell)
        && !ctx.in_heading
        && deferred.iter().any(|handle| subtree_has_table(handle, parser, dom_ctx))
}

/// Short-circuiting DFS: does `handle`'s subtree (itself or any descendant) contain a
/// `<table>`?
///
/// ~keep Checked only against a deferred block child of an `<a>` (issue #490), so the cost
/// ~keep is paid only when the anchor actually has a block child, and the walk stops at the
/// ~keep first `<table>` found regardless of subtree size.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn subtree_has_table(handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    let Some(tl::Node::Tag(tag)) = handle.get(parser) else {
        return false;
    };
    if normalized_tag_name(tag.name().as_utf8_str()) == "table" {
        return true;
    }
    if let Some(children) = dom_ctx.children_of(handle.get_inner()) {
        children.iter().any(|child| subtree_has_table(child, parser, dom_ctx))
    } else {
        tag.children()
            .top()
            .iter()
            .any(|child| subtree_has_table(child, parser, dom_ctx))
    }
}
