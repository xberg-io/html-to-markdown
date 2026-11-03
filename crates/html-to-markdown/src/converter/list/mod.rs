//! List element handlers for HTML to Markdown conversion.
//!
//! This module provides specialized handling for various list types:
//! - **Ordered lists**: `<ol>` with counter management and formatting options
//! - **Unordered lists**: `<ul>` with bullet cycling based on nesting depth
//! - **List items**: `<li>` with task list and block-level detection
//! - **Definition lists**: `<dl>`, `<dt>`, `<dd>` elements
//! - **List utilities**: Indentation, loose/tight list detection, nesting depth calculation

pub mod definition;
pub mod item;
pub mod ordered;
pub mod unordered;
pub mod utils;

#[derive(Clone, Copy)]
pub struct ListContext<'a> {
    pub options: &'a crate::options::ConversionOptions,
    pub ctx: &'a super::Context,
    pub depth: usize,
    pub dom_ctx: &'a super::DomContext,
}

fn is_list_item(node_handle: crate::dom::NodeHandle, parser: &crate::dom::Parser, dom_ctx: &super::DomContext) -> bool {
    if let Some(info) = dom_ctx.tag_info(node_handle.get_inner(), parser) {
        return info.name == "li";
    }
    matches!(
        node_handle.get(parser),
        Some(crate::dom::Node::Tag(tag)) if crate::converter::main_helpers::tag_name_eq(tag.name().as_utf8_str(), "li")
    )
}

fn has_list_item_child(
    node_handle: crate::dom::NodeHandle,
    parser: &crate::dom::Parser,
    dom_ctx: &super::DomContext,
) -> bool {
    let Some(crate::dom::Node::Tag(tag)) = node_handle.get(parser) else {
        return false;
    };
    tag.children()
        .top()
        .iter()
        .any(|child| is_list_item(*child, parser, dom_ctx))
}

pub(super) fn render_itemless_list_as_div(
    node_handle: &crate::dom::NodeHandle,
    parser: &crate::dom::Parser,
    output: &mut String,
    context: ListContext<'_>,
) -> bool {
    if has_list_item_child(*node_handle, parser, context.dom_ctx) {
        return false;
    }
    crate::converter::block::div::handle(
        node_handle,
        parser,
        output,
        crate::converter::block::container::HandlerContext::new(
            context.options,
            context.ctx,
            context.depth,
            context.dom_ctx,
        ),
    );
    true
}

#[cfg(feature = "visitor")]
pub(super) enum ListStartResult {
    Continue(Option<String>),
    Stop,
}

#[cfg(feature = "visitor")]
pub(super) fn visit_list_start(
    node_handle: &crate::dom::NodeHandle,
    tag: &crate::dom::HTMLTag,
    parser: &crate::dom::Parser,
    output: &mut String,
    context: ListContext<'_>,
    ordered: bool,
) -> ListStartResult {
    use crate::visitor::{NodeContext, NodeType, VisitResult};
    use std::borrow::Cow;

    let Some(visitor_handle) = context.ctx.visitor.as_ref() else {
        return ListStartResult::Continue(None);
    };
    let node_id = node_handle.get_inner();
    let parent_tag = context.dom_ctx.parent_tag_name(node_id, parser);
    let index = context.dom_ctx.get_sibling_index(node_id).unwrap_or(0);
    let tag_name = if ordered { "ol" } else { "ul" };
    let node_ctx = NodeContext::with_lazy_attributes(
        NodeType::List,
        Cow::Borrowed(tag_name),
        tag,
        context.depth,
        index,
        parent_tag.map(Cow::Borrowed),
        false,
    );
    let visit_result = {
        let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
        visitor.visit_list_start(&node_ctx, ordered)
    };
    match visit_result {
        VisitResult::Continue => ListStartResult::Continue(None),
        VisitResult::Custom(custom) => ListStartResult::Continue(Some(custom)),
        VisitResult::Skip => ListStartResult::Stop,
        VisitResult::PreserveHtml => {
            crate::converter::serialize_node_to_html(node_handle, parser, output);
            ListStartResult::Stop
        }
        VisitResult::Error(err) => {
            if context.ctx.visitor_error.borrow().is_none() {
                *context.ctx.visitor_error.borrow_mut() = Some(err);
            }
            ListStartResult::Stop
        }
    }
}

