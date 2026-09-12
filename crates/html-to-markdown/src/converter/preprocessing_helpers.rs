//! HTML preprocessing and validation helpers.
//!
//! This module contains helper functions for preprocessing HTML before conversion,
//! including validation and normalization checks.

use crate::converter::dom_context::{DomContext, TagInfo};
use crate::converter::main_helpers::{is_ascii_whitespace_only, is_inline_element};
use crate::converter::utility::attributes::{attribute_matches_any, element_has_navigation_hint};
use crate::options::ConversionOptions;

/// Check if an inline ancestor element is allowed to contain block-level elements.
pub fn inline_ancestor_allows_block(tag_name: &str) -> bool {
    matches!(tag_name, "a" | "ins" | "del")
}

/// Ancestor state inherited top-down while scanning for misnested elements.
///
/// ~keep Each field mirrors one of the three independent ancestor-chain scans the
/// ~keep original implementation ran separately per node (O(depth) each); carrying
/// ~keep the already-computed parent result down to children makes every node O(1)
/// ~keep instead, since each scan only ever needs the *nearest* qualifying ancestor.
#[derive(Clone, Copy)]
struct MisnestState {
    /// True if a strict ancestor is an anchor.
    inside_anchor: bool,
    /// True if this node or any strict ancestor is `<pre>`/`<code>`.
    inside_preformatted: bool,
    /// True if any strict ancestor is an inline element that disallows block children.
    blocked_by_inline_ancestor: bool,
    /// Result of a `has_p_ancestor` scan started at this node (used by its children).
    p_ancestor_state: bool,
    /// True if a strict ancestor is `<li>`/`<dt>`/`<dd>` with no intervening
    /// `<ul>`/`<ol>`/`<dl>` (i.e. we are still "inside" that same list item).
    list_item_ancestor_state: bool,
    /// True if a strict ancestor is `<td>`/`<th>` with no intervening
    /// `<table>`/`<thead>`/`<tbody>`/`<tfoot>`/`<tr>` (i.e. we are still "inside"
    /// that same cell's content, not a fresh row).
    cell_ancestor_state: bool,
}

impl MisnestState {
    const ROOT: Self = Self {
        inside_anchor: false,
        inside_preformatted: false,
        blocked_by_inline_ancestor: false,
        p_ancestor_state: false,
        list_item_ancestor_state: false,
        cell_ancestor_state: false,
    };
}

/// True when `info`, reached with inherited `state`, is one of the misnestings this pass repairs.
///
/// ~keep Each disjunct is an independent, side-effect-free shape check, so collecting them here
/// ~keep preserves the original sequence of early returns exactly -- same order, same
/// ~keep short-circuiting, same answer -- while keeping the walk itself under the complexity limit
/// ~keep (issue #465).
fn node_is_misnested(info: &TagInfo, state: MisnestState, self_inside_preformatted: bool) -> bool {
    // An anchor nested inside another anchor, outside preformatted content.
    (info.name == "a" && state.inside_anchor && !state.inside_preformatted)
        // ~keep Table elements under <p>: tl misparsed an unclosed <p> in <td>.
        || (matches!(info.name.as_str(), "td" | "tr" | "th") && state.p_ancestor_state)
        || (info.is_block && !self_inside_preformatted && state.blocked_by_inline_ancestor)
        // ~keep <li>/<dt>/<dd> nested under another one without an intervening list
        // ~keep container: tl absorbed the next item because a <p>/<div> inside the
        // ~keep previous one was left unclosed.
        || (matches!(info.name.as_str(), "li" | "dt" | "dd") && state.list_item_ancestor_state)
        // ~keep <tr> nested directly inside a <td>/<th>: not reachable per HTML5 tree
        // ~keep construction, and silently dropped by the table renderer (issue #486).
        || (info.name == "tr" && state.cell_ancestor_state)
}

/// Inherited state to hand to every child of `info`.
fn child_misnest_state(info: &TagInfo, state: MisnestState, self_inside_preformatted: bool) -> MisnestState {
    MisnestState {
        inside_anchor: state.inside_anchor || info.name == "a",
        inside_preformatted: self_inside_preformatted,
        blocked_by_inline_ancestor: state.blocked_by_inline_ancestor
            || (is_inline_element(&info.name) && !inline_ancestor_allows_block(&info.name)),
        p_ancestor_state: p_ancestor_state_for(&info.name, state.p_ancestor_state),
        list_item_ancestor_state: list_item_ancestor_state_for(&info.name, state.list_item_ancestor_state),
        cell_ancestor_state: cell_ancestor_state_for(&info.name, state.cell_ancestor_state),
    }
}

