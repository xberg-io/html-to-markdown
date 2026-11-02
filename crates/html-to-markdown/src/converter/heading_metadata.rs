use super::Context;
use super::block::container::HandlerContext;
use crate::metadata::MetadataCollector;

enum TextEvent {
    Text(String, bool),
    Break(bool),
    Trim(bool),
    Space,
}

struct Heading {
    start: usize,
    level: u8,
    id: Option<String>,
    depth: usize,
    eligible: bool,
}

struct Element {
    node: u32,
    text_start: usize,
    pending_start: usize,
    heading: Option<Heading>,
    replaced: bool,
    replacement: Option<String>,
    trim_prefix: bool,
}

struct PendingHeader {
    level: u8,
    text: String,
    id: Option<String>,
    depth: usize,
}

pub struct HeaderCapture {
    // ~keep Semantic edits stay append-only until an element commits, so ancestor rollback
    // also restores whitespace removed by a discarded child without copying heading prefixes.
    events: Vec<TextEvent>,
    active: usize,
    elements: Vec<Element>,
    pending: Vec<PendingHeader>,
}

impl HeaderCapture {
    pub(super) const fn new() -> Self {
        Self {
            events: Vec::new(),
            active: 0,
            elements: Vec::new(),
            pending: Vec::new(),
        }
    }

    fn begin(&mut self, node: u32, depth: usize, name: &str, tag: &tl::HTMLTag<'_>) {
        let heading = matches!(name, "h1" | "h2" | "h3" | "h4" | "h5" | "h6").then(|| Heading {
            start: self.events.len(),
            level: name.as_bytes()[1] - b'0',
            id: tag
                .attributes()
                .get("id")
                .flatten()
                .map(|id| id.as_utf8_str().to_string()),
            depth,
            eligible: false,
        });
        self.active += usize::from(heading.is_some());
        self.elements.push(Element {
            node,
            text_start: self.events.len(),
            pending_start: self.pending.len(),
            heading,
            replaced: false,
            replacement: None,
            trim_prefix: false,
        });
    }

    fn finish(&mut self, node: u32) {
        let Some(element) = self.elements.pop() else { return };
        assert_eq!(element.node, node, "heading capture must close its own source element");
        if element.replaced {
            self.events.truncate(element.text_start);
            self.pending.truncate(element.pending_start);
            if self.active > 0 && element.trim_prefix {
                self.events.push(TextEvent::Trim(false));
            }
            if self.active > 0
                && let Some(replacement) = &element.replacement
            {
                self.events.push(TextEvent::Text(replacement.clone(), true));
            }
        }
        if let Some(heading) = element.heading {
            if heading.eligible && (!element.replaced || element.replacement.is_some()) {
                self.pending.push(PendingHeader {
                    level: heading.level,
                    text: semantic_text(&self.events[heading.start..]),
                    id: heading.id,
                    depth: heading.depth,
                });
            }
            self.active -= 1;
            if self.active == 0 {
                self.events.clear();
            }
        }
    }

    fn replace(&mut self, replacement: Option<&str>) {
        if let Some(element) = self.elements.last_mut() {
            element.replaced = true;
            element.replacement = replacement.map(str::to_string);
        }
    }
}

fn semantic_text(events: &[TextEvent]) -> String {
    let mut text = String::new();
    let mut after_break = false;
    let mut trim_boundary = false;
    for event in events {
        match event {
            TextEvent::Text(value, code) => {
                if trim_boundary && value.starts_with([' ', '\t']) {
                    text.truncate(text.trim_end_matches([' ', '\t']).len());
                }
                if !value.is_empty() {
                    trim_boundary = false;
                }
                let value = if after_break && !code {
                    value.trim_start_matches([' ', '\t', '\n', '\r'])
                } else {
                    value
                };
                text.push_str(value);
                if !value.is_empty() {
                    after_break = false;
                }
            }
            TextEvent::Break(code) => {
                if *code || !text.ends_with([' ', '\t']) {
                    text.push(' ');
                }
                after_break = !code;
            }
            TextEvent::Trim(preserve_owed) => {
                trim_boundary = *preserve_owed;
                if !preserve_owed {
                    text.truncate(text.trim_end_matches([' ', '\t']).len());
                }
            }
            TextEvent::Space => push_boundary_space(&mut text, after_break, &mut trim_boundary),
        }
    }
    super::block::heading::normalize_heading_text(text.trim()).into_owned()
}

fn push_boundary_space(text: &mut String, after_break: bool, trim_boundary: &mut bool) {
    if *trim_boundary {
        text.truncate(text.trim_end_matches([' ', '\t']).len());
        *trim_boundary = false;
    }
    if !after_break || !text.ends_with(' ') {
        text.push(' ');
    }
}

pub(super) fn begin(ctx: &Context, node: u32, depth: usize, name: &str, tag: &tl::HTMLTag<'_>) -> Option<u32> {
    let capture = ctx.header_capture.as_ref()?;
    capture.borrow_mut().begin(node, depth, name, tag);
    Some(node)
}

pub(super) fn finish(ctx: &Context, node: Option<u32>) {
    if let Some((capture, node)) = ctx.header_capture.as_ref().zip(node) {
        capture.borrow_mut().finish(node);
    }
}

pub(super) fn active(ctx: &Context) -> bool {
    ctx.header_capture
        .as_ref()
        .is_some_and(|capture| capture.borrow().active > 0)
}

fn record(ctx: &Context, event: TextEvent) {
    if let Some(capture) = ctx.header_capture.as_ref() {
        let mut capture = capture.borrow_mut();
        if capture.active > 0 {
            capture.events.push(event);
        }
    }
}

