//! Type definitions for the visitor pattern.
//!
//! This module contains the core data types used in the visitor pattern:
//! - `NodeType`: Categorization of HTML elements
//! - `NodeContext`: Metadata about the current node being visited
//! - `VisitResult`: Control flow signals from visitor methods

use std::borrow::Cow;
use std::cell::OnceCell;
use std::collections::BTreeMap;

/// Shared empty attribute map used by visitor call sites that have no attributes.
///
/// Allows constructing a [`NodeContext`] with borrowed empty attributes
/// instead of allocating a fresh `BTreeMap` per dispatch.
pub static EMPTY_ATTRS: BTreeMap<String, String> = BTreeMap::new();

/// Node type enumeration covering all HTML element types.
///
/// This enum categorizes all HTML elements that the converter recognizes,
/// providing a coarse-grained classification for visitor dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeType {
    /// Text node (most frequent - 100+ per document)
    Text,

    /// Generic element node
    Element,

    /// Heading elements (h1-h6)
    Heading,
    /// Paragraph element
    Paragraph,
    /// Generic div container
    Div,
    /// Blockquote element
    Blockquote,
    /// Preformatted text block
    Pre,
    /// Horizontal rule
    Hr,

    /// Ordered or unordered list (ul, ol)
    List,
    /// List item (li)
    ListItem,
    /// Definition list (dl)
    DefinitionList,
    /// Definition term (dt)
    DefinitionTerm,
    /// Definition description (dd)
    DefinitionDescription,

    /// Table element
    Table,
    /// Table row (tr)
    TableRow,
    /// Table cell (td, th)
    TableCell,
    /// Table header cell (th)
    TableHeader,
    /// Table body (tbody)
    TableBody,
    /// Table head (thead)
    TableHead,
    /// Table foot (tfoot)
    TableFoot,

    /// Anchor link (a)
    Link,
    /// Image (img)
    Image,
    /// Strong/bold (strong, b)
    Strong,
    /// Emphasis/italic (em, i)
    Em,
    /// Inline code (code)
    Code,
    /// Strikethrough (s, del, strike)
    Strikethrough,
    /// Underline (u, ins)
    Underline,
    /// Subscript (sub)
    Subscript,
    /// Superscript (sup)
    Superscript,
    /// Mark/highlight (mark)
    Mark,
    /// Small text (small)
    Small,
    /// Line break (br)
    Br,
    /// Span element
    Span,

    /// Article element
    Article,
    /// Section element
    Section,
    /// Navigation element
    Nav,
    /// Aside element
    Aside,
    /// Header element
    Header,
    /// Footer element
    Footer,
    /// Main element
    Main,
    /// Figure element
    Figure,
    /// Figure caption
    Figcaption,
    /// Time element
    Time,
    /// Details element
    Details,
    /// Summary element
    Summary,

    /// Form element
    Form,
    /// Input element
    Input,
    /// Select element
    Select,
    /// Option element
    Option,
    /// Button element
    Button,
    /// Textarea element
    Textarea,
    /// Label element
    Label,
    /// Fieldset element
    Fieldset,
    /// Legend element
    Legend,

    /// Audio element
    Audio,
    /// Video element
    Video,
    /// Picture element
    Picture,
    /// Source element
    Source,
    /// Iframe element
    Iframe,
    /// SVG element
    Svg,
    /// Canvas element
    Canvas,

    /// Ruby annotation
    Ruby,
    /// Ruby text
    Rt,
    /// Ruby parenthesis
    Rp,
    /// Abbreviation
    Abbr,
    /// Keyboard input
    Kbd,
    /// Sample output
    Samp,
    /// Variable
    Var,
    /// Citation
    Cite,
    /// Quote
    Q,
    /// Deleted text
    Del,
    /// Inserted text
    Ins,
    /// Data element
    Data,
    /// Meter element
    Meter,
    /// Progress element
    Progress,
    /// Output element
    Output,
    /// Template element
    Template,
    /// Slot element
    Slot,

    /// HTML root element
    Html,
    /// Head element
    Head,
    /// Body element
    Body,
    /// Title element
    Title,
    /// Meta element
    Meta,
    /// Link element (not anchor)
    LinkTag,
    /// Style element
    Style,
    /// Script element
    Script,
    /// Base element
    Base,

    /// Custom element (web components) or unknown tag
    Custom,
}

/// Lazy-evaluated attribute source for a [`NodeContext`].
///
/// Attributes are only materialized from the DOM when [`NodeContext::attributes`]
/// is first called. If the visitor never reads attributes, the `BTreeMap`
/// allocation is skipped entirely.
///
/// This type is opaque — callers interact with it only through
/// [`NodeContext::attributes`]. The only producer of the `Lazy` variant is
/// [`NodeContext::with_lazy_attributes`], which is `pub(crate)`.
pub struct AttributesSource<'a> {
    inner: AttributesInner<'a>,
}

