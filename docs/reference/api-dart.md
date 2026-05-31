---
title: "Dart API Reference"
---

## Dart API Reference <span class="version-badge">v3.5.7</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```dart
ConversionResult convert(String html, [ConversionOptions? options])
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The html |
| `options` | `ConversionOptions?` | No | The options to use |

**Returns:** `ConversionResult`
**Errors:** Throws `Error`.

---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `headingStyle` | `HeadingStyle` | `HeadingStyle.Atx` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `listIndentType` | `ListIndentType` | `ListIndentType.Spaces` | How to indent nested list items (spaces or tab). |
| `listIndentWidth` | `int` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `String` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strongEmSymbol` | `String` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escapeAsterisks` | `bool` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escapeUnderscores` | `bool` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escapeMisc` | `bool` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escapeAscii` | `bool` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `codeLanguage` | `String` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `bool` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `defaultTitle` | `bool` | `false` | Emit a default title when no `<title>` tag is present. |
| `brInTables` | `bool` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `compactTables` | `bool` | `false` | Emit tables without column padding (compact GFM format). When `true`, column widths are not computed and cells are emitted with no trailing spaces. Separator rows use exactly `---` per column. Produces token-efficient output suitable for RAG / LLM contexts. Default `false` (aligned padding preserved). |
| `highlightStyle` | `HighlightStyle` | `HighlightStyle.DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extractMetadata` | `bool` | `true` | Populate `result.metadata` with `<head>` / `<meta>` extraction (title, description, Open Graph, Twitter Card, JSON-LD, …). Default `true`. Disabling skips the metadata pass only — table extraction into `result.tables` runs unconditionally. |
| `whitespaceMode` | `WhitespaceMode` | `WhitespaceMode.Normalized` | Controls how whitespace sequences are normalised in the converted output. - `WhitespaceMode.Normalized` (default) — collapses consecutive whitespace characters (spaces, tabs, newlines) to a single space, matching browser rendering behaviour. - `WhitespaceMode.Strict` — preserves all whitespace exactly as it appears in the source HTML, including runs of spaces and embedded newlines. Choose `Strict` only when the source HTML uses deliberate whitespace (e.g. pre-formatted content outside `<pre>` tags). For most documents `Normalized` produces cleaner output. |
| `stripNewlines` | `bool` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrapWidth` | `int` | `80` | Maximum output line width in characters when `wrap` is `true` (default `80`). Lines are broken at word boundaries so that no line exceeds this length. A value of `0` is treated as "no limit" — equivalent to leaving `wrap` disabled. Has no effect when `wrap` is `false`. |
| `convertAsInline` | `bool` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `subSymbol` | `String` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `supSymbol` | `String` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newlineStyle` | `NewlineStyle` | `NewlineStyle.Spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `codeBlockStyle` | `CodeBlockStyle` | `CodeBlockStyle.Backticks` | Style used for fenced code blocks (backticks or tilde). |
| `keepInlineImagesIn` | `List<String>` | `[]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Options for the HTML pre-processing pass applied before conversion begins. Pre-processing runs before the HTML is handed to the converter and can perform operations such as unwrapping redundant wrapper elements, removing tracking pixels, and normalising vendor-specific markup. See `PreprocessingOptions` for the full set of knobs. Defaults to `PreprocessingOptions.default()`, which enables the standard cleaning passes. Set individual fields on `PreprocessingOptions` (or construct via `ConversionOptions.builder`) to opt in or out of specific passes. |
| `encoding` | `String` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `false` | Emit debug information during conversion. |
| `stripTags` | `List<String>` | `[]` | HTML tag names whose content is stripped from the output entirely. |
| `preserveTags` | `List<String>` | `[]` | HTML tag names that are preserved verbatim in the output. |
| `skipImages` | `bool` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `urlEscapeStyle` | `UrlEscapeStyle` | `UrlEscapeStyle.Angle` | URL encoding strategy for link and image destinations. Controls how special characters in URL destinations are escaped: - `UrlEscapeStyle.Angle` (default) — wraps the destination in angle brackets when it contains spaces or newlines. Some parsers misinterpret `>` inside such a destination. - `UrlEscapeStyle.Percent` — percent-encodes every character that is not an RFC 3986 unreserved character or `/`, producing a destination that all Markdown parsers handle correctly even when the URL contains `<`, `>`, spaces, or parentheses. |
| `linkStyle` | `LinkStyle` | `LinkStyle.Inline` | Link rendering style (inline or reference). |
| `outputFormat` | `OutputFormat` | `OutputFormat.Markdown` | Target output format (Markdown, plain text, etc.). |
| `includeDocumentStructure` | `bool` | `false` | Include structured document tree in result. |
| `extractImages` | `bool` | `false` | Extract inline images from data URIs and SVGs. |
| `maxImageSize` | `int` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `captureSvg` | `bool` | `false` | Capture SVG elements as images. |
| `inferDimensions` | `bool` | `true` | Infer image dimensions from data. |
| `maxDepth` | `int?` | `null` | Maximum DOM traversal depth. `null` means unlimited. When set, subtrees beyond this depth are silently truncated. |
| `excludeSelectors` | `List<String>` | `[]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]` |
| `visitor` | `VisitorHandle?` | `null` | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`. |

