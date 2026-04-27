---
title: "Java API Reference"
---

## Java API Reference <span class="version-badge">v3.4.0-rc.3</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```java
public static ConversionResult convert(String html, ConversionOptions options, String visitor) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The HTML string to convert |
| `options` | `Optional<ConversionOptions>` | No | Optional conversion options (defaults to `default options`) |
| `visitor` | `Optional<String>` | No | The visitor |

**Returns:** `ConversionResult`

**Errors:** Throws `ErrorException`.


---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `headingStyle` | `HeadingStyle` | `HeadingStyle.ATX` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `listIndentType` | `ListIndentType` | `ListIndentType.SPACES` | How to indent nested list items (spaces or tab). |
| `listIndentWidth` | `long` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `String` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strongEmSymbol` | `String` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escapeAsterisks` | `boolean` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escapeUnderscores` | `boolean` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escapeMisc` | `boolean` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escapeAscii` | `boolean` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `codeLanguage` | `String` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `boolean` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `defaultTitle` | `boolean` | `false` | Emit a default title when no `<title>` tag is present. |
| `brInTables` | `boolean` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlightStyle` | `HighlightStyle` | `HighlightStyle.DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extractMetadata` | `boolean` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespaceMode` | `WhitespaceMode` | `WhitespaceMode.NORMALIZED` | Controls how whitespace is normalised during conversion. |
| `stripNewlines` | `boolean` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `boolean` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrapWidth` | `long` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convertAsInline` | `boolean` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `subSymbol` | `String` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `supSymbol` | `String` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newlineStyle` | `NewlineStyle` | `NewlineStyle.SPACES` | How to encode hard line breaks (`<br>`) in Markdown. |
| `codeBlockStyle` | `CodeBlockStyle` | `CodeBlockStyle.BACKTICKS` | Style used for fenced code blocks (backticks or tilde). |
| `keepInlineImagesIn` | `List<String>` | `Collections.emptyList()` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `String` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `boolean` | `false` | Emit debug information during conversion. |
| `stripTags` | `List<String>` | `Collections.emptyList()` | HTML tag names whose content is stripped from the output entirely. |
| `preserveTags` | `List<String>` | `Collections.emptyList()` | HTML tag names that are preserved verbatim in the output. |
| `skipImages` | `boolean` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `linkStyle` | `LinkStyle` | `LinkStyle.INLINE` | Link rendering style (inline or reference). |
| `outputFormat` | `OutputFormat` | `OutputFormat.MARKDOWN` | Target output format (Markdown, plain text, etc.). |
| `includeDocumentStructure` | `boolean` | `false` | Include structured document tree in result. |
| `extractImages` | `boolean` | `false` | Extract inline images from data URIs and SVGs. |
| `maxImageSize` | `long` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `captureSvg` | `boolean` | `false` | Capture SVG elements as images. |
| `inferDimensions` | `boolean` | `true` | Infer image dimensions from data. |
| `maxDepth` | `Optional<Long>` | `null` | Maximum DOM traversal depth. `null` means unlimited. When set, subtrees beyond this depth are silently truncated. |
| `excludeSelectors` | `List<String>` | `Collections.emptyList()` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]` |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static ConversionOptions defaultOptions()
```

###### builder()

Create a new builder with default values.

**Signature:**

```java
public static ConversionOptionsBuilder builder()
```

###### applyUpdate()

Apply a partial update to these conversion options.

**Signature:**

```java
public void applyUpdate(ConversionOptionsUpdate update)
```

###### fromUpdate()

Create from a partial update, applying to defaults.

**Signature:**

```java
public static ConversionOptions fromUpdate(ConversionOptionsUpdate update)
```

###### from()

**Signature:**

