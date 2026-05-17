//! Collector that builds a [`DocumentStructure`] during the converter's HTML DOM walk.
//!
//! Follows the same single-pass collector pattern used by [`crate::metadata::MetadataCollector`]:
//! an `Rc<RefCell<StructureCollector>>` handle is threaded through the [`crate::converter::Context`]
//! and individual element handlers call `push_*` methods as they encounter content.
//!
//! # Design
//!
//! - **Flat node array** with index-based parent/child links (matches [`DocumentStructure`]).
//! - **section_stack** tracks the currently-open heading groups (`(level, group_node_index)`).
//! - **container_stack** tracks open blockquote containers.
//! - **list_stack** tracks open list containers so `push_list_item` attaches items to the right list.
//! - IDs are deterministic hashes of `(node_type, text_prefix, index)`.

use std::cell::RefCell;
use std::rc::Rc;

use super::document::{DocumentNode, DocumentStructure, NodeContent};
use super::tables::{TableData, TableGrid};

/// Shared mutable handle used in [`crate::converter::Context`].
pub type StructureCollectorHandle = Rc<RefCell<StructureCollector>>;

/// Incremental builder for [`DocumentStructure`] during a single DOM walk.
pub struct StructureCollector {
    /// Accumulated nodes in document order.
    nodes: Vec<DocumentNode>,
    /// Open heading-group stack: `(heading_level, node_index)`.
    /// Mirrors the `group_stack` in `structure_builder`.
    section_stack: Vec<(u8, u32)>,
    /// Open blockquote container indices (innermost last).
    container_stack: Vec<u32>,
    /// Open list container indices (innermost last).
    list_stack: Vec<u32>,
    /// Extracted tables with both structured grid data and markdown rendering.
    ///
    /// Populated by [`push_table_data`] when document structure extraction is enabled.
    tables: Vec<TableData>,
}