### Methods

#### default()

**Signature:**

```dart
static ConversionOptions default()
```

---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String?` | `null` | Converted text output (markdown, djot, or plain text). `null` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `DocumentStructure?` | `null` | Structured document tree with semantic elements. Populated when `ConversionOptions.include_document_structure` is `true`. `null` otherwise (the default), which avoids the overhead of building the tree. When present, the tree mirrors the converted document: headings open `Group` sections, paragraphs and list items carry inline `TextAnnotation`s, and tables reference the same `TableGrid` data exposed in `Self.tables`. Note: this field is independent of the `metadata` feature flag. Document structure collection is always available at runtime; it is gated only by the runtime option, not by a compile-time feature. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `List<TableData>` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `List<String>` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `true` in options. |
| `warnings` | `List<ProcessingWarning>` | `[]` | Non-fatal processing warnings. |

---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String?` | `null` | Document title from `<title>` tag |
| `description` | `String?` | `null` | Document description from `<meta name="description">` tag |
| `keywords` | `List<String>` | `[]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `String?` | `null` | Document author from `<meta name="author">` tag |
| `canonicalUrl` | `String?` | `null` | Canonical URL from `<link rel="canonical">` tag |
| `baseHref` | `String?` | `null` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `String?` | `null` | Document language from `lang` attribute |
| `textDirection` | `TextDirection?` | `null` | Document text direction from `dir` attribute |
| `openGraph` | `Map<String, String>` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitterCard` | `Map<String, String>` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `metaTags` | `Map<String, String>` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |

---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `int?` | `null` | Index of the parent node (None for root nodes). |
| `children` | `List<int>` | `/* serde(default) */` | Indices of child nodes in reading order. |
| `annotations` | `List<TextAnnotation>` | `/* serde(default) */` | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `Map<String, String>?` | `null` | Format-specific attributes preserved from the source HTML element. Keys are lowercased attribute names as they appear in the HTML (e.g. `"class"`, `"id"`, `"data-foo"`). Values are the raw attribute strings, copied verbatim from the source — no HTML entity decoding is applied here. The map is `null` when no attributes are present (omitted entirely in serialized output). Not every HTML attribute is preserved: only attributes that carry semantic or structural significance for the node type are collected. For example, heading nodes capture the `"id"` attribute for anchor linking; other element-level attributes may be silently dropped. |

---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `List<DocumentNode>` | — | All nodes in document reading order. |
| `sourceFormat` | `String?` | `null` | The source format (always "html" for this library). |

---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The text content of the cell. |
| `row` | `int` | — | 0-indexed row position. |
| `col` | `int` | — | 0-indexed column position. |
| `rowSpan` | `int` | `/* serde(default) */` | Number of rows this cell spans (default 1). |
| `colSpan` | `int` | `/* serde(default) */` | Number of columns this cell spans (default 1). |
| `isHeader` | `bool` | `/* serde(default) */` | Whether this is a header cell (`<th>`). |