#[cfg(feature = "visitor")]
pub(super) struct ListVisitorOutput {
    pub start: usize,
    pub custom_start: Option<String>,
    pub ordered: bool,
}

#[cfg(feature = "visitor")]
pub(super) fn visit_list_end(
    node_handle: &crate::dom::NodeHandle,
    tag: &crate::dom::HTMLTag,
    parser: &crate::dom::Parser,
    output: &mut String,
    context: ListContext<'_>,
    visitor_output: ListVisitorOutput,
) {
    use crate::visitor::{NodeContext, NodeType, VisitResult};
    use std::borrow::Cow;

    let Some(ref visitor_handle) = context.ctx.visitor else {
        return;
    };
    let node_id = node_handle.get_inner();
    let parent_tag = context.dom_ctx.parent_tag_name(node_id, parser);
    let index = context.dom_ctx.get_sibling_index(node_id).unwrap_or(0);
    let tag_name = if visitor_output.ordered { "ol" } else { "ul" };
    let node_ctx = NodeContext::with_lazy_attributes(
        NodeType::List,
        Cow::Borrowed(tag_name),
        tag,
        context.depth,
        index,
        parent_tag.map(Cow::Borrowed),
        false,
    );
    let output_start =
        crate::converter::utility::content::floor_char_boundary(output, visitor_output.start.min(output.len()));
    let visit_result = {
        let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
        visitor.visit_list_end(&node_ctx, visitor_output.ordered, &output[output_start..])
    };
    match visit_result {
        VisitResult::Continue => {
            if let Some(custom_start) = visitor_output.custom_start {
                output.insert_str(output_start, &custom_start);
            }
        }
        VisitResult::Custom(custom) => {
            let children_output = output[output_start..].to_string();
            output.truncate(output_start);
            if let Some(custom_start) = visitor_output.custom_start {
                output.push_str(&custom_start);
            }
            output.push_str(&children_output);
            output.push_str(&custom);
        }
        VisitResult::Skip => output.truncate(output_start),
        VisitResult::PreserveHtml => {
            output.truncate(output_start);
            crate::converter::serialize_node_to_html(node_handle, parser, output);
        }
        VisitResult::Error(err) => {
            if context.ctx.visitor_error.borrow().is_none() {
                *context.ctx.visitor_error.borrow_mut() = Some(err);
            }
            output.truncate(output_start);
        }
    }
}

/// Dispatches list element handling to the appropriate handler.
///
/// Returns `true` if the element was handled, `false` otherwise.
///
/// # Supported Elements
///
/// - `ol`: Ordered list - routed to `ordered::handle`
/// - `ul`: Unordered list - routed to `unordered::handle`
/// - `li`: List item - routed to `item::handle_li`
/// - `dl`: Definition list - routed to `definition::handle_dl`
/// - `dt`: Definition term - routed to `definition::handle_dt`
/// - `dd`: Definition description - routed to `definition::handle_dd`
pub fn dispatch_list_handler(
    tag_name: &str,
    node_handle: &crate::dom::NodeHandle,
    tag: &crate::dom::HTMLTag,
    parser: &crate::dom::Parser,
    output: &mut String,
    context: ListContext<'_>,
) -> bool {
    match tag_name {
        "ol" => {
            ordered::handle(node_handle, parser, output, context);
            true
        }
        "ul" => {
            unordered::handle(node_handle, parser, output, context);
            true
        }
        "li" => {
            item::handle_li(node_handle, tag, parser, output, context);
            true
        }
        "dl" => {
            definition::handle_dl(node_handle, parser, output, context);
            true
        }
        "dt" => {
            definition::handle_dt(node_handle, parser, output, context);
            true
        }
        "dd" => {
            definition::handle_dd(node_handle, parser, output, context);
            true
        }
        _ => false,
    }
}