```java
public static ConversionOptions from(ConversionOptionsUpdate update)
```


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Optional<String>` | `null` | Converted text output (markdown, djot, or plain text). `null` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `Optional<DocumentStructure>` | `null` | Structured document tree with semantic elements. Populated when `include_document_structure` is `true` in options. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `List<TableData>` | `Collections.emptyList()` | Extracted tables with structured cell data and markdown representation. |
| `images` | `List<String>` | `Collections.emptyList()` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `true` in options. |
| `warnings` | `List<ProcessingWarning>` | `Collections.emptyList()` | Non-fatal processing warnings. |


---

#### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### stripTags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```java
public ConversionOptionsBuilder stripTags(List<String> tags)
```

###### preserveTags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```java
public ConversionOptionsBuilder preserveTags(List<String> tags)
```

###### keepInlineImagesIn()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```java
public ConversionOptionsBuilder keepInlineImagesIn(List<String> tags)
```

###### excludeSelectors()

Set the list of CSS selectors for elements to exclude entirely from output.

**Signature:**

```java
public ConversionOptionsBuilder excludeSelectors(List<String> selectors)
```

###### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```java
public ConversionOptionsBuilder preprocessing(PreprocessingOptions preprocessing)
```

###### build()

Build the final `ConversionOptions`.

**Signature:**

```java
public ConversionOptions build()
```


---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `Optional<String>` | `null` | Document title from `<title>` tag |
| `description` | `Optional<String>` | `null` | Document description from `<meta name="description">` tag |
| `keywords` | `List<String>` | `Collections.emptyList()` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `Optional<String>` | `null` | Document author from `<meta name="author">` tag |
| `canonicalUrl` | `Optional<String>` | `null` | Canonical URL from `<link rel="canonical">` tag |
| `baseHref` | `Optional<String>` | `null` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `Optional<String>` | `null` | Document language from `lang` attribute |
| `textDirection` | `Optional<TextDirection>` | `null` | Document text direction from `dir` attribute |
| `openGraph` | `Map<String, String>` | `Collections.emptyMap()` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitterCard` | `Map<String, String>` | `Collections.emptyMap()` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `metaTags` | `Map<String, String>` | `Collections.emptyMap()` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `Optional<Integer>` | `null` | Index of the parent node (None for root nodes). |
| `children` | `List<Integer>` | — | Indices of child nodes in reading order. |
| `annotations` | `List<TextAnnotation>` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `Optional<Map<String, String>>` | `null` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `List<DocumentNode>` | — | All nodes in document reading order. |
| `sourceFormat` | `Optional<String>` | `null` | The source format (always "html" for this library). |


---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The text content of the cell. |
| `row` | `int` | — | 0-indexed row position. |
| `col` | `int` | — | 0-indexed column position. |
| `rowSpan` | `int` | — | Number of rows this cell spans (default 1). |
| `colSpan` | `int` | — | Number of columns this cell spans (default 1). |
| `isHeader` | `boolean` | — | Whether this is a header cell (`<th>`). |


---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `byte` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `String` | — | Normalized text content of the header |
| `id` | `Optional<String>` | `null` | HTML id attribute if present |
| `depth` | `long` | — | Document tree depth at the header element |
| `htmlOffset` | `long` | — | Byte offset in original HTML document |

##### Methods

###### isValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```java
public boolean isValid()
```


---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `List<HeaderMetadata>` | `Collections.emptyList()` | Extracted header elements with hierarchy |
| `links` | `List<LinkMetadata>` | `Collections.emptyList()` | Extracted hyperlinks with type classification |
| `images` | `List<ImageMetadata>` | `Collections.emptyList()` | Extracted images with source and dimensions |
| `structuredData` | `List<StructuredData>` | `Collections.emptyList()` | Extracted structured data blocks |


---

#### HtmlVisitor

Visitor trait for HTML→Markdown conversion.

Implement this trait to customize the conversion behavior for any HTML element type.
All methods have default implementations that return `VisitResult.Continue`, allowing
selective override of only the elements you care about.

## Method Naming Convention

- `visit_*_start`: Called before entering an element (pre-order traversal)
- `visit_*_end`: Called after exiting an element (post-order traversal)
- `visit_*`: Called for specific element types (e.g., `visit_link`, `visit_image`)

## Execution Order

For a typical element like `<div><p>text</p></div>`:

1. `visit_element_start` for `<div>`
2. `visit_element_start` for `<p>`
3. `visit_text` for "text"
4. `visit_element_end` for `<p>`
5. `visit_element_end` for `</div>`

## Performance Notes

- `visit_text` is the most frequently called method (~100+ times per document)
- Return `VisitResult.Continue` quickly for elements you don't need to customize
- Avoid heavy computation in visitor methods; consider caching if needed

### Methods

#### visitElementStart()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```java
public VisitResult visitElementStart(NodeContext ctx)
```

##### visitElementEnd()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```java
public VisitResult visitElementEnd(NodeContext ctx, String output)
```

###### visitText()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```java
public VisitResult visitText(NodeContext ctx, String text)
```

###### visitLink()

Visit anchor links `<a href="...">`.

**Signature:**

```java
public VisitResult visitLink(NodeContext ctx, String href, String text, String title)
```

###### visitImage()

Visit images `<img src="...">`.

**Signature:**

```java
public VisitResult visitImage(NodeContext ctx, String src, String alt, String title)
```

###### visitHeading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```java
public VisitResult visitHeading(NodeContext ctx, int level, String text, String id)
```

###### visitCodeBlock()

Visit code blocks `<pre><code>`.

**Signature:**

```java
public VisitResult visitCodeBlock(NodeContext ctx, String lang, String code)
```

###### visitCodeInline()

Visit inline code `<code>`.

**Signature:**

```java
public VisitResult visitCodeInline(NodeContext ctx, String code)
```

###### visitListItem()

Visit list items `<li>`.

**Signature:**

```java
public VisitResult visitListItem(NodeContext ctx, boolean ordered, String marker, String text)
```

###### visitListStart()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```java
public VisitResult visitListStart(NodeContext ctx, boolean ordered)
```

###### visitListEnd()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```java
public VisitResult visitListEnd(NodeContext ctx, boolean ordered, String output)
```

###### visitTableStart()

Called before processing a table `<table>`.

**Signature:**

```java
public VisitResult visitTableStart(NodeContext ctx)
```

###### visitTableRow()

Visit table rows `<tr>`.

**Signature:**

```java
public VisitResult visitTableRow(NodeContext ctx, List<String> cells, boolean isHeader)
```

###### visitTableEnd()

Called after processing a table `</table>`.

**Signature:**

```java
public VisitResult visitTableEnd(NodeContext ctx, String output)
```

###### visitBlockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```java
public VisitResult visitBlockquote(NodeContext ctx, String content, long depth)
```

###### visitStrong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```java
public VisitResult visitStrong(NodeContext ctx, String text)
```

###### visitEmphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```java
public VisitResult visitEmphasis(NodeContext ctx, String text)
```

###### visitStrikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```java
public VisitResult visitStrikethrough(NodeContext ctx, String text)
```

###### visitUnderline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```java
public VisitResult visitUnderline(NodeContext ctx, String text)
```

###### visitSubscript()

Visit subscript elements `<sub>`.

**Signature:**

```java
public VisitResult visitSubscript(NodeContext ctx, String text)
```

###### visitSuperscript()

Visit superscript elements `<sup>`.

**Signature:**

```java
public VisitResult visitSuperscript(NodeContext ctx, String text)
```

###### visitMark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```java
public VisitResult visitMark(NodeContext ctx, String text)
```

###### visitLineBreak()

Visit line break elements `<br>`.

**Signature:**

```java
public VisitResult visitLineBreak(NodeContext ctx)
```

###### visitHorizontalRule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```java
public VisitResult visitHorizontalRule(NodeContext ctx)
```

###### visitCustomElement()

Visit custom elements (web components) or unknown tags.

**Signature:**

```java
public VisitResult visitCustomElement(NodeContext ctx, String tagName, String html)
```

###### visitDefinitionListStart()

Visit definition list `<dl>`.

**Signature:**

```java
public VisitResult visitDefinitionListStart(NodeContext ctx)
```

###### visitDefinitionTerm()

Visit definition term `<dt>`.

**Signature:**

```java
public VisitResult visitDefinitionTerm(NodeContext ctx, String text)
```

###### visitDefinitionDescription()

Visit definition description `<dd>`.

**Signature:**

```java
public VisitResult visitDefinitionDescription(NodeContext ctx, String text)
```

###### visitDefinitionListEnd()

Called after processing a definition list `</dl>`.

**Signature:**

```java
public VisitResult visitDefinitionListEnd(NodeContext ctx, String output)
```

###### visitForm()

Visit form elements `<form>`.

**Signature:**

```java
public VisitResult visitForm(NodeContext ctx, String action, String method)
```

###### visitInput()

Visit input elements `<input>`.

**Signature:**

```java
public VisitResult visitInput(NodeContext ctx, String inputType, String name, String value)
```

###### visitButton()

Visit button elements `<button>`.

**Signature:**

```java
public VisitResult visitButton(NodeContext ctx, String text)
```

###### visitAudio()

Visit audio elements `<audio>`.

**Signature:**

```java
public VisitResult visitAudio(NodeContext ctx, String src)
```

###### visitVideo()

Visit video elements `<video>`.

**Signature:**

```java
public VisitResult visitVideo(NodeContext ctx, String src)
```

###### visitIframe()

Visit iframe elements `<iframe>`.

**Signature:**

```java
public VisitResult visitIframe(NodeContext ctx, String src)
```

###### visitDetails()

Visit details elements `<details>`.

**Signature:**

```java
public VisitResult visitDetails(NodeContext ctx, boolean open)
```

###### visitSummary()

Visit summary elements `<summary>`.

**Signature:**

```java
public VisitResult visitSummary(NodeContext ctx, String text)
```

###### visitFigureStart()

Visit figure elements `<figure>`.

**Signature:**

```java
public VisitResult visitFigureStart(NodeContext ctx)
```

###### visitFigcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```java
public VisitResult visitFigcaption(NodeContext ctx, String text)
```

###### visitFigureEnd()

Called after processing a figure `</figure>`.

**Signature:**

```java
public VisitResult visitFigureEnd(NodeContext ctx, String output)
```


---

##### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `String` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `Optional<String>` | `null` | Alternative text from alt attribute (for accessibility) |
| `title` | `Optional<String>` | `null` | Title attribute (often shown as tooltip) |
| `dimensions` | `Optional<List<Integer>>` | `null` | Image dimensions as (width, height) if available |
| `imageType` | `ImageType` | — | Image type classification |
| `attributes` | `Map<String, String>` | — | Additional HTML attributes |


---

##### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `String` | — | The href URL value |
| `text` | `String` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `Optional<String>` | `null` | Optional title attribute (often shown as tooltip) |
| `linkType` | `LinkType` | — | Link type classification |
| `rel` | `List<String>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `Map<String, String>` | — | Additional HTML attributes |