---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `int` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `String` | — | Normalized text content of the header |
| `id` | `String?` | `null` | HTML id attribute if present |
| `depth` | `int` | — | Document tree depth at the header element |
| `htmlOffset` | `int` | — | Byte offset in original HTML document |

### Methods

#### isValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```dart
bool isValid()
```

---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `List<HeaderMetadata>` | `[]` | Extracted header elements with hierarchy |
| `links` | `List<LinkMetadata>` | `[]` | Extracted hyperlinks with type classification |
| `images` | `List<ImageMetadata>` | `[]` | Extracted images with source and dimensions |
| `structuredData` | `List<StructuredData>` | `[]` | Extracted structured data blocks |

---

#### HtmlVisitor

Visitor for HTML→Markdown conversion.

Provide a visitor object whose methods customize the conversion behavior for any
HTML element type. Override only the methods you care about; unimplemented methods
default to `Continue` (emit the standard rendering).

Each callback returns one of:

- `Continue` (the default) — keep the standard rendering.
- `Skip` — drop the element from the output entirely.
- `PreserveHtml` — pass the original HTML through verbatim.
- `Custom(text)` — replace the rendering with `text`.
- `Error(message)` — abort conversion with `message`.

**Language idioms.** In Rust, return one of the `VisitResult` variants directly.
In Python, Ruby, JavaScript/TypeScript, and other duck-typed bindings, define a
plain class (no base class required) and return either a string (`"continue"`,
`"skip"`, `"preserve_html"`) or a tagged map (`{"custom": "..."}`,
`{"error": "..."}`) — the binding converts the return value to the corresponding
`VisitResult` variant automatically.

### Method Naming Convention

- `visit_*_start`: Called before entering an element (pre-order traversal)
- `visit_*_end`: Called after exiting an element (post-order traversal)
- `visit_*`: Called for specific element types (e.g., `visit_link`, `visit_image`)

### Execution Order

For a typical element like `<div><p>text</p></div>`:

1. `visit_element_start` for `<div>`
2. `visit_element_start` for `<p>`
3. `visit_text` for "text"
4. `visit_element_end` for `<p>`
5. `visit_element_end` for `</div>`

### Performance Notes

- `visit_text` is the most frequently called method (~100+ times per document)
- Return `Continue` quickly for elements you don't need to customize
- Avoid heavy computation in visitor methods; consider caching if needed

### Methods

#### visitText()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```dart
VisitResult visitText(NodeContext ctx, String text)
```

#### visitElementStart()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```dart
VisitResult visitElementStart(NodeContext ctx)
```

#### visitElementEnd()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```dart
VisitResult visitElementEnd(NodeContext ctx, String output)
```

#### visitLink()

Visit anchor links `<a href="...">`.

**Signature:**

```dart
VisitResult visitLink(NodeContext ctx, String href, String text, [String? title])
```

#### visitImage()

Visit images `<img src="...">`.

**Signature:**

```dart
VisitResult visitImage(NodeContext ctx, String src, String alt, [String? title])
```

#### visitHeading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```dart
VisitResult visitHeading(NodeContext ctx, int level, String text, [String? id])
```

#### visitCodeBlock()

Visit code blocks `<pre><code>`.

**Signature:**

```dart
VisitResult visitCodeBlock(NodeContext ctx, String code, [String? lang])
```

#### visitCodeInline()

Visit inline code `<code>`.

**Signature:**

```dart
VisitResult visitCodeInline(NodeContext ctx, String code)
```

#### visitListItem()

Visit list items `<li>`.

**Signature:**

```dart
VisitResult visitListItem(NodeContext ctx, bool ordered, String marker, String text)
```

#### visitListStart()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```dart
VisitResult visitListStart(NodeContext ctx, bool ordered)
```

#### visitListEnd()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```dart
VisitResult visitListEnd(NodeContext ctx, bool ordered, String output)
```

#### visitTableStart()

Called before processing a table `<table>`.

**Signature:**

```dart
VisitResult visitTableStart(NodeContext ctx)
```

#### visitTableRow()

Visit table rows `<tr>`.

**Signature:**

```dart
VisitResult visitTableRow(NodeContext ctx, List<String> cells, bool isHeader)
```

#### visitTableEnd()

Called after processing a table `</table>`.

**Signature:**

```dart
VisitResult visitTableEnd(NodeContext ctx, String output)
```

#### visitBlockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```dart
VisitResult visitBlockquote(NodeContext ctx, String content, int depth)
```

#### visitStrong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```dart
VisitResult visitStrong(NodeContext ctx, String text)
```

#### visitEmphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```dart
VisitResult visitEmphasis(NodeContext ctx, String text)
```

#### visitStrikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```dart
VisitResult visitStrikethrough(NodeContext ctx, String text)
```

#### visitUnderline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```dart
VisitResult visitUnderline(NodeContext ctx, String text)
```

#### visitSubscript()

Visit subscript elements `<sub>`.

**Signature:**

```dart
VisitResult visitSubscript(NodeContext ctx, String text)
```

#### visitSuperscript()

Visit superscript elements `<sup>`.

**Signature:**

```dart
VisitResult visitSuperscript(NodeContext ctx, String text)
```

#### visitMark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```dart
VisitResult visitMark(NodeContext ctx, String text)
```

#### visitLineBreak()

Visit line break elements `<br>`.

**Signature:**

```dart
VisitResult visitLineBreak(NodeContext ctx)
```

#### visitHorizontalRule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```dart
VisitResult visitHorizontalRule(NodeContext ctx)
```

#### visitCustomElement()

Visit custom elements (web components) or unknown tags.

**Signature:**

```dart
VisitResult visitCustomElement(NodeContext ctx, String tagName, String html)
```

#### visitDefinitionListStart()

Visit definition list `<dl>`.

**Signature:**

```dart
VisitResult visitDefinitionListStart(NodeContext ctx)
```

#### visitDefinitionTerm()

Visit definition term `<dt>`.

**Signature:**

```dart
VisitResult visitDefinitionTerm(NodeContext ctx, String text)
```

#### visitDefinitionDescription()

Visit definition description `<dd>`.

**Signature:**

```dart
VisitResult visitDefinitionDescription(NodeContext ctx, String text)
```

#### visitDefinitionListEnd()

Called after processing a definition list `</dl>`.

**Signature:**

```dart
VisitResult visitDefinitionListEnd(NodeContext ctx, String output)
```

#### visitForm()

Visit form elements `<form>`.

**Signature:**

```dart
VisitResult visitForm(NodeContext ctx, [String? action, String? method])
```

#### visitInput()

Visit input elements `<input>`.

**Signature:**

```dart
VisitResult visitInput(NodeContext ctx, String inputType, [String? name, String? value])
```

#### visitButton()

Visit button elements `<button>`.

**Signature:**

```dart
VisitResult visitButton(NodeContext ctx, String text)
```

#### visitAudio()

Visit audio elements `<audio>`.

**Signature:**

```dart
VisitResult visitAudio(NodeContext ctx, [String? src])
```

#### visitVideo()

Visit video elements `<video>`.

**Signature:**

```dart
VisitResult visitVideo(NodeContext ctx, [String? src])
```

#### visitIframe()

Visit iframe elements `<iframe>`.

**Signature:**

```dart
VisitResult visitIframe(NodeContext ctx, [String? src])
```

#### visitDetails()

Visit details elements `<details>`.

**Signature:**

```dart
VisitResult visitDetails(NodeContext ctx, bool open)
```

#### visitSummary()

Visit summary elements `<summary>`.

**Signature:**

```dart
VisitResult visitSummary(NodeContext ctx, String text)
```

#### visitFigureStart()

Visit figure elements `<figure>`.

**Signature:**

```dart
VisitResult visitFigureStart(NodeContext ctx)
```

#### visitFigcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```dart
VisitResult visitFigcaption(NodeContext ctx, String text)
```

#### visitFigureEnd()

Called after processing a figure `</figure>`.

**Signature:**

```dart
VisitResult visitFigureEnd(NodeContext ctx, String output)
```

---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `String` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `String?` | `null` | Alternative text from alt attribute (for accessibility) |
| `title` | `String?` | `null` | Title attribute (often shown as tooltip) |
| `dimensions` | `List<int>?` | `null` | Image dimensions as (width, height) if available |
| `imageType` | `ImageType` | — | Image type classification |
| `attributes` | `Map<String, String>` | — | Additional HTML attributes |

---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `String` | — | The href URL value |
| `text` | `String` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `String?` | `null` | Optional title attribute (often shown as tooltip) |
| `linkType` | `LinkType` | — | Link type classification |
| `rel` | `List<String>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `Map<String, String>` | — | Additional HTML attributes |

