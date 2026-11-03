//! A mutable DOM with source offsets, built on the `astral_html` token stream.

use std::borrow::Cow;

/// CSS selectors over a parsed document.
pub mod queryselector;
mod stream;
mod util;
pub use queryselector::Selector;

/// The index of a node in the parser's node list.
pub type InnerNodeHandle = u32;

const VOID: [&str; 15] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "meta", "param", "source", "track",
    "wbr",
];

/// A byte string that borrows from the source or owns an edited value.
#[derive(Debug, Clone, Default, Eq)]
pub struct Bytes<'a>(Cow<'a, [u8]>);

/// The error of [`Bytes::set`].
#[derive(Debug)]
pub enum SetBytesError {
    /// The value is too long to store.
    LengthOverflow,
}

impl Bytes<'_> {
    /// Returns an empty byte string.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns the bytes as text, with invalid UTF-8 replaced.
    #[must_use]
    pub fn as_utf8_str(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.0)
    }
    /// Returns the bytes as text, or `None` if they are not valid UTF-8.
    #[must_use]
    pub fn try_as_utf8_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.0).ok()
    }
    /// Returns the raw bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// Returns a pointer to the first byte.
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
    /// Replaces the bytes with an owned value.
    pub fn set<B: Into<Vec<u8>>>(&mut self, data: B) -> Result<Option<Box<[u8]>>, SetBytesError> {
        self.0 = Cow::Owned(data.into());
        Ok(None)
    }
}

impl std::hash::Hash for Bytes<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl PartialEq for Bytes<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<str> for Bytes<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}
impl PartialEq<&str> for Bytes<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}
impl PartialEq<[u8]> for Bytes<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_bytes() == other
    }
}
impl PartialEq<&[u8]> for Bytes<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.as_bytes() == *other
    }
}
impl<'a> From<&'a str> for Bytes<'a> {
    fn from(value: &'a str) -> Self {
        Self(Cow::Borrowed(value.as_bytes()))
    }
}
impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}
impl TryFrom<String> for Bytes<'static> {
    type Error = SetBytesError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Cow::Owned(value.into_bytes())))
    }
}

/// Options of [`parse`]. It holds none.
#[derive(Debug, Clone, Copy, Default)]
pub struct ParserOptions;
impl ParserOptions {
    /// Returns the default options.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// The error of [`parse`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// The input is longer than `u32::MAX` bytes.
    InvalidLength,
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("the input length exceeds the capacity of the parser")
    }
}
impl std::error::Error for ParseError {}

/// A handle to a node in a [`Parser`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeHandle(u32);
impl NodeHandle {
    /// Returns the handle of the node at this index.
    #[must_use]
    pub const fn new(node: u32) -> Self {
        Self(node)
    }
    /// Returns the node this handle points to.
    #[must_use]
    pub fn get<'p, 'buf>(&self, parser: &'p Parser<'buf>) -> Option<&'p Node<'buf>> {
        parser.tags.get(self.0 as usize)
    }
    /// Returns the node this handle points to, for an edit.
    pub fn get_mut<'p, 'buf>(&self, parser: &'p mut Parser<'buf>) -> Option<&'p mut Node<'buf>> {
        parser.tags.get_mut(self.0 as usize)
    }
    /// Returns the index of the node.
    #[must_use]
    pub const fn get_inner(&self) -> u32 {
        self.0
    }
}

/// The direct children of an element.
pub type RawChildren = smallvec::SmallVec<[NodeHandle; 4]>;

/// The attributes of an element, in source order.
#[derive(Debug, Clone, Default)]
pub struct Attributes<'a> {
    raw: Vec<(Bytes<'a>, Option<Bytes<'a>>)>,
}