/// Detect malformed nesting that requires HTML5 tree repair.
///
/// Nested anchors must be closed by the HTML5 adoption agency algorithm so their
/// destinations remain separate Markdown links (issue #479).
///
/// Excludes elements inside `<pre>` or `<code>` blocks, as they have special
/// whitespace preservation rules and should not be repaired.
///
/// Also detects table structural elements (`td`, `tr`, `th`) nested under `<p>` —
/// a structural impossibility in valid HTML that signals the `tl` parser absorbed
/// a table into a paragraph because of an unclosed `<p>` (common in Word/Outlook
/// HTML such as `<p class='MsoNormal'>` cells). Issue #336.
///
/// Also detects a `<li>`/`<dt>`/`<dd>` nested under another `<li>`/`<dt>`/`<dd>`
/// with no intervening `<ul>`/`<ol>`/`<dl>`. `<li>content<div><li>more` is a
/// structural impossibility in valid HTML: the HTML5 spec implies a closing
/// `</li>` when a new `<li>` starts, regardless of what block elements (`<p>`,
/// `<div>`, …) are still open inside it. `tl` does not apply that rule, so an
/// unclosed `<p>`/`<div>` inside a list item causes it to nest the next `<li>`
/// as a child instead of treating it as a sibling.
///
/// Also detects a `<tr>` nested directly inside a `<td>`/`<th>` — a structural
/// impossibility in valid HTML (a `<tr>` is only ever a child of `<table>`,
/// `<thead>`, `<tbody>`, or `<tfoot>`). HTML5's "in cell" insertion mode closes the
/// open cell (and its row) before starting the new row as a sibling of the one that
/// held the cell; `tl` instead nests it literally, and the table renderer only walks
/// a cell's own row/table ancestry, so the nested row's content is never reached and
/// is silently dropped. Issue #486.
///
/// Also detects a `<table>` with a direct child that HTML5's "in table" insertion
/// mode would foster-parent (non-whitespace text) or restructure (any element
/// outside the small set valid directly under `<table>`, e.g. a stray `<td>` or
/// `<a>` with no `<tr>`). `tl` has no foster-parenting or table auto-fixup: it
/// leaves such content exactly where it was written, as a direct child of the
/// `<table>` node, and the table handler (`converter::block::table::builder`)
/// only recognises `caption`/`thead`/`tbody`/`tfoot`/`tr`/`colgroup`/`col` there —
/// silently dropping raw text and routing anything else through a no-op handler.
///
/// ~keep Walks the tree top-down exactly once, carrying inherited ancestor state
/// ~keep (see [`MisnestState`]) instead of re-walking every node's ancestor chain.
/// ~keep The original per-node ancestor walk was O(depth) per node — O(n²) total on
/// ~keep a deeply nested chain (e.g. 20k nested `<div>`s took ~30s; this pass alone
/// ~keep accounted for essentially all of it, confirmed via phase timing in
/// ~keep `tools/benchmark-harness/examples/profile_deep_nesting_phases.rs`).
pub fn has_inline_block_misnest(dom_ctx: &DomContext, parser: &tl::Parser) -> bool {
    let mut stack: Vec<(tl::NodeHandle, MisnestState)> = dom_ctx
        .root_children
        .iter()
        .map(|handle| (*handle, MisnestState::ROOT))
        .collect();

    while let Some((handle, state)) = stack.pop() {
        let node_id = handle.get_inner();
        if !matches!(handle.get(parser), Some(tl::Node::Tag(_))) {
            continue;
        }
        let Some(info) = dom_ctx.tag_info(node_id, parser) else {
            continue;
        };

        let self_inside_preformatted = state.inside_preformatted || matches!(info.name.as_str(), "pre" | "code");
        if node_is_misnested(info, state, self_inside_preformatted) {
            return true;
        }

        if let Some(children) = dom_ctx.children_of(node_id) {
            // ~keep <table> with a direct child that a spec-compliant parser would
            // ~keep foster-parent or restructure (see the doc comment above).
            if info.name == "table"
                && children
                    .iter()
                    .any(|child| is_foster_parenting_candidate(*child, parser, dom_ctx))
            {
                return true;
            }

            let child_state = child_misnest_state(info, state, self_inside_preformatted);
            stack.extend(children.iter().map(|child| (*child, child_state)));
        }
    }

    false
}

/// True if `child`, as a direct child of a `<table>` element, is content that a
/// spec-compliant HTML5 parser would foster-parent (non-whitespace text) or
/// restructure (an element outside the small set valid directly under `<table>`)
/// rather than leave in place.
///
/// A comment is neither: HTML5's "in table" insertion mode inserts a comment token
/// as a child of the current node (the table itself), so it is not lost and needs
/// no repair.
fn is_foster_parenting_candidate(child: tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    match child.get(parser) {
        Some(tl::Node::Raw(bytes)) => {
            let raw = bytes.as_utf8_str();
            let decoded = crate::text::decode_html_entities_cow(raw.as_ref());
            !is_ascii_whitespace_only(&decoded)
        }
        Some(tl::Node::Tag(_)) => dom_ctx
            .tag_name_for(child, parser)
            .is_some_and(|name| !is_table_structural_child(name.as_ref())),
        _ => false,
    }
}