pub(super) fn append(ctx: &Context, text: &str) {
    if active(ctx) {
        record(ctx, TextEvent::Text(text.to_string(), ctx.in_code));
    }
}

pub(super) fn line_break(ctx: &Context) {
    record(ctx, TextEvent::Break(ctx.in_code));
}

pub(super) fn trim(ctx: &Context, preserve_owed: bool) {
    record(ctx, TextEvent::Trim(preserve_owed));
}

pub(super) fn checkpoint(ctx: &Context) -> Option<usize> {
    let capture = ctx.header_capture.as_ref()?.borrow();
    (capture.active > 0).then_some(capture.events.len())
}

pub(super) fn insert_space(ctx: &Context, start: Option<usize>) {
    if let Some((capture, start)) = ctx.header_capture.as_ref().zip(start) {
        let mut capture = capture.borrow_mut();
        if capture.active == 0 {
            return;
        }
        capture.events.insert(start, TextEvent::Space);
        for element in &mut capture.elements {
            if element.text_start > start {
                element.text_start += 1;
            }
            if let Some(heading) = &mut element.heading
                && heading.start > start
            {
                heading.start += 1;
            }
        }
    }
}

pub(super) fn preserved(node: &tl::NodeHandle, parser: &tl::Parser, handler: HandlerContext<'_>) {
    if !active(handler.ctx) {
        return;
    }
    let mut stack = vec![(*node, handler.ctx.in_code, 0)];
    while let Some((handle, in_code, depth)) = stack.pop() {
        match handle.get(parser) {
            Some(tl::Node::Raw(bytes)) => {
                let raw = bytes.as_utf8_str();
                let decoded = crate::text::decode_html_entities_cow(raw.as_ref());
                record(handler.ctx, TextEvent::Text(decoded.into_owned(), in_code));
            }
            Some(tl::Node::Tag(tag)) => {
                if let Some(event) = preserved_leaf(tag, parser, handler, in_code, depth) {
                    record(handler.ctx, event);
                    continue;
                }
                if depth >= crate::options::conversion::NATIVE_STACK_SAFE_DEPTH {
                    continue;
                }
                let name = tag.name().as_utf8_str();
                let in_code = in_code || name == "pre" || super::utility::content::is_inline_code(name.as_ref());
                if let Some(children) = handler.dom_ctx.children_of(handle.get_inner()) {
                    stack.extend(children.iter().rev().map(|child| (*child, in_code, depth + 1)));
                } else {
                    let mut children: Vec<_> = tag.children().top().iter().copied().collect();
                    children.reverse();
                    stack.extend(children.into_iter().map(|child| (child, in_code, depth + 1)));
                }
            }
            _ => {}
        }
    }
}

fn preserved_leaf(
    tag: &tl::HTMLTag<'_>,
    parser: &tl::Parser,
    handler: HandlerContext<'_>,
    in_code: bool,
    depth: usize,
) -> Option<TextEvent> {
    match tag.name().as_utf8_str().as_ref() {
        "br" => Some(TextEvent::Break(in_code)),
        "img" => Some(TextEvent::Text(
            tag.attributes().get("alt").flatten().map_or_else(String::new, |alt| {
                crate::text::decode_html_entities_cow(alt.as_utf8_str().as_ref()).into_owned()
            }),
            in_code,
        )),
        "svg" => Some(TextEvent::Text(
            super::media::svg::preserved_graphic_text(
                tag,
                parser,
                handler.options,
                handler.ctx.accessible_names,
                crate::options::conversion::NATIVE_STACK_SAFE_DEPTH - depth,
            ),
            in_code,
        )),
        _ => None,
    }
}

pub(super) fn replace(ctx: &Context, replacement: Option<&str>) {
    if let Some(capture) = ctx.header_capture.as_ref() {
        capture.borrow_mut().replace(replacement);
    }
}

#[cfg(feature = "visitor")]
pub(super) fn trim_replaced_prefix(ctx: &Context) {
    if let Some(capture) = &ctx.header_capture {
        let mut capture = capture.borrow_mut();
        if capture.active > 0
            && let Some(element) = capture.elements.last_mut()
        {
            element.trim_prefix = true;
        }
    }
}

#[cfg(feature = "visitor")]
pub(super) fn replace_owned(ctx: &Context, node: u32, replacement: Option<&str>) {
    if let Some(capture) = ctx.header_capture.as_ref() {
        let mut capture = capture.borrow_mut();
        assert_eq!(capture.elements.last().map(|element| element.node), Some(node));
        capture.replace(replacement);
    }
}

pub(super) fn eligible(ctx: &Context, node: u32) {
    if let Some(capture) = ctx.header_capture.as_ref() {
        let mut capture = capture.borrow_mut();
        if let Some(element) = capture.elements.last_mut().filter(|element| element.node == node)
            && let Some(heading) = element.heading.as_mut()
        {
            heading.eligible = true;
        }
    }
}

pub(super) fn flush(ctx: &Context) {
    if let Some((capture, collector)) = ctx.header_capture.as_ref().zip(ctx.metadata_collector.as_ref()) {
        let mut capture = capture.borrow_mut();
        let mut collector = collector.borrow_mut();
        write_headers(&mut collector, capture.pending.drain(..));
    }
}

fn write_headers(collector: &mut MetadataCollector, headers: impl Iterator<Item = PendingHeader>) {
    for header in headers {
        collector.add_header(header.level, header.text, header.id, header.depth, 0);
    }
}

#[cfg(test)]
mod preserved_depth_tests;