impl<'a> Attributes<'a> {
    /// Returns the number of attributes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.raw.len()
    }
    /// Returns `true` if the element has no attributes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
    /// Returns the value of the first attribute with this name. The inner `None` is an attribute with no value.
    pub fn get<B: Into<Bytes<'a>>>(&self, key: B) -> Option<Option<&Bytes<'a>>> {
        let key = key.into();
        self.raw
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| value.as_ref())
    }
    /// Returns `true` if an attribute has this name.
    pub fn contains<B: Into<Bytes<'a>>>(&self, key: B) -> bool {
        self.get(key).is_some()
    }
    /// Removes the first attribute with this name and returns its value.
    pub fn remove<B: Into<Bytes<'a>>>(&mut self, key: B) -> Option<Option<Bytes<'a>>> {
        let key = key.into();
        let index = self.raw.iter().position(|(name, _)| *name == key)?;
        Some(self.raw.remove(index).1)
    }
    /// Returns each attribute as a name and an optional value, in source order.
    pub fn iter(&self) -> impl Iterator<Item = (Cow<'_, str>, Option<Cow<'_, str>>)> + '_ {
        self.raw
            .iter()
            .map(|(name, value)| (name.as_utf8_str(), value.as_ref().map(Bytes::as_utf8_str)))
    }
    /// Returns `true` if the `class` attribute holds this class name.
    pub fn is_class_member<B: AsRef<[u8]>>(&self, member: B) -> bool {
        self.class().is_some_and(|class| {
            class
                .as_utf8_str()
                .split_ascii_whitespace()
                .any(|name| name.as_bytes() == member.as_ref())
        })
    }
    /// Returns the value of the `id` attribute.
    #[must_use]
    pub fn id(&self) -> Option<&Bytes<'a>> {
        self.get("id").flatten()
    }
    /// Returns the value of the `class` attribute.
    #[must_use]
    pub fn class(&self) -> Option<&Bytes<'a>> {
        self.get("class").flatten()
    }
}

/// An element with its name, attributes, children and source bytes.
#[derive(Debug, Clone)]
pub struct HTMLTag<'a> {
    name: Bytes<'a>,
    attributes: Attributes<'a>,
    children: RawChildren,
    raw: Bytes<'a>,
    id: u32,
    subtree_end: u32,
}

impl<'a> HTMLTag<'a> {
    /// Returns the children of the element.
    #[must_use]
    pub const fn children(&self) -> Children<'a, '_> {
        Children(self)
    }
    /// Returns the children of the element, for an edit.
    pub const fn children_mut(&mut self) -> ChildrenMut<'a, '_> {
        ChildrenMut(self)
    }
    /// Returns the tag name as written in the source.
    #[must_use]
    pub const fn name(&self) -> &Bytes<'a> {
        &self.name
    }
    /// Returns the attributes of the element.
    #[must_use]
    pub const fn attributes(&self) -> &Attributes<'a> {
        &self.attributes
    }
    /// Returns the attributes of the element, for an edit.
    pub const fn attributes_mut(&mut self) -> &mut Attributes<'a> {
        &mut self.attributes
    }
    /// Returns the source bytes of the element, from its start tag to its end tag.
    #[must_use]
    pub const fn raw(&self) -> &Bytes<'a> {
        &self.raw
    }
    /// Returns the first and the last byte offset of the element in the source.
    #[must_use]
    pub fn boundaries(&self, parser: &Parser<'a>) -> (usize, usize) {
        let offset = self.raw.as_ptr() as usize - parser.source.as_ptr() as usize;
        (offset, offset + self.raw.as_bytes().len() - 1)
    }
    /// Returns the text of every descendant text node, joined.
    #[must_use]
    pub fn inner_text<'p>(&self, parser: &'p Parser<'a>) -> Cow<'p, str> {
        let mut text = String::new();
        for node in &parser.tags[self.id as usize + 1..self.subtree_end as usize] {
            if let Node::Raw(bytes) = node {
                text.push_str(&bytes.as_utf8_str());
            }
        }
        Cow::Owned(text)
    }
}

/// The children of an element.
pub struct Children<'a, 'b>(&'b HTMLTag<'a>);

impl<'a, 'b> Children<'a, 'b> {
    /// Returns the direct children.
    #[must_use]
    pub const fn top(&self) -> &'b RawChildren {
        &self.0.children
    }
    /// Returns every descendant node, in document order.
    #[must_use]
    pub fn all(&self, parser: &'b Parser<'a>) -> &'b [Node<'a>] {
        &parser.tags[self.0.id as usize + 1..self.0.subtree_end as usize]
    }
}