enum AttributesInner<'a> {
    /// Borrowed reference to an existing map (no allocation on access).
    Borrowed(&'a BTreeMap<String, String>),
    /// Owned map (returned by reference on access).
    Owned(BTreeMap<String, String>),
    /// Lazy: materialize from the tl tag on first access.
    ///
    /// `tl` does not appear in the public signature of `AttributesSource` or
    /// `NodeContext`; this variant is only reachable via `with_lazy_attributes`
    /// which is `pub(crate)`.
    Lazy {
        tag: &'a tl::HTMLTag<'a>,
        cell: OnceCell<BTreeMap<String, String>>,
    },
}

impl<'a> AttributesSource<'a> {
    /// Construct from a borrowed map reference.
    pub(crate) fn borrowed(map: &'a BTreeMap<String, String>) -> Self {
        Self {
            inner: AttributesInner::Borrowed(map),
        }
    }

    /// Construct from an owned map.
    pub(crate) fn owned(map: BTreeMap<String, String>) -> Self {
        Self {
            inner: AttributesInner::Owned(map),
        }
    }

    /// Construct the lazy variant backed by a `tl::HTMLTag`.
    ///
    /// Only callable within the `html-to-markdown-rs` crate.
    pub(crate) fn lazy(tag: &'a tl::HTMLTag<'a>) -> Self {
        Self {
            inner: AttributesInner::Lazy {
                tag,
                cell: OnceCell::new(),
            },
        }
    }

    /// Return a reference to the attribute map, materializing it if needed.
    pub fn get(&self) -> &BTreeMap<String, String> {
        match &self.inner {
            AttributesInner::Borrowed(map) => map,
            AttributesInner::Owned(map) => map,
            AttributesInner::Lazy { tag, cell } => cell.get_or_init(|| {
                tag.attributes()
                    .iter()
                    .filter_map(|(k, v)| v.as_ref().map(|val| (k.to_string(), val.to_string())))
                    .collect()
            }),
        }
    }
}

impl Clone for AttributesSource<'_> {
    fn clone(&self) -> Self {
        // Materialize before cloning so the clone is self-contained (owned).
        Self {
            inner: AttributesInner::Owned(self.get().clone()),
        }
    }
}

impl std::fmt::Debug for AttributesSource<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.get()).finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AttributesSource<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AttributesSource<'static> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map = BTreeMap::<String, String>::deserialize(deserializer)?;
        Ok(Self::owned(map))
    }
}

/// Context information passed to all visitor methods.
///
/// Provides comprehensive metadata about the current node being visited,
/// including its type, tag name, position in the DOM tree, and parent context.
///
/// ## Attributes
///
/// Access attributes via [`NodeContext::attributes`], which returns
/// `&BTreeMap<String, String>`. When the context was built with
/// [`NodeContext::with_lazy_attributes`] (the hot path inside the converter),
/// the map is only materialized on the first call — if the visitor never reads
/// attributes, the allocation is skipped.
///
/// ## Lifetimes
///
/// String fields use [`Cow<'_, str>`] so the converter can pass slices directly
/// out of the parsed DOM without allocating. Visitor implementations that need
/// to outlive the callback should call [`NodeContext::into_owned`].
#[derive(Debug, Clone)]
pub struct NodeContext<'a> {
    /// Coarse-grained node type classification
    pub node_type: NodeType,

    /// Raw HTML tag name (e.g., "div", "h1", "custom-element")
    pub tag_name: Cow<'a, str>,

    /// Lazily-materialized HTML attributes. Access via [`NodeContext::attributes`].
    attributes: AttributesSource<'a>,

    /// Depth in the DOM tree (0 = root)
    pub depth: usize,

    /// Index among siblings (0-based)
    pub index_in_parent: usize,

    /// Parent element's tag name (None if root)
    pub parent_tag: Option<Cow<'a, str>>,

    /// Whether this element is treated as inline vs block
    pub is_inline: bool,
}