###### Methods

###### classifyLink()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```java
public static LinkType classifyLink(String href)
```


---

##### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodeType` | `NodeType` | — | Coarse-grained node type classification |
| `tagName` | `String` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `Map<String, String>` | — | All HTML attributes as key-value pairs |
| `depth` | `long` | — | Depth in the DOM tree (0 = root) |
| `indexInParent` | `long` | — | Index among siblings (0-based) |
| `parentTag` | `Optional<String>` | `null` | Parent element's tag name (None if root) |
| `isInline` | `boolean` | — | Whether this element is treated as inline vs block |


---

##### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `removeNavigation` | `boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `removeForms` | `boolean` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

###### Methods

###### defaultOptions()

**Signature:**

```java
public static PreprocessingOptions defaultOptions()
```

###### applyUpdate()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```java
public void applyUpdate(PreprocessingOptionsUpdate update)
```

###### fromUpdate()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```java
public static PreprocessingOptions fromUpdate(PreprocessingOptionsUpdate update)
```

###### from()

**Signature:**

```java
public static PreprocessingOptions from(PreprocessingOptionsUpdate update)
```


---

##### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |


---

##### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `rawJson` | `String` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schemaType` | `Optional<String>` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

##### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `String` | — | The markdown rendering of this table. |


---

##### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `int` | — | Number of rows. |
| `cols` | `int` | — | Number of columns. |
| `cells` | `List<GridCell>` | `Collections.emptyList()` | All cells in the table (may be fewer than rows*cols due to spans). |


---

##### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `int` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `int` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `AnnotationKind` | — | The type of annotation. |


---

#### Enums

##### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `LEFT_TO_RIGHT` | Left-to-right text flow (default for Latin scripts) |
| `RIGHT_TO_LEFT` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `AUTO` | Automatic directionality detection |


---

##### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value | Description |
|-------|-------------|
| `ANCHOR` | Anchor link within same document (href starts with #) |
| `INTERNAL` | Internal link within same domain |
| `EXTERNAL` | External link to different domain |
| `EMAIL` | Email link (mailto:) |
| `PHONE` | Phone link (tel:) |
| `OTHER` | Other protocol or unclassifiable |


---

##### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `DATA_URI` | Data URI embedded image (base64 or other encoding) |
| `INLINE_SVG` | Inline SVG element |
| `EXTERNAL` | External image URL (http/https) |
| `RELATIVE` | Relative image path |


---

##### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `JSON_LD` | JSON-LD (JSON for Linking Data) script blocks |
| `MICRODATA` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFA` | RDF in Attributes (RDFa) markup |


---

##### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `MINIMAL` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `STANDARD` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `AGGRESSIVE` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

##### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `UNDERLINED` | Underlined style (=== for h1, --- for h2). |
| `ATX` | ATX style (# for h1, ## for h2, etc.). Default. |
| `ATX_CLOSED` | ATX closed style (# title #, with closing hashes). |


---

##### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `SPACES` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `TABS` | Use tabs for indentation. |


---

##### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `NORMALIZED` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `STRICT` | Preserve all whitespace exactly as it appears in the HTML. |


---

##### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `SPACES` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `BACKSLASH` | Backslash at end of line. Alternative Markdown syntax. |


---

##### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `INDENTED` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `BACKTICKS` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `TILDES` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

##### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `DOUBLE_EQUAL` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `HTML` | Preserve as HTML (==text==). Original HTML tag. |
| `BOLD` | Render as bold (**text**). Uses strong emphasis. |
| `NONE` | Strip formatting, render as plain text. No markup. |


---

##### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `INLINE` | Inline links: `[text](url)`. Default. |
| `REFERENCE` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

##### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `MARKDOWN` | Standard Markdown (CommonMark compatible). Default. |
| `DJOT` | Djot lightweight markup language. |
| `PLAIN` | Plain text output (no markup, visible text only). |


---

##### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `HEADING` | A heading element (h1-h6). — Fields: `level`: `byte`, `text`: `String` |
| `PARAGRAPH` | A paragraph of text. — Fields: `text`: `String` |
| `LIST` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `boolean` |
| `LIST_ITEM` | A single list item. — Fields: `text`: `String` |
| `TABLE` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `IMAGE` | An image element. — Fields: `description`: `String`, `src`: `String`, `imageIndex`: `int` |
| `CODE` | A code block or inline code. — Fields: `text`: `String`, `language`: `String` |
| `QUOTE` | A block quote container. |
| `DEFINITION_LIST` | A definition list container. |
| `DEFINITION_ITEM` | A definition list entry with term and description. — Fields: `term`: `String`, `definition`: `String` |
| `RAW_BLOCK` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `String`, `content`: `String` |
| `METADATA_BLOCK` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `List<String>` |
| `GROUP` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `String`, `headingLevel`: `byte`, `headingText`: `String` |


---

##### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `BOLD` | Bold / strong emphasis. |
| `ITALIC` | Italic / emphasis. |
| `UNDERLINE` | Underline. |
| `STRIKETHROUGH` | Strikethrough / deleted text. |
| `CODE` | Inline code. |
| `SUBSCRIPT` | Subscript text. |
| `SUPERSCRIPT` | Superscript text. |
| `HIGHLIGHT` | Highlighted / marked text. |
| `LINK` | A hyperlink. — Fields: `url`: `String`, `title`: `String` |


---

##### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `IMAGE_EXTRACTION_FAILED` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `ENCODING_FALLBACK` | The input encoding was not recognized; fell back to UTF-8. |
| `TRUNCATED_INPUT` | The input was truncated due to size limits. |
| `MALFORMED_HTML` | The HTML was malformed but processing continued with best effort. |
| `SANITIZATION_APPLIED` | Sanitization was applied to remove potentially unsafe content. |
| `DEPTH_LIMIT_EXCEEDED` | DOM traversal was truncated because max_depth was exceeded. |


---

##### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value | Description |
|-------|-------------|
| `TEXT` | Text node (most frequent - 100+ per document) |
| `ELEMENT` | Generic element node |
| `HEADING` | Heading elements (h1-h6) |
| `PARAGRAPH` | Paragraph element |
| `DIV` | Generic div container |
| `BLOCKQUOTE` | Blockquote element |
| `PRE` | Preformatted text block |
| `HR` | Horizontal rule |
| `LIST` | Ordered or unordered list (ul, ol) |
| `LIST_ITEM` | List item (li) |
| `DEFINITION_LIST` | Definition list (dl) |
| `DEFINITION_TERM` | Definition term (dt) |
| `DEFINITION_DESCRIPTION` | Definition description (dd) |
| `TABLE` | Table element |
| `TABLE_ROW` | Table row (tr) |
| `TABLE_CELL` | Table cell (td, th) |
| `TABLE_HEADER` | Table header cell (th) |
| `TABLE_BODY` | Table body (tbody) |
| `TABLE_HEAD` | Table head (thead) |
| `TABLE_FOOT` | Table foot (tfoot) |
| `LINK` | Anchor link (a) |
| `IMAGE` | Image (img) |
| `STRONG` | Strong/bold (strong, b) |
| `EM` | Emphasis/italic (em, i) |
| `CODE` | Inline code (code) |
| `STRIKETHROUGH` | Strikethrough (s, del, strike) |
| `UNDERLINE` | Underline (u, ins) |
| `SUBSCRIPT` | Subscript (sub) |
| `SUPERSCRIPT` | Superscript (sup) |
| `MARK` | Mark/highlight (mark) |
| `SMALL` | Small text (small) |
| `BR` | Line break (br) |
| `SPAN` | Span element |
| `ARTICLE` | Article element |
| `SECTION` | Section element |
| `NAV` | Navigation element |
| `ASIDE` | Aside element |
| `HEADER` | Header element |
| `FOOTER` | Footer element |
| `MAIN` | Main element |
| `FIGURE` | Figure element |
| `FIGCAPTION` | Figure caption |
| `TIME` | Time element |
| `DETAILS` | Details element |
| `SUMMARY` | Summary element |
| `FORM` | Form element |
| `INPUT` | Input element |
| `SELECT` | Select element |
| `OPTION` | Option element |
| `BUTTON` | Button element |
| `TEXTAREA` | Textarea element |
| `LABEL` | Label element |
| `FIELDSET` | Fieldset element |
| `LEGEND` | Legend element |
| `AUDIO` | Audio element |
| `VIDEO` | Video element |
| `PICTURE` | Picture element |
| `SOURCE` | Source element |
| `IFRAME` | Iframe element |
| `SVG` | SVG element |
| `CANVAS` | Canvas element |
| `RUBY` | Ruby annotation |
| `RT` | Ruby text |
| `RP` | Ruby parenthesis |
| `ABBR` | Abbreviation |
| `KBD` | Keyboard input |
| `SAMP` | Sample output |
| `VAR` | Variable |
| `CITE` | Citation |
| `Q` | Quote |
| `DEL` | Deleted text |
| `INS` | Inserted text |
| `DATA` | Data element |
| `METER` | Meter element |
| `PROGRESS` | Progress element |
| `OUTPUT` | Output element |
| `TEMPLATE` | Template element |
| `SLOT` | Slot element |
| `HTML` | HTML root element |
| `HEAD` | Head element |
| `BODY` | Body element |
| `TITLE` | Title element |
| `META` | Meta element |
| `LINK_TAG` | Link element (not anchor) |
| `STYLE` | Style element |
| `SCRIPT` | Script element |
| `BASE` | Base element |
| `CUSTOM` | Custom element (web components) or unknown tag |


---

##### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value | Description |
|-------|-------------|
| `CONTINUE` | Continue with default conversion behavior |
| `CUSTOM` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `String` |
| `SKIP` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `PRESERVE_HTML` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `ERROR` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `String` |


---

#### Errors

##### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `PARSE_ERROR` | HTML parsing error |
| `SANITIZATION_ERROR` | HTML sanitization error |
| `CONFIG_ERROR` | Invalid configuration |
| `IO_ERROR` | I/O error |
| `PANIC` | Internal error caught during conversion |
| `INVALID_INPUT` | Invalid input data |
| `OTHER` | Generic conversion error |


---