/// The children of an element, for an edit.
pub struct ChildrenMut<'a, 'b>(&'b mut HTMLTag<'a>);

impl ChildrenMut<'_, '_> {
    /// Returns the direct children, for an edit.
    pub const fn top_mut(&mut self) -> &mut RawChildren {
        &mut self.0.children
    }
}

/// A node of the tree.
#[derive(Debug, Clone)]
pub enum Node<'a> {
    /// An element.
    Tag(HTMLTag<'a>),
    /// A text node.
    Raw(Bytes<'a>),
    /// A comment, with its delimiters.
    Comment(Bytes<'a>),
}

impl<'a> Node<'a> {
    /// Returns the text of the node and its descendants.
    #[must_use]
    pub fn inner_text<'s, 'p: 's>(&'s self, parser: &'p Parser<'a>) -> Cow<'s, str> {
        match self {
            Node::Tag(tag) => tag.inner_text(parser),
            Node::Raw(bytes) => bytes.as_utf8_str(),
            Node::Comment(_) => Cow::Borrowed(""),
        }
    }
    /// Returns the children of an element, or `None` for another node.
    #[must_use]
    pub const fn children(&self) -> Option<Children<'a, '_>> {
        match self {
            Node::Tag(tag) => Some(tag.children()),
            _ => None,
        }
    }
    /// Returns the element, or `None` for another node.
    #[must_use]
    pub const fn as_tag(&self) -> Option<&HTMLTag<'a>> {
        match self {
            Node::Tag(tag) => Some(tag),
            _ => None,
        }
    }
    /// Returns the element for an edit, or `None` for another node.
    pub const fn as_tag_mut(&mut self) -> Option<&mut HTMLTag<'a>> {
        match self {
            Node::Tag(tag) => Some(tag),
            _ => None,
        }
    }
    /// Returns the comment, or `None` for another node.
    #[must_use]
    pub const fn as_comment(&self) -> Option<&Bytes<'a>> {
        match self {
            Node::Comment(bytes) => Some(bytes),
            _ => None,
        }
    }
    /// Returns the text, or `None` for another node.
    #[must_use]
    pub const fn as_raw(&self) -> Option<&Bytes<'a>> {
        match self {
            Node::Raw(bytes) => Some(bytes),
            _ => None,
        }
    }
    /// Returns the text for an edit, or `None` for another node.
    pub const fn as_raw_mut(&mut self) -> Option<&mut Bytes<'a>> {
        match self {
            Node::Raw(bytes) => Some(bytes),
            _ => None,
        }
    }
}

/// The node list of a parsed document.
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub(crate) tags: Vec<Node<'a>>,
    pub(crate) ast: Vec<NodeHandle>,
    source: &'a str,
}

impl<'a> Parser<'a> {
    /// Returns the node at this index.
    #[must_use]
    pub fn resolve_node_id(&self, id: u32) -> Option<&Node<'a>> {
        self.tags.get(id as usize)
    }

    fn tag_mut(&mut self, id: u32) -> &mut HTMLTag<'a> {
        match &mut self.tags[id as usize] {
            Node::Tag(tag) => tag,
            _ => unreachable!("the open stack holds tags"),
        }
    }

    fn append(&mut self, node: Node<'a>, parent: Option<u32>) -> u32 {
        let id = self.tags.len() as u32;
        self.tags.push(node);
        match parent {
            Some(parent) => self.tag_mut(parent).children.push(NodeHandle(id)),
            None => self.ast.push(NodeHandle(id)),
        }
        id
    }

    fn slice(&self, start: usize, end: usize) -> Bytes<'a> {
        Bytes(Cow::Borrowed(&self.source.as_bytes()[start..end]))
    }
}

/// A parsed document.
#[derive(Debug, Clone)]
pub struct VDom<'a> {
    parser: Parser<'a>,
}

