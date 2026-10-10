//! Anchor provenance across html5ever's tree repair.
//!
//! The adoption agency algorithm closes an `<a>` at a block boundary and *reconstructs* it
//! inside the block, so `<a href="/o"><div><a href="/i">Inner</a></div></a>` repairs to
//! `<a href="/o"></a><div><a href="/o"></a><a href="/i">Inner</a></div>` -- the same tree a
//! browser builds. Rendered faithfully that is two links to `/o`, one of them empty, which
//! is the duplication issue #493 reports. html5ever creates the clone through the same
//! `TreeSink::create_element` call as an authored element, with no flag, so the renderer
//! cannot tell them apart -- and a renderer-side rule keyed on shape either drops a genuine
//! empty anchor (`CommonMark` example 484) or a deliberately authored duplicate.
//!
//! The clone does carry one exact signal: it is created from the *original start-tag token's*
//! attributes. Stamping every `<a>` start tag with a private origin id before it reaches the
//! tree builder therefore marks the authored element and each of its clones with the same id,
//! and the split can be undone on the repaired tree before it is serialized. ~keep
//!
//! The same token sink records whether the input wrote a `<body>` start tag, which the tree
//! also no longer shows, and gives the repair up when the tree builder nests too deep: see
//! [`parse_with_anchor_origins`]. ~keep

use std::cell::Cell;
use std::collections::BTreeMap;
use std::rc::Rc;

use html5ever::tendril::StrTendril;
use html5ever::tokenizer::{BufferQueue, Tag, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts};
use html5ever::tree_builder::{Tracer, TreeBuilder, TreeBuilderOpts, TreeSink};
use html5ever::{Attribute, LocalName, QualName, TokenizerResult, ns};

use crate::rcdom::{Handle, NodeData, RcDom};

/// The private attribute that carries an anchor's origin id through the tree builder.
const ORIGIN_ATTR: &str = "data-h2m-anchor-origin";

/// Elements that make an anchor non-empty even without text.
const CONTENT_BEARING: [&str; 14] = [
    "img", "picture", "video", "audio", "svg", "canvas", "iframe", "object", "embed", "input", "math", "button",
    "textarea", "select",
];

/// The number of elements the tree builder may hold open before the repair is given up.
///
/// ~keep For most start tags the tree builder searches its stack of open elements (the scope
/// ~keep checks of the standard), so each tag costs the depth of the tree and a document that
/// ~keep never ends its blocks costs the square of its size. Chrome and Safari keep at most 512
/// ~keep elements open and flatten what is deeper, which html5ever does not do: past this depth
/// ~keep its tree is not the tree of a browser, and the converter cuts the page long before
/// ~keep (`max_depth`). The count includes the active formatting elements.
const MAX_OPEN_ELEMENTS: usize = 512;

/// The tree builder is measured once in this many start tags. One measurement reads the whole
/// stack, so this keeps its cost per tag constant.
const START_TAGS_PER_MEASUREMENT: usize = 64;

/// True when the tree builder is measured after start tag number `start_tags`.
const fn is_measured(start_tags: usize) -> bool {
    start_tags.is_multiple_of(START_TAGS_PER_MEASUREMENT)
}

/// Counts the handles that the tree builder holds.
struct HandleCount(Cell<usize>);

impl Tracer for HandleCount {
    type Handle = Handle;

    fn trace_handle(&self, _node: &Handle) {
        self.0.set(self.0.get() + 1);
    }
}

/// Token sink that stamps each `<a>` start tag with a fresh origin id before forwarding it,
/// records whether a `<body>` start tag went by, and stops forwarding when the tree builder
/// holds more than [`MAX_OPEN_ELEMENTS`] elements open.
struct AnchorOriginStamper {
    inner: TreeBuilder<Handle, RcDom>,
    next_origin: Cell<u32>,
    saw_body_tag: Cell<bool>,
    start_tags: Cell<usize>,
    too_deep: Cell<bool>,
}

