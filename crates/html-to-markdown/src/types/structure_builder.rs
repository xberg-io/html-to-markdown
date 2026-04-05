//! Builds a [`DocumentStructure`] from a parsed `tl::VDom`.
//!
//! Walk the DOM once, mapping each HTML element to the appropriate [`NodeContent`] variant,
//! collecting inline [`TextAnnotation`]s, tracking parent/child relationships, and generating
//! heading-based [`Group`] hierarchy.

use std::collections::HashMap;

use super::document::{AnnotationKind, DocumentNode, DocumentStructure, NodeContent, TextAnnotation};
use super::tables::{GridCell, TableGrid};

// ── Text extraction ───────────────────────────────────────────────────────────

/// Extract plain text from a tag's descendants, decoding HTML entities.
fn extract_text(tag: &tl::HTMLTag, parser: &tl::Parser) -> String {
    let mut buf = String::new();
    collect_text_from_tag(tag, parser, &mut buf);
    buf
}

/// Recursively accumulate text content from a tag's children.
fn collect_text_from_tag(tag: &tl::HTMLTag, parser: &tl::Parser, buf: &mut String) {
    let children = tag.children();
    for handle in children.top().iter() {
        let Some(node) = handle.get(parser) else {
            continue;
        };
        match node {
            tl::Node::Raw(bytes) => {
                let raw = bytes.as_utf8_str();
                let decoded = crate::text::decode_html_entities_cow(raw.as_ref());
                buf.push_str(&decoded);
            }
            tl::Node::Tag(child_tag) => {
                let name = child_tag.name().as_utf8_str().to_ascii_lowercase();
                // Skip invisible elements
                if matches!(name.as_str(), "script" | "style" | "head") {
                    continue;
                }
                collect_text_from_tag(child_tag, parser, buf);
            }
            tl::Node::Comment(_) => {}
        }
    }
}

// ── Inline annotation extraction ─────────────────────────────────────────────

/// Scan the children of `tag` and collect [`TextAnnotation`]s into `annotations`.
///
/// `text` is the pre-extracted full text of the enclosing block node; annotation
/// byte offsets are computed relative to that string.
fn collect_annotations(tag: &tl::HTMLTag, parser: &tl::Parser, text: &str, annotations: &mut Vec<TextAnnotation>) {
    collect_annotations_from_tag(tag, parser, text, &mut 0usize, annotations);
}

/// Recursive helper.  `offset` tracks how many bytes of `full_text` have been consumed
/// so far; it is mutated in place as we walk the tree.
fn collect_annotations_from_tag(
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    full_text: &str,
    offset: &mut usize,
    annotations: &mut Vec<TextAnnotation>,
) {
    let children = tag.children();
    for handle in children.top().iter() {
        let Some(node) = handle.get(parser) else {
            continue;
        };
        match node {
            tl::Node::Raw(bytes) => {
                let raw = bytes.as_utf8_str();
                let decoded = crate::text::decode_html_entities_cow(raw.as_ref());
                *offset += decoded.len();
            }
            tl::Node::Tag(child_tag) => {
                let name = child_tag.name().as_utf8_str().to_ascii_lowercase();
                if matches!(name.as_str(), "script" | "style" | "head") {
                    continue;
                }

                let start = *offset;
                // Recurse to advance offset over the child's text span.
                collect_annotations_from_tag(child_tag, parser, full_text, offset, annotations);
                let end = *offset;

                // Emit annotation only for non-empty spans that fit within the text.
                if start < end && end <= full_text.len() {
                    let kind = match name.as_str() {
                        "strong" | "b" => Some(AnnotationKind::Bold),
                        "em" | "i" => Some(AnnotationKind::Italic),
                        "u" | "ins" => Some(AnnotationKind::Underline),
                        "s" | "del" | "strike" => Some(AnnotationKind::Strikethrough),
                        "code" | "kbd" | "samp" => Some(AnnotationKind::Code),
                        "sub" => Some(AnnotationKind::Subscript),
                        "sup" => Some(AnnotationKind::Superscript),
                        "mark" => Some(AnnotationKind::Highlight),
                        "a" => {
                            let url = child_tag
                                .attributes()
                                .get("href")
                                .flatten()
                                .map(|v| v.as_utf8_str().to_string())
                                .unwrap_or_default();
                            let title = child_tag
                                .attributes()
                                .get("title")
                                .flatten()
                                .map(|v| v.as_utf8_str().to_string());
                            Some(AnnotationKind::Link { url, title })
                        }
                        _ => None,
                    };

                    if let Some(kind) = kind {
                        annotations.push(TextAnnotation {
                            start: start as u32,
                            end: end as u32,
                            kind,
                        });
                    }
                }
            }
            tl::Node::Comment(_) => {}
        }
    }
}

