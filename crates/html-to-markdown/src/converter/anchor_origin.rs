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
//! The same token sink records whether the input wrote an `<html>` and a `<body>` start tag,
//! which the tree also no longer shows, and can give the repair up when the tree builder nests
//! too deep: see [`parse_with_anchor_origins_within_depth`]. ~keep

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

/// The private attribute that marks a `<tbody>` or a `<colgroup>` that a start tag asked for.
const AUTHORED_ATTR: &str = "data-h2m-authored";

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
/// marks each `<tbody>` and `<colgroup>` start tag, records whether an `<html>` start tag, a
/// `<table>` start tag and a `<body>` start tag that opens the body went by, and stops
/// forwarding when the tree builder holds more than `max_open_elements` elements open.
struct AnchorOriginStamper {
    inner: TreeBuilder<Handle, RcDom>,
    next_origin: Cell<u32>,
    saw_html_tag: Cell<bool>,
    saw_body_tag: Cell<bool>,
    saw_table_tag: Cell<bool>,
    max_open_elements: Option<usize>,
    start_tags: Cell<usize>,
    too_deep: Cell<bool>,
}

impl AnchorOriginStamper {
    fn measure_depth(&self) {
        let Some(max_open_elements) = self.max_open_elements else {
            return;
        };
        let start_tags = self.start_tags.get() + 1;
        self.start_tags.set(start_tags);
        if !is_measured(start_tags) {
            return;
        }
        let held = HandleCount(Cell::new(0));
        self.inner.trace_handles(&held);
        self.too_deep.set(held.0.get() > max_open_elements);
    }

    fn stamp(&self, tag: &mut Tag) {
        if tag.kind != TagKind::StartTag {
            return;
        }
        match &*tag.name {
            "html" => self.saw_html_tag.set(true),
            // ~keep A `<body>` start tag after the body is open opens no element: the tree
            // ~keep builder copies its attributes to the body it implied. That body is still
            // ~keep one that no start tag asked for.
            "body" if !self.has_body() => self.saw_body_tag.set(true),
            "table" => self.saw_table_tag.set(true),
            "tbody" | "colgroup" => set_private_attr(tag, AUTHORED_ATTR, StrTendril::new()),
            "a" => self.stamp_anchor(tag),
            _ => {}
        }
    }

    /// True when the tree already holds a `<body>`.
    fn has_body(&self) -> bool {
        child_element(&self.inner.sink.document, "html").is_some_and(|html| child_element(&html, "body").is_some())
    }

    fn stamp_anchor(&self, tag: &mut Tag) {
        let origin = self.next_origin.get();
        self.next_origin.set(origin.wrapping_add(1));
        set_private_attr(tag, ORIGIN_ATTR, StrTendril::from(origin.to_string()));
    }
}