impl AnchorOriginStamper {
    fn measure_depth(&self) {
        let start_tags = self.start_tags.get() + 1;
        self.start_tags.set(start_tags);
        if !is_measured(start_tags) {
            return;
        }
        let held = HandleCount(Cell::new(0));
        self.inner.trace_handles(&held);
        self.too_deep.set(held.0.get() > MAX_OPEN_ELEMENTS);
    }

    fn stamp(&self, tag: &mut Tag) {
        if tag.kind != TagKind::StartTag {
            return;
        }
        if &*tag.name == "body" {
            self.saw_body_tag.set(true);
        }
        if &*tag.name != "a" {
            return;
        }
        // ~keep Input may already carry the private attribute; it must never survive as an
        // ~keep origin claim, so it is dropped before the genuine stamp is added.
        tag.attrs.retain(|attribute| &*attribute.name.local != ORIGIN_ATTR);
        let origin = self.next_origin.get();
        self.next_origin.set(origin.wrapping_add(1));
        tag.attrs.push(Attribute {
            name: QualName::new(None, ns!(), LocalName::from(ORIGIN_ATTR)),
            value: StrTendril::from(origin.to_string()),
        });
    }
}

impl TokenSink for AnchorOriginStamper {
    type Handle = Handle;

    fn process_token(&self, mut token: Token, line_number: u64) -> TokenSinkResult<Self::Handle> {
        if self.too_deep.get() {
            return TokenSinkResult::Continue;
        }
        let is_start_tag = matches!(&token, Token::TagToken(tag) if tag.kind == TagKind::StartTag);
        if let Token::TagToken(ref mut tag) = token {
            self.stamp(tag);
        }
        let result = self.inner.process_token(token, line_number);
        if is_start_tag {
            self.measure_depth();
        }
        result
    }

    fn end(&self) {
        self.inner.end();
    }

    fn adjusted_current_node_present_but_not_in_html_namespace(&self) -> bool {
        self.inner.adjusted_current_node_present_but_not_in_html_namespace()
    }
}

/// Parse `html` as a document with every `<a>` stamped with its origin id.
///
/// Drives the tokenizer by hand: `html5ever::parse_document` hard-codes
/// `Tokenizer<TreeBuilder<..>>` and leaves no room for a sink between the two. The input is
/// already a `str`, so nothing the `TendrilSink` driver adds (UTF-8 decoding) is lost.
///
/// ~keep The tree builder gives every document a `<body>`. Input with no `<body>` start tag is a
/// ~keep fragment, and rules that ask "is this element in the body of a page" (the page header
/// ~keep rule) must still see a fragment after the repair, so the body that no tag asked for is
/// ~keep unwrapped and its children become children of `<html>`.
///
/// Returns `None` when the tree builder holds more than [`MAX_OPEN_ELEMENTS`] elements open.
/// The rest of the input is then tokenized and not built, so the call stays linear in the size
/// of the input.
pub fn parse_with_anchor_origins(html: &str) -> Option<RcDom> {
    let tree_builder = TreeBuilder::new(RcDom::default(), TreeBuilderOpts::default());
    let tokenizer = Tokenizer::new(
        AnchorOriginStamper {
            inner: tree_builder,
            next_origin: Cell::new(0),
            saw_body_tag: Cell::new(false),
            start_tags: Cell::new(0),
            too_deep: Cell::new(false),
        },
        TokenizerOpts::default(),
    );
    let input = BufferQueue::default();
    input.push_back(StrTendril::from(html));
    while !matches!(tokenizer.feed(&input), TokenizerResult::Done) {}
    tokenizer.end();
    if tokenizer.sink.too_deep.get() {
        return None;
    }
    let saw_body_tag = tokenizer.sink.saw_body_tag.get();
    let dom = TreeSink::finish(tokenizer.sink.inner.sink);
    if !saw_body_tag {
        unwrap_implied_body(&dom.document);
    }
    Some(dom)
}