// ── Table extraction ──────────────────────────────────────────────────────────

/// Build a [`TableGrid`] from a `<table>` element.
fn extract_table_grid(table_tag: &tl::HTMLTag, parser: &tl::Parser) -> TableGrid {
    // Gather all <tr> handles (recursing through thead/tbody/tfoot).
    let mut row_handles: Vec<tl::NodeHandle> = Vec::new();
    collect_tr_handles(table_tag, parser, &mut row_handles);

    let mut cells: Vec<GridCell> = Vec::new();
    let mut max_col: u32 = 0;

    for (row_idx, row_handle) in row_handles.iter().enumerate() {
        let Some(tl::Node::Tag(row_tag)) = row_handle.get(parser) else {
            continue;
        };

        let mut col_idx: u32 = 0;
        let row_children = row_tag.children();

        for child_handle in row_children.top().iter() {
            let Some(tl::Node::Tag(cell_tag)) = child_handle.get(parser) else {
                continue;
            };
            let cell_name = cell_tag.name().as_utf8_str().to_ascii_lowercase();
            let is_cell = cell_name == "td" || cell_name == "th";
            if !is_cell {
                continue;
            }

            let is_header = cell_name == "th";

            let row_span = cell_tag
                .attributes()
                .get("rowspan")
                .flatten()
                .and_then(|v| v.as_utf8_str().parse::<u32>().ok())
                .unwrap_or(1)
                .max(1);

            let col_span = cell_tag
                .attributes()
                .get("colspan")
                .flatten()
                .and_then(|v| v.as_utf8_str().parse::<u32>().ok())
                .unwrap_or(1)
                .max(1);

            let content = extract_text(cell_tag, parser).trim().to_string();

            cells.push(GridCell {
                content,
                row: row_idx as u32,
                col: col_idx,
                row_span,
                col_span,
                is_header,
            });

            col_idx += col_span;
            if col_idx > max_col {
                max_col = col_idx;
            }
        }
    }

    let rows = row_handles.len() as u32;
    TableGrid {
        rows,
        cols: max_col,
        cells,
    }
}

/// Recursively collect all `<tr>` `NodeHandle`s from within a table element.
fn collect_tr_handles(tag: &tl::HTMLTag, parser: &tl::Parser, result: &mut Vec<tl::NodeHandle>) {
    let children = tag.children();
    for handle in children.top().iter() {
        if let Some(tl::Node::Tag(child_tag)) = handle.get(parser) {
            let name = child_tag.name().as_utf8_str().to_ascii_lowercase();
            if name == "tr" {
                result.push(*handle);
            } else {
                collect_tr_handles(child_tag, parser, result);
            }
        }
    }
}

// ── Node ID generation ────────────────────────────────────────────────────────

/// Generate a deterministic node ID from the node type, an excerpt of its text content,
/// and its position (index) in the flat node list.
fn make_node_id(node_type: &str, text: &str, index: usize) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    node_type.hash(&mut hasher);
    // Only hash a prefix of the text to keep cost bounded.
    let end = crate::converter::utility::content::floor_char_boundary(text, text.len().min(64));
    text[..end].hash(&mut hasher);
    index.hash(&mut hasher);
    let digest = hasher.finish();
    format!("{node_type}-{digest:016x}")
}

