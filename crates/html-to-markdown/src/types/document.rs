//! Structured document tree types aligned with kreuzberg's `DocumentStructure`.

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::tables::TableGrid;

/// A structured document tree representing the semantic content of an HTML document.
///
/// Uses a flat node array with index-based parent/child references for efficient traversal.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DocumentStructure {
    /// All nodes in document reading order.
    pub nodes: Vec<DocumentNode>,
    /// The source format (always "html" for this crate).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub source_format: Option<String>,
}

/// A single node in the document tree.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DocumentNode {
    /// Deterministic node identifier.
    pub id: String,
    /// The semantic content of this node.
    pub content: NodeContent,
    /// Index of the parent node (None for root nodes).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub parent: Option<u32>,
    /// Indices of child nodes in reading order.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub children: Vec<u32>,
    /// Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub annotations: Vec<TextAnnotation>,
    /// Format-specific attributes preserved from the source HTML element.
    ///
    /// Keys are lowercased attribute names as they appear in the HTML (e.g. `"class"`, `"id"`,
    /// `"data-foo"`). Values are the raw attribute strings, copied verbatim from the source —
    /// no HTML entity decoding is applied here.
    ///
    /// The map is `None` when no attributes are present (omitted entirely in serialized output).
    /// Not every HTML attribute is preserved: only attributes that carry semantic or structural
    /// significance for the node type are collected. For example, heading nodes capture the `"id"`
    /// attribute for anchor linking; other element-level attributes may be silently dropped.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub attributes: Option<HashMap<String, String>>,
}

/// The semantic content type of a document node.
///
/// Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "node_type", rename_all = "snake_case"))]
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
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        description: Option<String>,
        /// Image source URL.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        src: Option<String>,
        /// Index into `ConversionResult.images` when image extraction is enabled.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        image_index: Option<u32>,
    },
    /// A code block or inline code.
    Code {
        /// The code text content.
        text: String,
        /// Programming language (from class="language-*" or similar).
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        label: Option<String>,
        /// The heading level that created this group.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        heading_level: Option<u8>,
        /// The heading text that created this group.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        heading_text: Option<String>,
    },
}

/// A styling or semantic annotation that applies to a byte range within a node's text.
///
/// Unlike [`DocumentNode`], which captures block-level structure (headings, paragraphs, etc.),
/// a `TextAnnotation` describes inline-level markup — bold, italic, links, code spans, and
/// similar — that spans a contiguous run of bytes inside `DocumentNode::content`'s text field.
///
/// Byte offsets (`start`..`end`) are into the UTF-8 encoded text of the parent node. The range
/// follows Rust slice conventions: `start` is inclusive and `end` is exclusive, so the annotated
/// text is `text[start as usize..end as usize]`.
///
/// Multiple annotations on the same node can overlap (e.g. bold-italic text), and they are
/// stored in the order they are encountered during DOM traversal.
///
/// See [`AnnotationKind`] for the full list of supported annotation types.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "annotation_type", rename_all = "snake_case"))]
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
    /// A hyperlink sourced from an `<a href="...">` element.
    Link {
        /// The URL from the `href` attribute, copied verbatim from the source HTML.
        ///
        /// No URL decoding or normalization is performed: percent-encoded sequences, relative
        /// paths, and protocol-relative URLs (`//example.com`) are all preserved exactly as
        /// written in the source. Callers that need an absolute URL must resolve it against the
        /// document base URL themselves.
        url: String,
        /// The `title` attribute of the `<a>` element, if present.
        ///
        /// `None` when the `<a>` tag has no `title="..."` attribute. When present, the value
        /// is copied verbatim — HTML entities within the title are not decoded.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