impl StructureCollector {
    /// Create a new empty collector ready to accumulate nodes for a single conversion pass.
    ///
    /// The intended lifecycle is:
    ///
    /// 1. Create one `StructureCollector` (or wrap it in a [`StructureCollectorHandle`]) at the
    ///    start of a conversion.
    /// 2. Thread the handle through the converter context; individual element handlers call the
    ///    `push_*` methods as they encounter headings, paragraphs, lists, tables, and so on.
    /// 3. Call [`Self::finish`] exactly once to consume the collector and obtain the completed
    ///    [`super::document::DocumentStructure`] together with the flat list of extracted
    ///    [`super::tables::TableData`] entries.
    ///
    /// `StructureCollector` is not `Send` (it uses `Rc<RefCell<…>>` handles internally) and must
    /// not be shared across threads. Create a fresh collector per conversion.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            section_stack: Vec::new(),
            container_stack: Vec::new(),
            list_stack: Vec::new(),
            tables: Vec::new(),
        }
    }

    // ── Public push methods ──────────────────────────────────────────────────

    /// Record a heading element.
    ///
    /// Creates a [`NodeContent::Group`] (which owns all subsequent sibling content until a
    /// heading of equal or higher rank closes it) followed by a [`NodeContent::Heading`] child.
    ///
    /// Returns the index of the **heading** node (the group node is one before it).
    pub fn push_heading(&mut self, level: u8, text: &str, id: Option<&str>) -> u32 {
        // Close any open groups at the same or deeper heading level.
        while let Some(&(open_level, _)) = self.section_stack.last() {
            if open_level >= level {
                self.section_stack.pop();
            } else {
                break;
            }
        }

        // The group's parent is the surrounding open group or a container/list (if any).
        let group_parent = self.current_structural_parent();

        // Create the Group node.
        let group_id = Self::generate_id("group", text, self.nodes.len() as u32);
        let group_idx = self.raw_push(DocumentNode {
            id: group_id,
            content: NodeContent::Group {
                label: Some(text.to_string()),
                heading_level: Some(level),
                heading_text: Some(text.to_string()),
            },
            parent: group_parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(gp) = group_parent {
            self.add_child(gp, group_idx);
        }
        self.section_stack.push((level, group_idx));

        // Create the Heading node as a child of the new group.
        let heading_id = Self::generate_id("heading", text, self.nodes.len() as u32);
        let heading_idx = self.raw_push(DocumentNode {
            id: heading_id,
            content: NodeContent::Heading {
                level,
                text: text.to_string(),
            },
            parent: Some(group_idx),
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: id.map(|v| {
                let mut m = std::collections::HashMap::new();
                m.insert("id".to_string(), v.to_string());
                m
            }),
        });
        self.add_child(group_idx, heading_idx);
        heading_idx
    }

    /// Record a paragraph element.
    ///
    /// Returns the node index.
    pub fn push_paragraph(&mut self, text: &str) -> u32 {
        if text.is_empty() {
            return u32::MAX;
        }
        let parent = self.current_structural_parent();
        let id = Self::generate_id("paragraph", text, self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::Paragraph { text: text.to_string() },
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        idx
    }

    /// Open a list container.
    ///
    /// Returns the node index; call [`Self::push_list_end`] to close it.
    pub fn push_list_start(&mut self, ordered: bool) -> u32 {
        let parent = self.current_structural_parent();
        let label = if ordered { "ordered" } else { "unordered" };
        let id = Self::generate_id("list", label, self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::List { ordered },
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        self.list_stack.push(idx);
        idx
    }

    /// Close the innermost open list container.
    pub fn push_list_end(&mut self) {
        self.list_stack.pop();
    }

    /// Record a list item under the current open list.
    ///
    /// If there is no open list, the item is parented under the current section/container.
    /// Returns the node index.
    pub fn push_list_item(&mut self, text: &str) -> u32 {
        let parent = self
            .list_stack
            .last()
            .copied()
            .or_else(|| self.current_structural_parent());
        let id = Self::generate_id("list_item", text, self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::ListItem { text: text.to_string() },
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        idx
    }

    /// Record a table with both structured grid data and its markdown rendering.
    ///
    /// Adds the table to the document tree as a [`NodeContent::Table`] node and also
    /// appends a [`TableData`] entry (grid + markdown) to the flat tables list that is
    /// exposed via [`crate::ConversionResult::tables`].
    ///
    /// Returns the node index.
    pub fn push_table_data(&mut self, grid: TableGrid, markdown: String) -> u32 {
        let parent = self.current_structural_parent();
        let label = grid.rows.to_string();
        let id = Self::generate_id("table", &label, self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::Table { grid: grid.clone() },
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        self.tables.push(TableData { grid, markdown });
        idx
    }

    /// Record a table (grid only, no markdown rendering).
    ///
    /// Prefer [`Self::push_table_data`] when the markdown rendering is available; use this
    /// method only when the markdown is not yet computed.
    ///
    /// Returns the node index.
    pub fn push_table(&mut self, grid: TableGrid) -> u32 {
        let parent = self.current_structural_parent();
        let label = grid.rows.to_string();
        let id = Self::generate_id("table", &label, self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::Table { grid },
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        idx
    }

    /// Record an image element.
    ///
    /// Returns the node index.
    pub fn push_image(&mut self, src: Option<&str>, alt: Option<&str>) -> u32 {
        let parent = self.current_structural_parent();
        let label = src.unwrap_or("img");
        let id = Self::generate_id("image", label, self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::Image {
                description: alt.filter(|s| !s.is_empty()).map(str::to_string),
                src: src.map(str::to_string),
                image_index: None,
            },
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        idx
    }

    /// Record a code block.
    ///
    /// Returns the node index.
    pub fn push_code(&mut self, text: &str, language: Option<&str>) -> u32 {
        let parent = self.current_structural_parent();
        let id = Self::generate_id("code", text, self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::Code {
                text: text.to_string(),
                language: language.map(str::to_string),
            },
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        idx
    }

    /// Open a blockquote container.
    ///
    /// Returns the node index; call [`Self::push_quote_end`] to close it.
    pub fn push_quote_start(&mut self) -> u32 {
        let parent = self.current_structural_parent();
        let id = Self::generate_id("quote", "blockquote", self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::Quote,
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        self.container_stack.push(idx);
        idx
    }

    /// Close the innermost open blockquote container.
    pub fn push_quote_end(&mut self) {
        self.container_stack.pop();
    }

    /// Record a raw block (e.g. preserved `<script>` or `<style>` content).
    ///
    /// Returns the node index.
    pub fn push_raw_block(&mut self, format: &str, content: &str) -> u32 {
        let parent = self.current_structural_parent();
        let id = Self::generate_id("raw_block", format, self.nodes.len() as u32);
        let idx = self.raw_push(DocumentNode {
            id,
            content: NodeContent::RawBlock {
                format: format.to_string(),
                content: content.to_string(),
            },
            parent,
            children: Vec::new(),
            annotations: Vec::new(),
            attributes: None,
        });
        if let Some(p) = parent {
            self.add_child(p, idx);
        }
        idx
    }

    /// Consume the collector and return the completed [`DocumentStructure`] and extracted
    /// [`TableData`] entries.
    ///
    /// Returns `(DocumentStructure, Vec<TableData>)`.  The tables vec contains one entry per
    /// `<table>` element that was processed via [`Self::push_table_data`].
    pub fn finish(self) -> (DocumentStructure, Vec<TableData>) {
        let doc = DocumentStructure {
            nodes: self.nodes,
            source_format: Some("html".to_string()),
        };
        (doc, self.tables)
    }

    // ── Private helpers ──────────────────────────────────────────────────────

    /// The effective structural parent for a new node:
    /// list stack > container stack > section stack > None.
    fn current_structural_parent(&self) -> Option<u32> {
        // List items: already handled explicitly in push_list_item.
        // For non-list-item content, prefer the innermost container (blockquote),
        // then innermost section group.
        if let Some(&q) = self.container_stack.last() {
            return Some(q);
        }
        if let Some(&(_, g)) = self.section_stack.last() {
            return Some(g);
        }
        None
    }

    /// Append a node to the flat list and return its index.
    fn raw_push(&mut self, node: DocumentNode) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(node);
        idx
    }

    /// Record `child_idx` as a child of `parent_idx`.
    fn add_child(&mut self, parent_idx: u32, child_idx: u32) {
        if let Some(parent) = self.nodes.get_mut(parent_idx as usize) {
            parent.children.push(child_idx);
        }
    }

    /// Generate a deterministic node ID: `"{node_type}-{hash:016x}"`.
    ///
    /// Hashes `(node_type, text[..64], index)` with `DefaultHasher`.
    fn generate_id(node_type: &str, text: &str, index: u32) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        node_type.hash(&mut hasher);
        let end = crate::converter::utility::content::floor_char_boundary(text, text.len().min(64));
        text[..end].hash(&mut hasher);
        index.hash(&mut hasher);
        let digest = hasher.finish();
        format!("{node_type}-{digest:016x}")
    }
}

impl Default for StructureCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_creates_group_and_heading() {
        let mut c = StructureCollector::new();
        let heading_idx = c.push_heading(1, "Title", None);
        // Group is at index 0, Heading at index 1.
        assert_eq!(heading_idx, 1);
        assert_eq!(c.nodes.len(), 2);

        let group = &c.nodes[0];
        matches!(
            &group.content,
            NodeContent::Group {
                heading_level: Some(1),
                ..
            }
        );
        assert!(group.children.contains(&1));

        let heading = &c.nodes[1];
        assert!(matches!(&heading.content, NodeContent::Heading { level: 1, .. }));
        assert_eq!(heading.parent, Some(0));
    }

    #[test]
    fn test_heading_closes_deeper_groups() {
        let mut c = StructureCollector::new();
        c.push_heading(1, "H1", None);
        c.push_heading(2, "H2", None);
        // Now push another H1 — must close the H2 group.
        c.push_heading(1, "H1b", None);
        // After the second H1 there should be 2 open groups gone and 1 new one.
        assert_eq!(c.section_stack.len(), 1);
        let (level, _) = c.section_stack[0];
        assert_eq!(level, 1);
    }

    #[test]
    fn test_paragraph_parents_under_section() {
        let mut c = StructureCollector::new();
        c.push_heading(1, "Title", None);
        let p_idx = c.push_paragraph("Some text");
        let para = &c.nodes[p_idx as usize];
        // Parent should be the group node (index 0).
        assert_eq!(para.parent, Some(0));
    }

    #[test]
    fn test_list_items_attach_to_list() {
        let mut c = StructureCollector::new();
        let list_idx = c.push_list_start(false);
        let item_idx = c.push_list_item("Item 1");
        c.push_list_end();
        assert_eq!(c.nodes[item_idx as usize].parent, Some(list_idx));
        let list = &c.nodes[list_idx as usize];
        assert!(list.children.contains(&item_idx));
    }

    #[test]
    fn test_quote_container() {
        let mut c = StructureCollector::new();
        let q_idx = c.push_quote_start();
        let p_idx = c.push_paragraph("Quoted text");
        c.push_quote_end();
        assert_eq!(c.nodes[p_idx as usize].parent, Some(q_idx));
    }

    #[test]
    fn test_finish_returns_document_structure() {
        let mut c = StructureCollector::new();
        c.push_heading(1, "Title", None);
        c.push_paragraph("Text");
        let (doc, tables) = c.finish();
        assert_eq!(doc.source_format, Some("html".to_string()));
        assert_eq!(doc.nodes.len(), 3); // Group + Heading + Paragraph
        assert!(tables.is_empty());
    }
}