/// The end of the last ordered list and the buffers it can still end.
///
/// ~keep Nodes that pass one buffer down form a run. The key names the run holding the list's
/// ~keep end and its offset, then waits for the parent when that run ends. A later child can take
/// ~keep the key back only when the same buffer still holds the same bytes at that offset.
#[derive(Clone, Default)]
pub struct LastList(std::rc::Rc<std::cell::RefCell<ListTracker>>);

#[derive(Default)]
struct ListTracker {
    frames: Vec<Frame>,
    key: Option<ListEnd>,
}

#[derive(Clone, Copy)]
struct Frame {
    buffer: usize,
    run_start: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Owner {
    Run(usize),
    Waiting(Option<usize>),
}

const END_BYTES: usize = 32;

struct ListEnd {
    owner: Owner,
    buffer: usize,
    end: usize,
    checked: usize,
    end_bytes: [u8; END_BYTES],
    place: ListPlace,
    delimiter: char,
}

impl ListEnd {
    fn new(owner: Owner, buffer: usize, output: &str, place: ListPlace, delimiter: char) -> Self {
        let mut key = Self {
            owner,
            buffer,
            end: output.len(),
            checked: output.len(),
            end_bytes: [0; END_BYTES],
            place,
            delimiter,
        };
        key.take_end_bytes(output);
        key
    }

    fn take_end_bytes(&mut self, output: &str) {
        let bytes = &output.as_bytes()[self.end.saturating_sub(END_BYTES)..self.end];
        self.end_bytes = [0; END_BYTES];
        self.end_bytes[..bytes.len()].copy_from_slice(bytes);
    }

    fn holds_end_bytes(&self, output: &str) -> bool {
        let start = self.end.saturating_sub(END_BYTES);
        output.as_bytes().get(start..self.end) == Some(&self.end_bytes[..self.end - start])
    }