/// Replace the `<body>` of `document` with its children.
fn unwrap_implied_body(document: &Handle) {
    if let Some(body) = child_element(document, "html").and_then(|html| child_element(&html, "body")) {
        unwrap_element(&body);
    }
}

/// The first child of `parent` that is an element named `name`.
fn child_element(parent: &Handle, name: &str) -> Option<Handle> {
    parent
        .children
        .borrow()
        .iter()
        .find(|child| matches!(&child.data, NodeData::Element { name: qualified, .. } if &*qualified.local == name))
        .cloned()
}

/// Undo the adoption agency's split of an anchor around a block, then strip every origin stamp.
///
/// An origin that the repair left as more than one `<a>` keeps only the members with content
/// of their own; the rest are unwrapped in place so their children (whitespace, an empty
/// `<span>`) stay where they were. When no member has content the first -- the authored one --
/// is kept, so `<a href="/o"><div>…</div></a>` still records its destination once as `[](/o)`,
/// exactly as an authored `<a href></a>` renders. An origin with a single member is untouched.
pub fn collapse_split_anchors(document: &Handle) {
    let anchors = collect_stamped_anchors(document);
    let mut groups: BTreeMap<u32, Vec<Handle>> = BTreeMap::new();
    for (origin, handle) in &anchors {
        groups.entry(*origin).or_default().push(Rc::clone(handle));
    }
    for members in groups.values().filter(|members| members.len() > 1) {
        collapse_group(members);
    }
    for (_, handle) in &anchors {
        strip_origin_attr(handle);
    }
}

/// Every stamped `<a>` under `document`, in document order.
fn collect_stamped_anchors(document: &Handle) -> Vec<(u32, Handle)> {
    let mut anchors = Vec::new();
    let mut stack = vec![Rc::clone(document)];
    while let Some(node) = stack.pop() {
        if let Some(origin) = origin_of(&node) {
            anchors.push((origin, Rc::clone(&node)));
        }
        let children = node.children.borrow();
        stack.extend(children.iter().rev().cloned());
    }
    anchors
}

/// The origin id stamped on `node`, if it is a stamped `<a>`.
fn origin_of(node: &Handle) -> Option<u32> {
    let NodeData::Element { name, attrs, .. } = &node.data else {
        return None;
    };
    if &*name.local != "a" {
        return None;
    }
    attrs
        .borrow()
        .iter()
        .find(|attribute| &*attribute.name.local == ORIGIN_ATTR)
        .and_then(|attribute| attribute.value.parse().ok())
}

/// Unwrap every member of a split origin that has no content of its own, keeping the first
/// member when none has.
fn collapse_group(members: &[Handle]) {
    let empty: Vec<bool> = members.iter().map(|member| !has_own_content(member)).collect();
    let keep_first = empty.iter().all(|is_empty| *is_empty);
    for (index, member) in members.iter().enumerate() {
        if empty[index] && !(keep_first && index == 0) {
            unwrap_element(member);
        }
    }
}

/// Whether `anchor` holds non-whitespace text or a content-bearing element of its own.
///
/// Descendant `<a>` subtrees are not searched: whatever a nested anchor holds is its own
/// content, and must not keep an otherwise empty outer half alive. ~keep
fn has_own_content(anchor: &Handle) -> bool {
    let mut stack: Vec<Handle> = anchor.children.borrow().iter().cloned().collect();
    while let Some(node) = stack.pop() {
        match &node.data {
            NodeData::Text { contents } => {
                if !contents.borrow().trim().is_empty() {
                    return true;
                }
            }
            NodeData::Element { name, .. } => {
                let local: &str = &name.local;
                if local == "a" {
                    continue;
                }
                if CONTENT_BEARING.contains(&local) {
                    return true;
                }
                stack.extend(node.children.borrow().iter().cloned());
            }
            _ => {}
        }
    }
    false
}