// ── Definition list helpers ───────────────────────────────────────────────────

/// Collect `<dt>`/`<dd>` pairs from a `<dl>` element.
///
/// Returns `(term_text, definition_text)` tuples.  Consecutive `<dt>` elements share
/// the next `<dd>`; orphan `<dd>`s use an empty term.
fn collect_definition_items(dl_tag: &tl::HTMLTag, parser: &tl::Parser) -> Vec<(String, String)> {
    let mut items: Vec<(String, String)> = Vec::new();
    let mut pending_terms: Vec<String> = Vec::new();

    let children = dl_tag.children();
    for handle in children.top().iter() {
        let Some(tl::Node::Tag(child_tag)) = handle.get(parser) else {
            continue;
        };
        let name = child_tag.name().as_utf8_str().to_ascii_lowercase();
        match name.as_str() {
            "dt" => {
                pending_terms.push(extract_text(child_tag, parser).trim().to_string());
            }
            "dd" => {
                let definition = extract_text(child_tag, parser).trim().to_string();
                if pending_terms.is_empty() {
                    items.push((String::new(), definition));
                } else {
                    let mut drained: Vec<String> = std::mem::take(&mut pending_terms);
                    let last_term = drained.pop();
                    for term in drained {
                        items.push((term, String::new()));
                    }
                    if let Some(term) = last_term {
                        items.push((term, definition));
                    }
                }
            }
            _ => {}
        }
    }

    // Flush trailing <dt>s without a corresponding <dd>.
    for term in pending_terms {
        items.push((term, String::new()));
    }

    items
}

// ── Head metadata extraction ──────────────────────────────────────────────────

/// Extract `<meta name=… content=…>` and `<title>` entries from a `<head>` element.
fn extract_head_metadata_entries(head_tag: &tl::HTMLTag, parser: &tl::Parser) -> Vec<(String, String)> {
    let mut entries: Vec<(String, String)> = Vec::new();

    let children = head_tag.children();
    for handle in children.top().iter() {
        let Some(tl::Node::Tag(child_tag)) = handle.get(parser) else {
            continue;
        };
        let name = child_tag.name().as_utf8_str().to_ascii_lowercase();
        match name.as_str() {
            "title" => {
                let title = extract_text(child_tag, parser).trim().to_string();
                if !title.is_empty() {
                    entries.push(("title".to_string(), title));
                }
            }
            "meta" => {
                // name + content
                if let (Some(Some(meta_name)), Some(Some(meta_content))) = (
                    child_tag.attributes().get("name"),
                    child_tag.attributes().get("content"),
                ) {
                    entries.push((
                        meta_name.as_utf8_str().to_string(),
                        meta_content.as_utf8_str().to_string(),
                    ));
                }
                // property + content (Open Graph etc.)
                if let (Some(Some(property)), Some(Some(content))) = (
                    child_tag.attributes().get("property"),
                    child_tag.attributes().get("content"),
                ) {
                    entries.push((property.as_utf8_str().to_string(), content.as_utf8_str().to_string()));
                }
            }
            _ => {}
        }
    }

    entries
}

// ── Main builder ──────────────────────────────────────────────────────────────

/// State threaded through the recursive walk.
struct BuilderState {
    /// Accumulated nodes (flat list in document order).
    nodes: Vec<DocumentNode>,
    /// Stack of open heading-group indices: `(heading_level, node_index)`.
    group_stack: Vec<(u8, u32)>,
}

impl BuilderState {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            group_stack: Vec::new(),
        }
    }

    /// Append a node and return its index.
    fn push(&mut self, node: DocumentNode) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(node);
        idx
    }

    /// Index of the innermost open group, if any.
    fn current_group(&self) -> Option<u32> {
        self.group_stack.last().map(|(_, idx)| *idx)
    }

    /// Record `child_idx` as a child of `parent_idx`.
    fn add_child(&mut self, parent_idx: u32, child_idx: u32) {
        if let Some(parent) = self.nodes.get_mut(parent_idx as usize) {
            parent.children.push(child_idx);
        }
    }
}