#[cfg(feature = "serde")]
impl serde::Serialize for NodeContext<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("NodeContext", 7)?;
        s.serialize_field("node_type", &self.node_type)?;
        s.serialize_field("tag_name", &self.tag_name)?;
        s.serialize_field("attributes", &self.attributes)?;
        s.serialize_field("depth", &self.depth)?;
        s.serialize_field("index_in_parent", &self.index_in_parent)?;
        s.serialize_field("parent_tag", &self.parent_tag)?;
        s.serialize_field("is_inline", &self.is_inline)?;
        s.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for NodeContext<'static> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};

        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            NodeType,
            TagName,
            Attributes,
            Depth,
            IndexInParent,
            ParentTag,
            IsInline,
        }

        struct NodeContextVisitor;

        impl<'de> Visitor<'de> for NodeContextVisitor {
            type Value = NodeContext<'static>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct NodeContext")
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<NodeContext<'static>, V::Error> {
                let mut node_type = None;
                let mut tag_name: Option<String> = None;
                let mut attributes: Option<BTreeMap<String, String>> = None;
                let mut depth = None;
                let mut index_in_parent = None;
                let mut parent_tag: Option<Option<String>> = None;
                let mut is_inline = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::NodeType => node_type = Some(map.next_value()?),
                        Field::TagName => tag_name = Some(map.next_value()?),
                        Field::Attributes => attributes = Some(map.next_value()?),
                        Field::Depth => depth = Some(map.next_value()?),
                        Field::IndexInParent => index_in_parent = Some(map.next_value()?),
                        Field::ParentTag => parent_tag = Some(map.next_value()?),
                        Field::IsInline => is_inline = Some(map.next_value()?),
                    }
                }

                Ok(NodeContext {
                    node_type: node_type.ok_or_else(|| de::Error::missing_field("node_type"))?,
                    tag_name: Cow::Owned(tag_name.ok_or_else(|| de::Error::missing_field("tag_name"))?),
                    attributes: AttributesSource::owned(
                        attributes.ok_or_else(|| de::Error::missing_field("attributes"))?,
                    ),
                    depth: depth.ok_or_else(|| de::Error::missing_field("depth"))?,
                    index_in_parent: index_in_parent.ok_or_else(|| de::Error::missing_field("index_in_parent"))?,
                    parent_tag: parent_tag
                        .ok_or_else(|| de::Error::missing_field("parent_tag"))?
                        .map(Cow::Owned),
                    is_inline: is_inline.ok_or_else(|| de::Error::missing_field("is_inline"))?,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "node_type",
            "tag_name",
            "attributes",
            "depth",
            "index_in_parent",
            "parent_tag",
            "is_inline",
        ];
        deserializer.deserialize_struct("NodeContext", FIELDS, NodeContextVisitor)
    }
}

impl<'a> NodeContext<'a> {
    /// Return a reference to the attribute map.
    ///
    /// If the context was built with [`NodeContext::with_lazy_attributes`], the
    /// map is materialized on the first call and cached for subsequent calls.
    /// If this method is never called, no allocation occurs for attributes.
    #[inline]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        self.attributes.get()
    }

    /// Construct a `NodeContext` with a borrowed attribute map.
    ///
    /// Use this when you already have a `&BTreeMap` that outlives `'a`,
    /// such as [`EMPTY_ATTRS`] for text nodes or a locally-cached map.
    pub fn with_borrowed_attributes(
        node_type: NodeType,
        tag_name: Cow<'a, str>,
        attributes: &'a BTreeMap<String, String>,
        depth: usize,
        index_in_parent: usize,
        parent_tag: Option<Cow<'a, str>>,
        is_inline: bool,
    ) -> Self {
        Self {
            node_type,
            tag_name,
            attributes: AttributesSource::borrowed(attributes),
            depth,
            index_in_parent,
            parent_tag,
            is_inline,
        }
    }

    /// Construct a `NodeContext` with an owned attribute map.
    ///
    /// Prefer [`NodeContext::with_lazy_attributes`] (pub(crate)) inside the
    /// converter to avoid the eager `collect_tag_attributes` allocation.
    pub fn with_owned_attributes(
        node_type: NodeType,
        tag_name: Cow<'a, str>,
        attributes: BTreeMap<String, String>,
        depth: usize,
        index_in_parent: usize,
        parent_tag: Option<Cow<'a, str>>,
        is_inline: bool,
    ) -> Self {
        Self {
            node_type,
            tag_name,
            attributes: AttributesSource::owned(attributes),
            depth,
            index_in_parent,
            parent_tag,
            is_inline,
        }
    }

    /// Construct a `NodeContext` with lazy attribute evaluation backed by a
    /// `tl::HTMLTag`.
    ///
    /// Attributes are only materialized (via `collect_tag_attributes`) if
    /// [`NodeContext::attributes`] is actually called. This eliminates the
    /// per-element `BTreeMap` allocation on the hot path when visitors do not
    /// read attributes.
    ///
    /// This constructor is `pub(crate)` to keep `tl` out of the public API.
    #[cfg(feature = "visitor")]
    pub(crate) fn with_lazy_attributes(
        node_type: NodeType,
        tag_name: Cow<'a, str>,
        tag: &'a tl::HTMLTag<'a>,
        depth: usize,
        index_in_parent: usize,
        parent_tag: Option<Cow<'a, str>>,
        is_inline: bool,
    ) -> Self {
        Self {
            node_type,
            tag_name,
            attributes: AttributesSource::lazy(tag),
            depth,
            index_in_parent,
            parent_tag,
            is_inline,
        }
    }

    /// Promote any borrowed fields into owned storage so the context can outlive `'a`.
    #[must_use]
    pub fn into_owned(self) -> NodeContext<'static> {
        NodeContext {
            node_type: self.node_type,
            tag_name: Cow::Owned(self.tag_name.into_owned()),
            attributes: AttributesSource::owned(self.attributes.get().clone()),
            depth: self.depth,
            index_in_parent: self.index_in_parent,
            parent_tag: self.parent_tag.map(|p| Cow::Owned(p.into_owned())),
            is_inline: self.is_inline,
        }
    }
}