---

#### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodeType` | `NodeType` | — | Coarse-grained node type classification |
| `tagName` | `String` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `Map<String, String>` | — | All HTML attributes as key-value pairs |
| `depth` | `int` | — | Depth in the DOM tree (0 = root) |
| `indexInParent` | `int` | — | Index among siblings (0-based) |
| `parentTag` | `String?` | `null` | Parent element's tag name (None if root) |
| `isInline` | `bool` | — | Whether this element is treated as inline vs block |

---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.Standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `removeNavigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `removeForms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

### Methods

#### default()

**Signature:**

```dart
static PreprocessingOptions default()
```

---

#### ProcessingWarning

A non-fatal diagnostic produced during HTML conversion.

Warnings indicate that conversion completed but some content may have been handled
differently than expected — for example, an image that could not be extracted, a truncated
input, or malformed HTML that was repaired with best-effort parsing.

Conversion always succeeds (returns `ConversionResult`) even when warnings are
present. Callers should inspect `warnings` and decide how to
handle them based on their tolerance for partial results:

- **Logging pipelines**: emit each warning at `WARN` level and continue.
- **Strict pipelines**: treat any warning as a hard error by checking
  `result.warnings.is_empty()` before using the output.

See `WarningKind` for the full taxonomy of warning categories.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |

---

#### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `rawJson` | `String` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schemaType` | `String?` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |

---

#### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `String` | — | The markdown rendering of this table. |

---

#### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `int` | — | Number of rows. |
| `cols` | `int` | — | Number of columns. |
| `cells` | `List<GridCell>` | `[]` | All cells in the table as a flat, sparse list. The list is ordered by `(row, col)` but is **not** a dense `rows × cols` matrix: cells that are covered by a spanning cell (via `row_span > 1` or `col_span > 1`) do not appear in the list. Only the top-left "origin" cell of a span is present, with its `row_span` and `col_span` fields set accordingly. To reconstruct the full visual grid, iterate over all cells and mark the rectangular region `[row .. row+row_span, col .. col+col_span]` as occupied by that cell. Any `(row, col)` position that is not the origin of any cell is covered by a span from an earlier cell. The length of this vec is `≤ rows * cols`. An empty table (`rows == 0 \|\| cols == 0`) produces an empty vec. |

---

#### TextAnnotation

A styling or semantic annotation that applies to a byte range within a node's text.

Unlike `DocumentNode`, which captures block-level structure (headings, paragraphs, etc.),
a `TextAnnotation` describes inline-level markup — bold, italic, links, code spans, and
similar — that spans a contiguous run of bytes inside `DocumentNode.content`'s text field.

Byte offsets (`start`..`end`) are into the UTF-8 encoded text of the parent node. The range
follows Rust slice conventions: `start` is inclusive and `end` is exclusive, so the annotated
text is `text[start as usize..end as usize]`.

Multiple annotations on the same node can overlap (e.g. bold-italic text), and they are
stored in the order they are encountered during DOM traversal.

See `AnnotationKind` for the full list of supported annotation types.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `int` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `int` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `AnnotationKind` | — | The type of annotation. |

---

#### VisitorHandle

Shareable, thread-safe handle to a user-provided HTML visitor implementation.