impl<'a> VDom<'a> {
    /// Returns the node list.
    #[must_use]
    pub const fn parser(&self) -> &Parser<'a> {
        &self.parser
    }
    /// Returns the node list, for an edit.
    pub const fn parser_mut(&mut self) -> &mut Parser<'a> {
        &mut self.parser
    }
    /// Returns every node, in document order.
    #[must_use]
    pub fn nodes(&self) -> &[Node<'a>] {
        &self.parser.tags
    }
    /// Returns every node, for an edit.
    pub fn nodes_mut(&mut self) -> &mut [Node<'a>] {
        &mut self.parser.tags
    }
    /// Returns the top-level nodes.
    #[must_use]
    pub fn children(&self) -> &[NodeHandle] {
        &self.parser.ast
    }
    /// Returns the elements that match a CSS selector, or `None` if the selector does not parse.
    #[must_use]
    pub fn query_selector<'b>(
        &'b self,
        selector: &'b str,
    ) -> Option<queryselector::QuerySelectorIterator<'a, 'b, Self>> {
        let selector = queryselector::Parser::new(selector.as_bytes()).selector()?;
        Some(queryselector::QuerySelectorIterator::new(selector, &self.parser, self))
    }
}

/// The stack of open elements. An end tag closes the element on top when it spells that name and
/// closes nothing otherwise, so the walks in the converter that find an omitted or a discarded end
/// tag read this tree as they read `tl`.
#[derive(Default)]
struct Scopes {
    open: Vec<u32>,
    /// True inside a re-read body: text there stays text, so a re-read cannot re-read itself.
    rereading: bool,
}

impl Scopes {
    fn close_top(&mut self, end: u32, parser: &mut Parser<'_>) -> Option<u32> {
        let id = self.open.pop()?;
        parser.tag_mut(id).subtree_end = end;
        Some(id)
    }

    fn top_is(&self, parser: &Parser<'_>, name: &[u8]) -> bool {
        let top = self.open.last().and_then(|id| parser.tags[*id as usize].as_tag());
        top.is_some_and(|tag| tag.name.as_bytes().eq_ignore_ascii_case(name))
    }
}

fn source_name<'a>(parser: &Parser<'a>, start: usize, name: &str) -> Bytes<'a> {
    let from = start + 1;
    let spelled = parser.source.as_bytes().get(from..from + name.len());
    match spelled {
        Some(bytes) if bytes.eq_ignore_ascii_case(name.as_bytes()) => parser.slice(from, from + name.len()),
        _ => Bytes(Cow::Owned(name.as_bytes().to_vec())),
    }
}

fn attributes_of(list: Vec<astral_html::Attribute<'_>>) -> Attributes<'_> {
    let raw = list
        .into_iter()
        .map(|attribute| {
            let key = match attribute.name {
                Cow::Borrowed(name) => Bytes::from(name),
                Cow::Owned(name) => Bytes(Cow::Owned(name.into_bytes())),
            };
            (key, attribute.raw_value.map(Bytes::from))
        })
        .collect();
    Attributes { raw }
}

fn start_tag<'a>(parser: &mut Parser<'a>, scopes: &mut Scopes, tag: astral_html::Tag<'a>, start: usize, end: usize) {
    let name = tag.name.as_ref();
    let closed = tag.self_closing || VOID.contains(&name);
    let node = HTMLTag {
        name: source_name(parser, start, name),
        attributes: attributes_of(tag.attributes),
        children: RawChildren::new(),
        raw: parser.slice(start, end),
        id: parser.tags.len() as u32,
        subtree_end: parser.tags.len() as u32 + 1,
    };
    let parent = scopes.open.last().copied();
    let id = parser.append(Node::Tag(node), parent);
    if !closed {
        scopes.open.push(id);
    }
}