/// Replace `node` in its parent with its own children, in place.
fn unwrap_element(node: &Handle) {
    let Some(parent) = node.parent.take().and_then(|weak| weak.upgrade()) else {
        return;
    };
    let children = std::mem::take(&mut *node.children.borrow_mut());
    for child in &children {
        child.parent.set(Some(Rc::downgrade(&parent)));
    }
    let mut siblings = parent.children.borrow_mut();
    let Some(index) = siblings.iter().position(|sibling| Rc::ptr_eq(sibling, node)) else {
        return;
    };
    siblings.remove(index);
    for (offset, child) in children.into_iter().enumerate() {
        siblings.insert(index + offset, child);
    }
}

/// Remove the origin stamp from a stamped `<a>`.
fn strip_origin_attr(node: &Handle) {
    if let NodeData::Element { attrs, .. } = &node.data {
        attrs
            .borrow_mut()
            .retain(|attribute| &*attribute.name.local != ORIGIN_ATTR);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rcdom::SerializableHandle;
    use html5ever::serialize::{SerializeOpts, serialize};
    use std::fmt::Write as _;

    fn repaired(html: &str) -> String {
        let dom = parse_with_anchor_origins(html).expect("a tree");
        collapse_split_anchors(&dom.document);
        let mut buf = Vec::new();
        serialize(
            &mut buf,
            &SerializableHandle::from(dom.document),
            SerializeOpts::default(),
        )
        .expect("serialize");
        String::from_utf8(buf).expect("utf8")
    }

    fn body(html: &str) -> String {
        let out = repaired(html);
        out.trim_start_matches("<html><head></head>")
            .trim_end_matches("</html>")
            .to_string()
    }

    #[test]
    fn should_give_no_body_element_to_input_with_no_body_start_tag() {
        assert_eq!(
            repaired("<header>h</header><div><p>one</div>tail"),
            "<html><head></head><header>h</header><div><p>one</p></div>tail</html>"
        );
        assert_eq!(
            repaired("<title>T</title><p>one<!-- <body> --><p>two"),
            "<html><head><title>T</title></head><p>one<!-- <body> --></p><p>two</p></html>"
        );
    }

    #[test]
    fn should_give_no_body_element_to_input_with_an_html_start_tag_and_no_body_start_tag() {
        assert_eq!(
            repaired("<html><header>h</header><div><p>one</div>tail"),
            "<html><head></head><header>h</header><div><p>one</p></div>tail</html>"
        );
    }

    #[test]
    fn should_give_no_tree_when_the_tree_builder_nests_past_the_limit() {
        let nested = |depth: usize| format!("{}x", "<div>".repeat(depth));
        assert!(parse_with_anchor_origins(&nested(400)).is_some());
        assert!(parse_with_anchor_origins(&nested(600)).is_none());
        // ~keep A table moves the blocks in front of itself, so they are not its descendants,
        // ~keep and the tree builder still holds every one of them open.
        assert!(parse_with_anchor_origins(&format!("<table>{}", nested(600))).is_none());
        let closed = format!("{}x{}", "<div>".repeat(400), "</div>".repeat(400));
        assert!(parse_with_anchor_origins(&closed.repeat(4)).is_some());
    }

    #[test]
    fn should_give_a_tree_at_the_limit_and_none_one_element_past_it() {
        // ~keep The tree builder also holds the document, `html`, `body` and the `head` it
        // ~keep remembers. A `<br>` is a start tag that leaves nothing open, so start tag 512 is
        // ~keep measured with 512 handles in the first page and 513 in the second.
        let page = |breaks: usize, blocks: usize| format!("{}{}x", "<br>".repeat(breaks), "<div>".repeat(blocks));
        assert!(parse_with_anchor_origins(&page(4, 508)).is_some());
        assert!(parse_with_anchor_origins(&page(3, 509)).is_none());
    }

    #[test]
    fn should_give_no_tree_when_elements_that_no_start_tag_names_nest_past_the_limit() {
        // ~keep One `<td>` start tag opens a `tbody`, a `tr` and the cell.
        let cells = format!("{}x", "<table><td>".repeat(300));
        assert!(parse_with_anchor_origins(&cells).is_none());
        let foreign = format!("<svg>{}<text>x", "<g>".repeat(600));
        assert!(parse_with_anchor_origins(&foreign).is_none());
        let math = format!("<math>{}<mi>x", "<mrow>".repeat(600));
        assert!(parse_with_anchor_origins(&math).is_none());
        let templates = format!("{}x", "<template>".repeat(600));
        assert!(parse_with_anchor_origins(&templates).is_none());
        // ~keep Each `<b>` with its own attribute is held twice: open, and as a formatting
        // ~keep element that the tree builder opens again in the next paragraph.
        let mut formatting = String::from("<p>");
        for number in 0..300 {
            write!(formatting, "<b id='b{number}'>").expect("write to a string");
        }
        formatting.push('x');
        assert!(parse_with_anchor_origins(&formatting).is_none());
    }

    #[test]
    fn should_measure_the_tree_builder_once_in_sixty_four_start_tags() {
        assert!(!is_measured(1));
        assert!(!is_measured(63));
        assert!(is_measured(64));
        assert!(!is_measured(65));
        assert!(is_measured(128));
    }

    #[test]
    fn should_keep_the_body_element_when_the_input_has_a_body_start_tag() {
        assert_eq!(
            repaired("<body><header>h</header><p>one"),
            "<html><head></head><body><header>h</header><p>one</p></body></html>"
        );
        assert_eq!(
            repaired("<p>one<BODY class=late><p>two"),
            "<html><head></head><body class=\"late\"><p>one</p><p>two</p></body></html>"
        );
    }

    #[test]
    fn should_unwrap_the_reconstructed_empty_clone_and_keep_the_authored_anchor() {
        assert_eq!(
            body(r#"<a href="/o"><div><a href="/i">Inner</a></div></a>"#),
            r#"<a href="/o"></a><div><a href="/i">Inner</a></div>"#
        );
    }

    #[test]
    fn should_keep_the_clone_that_carries_text_and_drop_the_empty_authored_half() {
        assert_eq!(
            body(r#"<a href="/o"><div>Text<a href="/i">Inner</a></div></a>"#),
            r#"<div><a href="/o">Text</a><a href="/i">Inner</a></div>"#
        );
    }

    #[test]
    fn should_unwrap_a_clone_holding_only_an_empty_span() {
        // ~keep The adoption agency's second pass pops the `<span>` too, so the clone keeps an
        // ~keep empty `<span>` and the inner anchor lands after it; the clone has no content
        // ~keep of its own and is unwrapped, the `<span>` staying where it was.
        assert_eq!(
            body(r#"<a href="/o"><div><span><a href="/i">Inner</a></span></div></a>"#),
            r#"<a href="/o"></a><div><span></span><a href="/i">Inner</a></div>"#
        );
    }

    #[test]
    fn should_leave_authored_duplicate_anchors_alone() {
        let html = r#"<a href="/o"></a><div><a href="/o"></a>real</div>"#;
        assert_eq!(body(html), html);
    }

    #[test]
    fn should_never_leak_the_origin_attribute_even_when_the_input_carries_it() {
        let out = repaired(r#"<a href="/o" data-h2m-anchor-origin="7"><div><a href="/i">Inner</a></div></a>"#);
        assert!(!out.contains(ORIGIN_ATTR), "leaked: {out}");
    }

    #[test]
    fn should_count_an_image_as_content() {
        assert_eq!(
            body(r#"<a href="/o"><div><img src="s"><a href="/i">Inner</a></div></a>"#),
            r#"<div><a href="/o"><img src="s"></a><a href="/i">Inner</a></div>"#
        );
    }
}
