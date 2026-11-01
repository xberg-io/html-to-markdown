use super::Context;
use crate::metadata::MetadataCollector;
use crate::types::StructureCollector;
use crate::types::structure_collector::{TextCaptureId, TextCaptureKind};

struct Heading {
    capture: TextCaptureId,
    level: u8,
    id: Option<String>,
    depth: usize,
    eligible: bool,
}

struct Element {
    node: u32,
    pending_start: usize,
    heading: Option<Heading>,
    replaced: bool,
    discarded: bool,
}

struct PendingHeader {
    level: u8,
    text: String,
    id: Option<String>,
    depth: usize,
}

pub(crate) struct HeaderCapture {
    // ~keep Only text/checkpoint operations are used, so metadata never builds a public tree.
    text: StructureCollector,
    elements: Vec<Element>,
    pending: Vec<PendingHeader>,
}

impl HeaderCapture {
    pub(super) fn new() -> Self {
        Self {
            text: StructureCollector::new(),
            elements: Vec::new(),
            pending: Vec::new(),
        }
    }

    fn begin(&mut self, node: u32, depth: usize, name: &str, tag: &tl::HTMLTag<'_>) {
        let heading = matches!(name, "h1" | "h2" | "h3" | "h4" | "h5" | "h6").then(|| Heading {
            capture: self.text.begin_text_capture(TextCaptureKind::Heading),
            level: name.as_bytes()[1] - b'0',
            id: tag
                .attributes()
                .get("id")
                .flatten()
                .map(|id| id.as_utf8_str().to_string()),
            depth,
            eligible: false,
        });
        self.text.begin_element(None);
        self.elements.push(Element {
            node,
            pending_start: self.pending.len(),
            heading,
            replaced: false,
            discarded: false,
        });
    }

    fn finish(&mut self, node: u32) {
        let Some(element) = self.elements.pop() else { return };
        assert_eq!(element.node, node, "heading capture must close its own source element");
        self.text.finish_element(false);
        // ~keep An ancestor can discard a heading after that child has finished its own capture.
        if element.replaced {
            self.pending.truncate(element.pending_start);
        }
        if let Some(heading) = element.heading {
            let (text, _) = self.text.finish_text_capture(heading.capture);
            if heading.eligible && !element.discarded {
                self.pending.push(PendingHeader {
                    level: heading.level,
                    text,
                    id: heading.id,
                    depth: heading.depth,
                });
            }
        }
    }

    fn replace(&mut self, replacement: Option<&str>) {
        if let Some(element) = self.elements.last_mut() {
            element.replaced = true;
            element.discarded = replacement.is_none();
        }
        self.text.replace_current_element(replacement);
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
        .is_some_and(|capture| capture.borrow().text.has_active_text_capture())
}

pub(super) fn append(ctx: &Context, text: &str) {
    if let Some(capture) = ctx.header_capture.as_ref() {
        let mut capture = capture.borrow_mut();
        if ctx.in_code {
            capture.text.append_text(text);
        } else {
            capture.text.append_heading_text(text);
        }
    }
}

pub(super) fn line_break(ctx: &Context) {
    if let Some(capture) = ctx.header_capture.as_ref() {
        let mut capture = capture.borrow_mut();
        if ctx.in_code {
            capture.text.append_text(" ");
        } else {
            capture.text.append_heading_break();
        }
    }
}

pub(super) fn replace(ctx: &Context, replacement: Option<&str>) {
    if let Some(capture) = ctx.header_capture.as_ref() {
        capture.borrow_mut().replace(replacement);
    }
}

pub(super) fn eligible(ctx: &Context, node: u32) {
    if let Some(capture) = ctx.header_capture.as_ref() {
        let mut capture = capture.borrow_mut();
        if let Some(element) = capture.elements.last_mut().filter(|element| element.node == node) {
            if let Some(heading) = element.heading.as_mut() {
                heading.eligible = true;
            }
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
