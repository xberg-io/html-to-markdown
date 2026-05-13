---
title: "PHP API Reference"
---

## PHP API Reference <span class="version-badge">v3.4.0</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```php
public static function convert(string $html, ?ConversionOptions $options = null): ConversionResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `string` | Yes | The html |
| `options` | `?ConversionOptions` | No | The options to use |

**Returns:** `ConversionResult`
**Errors:** Throws `Error`.

---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions::builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `headingStyle` | `HeadingStyle` | `HeadingStyle::Atx` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `listIndentType` | `ListIndentType` | `ListIndentType::Spaces` | How to indent nested list items (spaces or tab). |
| `listIndentWidth` | `int` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `string` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strongEmSymbol` | `string` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escapeAsterisks` | `bool` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escapeUnderscores` | `bool` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escapeMisc` | `bool` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escapeAscii` | `bool` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `codeLanguage` | `string` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `bool` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `defaultTitle` | `bool` | `false` | Emit a default title when no `<title>` tag is present. |
| `brInTables` | `bool` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlightStyle` | `HighlightStyle` | `HighlightStyle::DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extractMetadata` | `bool` | `true` | Populate `result.metadata` with `<head>` / `<meta>` extraction (title, description, Open Graph, Twitter Card, JSON-LD, …). Default `true`. Disabling skips the metadata pass only — table extraction into `result.tables` runs unconditionally. |
| `whitespaceMode` | `WhitespaceMode` | `WhitespaceMode::Normalized` | Controls how whitespace is normalised during conversion. |
| `stripNewlines` | `bool` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrapWidth` | `int` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convertAsInline` | `bool` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `subSymbol` | `string` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `supSymbol` | `string` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newlineStyle` | `NewlineStyle` | `NewlineStyle::Spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `codeBlockStyle` | `CodeBlockStyle` | `CodeBlockStyle::Backticks` | Style used for fenced code blocks (backticks or tilde). |
| `keepInlineImagesIn` | `array<string>` | `[]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `string` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `false` | Emit debug information during conversion. |
| `stripTags` | `array<string>` | `[]` | HTML tag names whose content is stripped from the output entirely. |
| `preserveTags` | `array<string>` | `[]` | HTML tag names that are preserved verbatim in the output. |
| `skipImages` | `bool` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `linkStyle` | `LinkStyle` | `LinkStyle::Inline` | Link rendering style (inline or reference). |
| `outputFormat` | `OutputFormat` | `OutputFormat::Markdown` | Target output format (Markdown, plain text, etc.). |
| `includeDocumentStructure` | `bool` | `false` | Include structured document tree in result. |
| `extractImages` | `bool` | `false` | Extract inline images from data URIs and SVGs. |
| `maxImageSize` | `int` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `captureSvg` | `bool` | `false` | Capture SVG elements as images. |
| `inferDimensions` | `bool` | `true` | Infer image dimensions from data. |
| `maxDepth` | `?int` | `null` | Maximum DOM traversal depth. `null` means unlimited. When set, subtrees beyond this depth are silently truncated. |
| `excludeSelectors` | `array<string>` | `[]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]` |
| `visitor` | `?VisitorHandle` | `null` | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`. |

##### Methods

###### default()

**Signature:**

```php
public static function default(): ConversionOptions
```

###### builder()

Create a new builder with default values.

**Signature:**

```php
public static function builder(): ConversionOptionsBuilder
```

###### applyUpdate()

Apply a partial update to these conversion options.

**Signature:**

```php
public function applyUpdate(ConversionOptionsUpdate $update): void
```

###### fromUpdate()

Create from a partial update, applying to defaults.

**Signature:**

```php
public static function fromUpdate(ConversionOptionsUpdate $update): ConversionOptions
```

###### from()

**Signature:**

```php
public static function from(ConversionOptionsUpdate $update): ConversionOptions
```