/// Build a [`DocumentStructure`] from an already-parsed `tl::VDom`.
///
/// Walks the DOM once, mapping HTML elements to semantic [`NodeContent`] variants,
/// tracking parent/child relationships, extracting inline [`TextAnnotation`]s, and
/// constructing heading-based [`Group`] nodes.
pub fn build_document_structure(dom: &tl::VDom<'_>) -> DocumentStructure {
    let parser = dom.parser();
    let mut state = BuilderState::new();

    for handle in dom.children() {
        walk(&mut state, handle, parser, None);
    }

    DocumentStructure {
        nodes: state.nodes,
        source_format: Some("html".to_string()),
    }
}

/// Recursive DOM walker.
///
/// `parent_idx` is the flat-list index of the nearest structural parent, if any.
fn walk(state: &mut BuilderState, handle: &tl::NodeHandle, parser: &tl::Parser, parent_idx: Option<u32>) {
    let Some(node) = handle.get(parser) else {
        return;
    };

    match node {
        tl::Node::Raw(_) | tl::Node::Comment(_) => {}
        tl::Node::Tag(tag) => {
            let tag_name = tag.name().as_utf8_str().to_ascii_lowercase();
            process_tag(state, tag_name.as_str(), tag, parser, parent_idx);
        }
    }
}

