//! Structured document tree types aligned with kreuzberg's `DocumentStructure`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::tables::TableGrid;

/// A structured document tree representing the semantic content of an HTML document.
///
/// Uses a flat node array with index-based parent/child references for efficient traversal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentStructure {
    /// All nodes in document reading order.
    pub nodes: Vec<DocumentNode>,
    /// The source format (always "html" for this crate).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_format: Option<String>,
}

/// A single node in the document tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentNode {
    /// Deterministic node identifier.
    pub id: String,
    /// The semantic content of this node.
    pub content: NodeContent,
    /// Index of the parent node (None for root nodes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u32>,
    /// Indices of child nodes in reading order.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<u32>,
    /// Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub annotations: Vec<TextAnnotation>,
    /// Format-specific attributes (e.g. class, id, data-* attributes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,
}

/// The semantic content type of a document node.
///
/// Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "node_type", rename_all = "snake_case")]
pub enum NodeContent {
    /// A heading element (h1-h6).
    Heading {
        /// Heading level (1-6).
        level: u8,
        /// The heading text content.
        text: String,
    },
    /// A paragraph of text.
    Paragraph {
        /// The paragraph text content.
        text: String,
    },
    /// A list container (ordered or unordered). Children are `ListItem` nodes.
    List {
        /// Whether this is an ordered list.
        ordered: bool,
    },
    /// A single list item.
    ListItem {
        /// The list item text content.
        text: String,
    },
    /// A table with structured cell data.
    Table {
        /// The table grid structure.
        grid: TableGrid,
    },
    /// An image element.
    Image {
        /// Alt text or caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Image source URL.
        #[serde(skip_serializing_if = "Option::is_none")]
        src: Option<String>,
        /// Index into `ConversionResult.images` when image extraction is enabled.
        #[serde(skip_serializing_if = "Option::is_none")]
        image_index: Option<u32>,
    },
    /// A code block or inline code.
    Code {
        /// The code text content.
        text: String,
        /// Programming language (from class="language-*" or similar).
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,
    },
    /// A block quote container.
    Quote,
    /// A definition list container.
    DefinitionList,
    /// A definition list entry with term and description.
    DefinitionItem {
        /// The term being defined.
        term: String,
        /// The definition text.
        definition: String,
    },
    /// A raw block preserved as-is (e.g. `<script>`, `<style>` content).
    RawBlock {
        /// The format of the raw content (e.g. "html", "css", "javascript").
        format: String,
        /// The raw content.
        content: String,
    },
    /// A block of key-value metadata pairs (from `<head>` meta tags).
    MetadataBlock {
        /// Key-value metadata pairs.
        entries: Vec<(String, String)>,
    },
    /// A section grouping container (auto-generated from heading hierarchy).
    Group {
        /// Optional section label.
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        /// The heading level that created this group.
        #[serde(skip_serializing_if = "Option::is_none")]
        heading_level: Option<u8>,
        /// The heading text that created this group.
        #[serde(skip_serializing_if = "Option::is_none")]
        heading_text: Option<String>,
    },
}

/// An inline text annotation with byte-range offsets.
///
/// Annotations describe formatting (bold, italic, etc.) and links within a node's text content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAnnotation {
    /// Start byte offset (inclusive) into the parent node's text.
    pub start: u32,
    /// End byte offset (exclusive) into the parent node's text.
    pub end: u32,
    /// The type of annotation.
    pub kind: AnnotationKind,
}

/// The type of an inline text annotation.
///
/// Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "annotation_type", rename_all = "snake_case")]
#[derive(Default)]
pub enum AnnotationKind {
    /// Bold / strong emphasis.
    #[default]
    Bold,
    /// Italic / emphasis.
    Italic,
    /// Underline.
    Underline,
    /// Strikethrough / deleted text.
    Strikethrough,
    /// Inline code.
    Code,
    /// Subscript text.
    Subscript,
    /// Superscript text.
    Superscript,
    /// Highlighted / marked text.
    Highlight,
    /// A hyperlink.
    Link {
        /// The link URL.
        url: String,
        /// Optional link title attribute.
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
}

impl Default for NodeContent {
    fn default() -> Self {
        Self::Heading {
            level: 1,
            text: String::new(),
        }
    }
}