---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `?string` | `null` | Converted text output (markdown, djot, or plain text). `null` when `output_format` is set to `OutputFormat::None`, indicating extraction-only mode. |
| `document` | `?DocumentStructure` | `null` | Structured document tree with semantic elements. Populated when `include_document_structure` is `true` in options. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `array<TableData>` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `array<string>` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `true` in options. |
| `warnings` | `array<ProcessingWarning>` | `[]` | Non-fatal processing warnings. |


---

#### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### stripTags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```php
public function stripTags(array<string> $tags): ConversionOptionsBuilder
```

###### preserveTags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```php
public function preserveTags(array<string> $tags): ConversionOptionsBuilder
```

###### keepInlineImagesIn()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```php
public function keepInlineImagesIn(array<string> $tags): ConversionOptionsBuilder
```

###### excludeSelectors()

Set the list of CSS selectors for elements to exclude entirely from output.

**Signature:**

```php
public function excludeSelectors(array<string> $selectors): ConversionOptionsBuilder
```

###### visitor()

Set the visitor used during conversion.

**Signature:**

```php
public function visitor(VisitorHandle $visitor): ConversionOptionsBuilder
```

###### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```php
public function preprocessing(PreprocessingOptions $preprocessing): ConversionOptionsBuilder
```

###### build()

Build the final `ConversionOptions`.

**Signature:**

```php
public function build(): ConversionOptions
```

---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `?string` | `null` | Document title from `<title>` tag |
| `description` | `?string` | `null` | Document description from `<meta name="description">` tag |
| `keywords` | `array<string>` | `[]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `?string` | `null` | Document author from `<meta name="author">` tag |
| `canonicalUrl` | `?string` | `null` | Canonical URL from `<link rel="canonical">` tag |
| `baseHref` | `?string` | `null` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `?string` | `null` | Document language from `lang` attribute |
| `textDirection` | `?TextDirection` | `null` | Document text direction from `dir` attribute |
| `openGraph` | `array<string, string>` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitterCard` | `array<string, string>` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `metaTags` | `array<string, string>` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `string` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `?int` | `null` | Index of the parent node (None for root nodes). |
| `children` | `array<int>` | — | Indices of child nodes in reading order. |
| `annotations` | `array<TextAnnotation>` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `?array<string, string>` | `null` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `array<DocumentNode>` | — | All nodes in document reading order. |
| `sourceFormat` | `?string` | `null` | The source format (always "html" for this library). |


---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The text content of the cell. |
| `row` | `int` | — | 0-indexed row position. |
| `col` | `int` | — | 0-indexed column position. |
| `rowSpan` | `int` | — | Number of rows this cell spans (default 1). |
| `colSpan` | `int` | — | Number of columns this cell spans (default 1). |
| `isHeader` | `bool` | — | Whether this is a header cell (`<th>`). |


---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `int` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `string` | — | Normalized text content of the header |
| `id` | `?string` | `null` | HTML id attribute if present |
| `depth` | `int` | — | Document tree depth at the header element |
| `htmlOffset` | `int` | — | Byte offset in original HTML document |

##### Methods

###### isValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```php
public function isValid(): bool
```

---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `array<HeaderMetadata>` | `[]` | Extracted header elements with hierarchy |
| `links` | `array<LinkMetadata>` | `[]` | Extracted hyperlinks with type classification |
| `images` | `array<ImageMetadata>` | `[]` | Extracted images with source and dimensions |
| `structuredData` | `array<StructuredData>` | `[]` | Extracted structured data blocks |


---

#### HtmlVisitor

Visitor trait for HTML→Markdown conversion.

Implement this trait to customize the conversion behavior for any HTML element type.
All methods have default implementations that return `VisitResult::Continue`, allowing
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
- Return `VisitResult::Continue` quickly for elements you don't need to customize
- Avoid heavy computation in visitor methods; consider caching if needed

### Methods

#### visitText()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```php
public function visitText(NodeContext $ctx, string $text): VisitResult
```

##### visitElementStart()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```php
public function visitElementStart(NodeContext $ctx): VisitResult
```

###### visitElementEnd()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```php
public function visitElementEnd(NodeContext $ctx, string $output): VisitResult
```

###### visitLink()

Visit anchor links `<a href="...">`.

**Signature:**

```php
public function visitLink(NodeContext $ctx, string $href, string $text, string $title): VisitResult
```

###### visitImage()

Visit images `<img src="...">`.

**Signature:**

```php
public function visitImage(NodeContext $ctx, string $src, string $alt, string $title): VisitResult
```

###### visitHeading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```php
public function visitHeading(NodeContext $ctx, int $level, string $text, string $id): VisitResult
```

###### visitCodeBlock()

Visit code blocks `<pre><code>`.

**Signature:**

```php
public function visitCodeBlock(NodeContext $ctx, string $lang, string $code): VisitResult
```

###### visitCodeInline()

Visit inline code `<code>`.

**Signature:**

```php
public function visitCodeInline(NodeContext $ctx, string $code): VisitResult
```

###### visitListItem()

Visit list items `<li>`.

**Signature:**

```php
public function visitListItem(NodeContext $ctx, bool $ordered, string $marker, string $text): VisitResult
```

###### visitListStart()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```php
public function visitListStart(NodeContext $ctx, bool $ordered): VisitResult
```

###### visitListEnd()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```php
public function visitListEnd(NodeContext $ctx, bool $ordered, string $output): VisitResult
```

###### visitTableStart()

Called before processing a table `<table>`.

**Signature:**

```php
public function visitTableStart(NodeContext $ctx): VisitResult
```

###### visitTableRow()

Visit table rows `<tr>`.

**Signature:**

```php
public function visitTableRow(NodeContext $ctx, array<string> $cells, bool $isHeader): VisitResult
```

###### visitTableEnd()

Called after processing a table `</table>`.

**Signature:**

```php
public function visitTableEnd(NodeContext $ctx, string $output): VisitResult
```

###### visitBlockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```php
public function visitBlockquote(NodeContext $ctx, string $content, int $depth): VisitResult
```

###### visitStrong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```php
public function visitStrong(NodeContext $ctx, string $text): VisitResult
```

###### visitEmphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```php
public function visitEmphasis(NodeContext $ctx, string $text): VisitResult
```

###### visitStrikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```php
public function visitStrikethrough(NodeContext $ctx, string $text): VisitResult
```

###### visitUnderline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```php
public function visitUnderline(NodeContext $ctx, string $text): VisitResult
```

###### visitSubscript()

Visit subscript elements `<sub>`.

**Signature:**

```php
public function visitSubscript(NodeContext $ctx, string $text): VisitResult
```

###### visitSuperscript()

Visit superscript elements `<sup>`.

**Signature:**

```php
public function visitSuperscript(NodeContext $ctx, string $text): VisitResult
```

###### visitMark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```php
public function visitMark(NodeContext $ctx, string $text): VisitResult
```

###### visitLineBreak()

Visit line break elements `<br>`.

**Signature:**

```php
public function visitLineBreak(NodeContext $ctx): VisitResult
```

###### visitHorizontalRule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```php
public function visitHorizontalRule(NodeContext $ctx): VisitResult
```

###### visitCustomElement()

Visit custom elements (web components) or unknown tags.

**Signature:**

```php
public function visitCustomElement(NodeContext $ctx, string $tagName, string $html): VisitResult
```

###### visitDefinitionListStart()

Visit definition list `<dl>`.

**Signature:**

```php
public function visitDefinitionListStart(NodeContext $ctx): VisitResult
```

###### visitDefinitionTerm()

Visit definition term `<dt>`.

**Signature:**

```php
public function visitDefinitionTerm(NodeContext $ctx, string $text): VisitResult
```

###### visitDefinitionDescription()

Visit definition description `<dd>`.

**Signature:**

```php
public function visitDefinitionDescription(NodeContext $ctx, string $text): VisitResult
```

###### visitDefinitionListEnd()

Called after processing a definition list `</dl>`.

**Signature:**

```php
public function visitDefinitionListEnd(NodeContext $ctx, string $output): VisitResult
```

###### visitForm()

Visit form elements `<form>`.

**Signature:**

```php
public function visitForm(NodeContext $ctx, string $action, string $method): VisitResult
```

###### visitInput()

Visit input elements `<input>`.

**Signature:**

```php
public function visitInput(NodeContext $ctx, string $inputType, string $name, string $value): VisitResult
```

###### visitButton()

Visit button elements `<button>`.

**Signature:**

```php
public function visitButton(NodeContext $ctx, string $text): VisitResult
```

###### visitAudio()

Visit audio elements `<audio>`.

**Signature:**

```php
public function visitAudio(NodeContext $ctx, string $src): VisitResult
```

###### visitVideo()

Visit video elements `<video>`.

**Signature:**

```php
public function visitVideo(NodeContext $ctx, string $src): VisitResult
```

###### visitIframe()

Visit iframe elements `<iframe>`.

**Signature:**

```php
public function visitIframe(NodeContext $ctx, string $src): VisitResult
```

###### visitDetails()

Visit details elements `<details>`.

**Signature:**

```php
public function visitDetails(NodeContext $ctx, bool $open): VisitResult
```

###### visitSummary()

Visit summary elements `<summary>`.

**Signature:**

```php
public function visitSummary(NodeContext $ctx, string $text): VisitResult
```

###### visitFigureStart()

Visit figure elements `<figure>`.

**Signature:**

```php
public function visitFigureStart(NodeContext $ctx): VisitResult
```

###### visitFigcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```php
public function visitFigcaption(NodeContext $ctx, string $text): VisitResult
```

###### visitFigureEnd()

Called after processing a figure `</figure>`.

**Signature:**

```php
public function visitFigureEnd(NodeContext $ctx, string $output): VisitResult
```

---

##### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `string` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `?string` | `null` | Alternative text from alt attribute (for accessibility) |
| `title` | `?string` | `null` | Title attribute (often shown as tooltip) |
| `dimensions` | `?array<int>` | `null` | Image dimensions as (width, height) if available |
| `imageType` | `ImageType` | — | Image type classification |
| `attributes` | `array<string, string>` | — | Additional HTML attributes |


---

##### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `string` | — | The href URL value |
| `text` | `string` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `?string` | `null` | Optional title attribute (often shown as tooltip) |
| `linkType` | `LinkType` | — | Link type classification |
| `rel` | `array<string>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `array<string, string>` | — | Additional HTML attributes |