/// Tag names valid as a direct child of `<table>` per the HTML5 "in table" insertion
/// mode: `caption`, `colgroup`, `col`, `thead`/`tbody`/`tfoot`, `tr`, plus
/// `style`/`script`/`template`, which are left wherever they are written. `row` is
/// this codebase's normalized alias for `tr` from non-HTML input sources (see
/// `converter::block::table::scanner`).
fn is_table_structural_child(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "caption" | "colgroup" | "col" | "thead" | "tbody" | "tfoot" | "tr" | "row" | "style" | "script" | "template"
    )
}

/// Compute the `has_p_ancestor` scan result to hand down to a node's children,
/// given the node's own tag name and the state its own parent handed down.
///
/// Mirrors the original ancestor walk's stopping rule: a `<p>` ancestor found
/// before crossing a `table`/`body`/`html` boundary counts; hitting the boundary
/// first resets the search to "no `<p>` ancestor".
fn p_ancestor_state_for(tag_name: &str, inherited: bool) -> bool {
    if tag_name == "p" {
        true
    } else if matches!(tag_name, "table" | "body" | "html") {
        false
    } else {
        inherited
    }
}

/// Compute the `list_item_ancestor_state` scan result to hand down to a node's
/// children, given the node's own tag name and the state its own parent handed down.
///
/// A `<li>`/`<dt>`/`<dd>` ancestor found before crossing a `<ul>`/`<ol>`/`<dl>`
/// (a new list context) counts; crossing into a new list container, or the
/// `table`/`body`/`html` boundary, resets the search — a `<li>` nested inside a
/// genuinely new list (or a new formatting context such as a table cell) is not
/// misnesting.
fn list_item_ancestor_state_for(tag_name: &str, inherited: bool) -> bool {
    if matches!(tag_name, "li" | "dt" | "dd") {
        true
    } else if matches!(tag_name, "ul" | "ol" | "dl" | "table" | "body" | "html") {
        false
    } else {
        inherited
    }
}

/// Compute the `cell_ancestor_state` scan result to hand down to a node's children,
/// given the node's own tag name and the state its own parent handed down.
///
/// A `<td>`/`<th>` ancestor found before crossing a `<table>`/`<thead>`/`<tbody>`/
/// `<tfoot>`/`<tr>` boundary counts; crossing any of those boundaries resets the
/// search, since a `<tr>` there is legitimately positioned relative to a new row or
/// table context.
fn cell_ancestor_state_for(tag_name: &str, inherited: bool) -> bool {
    if matches!(tag_name, "td" | "th") {
        true
    } else if matches!(tag_name, "table" | "thead" | "tbody" | "tfoot" | "tr") {
        false
    } else {
        inherited
    }
}

/// Determine if a node should be dropped during preprocessing.
///
/// Behavior depends on the [`PreprocessingPreset`]:
///
/// - **Minimal**: Only scripts/styles are stripped (handled elsewhere). This function
///   drops nothing — all structural elements are preserved.
/// - **Standard** (default): Drops `<nav>` unconditionally. Drops `<header>`, `<footer>`,
///   and `<aside>` only when they have navigation hints (class/role/aria attributes
///   indicating site chrome). Drops `<form>` when `remove_forms` is enabled.
/// - **Aggressive**: All of Standard, plus: drops `<footer>`, `<aside>`, `<noscript>`
///   unconditionally. Drops ANY element with navigation hints in class/id/role
///   (e.g. `<div class="sidebar">`). Drops elements with noise-related classes/roles.
pub fn should_drop_for_preprocessing(tag_name: &str, tag: &tl::HTMLTag, options: &ConversionOptions) -> bool {
    use crate::options::PreprocessingPreset;

    if !options.preprocessing.enabled {
        return false;
    }

    let preset = options.preprocessing.preset;

    if preset == PreprocessingPreset::Minimal {
        return false;
    }

    if options.preprocessing.remove_forms && tag_name == "form" {
        return true;
    }

    let is_aggressive = preset == PreprocessingPreset::Aggressive;

    // ~keep Aggressive: drop <noscript> — its content is fallback for no-JS browsers.
    if is_aggressive && tag_name == "noscript" {
        return true;
    }

    if !options.preprocessing.remove_navigation {
        return false;
    }

    let has_nav_hint = element_has_navigation_hint(tag);

    if tag_name == "nav" {
        return true;
    }

    if tag_name == "header" {
        return has_nav_hint;
    }

    if tag_name == "footer" || tag_name == "aside" {
        return is_aggressive || has_nav_hint;
    }

    if is_aggressive && has_nav_hint {
        return true;
    }

    if is_aggressive {
        if element_has_noise_hint(tag) {
            return true;
        }
    }

    false
}

/// Check if an element has noise-related hints (ads, cookie banners, social sharing).
fn element_has_noise_hint(tag: &tl::HTMLTag) -> bool {
    const NOISE_KEYWORDS: &[&str] = &[
        "cookie",
        "consent",
        "gdpr",
        "banner",
        "advertisement",
        "ad-container",
        "advert",
        "social-share",
        "share-buttons",
        "popup",
        "modal-overlay",
        "newsletter-signup",
    ];

    attribute_matches_any(tag, "class", NOISE_KEYWORDS) || attribute_matches_any(tag, "id", NOISE_KEYWORDS)
}