/// Result of a visitor callback.
///
/// Allows visitors to control the conversion flow by either proceeding
/// with default behavior, providing custom output, skipping elements,
/// preserving HTML, or signaling errors.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VisitResult {
    #[default]
    /// Continue with default conversion behavior
    Continue,

    /// Replace default output with custom markdown
    ///
    /// The visitor takes full responsibility for the markdown output
    /// of this node and its children.
    Custom(String),

    /// Skip this element entirely (don't output anything)
    ///
    /// The element and all its children are ignored in the output.
    Skip,

    /// Preserve original HTML (don't convert to markdown)
    ///
    /// The element's raw HTML is included verbatim in the output.
    PreserveHtml,

    /// Stop conversion with an error
    ///
    /// The conversion process halts and returns this error message.
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_type_equality() {
        assert_eq!(NodeType::Text, NodeType::Text);
        assert_ne!(NodeType::Text, NodeType::Element);
        assert_eq!(NodeType::Heading, NodeType::Heading);
    }

    #[test]
    fn test_node_context_with_borrowed_attributes() {
        let attrs = BTreeMap::new();
        let ctx = NodeContext::with_borrowed_attributes(
            NodeType::Heading,
            Cow::Borrowed("h1"),
            &attrs,
            1,
            0,
            Some(Cow::Borrowed("body")),
            false,
        );

        assert_eq!(ctx.node_type, NodeType::Heading);
        assert_eq!(ctx.tag_name, "h1");
        assert_eq!(ctx.depth, 1);
        assert!(!ctx.is_inline);
        assert!(ctx.attributes().is_empty());
    }

    #[test]
    fn test_node_context_with_owned_attributes() {
        let mut attrs = BTreeMap::new();
        attrs.insert("class".to_string(), "hero".to_string());
        let ctx = NodeContext::with_owned_attributes(NodeType::Div, Cow::Borrowed("div"), attrs, 0, 0, None, false);

        assert_eq!(ctx.attributes().get("class").map(String::as_str), Some("hero"));
    }

    #[test]
    fn test_node_context_attributes_empty() {
        let ctx =
            NodeContext::with_borrowed_attributes(NodeType::Text, Cow::Borrowed(""), &EMPTY_ATTRS, 0, 0, None, true);
        assert!(ctx.attributes().is_empty());
    }

    #[test]
    fn test_node_context_into_owned() {
        let mut attrs = BTreeMap::new();
        attrs.insert("id".to_string(), "main".to_string());
        let ctx = NodeContext::with_owned_attributes(
            NodeType::Div,
            Cow::Borrowed("div"),
            attrs,
            1,
            0,
            Some(Cow::Borrowed("body")),
            false,
        );
        let owned = ctx.into_owned();
        assert_eq!(owned.tag_name, "div");
        assert_eq!(owned.attributes().get("id").map(String::as_str), Some("main"));
        assert_eq!(owned.parent_tag, Some(Cow::Borrowed("body")));
    }

    #[test]
    fn test_node_context_clone() {
        let mut attrs = BTreeMap::new();
        attrs.insert("href".to_string(), "https://example.com".to_string());
        let ctx = NodeContext::with_owned_attributes(
            NodeType::Link,
            Cow::Borrowed("a"),
            attrs,
            2,
            1,
            Some(Cow::Borrowed("p")),
            true,
        );
        let cloned = ctx.clone();
        assert_eq!(
            cloned.attributes().get("href").map(String::as_str),
            Some("https://example.com")
        );
    }

    #[test]
    fn test_visit_result_variants() {
        let continue_result = VisitResult::Continue;
        matches!(continue_result, VisitResult::Continue);

        let custom_result = VisitResult::Custom("# Custom Output".to_string());
        if let VisitResult::Custom(output) = custom_result {
            assert_eq!(output, "# Custom Output");
        }

        let skip_result = VisitResult::Skip;
        matches!(skip_result, VisitResult::Skip);

        let preserve_result = VisitResult::PreserveHtml;
        matches!(preserve_result, VisitResult::PreserveHtml);

        let error_result = VisitResult::Error("Test error".to_string());
        if let VisitResult::Error(msg) = error_result {
            assert_eq!(msg, "Test error");
        }
    }
}