###### Methods

###### classifyLink()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```php
public static function classifyLink(string $href): LinkType
```

---

##### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodeType` | `NodeType` | — | Coarse-grained node type classification |
| `tagName` | `string` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `array<string, string>` | — | All HTML attributes as key-value pairs |
| `depth` | `int` | — | Depth in the DOM tree (0 = root) |
| `indexInParent` | `int` | — | Index among siblings (0-based) |
| `parentTag` | `?string` | `null` | Parent element's tag name (None if root) |
| `isInline` | `bool` | — | Whether this element is treated as inline vs block |


---

##### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset::Standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `removeNavigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `removeForms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

###### Methods

###### default()

**Signature:**

```php
public static function default(): PreprocessingOptions
```

###### applyUpdate()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```php
public function applyUpdate(PreprocessingOptionsUpdate $update): void
```

###### fromUpdate()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```php
public static function fromUpdate(PreprocessingOptionsUpdate $update): PreprocessingOptions
```

###### from()

**Signature:**

```php
public static function from(PreprocessingOptionsUpdate $update): PreprocessingOptions
```

---

##### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `string` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |


---

##### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `rawJson` | `string` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schemaType` | `?string` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

##### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `string` | — | The markdown rendering of this table. |


---

##### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `int` | — | Number of rows. |
| `cols` | `int` | — | Number of columns. |
| `cells` | `array<GridCell>` | `[]` | All cells in the table (may be fewer than rows*cols due to spans). |


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