    fn clean_after(&mut self, output: &str) -> bool {
        let start = self.checked.min(output.len()).max(self.end);
        self.checked = output.len();
        output
            .as_bytes()
            .get(start..)
            .is_some_and(|rest| rest.iter().all(u8::is_ascii_whitespace))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ListPlace {
    columns: usize,
    quotes: usize,
}

impl ListPlace {
    const fn of(ctx: &super::Context) -> Self {
        Self {
            columns: ctx.list_indent_columns,
            quotes: ctx.blockquote_depth,
        }
    }
}

impl LastList {
    pub fn enter(&self, output: &String) {
        let mut tracker = self.0.borrow_mut();
        let buffer = std::ptr::from_ref::<String>(output) as usize;
        let index = tracker.frames.len();
        let run_start = match tracker.frames.last() {
            Some(parent) if parent.buffer == buffer => parent.run_start,
            _ => index,
        };
        tracker.frames.push(Frame { buffer, run_start });
        let ListTracker { key, .. } = &mut *tracker;
        if let Some(waiting) = key.as_mut().filter(|key| {
            key.owner == Owner::Waiting(index.checked_sub(1)) && key.buffer == buffer && run_start == index
        }) {
            if waiting.holds_end_bytes(output) {
                waiting.owner = Owner::Run(run_start);
                if !waiting.clean_after(output) {
                    *key = None;
                }
            }
        }
    }

    pub fn leave(&self, output: &str) {
        let mut tracker = self.0.borrow_mut();
        let Some(frame) = tracker.frames.pop() else { return };
        let index = tracker.frames.len();
        let ListTracker { key, .. } = &mut *tracker;
        let Some(list_end) = key.as_mut() else { return };
        if list_end.owner == Owner::Waiting(Some(index)) {
            *list_end = ListEnd::new(
                Owner::Run(frame.run_start),
                frame.buffer,
                output,
                list_end.place,
                list_end.delimiter,
            );
        }
        if list_end.owner == Owner::Run(frame.run_start) {
            if !list_end.clean_after(output) {
                *key = None;
            } else if frame.run_start == index {
                list_end.owner = Owner::Waiting(index.checked_sub(1));
            }
        }
    }

    pub fn set(&self, output: &str, ctx: &super::Context, delimiter: char) {
        use crate::converter::utility::escaping::{leading_indent, opens_block};
        let mut tracker = self.0.borrow_mut();
        let Some(frame) = tracker.frames.last().copied() else {
            return;
        };
        let content = output.trim_end_matches(|character: char| character.is_ascii_whitespace());
        let line = &content[content.rfind('\n').map_or(0, |position| position + 1)..];
        let (indent, column) = leading_indent(line);
        let rest = &line[indent..];
        let closed = column < ctx.list_indent_columns + 2
            && opens_block(rest)
            && utils::strip_leading_bare_marker(rest).is_none();
        tracker.key = (!closed).then(|| {
            ListEnd::new(
                Owner::Run(frame.run_start),
                frame.buffer,
                output,
                ListPlace::of(ctx),
                delimiter,
            )
        });
    }

    fn delimiter_before(&self, output: &str, ctx: &super::Context) -> Option<char> {
        let mut tracker = self.0.borrow_mut();
        let run = tracker.frames.last()?.run_start;
        let key = tracker.key.as_mut()?;
        let open = if key.owner == Owner::Run(run) {
            key.clean_after(output)
        } else {
            output.bytes().all(|byte| byte.is_ascii_whitespace())
        };
        (open && key.place == ListPlace::of(ctx)).then_some(key.delimiter)
    }
}

/// The delimiter of an ordered list that starts where another ordered list ended.
pub fn switched_delimiter(output: &str, ctx: &super::Context) -> Option<char> {
    (!ctx.convert_as_inline
        && ctx.inline_depth == 0
        && !ctx.text_in_markers
        && !ctx.in_marker_span
        && ctx.last_list.delimiter_before(output, ctx) == Some('.'))
    .then_some(')')
}

#[derive(Clone, Default)]
pub struct PreviousMarker(std::rc::Rc<std::cell::RefCell<Option<PreviousMarkerState>>>);

struct PreviousMarkerState {
    buffer: usize,
    line_start: usize,
    line_before: String,
    enclosing_column: usize,
    open: bool,
}

impl PreviousMarker {
    fn get(&self, buffer: usize, output: &str, enclosing_column: usize) -> Option<(usize, bool)> {
        let state = self.0.borrow();
        let state = state.as_ref()?;
        (state.buffer == buffer
            && state.enclosing_column == enclosing_column
            && output
                .get(..state.line_start)
                .is_some_and(|before| before.ends_with(state.line_before.as_str())))
        .then_some((state.line_start, state.open))
    }

    fn set(&self, buffer: usize, output: &str, line_start: usize, enclosing_column: usize, open: bool) {
        let before = &output[..line_start];
        let line_before = &before[before
            .trim_end_matches('\n')
            .rfind('\n')
            .map_or(0, |position| position + 1)..];
        *self.0.borrow_mut() = Some(PreviousMarkerState {
            buffer,
            line_start,
            line_before: line_before.to_string(),
            enclosing_column,
            open,
        });
    }
}

#[derive(Clone, Default)]
pub struct ItemLineScan(std::rc::Rc<std::cell::RefCell<Option<ItemLineScanState>>>);

struct ItemLineScanState {
    buffer: usize,
    indent: String,
    end: usize,
    last_line_start: usize,
    last_line: String,
    open: Option<bool>,
}

impl ItemLineScan {
    pub fn new_item() -> Self {
        Self::default()
    }

    fn read(&self, buffer: usize, indent: &str, output: &str) -> Option<(usize, Option<bool>)> {
        let state = self.0.borrow();
        let state = state.as_ref()?;
        (state.buffer == buffer
            && state.indent == indent
            && output.get(state.last_line_start..state.end) == Some(state.last_line.as_str()))
        .then_some((state.end, state.open))
    }

    fn write(&self, buffer: usize, indent: &str, output: &str, open: Option<bool>) {
        let last_line_start = output.trim_end().rfind('\n').map_or(0, |position| position + 1);
        *self.0.borrow_mut() = Some(ItemLineScanState {
            buffer,
            indent: indent.to_string(),
            end: output.len(),
            last_line_start,
            last_line: output[last_line_start..].to_string(),
            open,
        });
    }
}

#[cfg(test)]
mod tracking_tests {
    use super::utils::lines_are_open;
    use super::*;

    fn owner_after_sibling_enters(first: &str, second: &str) -> Option<Owner> {
        let list = LastList::default();
        let root = String::new();
        let mut buffer = String::from(first);
        list.enter(&root);
        list.enter(&buffer);
        let place = ListPlace { columns: 0, quotes: 0 };
        let address = std::ptr::from_ref::<String>(&buffer) as usize;
        list.0.borrow_mut().key = Some(ListEnd::new(Owner::Run(1), address, &buffer, place, '.'));
        list.leave(&buffer);
        buffer.clear();
        buffer.push_str(second);
        list.enter(&buffer);
        list.0.borrow().key.as_ref().map(|key| key.owner)
    }

    #[test]
    fn a_later_child_takes_the_list_end_back_only_from_the_same_buffer() {
        assert_eq!(owner_after_sibling_enters("1. a\n", "1. a\n\n"), Some(Owner::Run(1)));
        assert_eq!(
            owner_after_sibling_enters("1. a\n", "zzzzzz\n"),
            Some(Owner::Waiting(Some(0)))
        );
        assert_eq!(owner_after_sibling_enters("1. a\n", "1. a\nx"), None);
    }

    #[test]
    fn a_buffer_that_no_longer_reaches_the_list_end_is_not_clean() {
        let place = ListPlace { columns: 0, quotes: 0 };
        let mut key = ListEnd::new(Owner::Run(0), 0, "1. a\n", place, '.');
        assert!(key.clean_after("1. a\n\n"));
        assert!(!key.clean_after("1."));
    }

    #[test]
    fn a_later_child_with_another_buffer_leaves_the_list_end_waiting() {
        let list = LastList::default();
        let root = String::new();
        let first = String::from("1. a\n");
        let second = first.clone();
        list.enter(&root);
        list.enter(&first);
        let place = ListPlace { columns: 0, quotes: 0 };
        let address = std::ptr::from_ref::<String>(&first) as usize;
        list.0.borrow_mut().key = Some(ListEnd::new(Owner::Run(1), address, &first, place, '.'));
        list.leave(&first);
        list.enter(&second);
        let owner = list.0.borrow().key.as_ref().map(|key| key.owner);
        assert_eq!(owner, Some(Owner::Waiting(Some(0))));
    }

    #[test]
    fn item_line_scan_reads_the_lines_again_after_its_last_line_changed() {
        let scan = ItemLineScan::new_item();
        let mut output = String::from("- a\n  b\n");
        assert_eq!(lines_are_open(&output, "  ", &scan), Some(true));
        output.clear();
        output.push_str("a\nb\nzzz\n");
        assert_eq!(lines_are_open(&output, "  ", &scan), Some(false));
    }

    #[test]
    fn item_line_scan_reads_the_lines_of_another_buffer_again() {
        let scan = ItemLineScan::new_item();
        assert_eq!(lines_are_open("- a\n  b\n", "  ", &scan), Some(true));
        assert_eq!(lines_are_open("a\nb\n  b\n", "  ", &scan), Some(false));
    }

    #[test]
    fn item_line_scan_reads_the_lines_again_for_another_indent() {
        let scan = ItemLineScan::new_item();
        let output = String::from("- a\n  b\n");
        assert_eq!(lines_are_open(&output, "  ", &scan), Some(true));
        assert_eq!(lines_are_open(&output, "    ", &scan), Some(false));
    }

    #[test]
    fn previous_marker_answers_only_for_its_buffer_column_and_line_before_its_marker_line() {
        let previous = PreviousMarker::default();
        previous.set(1, "p\n- a\n", 2, 0, true);
        assert_eq!(previous.get(1, "p\n- a\n- b\n", 0), Some((2, true)));
        assert_eq!(previous.get(1, "p\n- a\n- b\n", 2), None);
        assert_eq!(previous.get(2, "p\n- a\n- b\n", 0), None);
        assert_eq!(previous.get(1, "q\n- a\n- b\n", 0), None);
    }
}