fn end_tag(parser: &mut Parser<'_>, scopes: &mut Scopes, start: usize, end: usize) {
    let source = parser.source.as_bytes();
    let spelled = source.get(start + 2..end.saturating_sub(1)).unwrap_or_default();
    let closes = scopes.open.last().is_some_and(|id| match &parser.tags[*id as usize] {
        Node::Tag(tag) => tag.name.as_bytes() == spelled,
        _ => false,
    });
    if !closes {
        return;
    }
    let subtree_end = parser.tags.len() as u32;
    if let Some(id) = scopes.close_top(subtree_end, parser) {
        let start = parser.tag_mut(id).raw.as_ptr() as usize - parser.source.as_ptr() as usize;
        let raw = parser.slice(start, end);
        parser.tag_mut(id).raw = raw;
    }
}

fn flush_text(parser: &mut Parser<'_>, scopes: &mut Scopes, text: &mut Option<(usize, usize)>) {
    let Some((from, to)) = text.take() else { return };
    // ~keep The reader takes these bodies as text, and the converter decides on the markup in them,
    // ~keep so the body is read again in place. An `xmp` that closes reaches this point with its
    // ~keep markup escaped. Only an `xmp` with no end tag holds markup here.
    let reread = [b"noframes".as_slice(), b"noembed", b"plaintext", b"xmp"];
    if !scopes.rereading && reread.iter().any(|name| scopes.top_is(parser, name)) {
        scopes.rereading = true;
        let mut tokens = astral_html::Tokenizer::new(&parser.source[from..to]);
        read_region(parser, scopes, &mut tokens, from);
        scopes.rereading = false;
        return;
    }
    let bytes = parser.slice(from, to);
    let parent = scopes.open.last().copied();
    parser.append(Node::Raw(bytes), parent);
}

/// The two token streams of the parse. The reader follows the raw-text rules of HTML. The
/// tokenizer stays in markup, so a re-read body holds no second raw-text body.
trait Tokens<'a> {
    fn next_token(&mut self) -> Option<astral_html::Token<'a>>;
    fn offset(&self) -> usize;
}

impl<'a> Tokens<'a> for astral_html::Reader<'a> {
    fn next_token(&mut self) -> Option<astral_html::Token<'a>> {
        self.next()
    }
    fn offset(&self) -> usize {
        self.position()
    }
}

impl<'a> Tokens<'a> for astral_html::Tokenizer<'a> {
    fn next_token(&mut self) -> Option<astral_html::Token<'a>> {
        self.next()
    }
    fn offset(&self) -> usize {
        self.position()
    }
}

fn read_region<'a>(parser: &mut Parser<'a>, scopes: &mut Scopes, tokens: &mut impl Tokens<'a>, base: usize) {
    let input = parser.source;
    let mut start = base;
    let mut text: Option<(usize, usize)> = None;
    while let Some(token) = tokens.next_token() {
        let end = base + tokens.offset();
        if matches!(token, astral_html::Token::Text(_)) {
            text = Some((text.map_or(start, |(from, _)| from), end));
            start = end;
            continue;
        }
        flush_text(parser, scopes, &mut text);
        match token {
            astral_html::Token::StartTag(tag) => start_tag(parser, scopes, tag, start, end),
            astral_html::Token::EndTag(_) => end_tag(parser, scopes, start, end),
            astral_html::Token::Comment(_) if input.as_bytes()[start..].starts_with(b"<!--") => {
                let bytes = parser.slice(start, end);
                let parent = scopes.open.last().copied();
                parser.append(Node::Comment(bytes), parent);
            }
            _ => {}
        }
        start = end;
    }
    flush_text(parser, scopes, &mut text);
}

/// Parses an HTML document into a tree.
pub fn parse(input: &str, _options: ParserOptions) -> Result<VDom<'_>, ParseError> {
    let Ok(_) = u32::try_from(input.len()) else {
        return Err(ParseError::InvalidLength);
    };
    let mut parser = Parser {
        tags: Vec::new(),
        ast: Vec::new(),
        source: input,
    };
    let mut scopes = Scopes::default();
    read_region(&mut parser, &mut scopes, &mut astral_html::Reader::new(input), 0);
    let subtree_end = parser.tags.len() as u32;
    while scopes.close_top(subtree_end, &mut parser).is_some() {}
    Ok(VDom { parser })
}