/// Give `tag` the private attribute `name` with `value`.
///
/// ~keep Input may already carry the private attribute; it must never survive as a claim of
/// ~keep the input, so it is dropped before the genuine one is added.
fn set_private_attr(tag: &mut Tag, name: &str, value: StrTendril) {
    tag.attrs.retain(|attribute| &*attribute.name.local != name);
    tag.attrs.push(Attribute {
        name: QualName::new(None, ns!(), LocalName::from(name)),
        value,
    });
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
/// ~keep The tree builder gives every document an `<html>` and a `<body>`, a table with a bare
/// ~keep row a `<tbody>`, and a table with a bare `<col>` a `<colgroup>`. Each one that no
/// ~keep start tag asked for is unwrapped, so the tree is no deeper than the input wrote it:
/// ~keep the depth limit of the converter counts every level, and a page must not reach it
/// ~keep sooner because it was repaired. The `<tr>` that the tree builder gives a cell with no
/// ~keep row stays: that page is a misnest, and the row is its repair.
/// ~keep Input with no `<body>` start tag is also a fragment,
/// ~keep and rules that ask "is this element in the body of a page" (the page header rule) must
/// ~keep still see a fragment after the repair.
pub fn parse_with_anchor_origins(html: &str) -> RcDom {
    build_tree(html, None).0
}

/// The tree of [`parse_with_anchor_origins`], or `None` when the tree builder holds more than
/// [`MAX_OPEN_ELEMENTS`] elements open.
///
/// The rest of the input is then tokenized and not built, so the call stays linear in the size
/// of the input.
pub fn parse_with_anchor_origins_within_depth(html: &str) -> Option<RcDom> {
    let (dom, too_deep) = build_tree(html, Some(MAX_OPEN_ELEMENTS));
    (!too_deep).then_some(dom)
}

/// Build the tree and say whether the tree builder went past `max_open_elements`. A tree that
/// went past it holds the input only up to that point.
fn build_tree(html: &str, max_open_elements: Option<usize>) -> (RcDom, bool) {
    let tree_builder = TreeBuilder::new(RcDom::default(), TreeBuilderOpts::default());
    let tokenizer = Tokenizer::new(
        AnchorOriginStamper {
            inner: tree_builder,
            next_origin: Cell::new(0),
            saw_html_tag: Cell::new(false),
            saw_body_tag: Cell::new(false),
            saw_table_tag: Cell::new(false),
            max_open_elements,
            start_tags: Cell::new(0),
            too_deep: Cell::new(false),
        },
        TokenizerOpts::default(),
    );
    let input = BufferQueue::default();
    input.push_back(StrTendril::from(html));
    while !matches!(tokenizer.feed(&input), TokenizerResult::Done) {}
    tokenizer.end();
    let too_deep = tokenizer.sink.too_deep.get();
    let saw_html_tag = tokenizer.sink.saw_html_tag.get();
    let saw_body_tag = tokenizer.sink.saw_body_tag.get();
    let saw_table_tag = tokenizer.sink.saw_table_tag.get();
    let dom = TreeSink::finish(tokenizer.sink.inner.sink);
    unwrap_implied_wrappers(&dom.document, saw_html_tag, saw_body_tag);
    // ~keep Only a table gets a `<tbody>` or a `<colgroup>`, so a page with no table skips the walk.
    if saw_table_tag {
        unwrap_implied_table_parts(&dom.document);
    }
    (dom, too_deep)
}

/// Replace each `<tbody>` and `<colgroup>` under `document` that no start tag asked for with its
/// children, and strip the mark from the others.
fn unwrap_implied_table_parts(document: &Handle) {
    let mut stack = vec![Rc::clone(document)];
    while let Some(node) = stack.pop() {
        stack.extend(node.children.borrow().iter().cloned());
        let NodeData::Element { name, attrs, .. } = &node.data else {
            continue;
        };
        if !matches!(&*name.local, "tbody" | "colgroup") {
            continue;
        }
        let mut attrs = attrs.borrow_mut();
        let before = attrs.len();
        attrs.retain(|attribute| &*attribute.name.local != AUTHORED_ATTR);
        if attrs.len() == before {
            drop(attrs);
            unwrap_element(&node);
        }
    }
}

/// Replace the `<body>` and then the `<html>` of `document` with its children, each one only
/// when the input wrote no start tag for it.
fn unwrap_implied_wrappers(document: &Handle, saw_html_tag: bool, saw_body_tag: bool) {
    let Some(html) = child_element(document, "html") else {
        return;
    };
    if !saw_body_tag && let Some(body) = child_element(&html, "body") {
        unwrap_element(&body);
    }
    if !saw_html_tag {
        unwrap_element(&html);
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
        written(parse_with_anchor_origins(html))
    }

    fn written(dom: RcDom) -> String {
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
        out.trim_start_matches("<head></head>").to_string()
    }

    #[test]
    fn should_give_no_body_element_to_input_with_no_body_start_tag() {
        assert_eq!(
            repaired("<header>h</header><div><p>one</div>tail"),
            "<head></head><header>h</header><div><p>one</p></div>tail"
        );
        assert_eq!(
            repaired("<title>T</title><p>one<!-- <body> --><p>two"),
            "<head><title>T</title></head><p>one<!-- <body> --></p><p>two</p>"
        );
    }

    #[test]
    fn should_give_no_body_element_to_input_with_a_body_start_tag_after_the_body_is_open() {
        // ~keep The late start tag opens no element; the tree builder copies its attributes.
        assert_eq!(
            repaired("<p>one<BODY class=late><p>two"),
            "<head></head><p>one</p><p>two</p>"
        );
        assert_eq!(
            repaired("<header>h</header><p>one</p><body class=b><div><p>two</div>tail"),
            "<head></head><header>h</header><p>one</p><div><p>two</p></div>tail"
        );
    }

    #[test]
    fn should_give_a_table_no_tbody_and_no_colgroup_that_the_input_did_not_write() {
        assert_eq!(
            body("<table><tr><td>one</td></tr></table><p>tail"),
            "<table><tr><td>one</td></tr></table><p>tail</p>"
        );
        assert_eq!(
            body("<table><col><tr><td>one</table>"),
            "<table><col><tr><td>one</td></tr></table>"
        );
        assert_eq!(
            body("<table><tr><td><table><tr><td>one"),
            "<table><tr><td><table><tr><td>one</td></tr></table></td></tr></table>"
        );
    }

    #[test]
    fn should_keep_the_row_that_the_tree_builder_gives_a_cell_with_no_row() {
        assert_eq!(body("<table><td>one"), "<table><tr><td>one</td></tr></table>");
    }

    #[test]
    fn should_keep_the_tbody_and_the_colgroup_that_the_input_wrote() {
        assert_eq!(
            body("<table><COLGROUP><col></colgroup><TBODY class=b><tr><td>one</table>"),
            "<table><colgroup><col></colgroup><tbody class=\"b\"><tr><td>one</td></tr></tbody></table>"
        );
        // ~keep The row after the written `tbody` gets a second one, which no start tag asked for.
        assert_eq!(
            body("<table><tbody><tr><td>one</td></tr></tbody><tr><td>two</td></tr></table>"),
            "<table><tbody><tr><td>one</td></tr></tbody><tr><td>two</td></tr></table>"
        );
    }

    #[test]
    fn should_never_leak_the_mark_of_a_written_tbody_even_when_the_input_carries_it() {
        assert_eq!(
            body("<table><tbody data-h2m-authored=x><tr><td>one</table>"),
            "<table><tbody><tr><td>one</td></tr></tbody></table>"
        );
    }

    #[test]
    fn should_measure_after_start_tags_only() {
        // ~keep Start tag 512 is measured with 512 handles. Ten more blocks open, 54 end tags
        // ~keep that end no element go by, the ten blocks end, and start tag 576 is measured
        // ~keep with 512 handles again. A count of every tag measures at the last stray end
        // ~keep tag, with 522 handles.
        let page = format!(
            "{}{}{}{}{}x",
            "<br>".repeat(4),
            "<div>".repeat(518),
            "</span>".repeat(54),
            "</div>".repeat(10),
            "<br>".repeat(54)
        );
        assert!(parse_with_anchor_origins_within_depth(&page).is_some());
    }

    #[test]
    fn should_build_nothing_more_once_the_tree_builder_went_past_the_limit() {
        // ~keep Start tags 512 and 576 are measured with 512 handles, start tag 640 with 576.
        // ~keep The page then ends its blocks, and start tags 704 and 768 go by with 4 open.
        let page = format!(
            "{}{}{}{}after{}{}<p>late",
            "<br>".repeat(4),
            "<div>".repeat(508),
            "<br>".repeat(64),
            "<div>".repeat(64),
            "</div>".repeat(572),
            "<br>".repeat(128)
        );
        let (dom, too_deep) = build_tree(&page, Some(MAX_OPEN_ELEMENTS));
        assert!(too_deep);
        let built = written(dom);
        assert_eq!(built.matches("<div>").count(), 572);
        assert!(!built.contains("after"), "{}", &built[built.len() - 40..]);
        assert!(!built.contains("late"));
        assert!(parse_with_anchor_origins_within_depth(&page).is_none());
    }

    #[test]
    fn should_give_no_body_element_to_input_with_an_html_start_tag_and_no_body_start_tag() {
        assert_eq!(
            repaired("<html><header>h</header><div><p>one</div>tail"),
            "<html><head></head><header>h</header><div><p>one</p></div>tail</html>"
        );
    }

    #[test]
    fn should_give_no_html_element_to_input_with_no_html_start_tag() {
        assert_eq!(
            repaired("<!doctype html><div><p>one</div>tail"),
            "<!DOCTYPE html><head></head><div><p>one</p></div>tail"
        );
        assert_eq!(repaired("<body><p>one"), "<head></head><body><p>one</p></body>");
    }

    #[test]
    fn should_keep_the_html_element_when_the_input_has_an_html_start_tag() {
        assert_eq!(
            repaired("<p>one<HTML lang=en><p>two"),
            "<html lang=\"en\"><head></head><p>one</p><p>two</p></html>"
        );
    }

    #[test]
    fn should_give_no_tree_when_the_tree_builder_nests_past_the_limit() {
        let nested = |depth: usize| format!("{}x", "<div>".repeat(depth));
        assert!(parse_with_anchor_origins_within_depth(&nested(400)).is_some());
        assert!(parse_with_anchor_origins_within_depth(&nested(600)).is_none());
        // ~keep A table moves the blocks in front of itself, so they are not its descendants,
        // ~keep and the tree builder still holds every one of them open.
        assert!(parse_with_anchor_origins_within_depth(&format!("<table>{}", nested(600))).is_none());
        let closed = format!("{}x{}", "<div>".repeat(400), "</div>".repeat(400));
        assert!(parse_with_anchor_origins_within_depth(&closed.repeat(4)).is_some());
    }

    #[test]
    fn should_give_a_tree_at_the_limit_and_none_one_element_past_it() {
        // ~keep The tree builder also holds the document, `html`, `body` and the `head` it
        // ~keep remembers. A `<br>` is a start tag that leaves nothing open, so start tag 512 is
        // ~keep measured with 512 handles in the first page and 513 in the second.
        let page = |breaks: usize, blocks: usize| format!("{}{}x", "<br>".repeat(breaks), "<div>".repeat(blocks));
        assert!(parse_with_anchor_origins_within_depth(&page(4, 508)).is_some());
        assert!(parse_with_anchor_origins_within_depth(&page(3, 509)).is_none());
    }

    #[test]
    fn should_give_a_tree_when_the_tree_builder_is_past_the_limit_only_between_two_measurements() {
        // ~keep Start tag 512 is measured with 512 handles. Ten more blocks open and end before
        // ~keep start tag 576, which is measured with 512 handles again.
        let page = format!(
            "{}{}{}{}x",
            "<br>".repeat(4),
            "<div>".repeat(518),
            "</div>".repeat(10),
            "<br>".repeat(54)
        );
        assert!(parse_with_anchor_origins_within_depth(&page).is_some());
    }

    #[test]
    fn should_give_no_tree_when_elements_that_no_start_tag_names_nest_past_the_limit() {
        // ~keep One `<td>` start tag opens a `tbody`, a `tr` and the cell.
        let cells = format!("{}x", "<table><td>".repeat(300));
        assert!(parse_with_anchor_origins_within_depth(&cells).is_none());
        let foreign = format!("<svg>{}<text>x", "<g>".repeat(600));
        assert!(parse_with_anchor_origins_within_depth(&foreign).is_none());
        let math = format!("<math>{}<mi>x", "<mrow>".repeat(600));
        assert!(parse_with_anchor_origins_within_depth(&math).is_none());
        let templates = format!("{}x", "<template>".repeat(600));
        assert!(parse_with_anchor_origins_within_depth(&templates).is_none());
        // ~keep Each `<b>` with its own attribute is held twice: open, and as a formatting
        // ~keep element that the tree builder opens again in the next paragraph.
        let mut formatting = String::from("<p>");
        for number in 0..300 {
            write!(formatting, "<b id='b{number}'>").expect("write to a string");
        }
        formatting.push('x');
        assert!(parse_with_anchor_origins_within_depth(&formatting).is_none());
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
            "<head></head><body><header>h</header><p>one</p></body>"
        );
        assert_eq!(
            repaired("<title>T</title><BODY class=b><p>one<body id=late><p>two"),
            "<head><title>T</title></head><body class=\"b\" id=\"late\"><p>one</p><p>two</p></body>"
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