/// Decide how to handle a given tag, creating nodes and recursing as needed.
#[allow(clippy::too_many_lines)]
fn process_tag(
    state: &mut BuilderState,
    tag_name: &str,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    parent_idx: Option<u32>,
) {
    match tag_name {
        // ── Headings ──────────────────────────────────────────────────────
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            let level = tag_name[1..].parse::<u8>().unwrap_or(1);
            let text = extract_text(tag, parser).trim().to_string();

            // Close any open groups at the same or deeper heading level.
            while let Some(&(open_level, _)) = state.group_stack.last() {
                if open_level >= level {
                    state.group_stack.pop();
                } else {
                    break;
                }
            }

            // Parent for the new group is the enclosing group or the explicit parent.
            let group_parent = state.group_stack.last().map(|(_, idx)| *idx).or(parent_idx);
            let group_id = make_node_id("group", &text, state.nodes.len());
            let group_idx = state.push(DocumentNode {
                id: group_id,
                content: NodeContent::Group {
                    label: Some(text.clone()),
                    heading_level: Some(level),
                    heading_text: Some(text.clone()),
                },
                parent: group_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
            if let Some(gp) = group_parent {
                state.add_child(gp, group_idx);
            }
            state.group_stack.push((level, group_idx));

            // Emit the Heading node as a child of the new group.
            let mut annotations = Vec::new();
            collect_annotations(tag, parser, &text, &mut annotations);
            let heading_id = make_node_id("heading", &text, state.nodes.len());
            let heading_idx = state.push(DocumentNode {
                id: heading_id,
                content: NodeContent::Heading { level, text },
                parent: Some(group_idx),
                children: Vec::new(),
                annotations,
                attributes: None,
            });
            state.add_child(group_idx, heading_idx);
        }

        // ── Paragraph ────────────────────────────────────────────────────
        "p" => {
            let text = extract_text(tag, parser).trim().to_string();
            if text.is_empty() {
                return;
            }
            let effective_parent = state.current_group().or(parent_idx);
            let mut annotations = Vec::new();
            collect_annotations(tag, parser, &text, &mut annotations);
            let id = make_node_id("paragraph", &text, state.nodes.len());
            let idx = state.push(DocumentNode {
                id,
                content: NodeContent::Paragraph { text },
                parent: effective_parent,
                children: Vec::new(),
                annotations,
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, idx);
            }
        }

        // ── Lists ─────────────────────────────────────────────────────────
        "ul" | "ol" => {
            let ordered = tag_name == "ol";
            let effective_parent = state.current_group().or(parent_idx);
            let id = make_node_id("list", if ordered { "ordered" } else { "unordered" }, state.nodes.len());
            let list_idx = state.push(DocumentNode {
                id,
                content: NodeContent::List { ordered },
                parent: effective_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, list_idx);
            }
            // Recurse with the list node as the parent so <li>s attach to it.
            let children = tag.children();
            for child_handle in children.top().iter() {
                walk(state, child_handle, parser, Some(list_idx));
            }
        }

        // ── List item ─────────────────────────────────────────────────────
        "li" => {
            let text = extract_text(tag, parser).trim().to_string();
            let effective_parent = parent_idx.or_else(|| state.current_group());
            let mut annotations = Vec::new();
            collect_annotations(tag, parser, &text, &mut annotations);
            let id = make_node_id("list_item", &text, state.nodes.len());
            let idx = state.push(DocumentNode {
                id,
                content: NodeContent::ListItem { text },
                parent: effective_parent,
                children: Vec::new(),
                annotations,
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, idx);
            }
        }

        // ── Table ─────────────────────────────────────────────────────────
        "table" => {
            let grid = extract_table_grid(tag, parser);
            let effective_parent = state.current_group().or(parent_idx);
            let id = make_node_id("table", &grid.rows.to_string(), state.nodes.len());
            let idx = state.push(DocumentNode {
                id,
                content: NodeContent::Table { grid },
                parent: effective_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, idx);
            }
        }

        // ── Image ─────────────────────────────────────────────────────────
        "img" => {
            let src = tag
                .attributes()
                .get("src")
                .flatten()
                .map(|v| v.as_utf8_str().to_string());
            let description = tag
                .attributes()
                .get("alt")
                .flatten()
                .map(|v| v.as_utf8_str().to_string())
                .filter(|s| !s.is_empty());
            let effective_parent = state.current_group().or(parent_idx);
            let label = src.as_deref().unwrap_or("img");
            let id = make_node_id("image", label, state.nodes.len());
            let idx = state.push(DocumentNode {
                id,
                content: NodeContent::Image {
                    description,
                    src,
                    image_index: None,
                },
                parent: effective_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, idx);
            }
        }

        // ── Code block (<pre><code …>) ────────────────────────────────────
        "pre" => {
            let mut language: Option<String> = None;
            let mut code_text: Option<String> = None;

            let children = tag.children();
            for child_handle in children.top().iter() {
                if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                    let child_name = child_tag.name().as_utf8_str().to_ascii_lowercase();
                    if child_name == "code" {
                        // Extract language from class="language-*"
                        if let Some(Some(class_val)) = child_tag.attributes().get("class") {
                            let class_str = class_val.as_utf8_str();
                            for token in class_str.split_whitespace() {
                                if let Some(lang) = token.strip_prefix("language-") {
                                    language = Some(lang.to_string());
                                    break;
                                }
                            }
                        }
                        code_text = Some(extract_text(child_tag, parser));
                        break;
                    }
                }
            }

            let text = code_text.unwrap_or_else(|| extract_text(tag, parser));
            let effective_parent = state.current_group().or(parent_idx);
            let id = make_node_id("code", &text, state.nodes.len());
            let idx = state.push(DocumentNode {
                id,
                content: NodeContent::Code { text, language },
                parent: effective_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, idx);
            }
        }

        // ── Blockquote ────────────────────────────────────────────────────
        "blockquote" => {
            let effective_parent = state.current_group().or(parent_idx);
            let id = make_node_id("quote", "blockquote", state.nodes.len());
            let quote_idx = state.push(DocumentNode {
                id,
                content: NodeContent::Quote,
                parent: effective_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, quote_idx);
            }
            // Recurse into blockquote children under the Quote node.
            let children = tag.children();
            for child_handle in children.top().iter() {
                walk(state, child_handle, parser, Some(quote_idx));
            }
        }

        // ── Definition list ───────────────────────────────────────────────
        "dl" => {
            let effective_parent = state.current_group().or(parent_idx);
            let id = make_node_id("definition_list", "dl", state.nodes.len());
            let dl_idx = state.push(DocumentNode {
                id,
                content: NodeContent::DefinitionList,
                parent: effective_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, dl_idx);
            }

            for (term, definition) in collect_definition_items(tag, parser) {
                let item_id = make_node_id("definition_item", &term, state.nodes.len());
                let item_idx = state.push(DocumentNode {
                    id: item_id,
                    content: NodeContent::DefinitionItem { term, definition },
                    parent: Some(dl_idx),
                    children: Vec::new(),
                    annotations: Vec::new(),
                    attributes: None,
                });
                state.add_child(dl_idx, item_idx);
            }
        }

        // ── Script / Style → RawBlock ─────────────────────────────────────
        "script" | "style" => {
            let format = if tag_name == "script" {
                tag.attributes()
                    .get("type")
                    .flatten()
                    .map(|v| v.as_utf8_str().to_string())
                    .unwrap_or_else(|| "javascript".to_string())
            } else {
                "css".to_string()
            };
            let content = extract_text(tag, parser);
            if content.trim().is_empty() {
                return;
            }
            let effective_parent = state.current_group().or(parent_idx);
            let id = make_node_id("raw_block", &format, state.nodes.len());
            let idx = state.push(DocumentNode {
                id,
                content: NodeContent::RawBlock { format, content },
                parent: effective_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, idx);
            }
        }

        // ── Head → MetadataBlock ──────────────────────────────────────────
        "head" => {
            let entries = extract_head_metadata_entries(tag, parser);
            if entries.is_empty() {
                return;
            }
            let id = make_node_id("metadata_block", "head", state.nodes.len());
            // Metadata blocks sit at the root level.
            state.push(DocumentNode {
                id,
                content: NodeContent::MetadataBlock { entries },
                parent: None,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: None,
            });
        }

        // ── Semantic containers → Group node ──────────────────────────────
        "main" | "article" | "section" | "header" | "footer" | "nav" | "aside" => {
            let label = tag
                .attributes()
                .get("aria-label")
                .flatten()
                .map(|v| v.as_utf8_str().to_string());
            let effective_parent = state.current_group().or(parent_idx);
            let id = make_node_id("group", tag_name, state.nodes.len());
            let group_idx = state.push(DocumentNode {
                id,
                content: NodeContent::Group {
                    label,
                    heading_level: None,
                    heading_text: None,
                },
                parent: effective_parent,
                children: Vec::new(),
                annotations: Vec::new(),
                attributes: collect_attributes(tag),
            });
            if let Some(ep) = effective_parent {
                state.add_child(ep, group_idx);
            }
            let children = tag.children();
            for child_handle in children.top().iter() {
                walk(state, child_handle, parser, Some(group_idx));
            }
        }

        // ── Transparent structural containers ─────────────────────────────
        "html" | "body" | "div" | "figure" | "figcaption" | "details" | "summary" | "address" | "hgroup" | "search"
        | "form" | "fieldset" => {
            let children = tag.children();
            for child_handle in children.top().iter() {
                walk(state, child_handle, parser, parent_idx);
            }
        }

        // ── Everything else: recurse transparently ────────────────────────
        _ => {
            let children = tag.children();
            for child_handle in children.top().iter() {
                walk(state, child_handle, parser, parent_idx);
            }
        }
    }
}

/// Collect a safe subset of attributes into a `HashMap`.
///
/// Only `id`, `class`, `lang`, `dir`, and `data-*` attributes are kept.
/// Event handlers (`on*`) and other potentially unsafe attributes are dropped.
fn collect_attributes(tag: &tl::HTMLTag) -> Option<HashMap<String, String>> {
    let raw = tag.attributes().clone();
    let mut map: HashMap<String, String> = HashMap::new();

    for (key_cow, val_opt) in raw.iter() {
        let key = key_cow.to_ascii_lowercase();
        // Drop event handlers.
        if key.starts_with("on") {
            continue;
        }
        if matches!(key.as_str(), "id" | "class" | "lang" | "dir") || key.starts_with("data-") {
            if let Some(val) = val_opt {
                map.insert(key, val.to_string());
            }
        }
    }

    if map.is_empty() { None } else { Some(map) }
}