##### VisitorHandle

Type alias for a visitor handle (Rc-wrapped `RefCell` for interior mutability).

This allows visitors to be passed around and shared while still being mutable.


---

#### Enums

##### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `LeftToRight` | Left-to-right text flow (default for Latin scripts) |
| `RightToLeft` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `Auto` | Automatic directionality detection |


---

##### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value | Description |
|-------|-------------|
| `Anchor` | Anchor link within same document (href starts with #) |
| `Internal` | Internal link within same domain |
| `External` | External link to different domain |
| `Email` | Email link (mailto:) |
| `Phone` | Phone link (tel:) |
| `Other` | Other protocol or unclassifiable |


---

##### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `DataUri` | Data URI embedded image (base64 or other encoding) |
| `InlineSvg` | Inline SVG element |
| `External` | External image URL (http/https) |
| `Relative` | Relative image path |


---

##### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `JsonLd` | JSON-LD (JSON for Linking Data) script blocks |
| `Microdata` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFa` | RDF in Attributes (RDFa) markup |


---

##### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `Minimal` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `Standard` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `Aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

##### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `Underlined` | Underlined style (=== for h1, --- for h2). |
| `Atx` | ATX style (# for h1, ## for h2, etc.). Default. |
| `AtxClosed` | ATX closed style (# title #, with closing hashes). |


---

##### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `Spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `Tabs` | Use tabs for indentation. |


---

##### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `Normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `Strict` | Preserve all whitespace exactly as it appears in the HTML. |


---

##### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `Spaces` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `Backslash` | Backslash at end of line. Alternative Markdown syntax. |


---

##### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `Indented` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `Backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `Tildes` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

##### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `DoubleEqual` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `Html` | Preserve as HTML (==text==). Original HTML tag. |
| `Bold` | Render as bold (**text**). Uses strong emphasis. |
| `None` | Strip formatting, render as plain text. No markup. |


---

##### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `Inline` | Inline links: `[text](url)`. Default. |
| `Reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

##### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `Markdown` | Standard Markdown (CommonMark compatible). Default. |
| `Djot` | Djot lightweight markup language. |
| `Plain` | Plain text output (no markup, visible text only). |


---

##### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `Heading` | A heading element (h1-h6). — Fields: `level`: `int`, `text`: `string` |
| `Paragraph` | A paragraph of text. — Fields: `text`: `string` |
| `List` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool` |
| `ListItem` | A single list item. — Fields: `text`: `string` |
| `Table` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `Image` | An image element. — Fields: `description`: `string`, `src`: `string`, `imageIndex`: `int` |
| `Code` | A code block or inline code. — Fields: `text`: `string`, `language`: `string` |
| `Quote` | A block quote container. |
| `DefinitionList` | A definition list container. |
| `DefinitionItem` | A definition list entry with term and description. — Fields: `term`: `string`, `definition`: `string` |
| `RawBlock` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `string`, `content`: `string` |
| `MetadataBlock` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `array<string>` |
| `Group` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `string`, `headingLevel`: `int`, `headingText`: `string` |


---

##### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `Bold` | Bold / strong emphasis. |
| `Italic` | Italic / emphasis. |
| `Underline` | Underline. |
| `Strikethrough` | Strikethrough / deleted text. |
| `Code` | Inline code. |
| `Subscript` | Subscript text. |
| `Superscript` | Superscript text. |
| `Highlight` | Highlighted / marked text. |
| `Link` | A hyperlink. — Fields: `url`: `string`, `title`: `string` |


---

##### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `ImageExtractionFailed` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `EncodingFallback` | The input encoding was not recognized; fell back to UTF-8. |
| `TruncatedInput` | The input was truncated due to size limits. |
| `MalformedHtml` | The HTML was malformed but processing continued with best effort. |
| `SanitizationApplied` | Sanitization was applied to remove potentially unsafe content. |
| `DepthLimitExceeded` | DOM traversal was truncated because max_depth was exceeded. |


---

##### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value | Description |
|-------|-------------|
| `Text` | Text node (most frequent - 100+ per document) |
| `Element` | Generic element node |
| `Heading` | Heading elements (h1-h6) |
| `Paragraph` | Paragraph element |
| `Div` | Generic div container |
| `Blockquote` | Blockquote element |
| `Pre` | Preformatted text block |
| `Hr` | Horizontal rule |
| `List` | Ordered or unordered list (ul, ol) |
| `ListItem` | List item (li) |
| `DefinitionList` | Definition list (dl) |
| `DefinitionTerm` | Definition term (dt) |
| `DefinitionDescription` | Definition description (dd) |
| `Table` | Table element |
| `TableRow` | Table row (tr) |
| `TableCell` | Table cell (td, th) |
| `TableHeader` | Table header cell (th) |
| `TableBody` | Table body (tbody) |
| `TableHead` | Table head (thead) |
| `TableFoot` | Table foot (tfoot) |
| `Link` | Anchor link (a) |
| `Image` | Image (img) |
| `Strong` | Strong/bold (strong, b) |
| `Em` | Emphasis/italic (em, i) |
| `Code` | Inline code (code) |
| `Strikethrough` | Strikethrough (s, del, strike) |
| `Underline` | Underline (u, ins) |
| `Subscript` | Subscript (sub) |
| `Superscript` | Superscript (sup) |
| `Mark` | Mark/highlight (mark) |
| `Small` | Small text (small) |
| `Br` | Line break (br) |
| `Span` | Span element |
| `Article` | Article element |
| `Section` | Section element |
| `Nav` | Navigation element |
| `Aside` | Aside element |
| `Header` | Header element |
| `Footer` | Footer element |
| `Main` | Main element |
| `Figure` | Figure element |
| `Figcaption` | Figure caption |
| `Time` | Time element |
| `Details` | Details element |
| `Summary` | Summary element |
| `Form` | Form element |
| `Input` | Input element |
| `Select` | Select element |
| `Option` | Option element |
| `Button` | Button element |
| `Textarea` | Textarea element |
| `Label` | Label element |
| `Fieldset` | Fieldset element |
| `Legend` | Legend element |
| `Audio` | Audio element |
| `Video` | Video element |
| `Picture` | Picture element |
| `Source` | Source element |
| `Iframe` | Iframe element |
| `Svg` | SVG element |
| `Canvas` | Canvas element |
| `Ruby` | Ruby annotation |
| `Rt` | Ruby text |
| `Rp` | Ruby parenthesis |
| `Abbr` | Abbreviation |
| `Kbd` | Keyboard input |
| `Samp` | Sample output |
| `Var` | Variable |
| `Cite` | Citation |
| `Q` | Quote |
| `Del` | Deleted text |
| `Ins` | Inserted text |
| `Data` | Data element |
| `Meter` | Meter element |
| `Progress` | Progress element |
| `Output` | Output element |
| `Template` | Template element |
| `Slot` | Slot element |
| `Html` | HTML root element |
| `Head` | Head element |
| `Body` | Body element |
| `Title` | Title element |
| `Meta` | Meta element |
| `LinkTag` | Link element (not anchor) |
| `Style` | Style element |
| `Script` | Script element |
| `Base` | Base element |
| `Custom` | Custom element (web components) or unknown tag |


---

##### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value | Description |
|-------|-------------|
| `Continue` | Continue with default conversion behavior |
| `Custom` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `string` |
| `Skip` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `Error` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `string` |


---

#### Errors

##### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `ParseError` | HTML parsing error |
| `SanitizationError` | HTML sanitization error |
| `ConfigError` | Invalid configuration |
| `IoError` | I/O error |
| `Panic` | Internal error caught during conversion |
| `InvalidInput` | Invalid input data |
| `Other` | Generic conversion error |


---