Pass an instance wrapped in this handle to `ConversionOptions` to
customise how the HTML document is traversed and converted to Markdown.
The handle may be cloned and shared across threads without additional
synchronisation on the caller's side.

---

### Enums

#### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `LeftToRight` | Left-to-right text flow (default for Latin scripts) |
| `RightToLeft` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `Auto` | Automatic directionality detection |

---

#### LinkType

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

#### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `DataUri` | Data URI embedded image (base64 or other encoding) |
| `InlineSvg` | Inline SVG element |
| `External` | External image URL (http/https) |
| `Relative` | Relative image path |

---

#### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `JsonLd` | JSON-LD (JSON for Linking Data) script blocks |
| `Microdata` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFa` | RDF in Attributes (RDFa) markup |

---

#### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `Minimal` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `Standard` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `Aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure. |

---

#### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `Underlined` | Underlined style (=== for h1, --- for h2). |
| `Atx` | ATX style (# for h1, ## for h2, etc.). Default. |
| `AtxClosed` | ATX closed style (# title #, with closing hashes). |

---

#### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `Spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `Tabs` | Use tabs for indentation. |

---

#### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `Normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `Strict` | Preserve all whitespace exactly as it appears in the HTML. |

---

#### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `Spaces` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `Backslash` | Backslash at end of line. Alternative Markdown syntax. |

---

#### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `Indented` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `Backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `Tildes` | Fenced code blocks with tildes (~~~). Supports language hints. |

---

#### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `DoubleEqual` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `Html` | Preserve as HTML (==text==). Original HTML tag. |
| `Bold` | Render as bold (**text**). Uses strong emphasis. |
| `None` | Strip formatting, render as plain text. No markup. |

---

#### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `Inline` | Inline links: `[text](url)`. Default. |
| `Reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |

---

#### UrlEscapeStyle

URL encoding strategy for link and image destinations.

Controls how special characters in URL destinations are handled when they
require escaping to produce valid Markdown.

The `Angle` variant (default) wraps the destination in angle brackets:
`[text](<url with spaces>)`. This is the CommonMark-specified escape hatch
but breaks when the URL itself contains `>`.

The `Percent` variant percent-encodes every character that is not an RFC 3986
unreserved character or `/`, producing a destination safe for all Markdown
parsers: `[text](url%20with%20spaces)`.

| Value | Description |
|-------|-------------|
| `Angle` | Wrap destinations that contain spaces or newlines in angle brackets. Default. |
| `Percent` | Percent-encode all characters that are not RFC 3986 unreserved or `/`. |

---

#### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `Markdown` | Standard Markdown (CommonMark compatible). Default. |
| `Djot` | Djot lightweight markup language. |
| `Plain` | Plain text output (no markup, visible text only). |

---

#### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `Heading` | A heading element (h1-h6). — Fields: `level`: `int`, `text`: `String` |
| `Paragraph` | A paragraph of text. — Fields: `text`: `String` |
| `List` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool` |
| `ListItem` | A single list item. — Fields: `text`: `String` |
| `Table` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `Image` | An image element. — Fields: `description`: `String`, `src`: `String`, `imageIndex`: `int` |
| `Code` | A code block or inline code. — Fields: `text`: `String`, `language`: `String` |
| `Quote` | A block quote container. |
| `DefinitionList` | A definition list container. |
| `DefinitionItem` | A definition list entry with term and description. — Fields: `term`: `String`, `definition`: `String` |
| `RawBlock` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `String`, `content`: `String` |
| `MetadataBlock` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `List<List<String>>` |
| `Group` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `String`, `headingLevel`: `int`, `headingText`: `String` |

---

#### AnnotationKind

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
| `Link` | A hyperlink sourced from an `<a href="...">` element. — Fields: `url`: `String`, `title`: `String` |

---

#### WarningKind

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

#### NodeType

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

#### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value | Description |
|-------|-------------|
| `Continue` | Continue with default conversion behavior |
| `Custom` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `String` |
| `Skip` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `Error` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `String` |

---

### Errors

#### ConversionError

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
