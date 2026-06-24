//! Performance caching utilities.
//!
//! Caching mechanisms for expensive operations during conversion, including
//! DOM context building and cache capacity management.

use crate::converter::DomContext;
use std::num::NonZeroUsize;

/// Build a DOM context with hierarchical node information.
///
/// Pre-computes parent-child relationships, sibling indices, and caches
/// tag information for efficient DOM navigation during conversion.
pub fn build_dom_context(dom: &tl::VDom, parser: &tl::Parser, input_len: usize) -> DomContext {
    let cache_capacity = text_cache_capacity_for_input(input_len);
    let mut ctx = DomContext {
        parent_map: Vec::new(),
        children_map: Vec::new(),
        sibling_index_map: Vec::new(),
        root_children: dom.children().to_vec(),
        node_map: Vec::new(),
        tag_info_map: Vec::new(),
        prev_inline_like_map: Vec::new(),
        next_inline_like_map: Vec::new(),
        next_tag_map: Vec::new(),
        next_whitespace_map: Vec::new(),
        text_cache: std::cell::RefCell::new(lru::LruCache::new(cache_capacity)),
    };

    for (index, child_handle) in dom.children().iter().enumerate() {
        let id = child_handle.get_inner();
        ctx.ensure_capacity(id);
        ctx.sibling_index_map[id as usize] = Some(index);
        record_node_hierarchy(*child_handle, None, parser, &mut ctx);
    }

    ctx
}

/// Calculate appropriate cache capacity based on input size.
///
/// Returns a cache capacity between 32 and `TEXT_CACHE_CAPACITY`,
/// scaled proportionally to input size (1KB = 1 slot).
pub fn text_cache_capacity_for_input(input_len: usize) -> NonZeroUsize {
    const TEXT_CACHE_CAPACITY: usize = 256;
    // `clamp(32, TEXT_CACHE_CAPACITY)` guarantees `target >= 32 > 0`, so `new` always returns Some.
    let target = (input_len / 1024).clamp(32, TEXT_CACHE_CAPACITY);
    NonZeroUsize::new(target).unwrap_or(NonZeroUsize::MIN)
}

/// Record node hierarchy into DOM context.
///
/// Builds the complete parent-child relationship map for efficient tree traversal.
pub fn record_node_hierarchy(
    node_handle: tl::NodeHandle,
    parent: Option<u32>,
    parser: &tl::Parser,
    ctx: &mut DomContext,
) {
    // Traverse with an explicit work stack rather than native recursion. `tl`
    // does not apply HTML5 implied-end-tags, so a document with thousands of
    // unclosed elements (e.g. `<td>` or `<div>`) nests into a linear chain
    // thousands deep; recursing it would overflow the native stack and abort
    // the process. Each node only writes its own slots, so visitation order is
    // immaterial to the resulting maps.
    let mut work = vec![(node_handle, parent)];
    while let Some((node_handle, parent)) = work.pop() {
        let id = node_handle.get_inner();
        ctx.ensure_capacity(id);
        ctx.parent_map[id as usize] = parent;
        ctx.node_map[id as usize] = Some(node_handle);

        if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
            let children: Vec<_> = tag.children().top().iter().copied().collect();
            for (index, child) in children.iter().enumerate() {
                let child_id = child.get_inner();
                ctx.ensure_capacity(child_id);
                ctx.sibling_index_map[child_id as usize] = Some(index);
                work.push((*child, Some(id)));
            }
            ctx.children_map[id as usize] = Some(children);
        }
    }
}
