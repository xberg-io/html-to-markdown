---
title: "Zig API Reference"
---

## Zig API Reference <span class="version-badge">v3.7.2</span>

### Functions

#### convert()

Convert HTML to Markdown, Djot, or plain text.

Returns a `ConversionResult` with converted content plus optional metadata,
document structure, table data, inline images, and warnings depending on the
enabled features and conversion options.

  `Some(options)`, or `null`. Language bindings expose the same option
  fields through native constructors or optional parameters.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```zig
pub fn convert(html: [:0]const u8, options: ?ConversionOptions) Error!ConversionResult
```

**Example:**

```zig
const result = try convert("value", .{});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `\[:0\]const u8` | Yes | The html |
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
| `listIndentWidth` | `u64` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `\[:0\]const u8` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strongEmSymbol` | `\[:0\]const u8` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escapeAsterisks` | `bool` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escapeUnderscores` | `bool` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escapeMisc` | `bool` | `false` | Escape miscellaneous Markdown metacharacters (`\[\]()#` etc.) in plain text. |
| `escapeAscii` | `bool` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `codeLanguage` | `\[:0\]const u8` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `bool` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `defaultTitle` | `bool` | `false` | Emit a default title when no `<title>` tag is present. |
| `brInTables` | `bool` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `compactTables` | `bool` | `false` | Emit tables without column padding (compact GFM format). When `true`, column widths are not computed and cells are emitted with no trailing spaces. Separator rows use exactly `---` per column. Produces token-efficient output suitable for RAG / LLM contexts. Default `false` (aligned padding preserved). |
| `highlightStyle` | `HighlightStyle` | `HighlightStyle.DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extractMetadata` | `bool` | `true` | Populate `result.metadata` with `<head>` / `<meta>` extraction (title, description, Open Graph, Twitter Card, JSON-LD, …). Default `true`. Disabling skips the metadata pass only — table extraction into `result.tables` runs unconditionally. |
| `whitespaceMode` | `WhitespaceMode` | `WhitespaceMode.Normalized` | Controls how whitespace sequences are normalised in the converted output. - `WhitespaceMode.Normalized` (default) — collapses consecutive whitespace characters (spaces, tabs, newlines) to a single space, matching browser rendering behaviour. - `WhitespaceMode.Strict` — preserves all whitespace exactly as it appears in the source HTML, including runs of spaces and embedded newlines. Choose `Strict` only when the source HTML uses deliberate whitespace (e.g. pre-formatted content outside `<pre>` tags). For most documents `Normalized` produces cleaner output. |
| `stripNewlines` | `bool` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrapWidth` | `u64` | `80` | Maximum output line width in characters when `wrap` is `true` (default `80`). Lines are broken at word boundaries so that no line exceeds this length. A value of `0` is treated as "no limit" — equivalent to leaving `wrap` disabled. Has no effect when `wrap` is `false`. |
| `convertAsInline` | `bool` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `subSymbol` | `\[:0\]const u8` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `supSymbol` | `\[:0\]const u8` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newlineStyle` | `NewlineStyle` | `NewlineStyle.Spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `codeBlockStyle` | `CodeBlockStyle` | `CodeBlockStyle.Backticks` | Style used for fenced code blocks (backticks or tilde). |
| `keepInlineImagesIn` | `\[\]const \[:0\]const u8` | `\[\]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Options for the HTML pre-processing pass applied before conversion begins. Pre-processing runs before the HTML is handed to the converter and can perform operations such as unwrapping redundant wrapper elements, removing tracking pixels, and normalising vendor-specific markup. See `PreprocessingOptions` for the full set of knobs. Defaults to the standard preprocessing options, which enables the standard cleaning passes. Set individual fields on `PreprocessingOptions` (or construct via `ConversionOptions.builder`) to opt in or out of specific passes. |
| `encoding` | `\[:0\]const u8` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `false` | Emit debug information during conversion. |
| `stripTags` | `\[\]const \[:0\]const u8` | `\[\]` | HTML tag names whose content is stripped from the output entirely. |
| `preserveTags` | `\[\]const \[:0\]const u8` | `\[\]` | HTML tag names that are preserved verbatim in the output. |
| `skipImages` | `bool` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `urlEscapeStyle` | `UrlEscapeStyle` | `UrlEscapeStyle.Angle` | URL encoding strategy for link and image destinations. Controls how special characters in URL destinations are escaped: - `UrlEscapeStyle.Angle` (default) — wraps the destination in angle brackets when it contains spaces or newlines. Some parsers misinterpret `>` inside such a destination. - `UrlEscapeStyle.Percent` — percent-encodes every character that is not an RFC 3986 unreserved character or `/`, producing a destination that all Markdown parsers handle correctly even when the URL contains `<`, `>`, spaces, or parentheses. |
| `linkStyle` | `LinkStyle` | `LinkStyle.Inline` | Link rendering style (inline or reference). |
| `outputFormat` | `OutputFormat` | `OutputFormat.Markdown` | Target output format (Markdown, plain text, etc.). |
| `includeDocumentStructure` | `bool` | `false` | Include structured document tree in result. |
| `extractImages` | `bool` | `false` | Extract inline images from data URIs and SVGs. |
| `maxImageSize` | `u64` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `captureSvg` | `bool` | `false` | Capture SVG elements as images. |
| `inferDimensions` | `bool` | `true` | Infer image dimensions from data. |
| `maxDepth` | `u64?` | `null` | Maximum DOM traversal depth. `null` means unlimited. When set, subtrees beyond this depth are silently truncated. |
| `excludeSelectors` | `\[\]const \[:0\]const u8` | `\[\]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `\[attribute\]`, etc. Invalid selectors are silently skipped at conversion time. Example: `\[".cookie-banner", "#ad-container", "\[role='complementary'\]"\]` |
| `tierStrategy` | `TierStrategy` | `TierStrategy.Auto` | Which conversion tier to use. - `TierStrategy.Auto` (default) — automatically choose the best path. - `TierStrategy.Tier2` — always use the Tier-2 DOM-walk path. - `TierStrategy.Tier1` — always attempt Tier-1 (testkit only). |
| `visitor` | `VisitorHandle?` | `null` | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`. |

##### Methods

###### default()

**Signature:**

```zig
pub fn default() ConversionOptions
```

**Example:**

```zig
const result = ConversionOptions.default();
```

**Returns:** `ConversionOptions`

---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `\[:0\]const u8?` | `null` | Converted text output in the selected format: Markdown, Djot, or plain text. |
| `document` | `DocumentStructure?` | `null` | Structured document tree with semantic elements. Populated when the `include_document_structure` option is `true`. `null` otherwise (the default), which avoids the overhead of building the tree. When present, the tree mirrors the converted document: headings open `Group` sections, paragraphs and list items carry inline `TextAnnotation`s, and tables reference the same `TableGrid` data exposed in the result's `tables` field. Note: this field is independent of the `metadata` feature flag. Document structure collection is always available at runtime; it is gated only by the runtime option, not by a compile-time feature. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `\[\]const TableData` | `\[\]` | Extracted tables with structured cell data and markdown representation. |
| `warnings` | `\[\]const ProcessingWarning` | `\[\]` | Non-fatal processing warnings. |

---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `\[:0\]const u8?` | `null` | Document title from `<title>` tag |
| `description` | `\[:0\]const u8?` | `null` | Document description from `<meta name="description">` tag |
| `keywords` | `\[\]const \[:0\]const u8` | `\[\]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `\[:0\]const u8?` | `null` | Document author from `<meta name="author">` tag |
| `canonicalUrl` | `\[:0\]const u8?` | `null` | Canonical URL from `<link rel="canonical">` tag |
| `baseHref` | `\[:0\]const u8?` | `null` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `\[:0\]const u8?` | `null` | Document language from `lang` attribute |
| `textDirection` | `TextDirection?` | `null` | Document text direction from `dir` attribute |
| `openGraph` | `std.StringHashMap(\[:0\]const u8)` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitterCard` | `std.StringHashMap(\[:0\]const u8)` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `metaTags` | `std.StringHashMap(\[:0\]const u8)` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |

---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `\[:0\]const u8` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `u32?` | `null` | Index of the parent node (None for root nodes). |
| `children` | `\[\]const u32` | `/* serde(default) */` | Indices of child nodes in reading order. |
| `annotations` | `\[\]const TextAnnotation` | `/* serde(default) */` | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `std.StringHashMap(\[:0\]const u8)?` | `null` | Format-specific attributes preserved from the source HTML element. Keys are lowercased attribute names as they appear in the HTML (e.g. `"class"`, `"id"`, `"data-foo"`). Values are the raw attribute strings, copied verbatim from the source — no HTML entity decoding is applied here. The map is `null` when no attributes are present (omitted entirely in serialized output). Not every HTML attribute is preserved: only attributes that carry semantic or structural significance for the node type are collected. For example, heading nodes capture the `"id"` attribute for anchor linking; other element-level attributes may be silently dropped. |

---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `\[\]const DocumentNode` | — | All nodes in document reading order. |
| `sourceFormat` | `\[:0\]const u8?` | `null` | The source format (always "html" for this library). |

---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `\[:0\]const u8` | — | The text content of the cell. |
| `row` | `u32` | — | 0-indexed row position. |
| `col` | `u32` | — | 0-indexed column position. |
| `rowSpan` | `u32` | `serde(default = "default_span")` | Number of rows this cell spans (default 1). |
| `colSpan` | `u32` | `serde(default = "default_span")` | Number of columns this cell spans (default 1). |
| `isHeader` | `bool` | `/* serde(default) */` | Whether this is a header cell (`<th>`). |

---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `u8` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `\[:0\]const u8` | — | Normalized text content of the header |
| `id` | `\[:0\]const u8?` | `null` | HTML id attribute if present |
| `depth` | `u64` | — | Document tree depth at the header element |
| `htmlOffset` | `u64` | — | Byte offset in original HTML document |

##### Methods

###### isValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```zig
pub fn isValid(self: *const HeaderMetadata) bool
```

**Example:**

```zig
const result = instance.isValid();
```

**Returns:** `bool`

---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `\[\]const HeaderMetadata` | `\[\]` | Extracted header elements with hierarchy |
| `links` | `\[\]const LinkMetadata` | `\[\]` | Extracted hyperlinks with type classification |
| `images` | `\[\]const ImageMetadata` | `\[\]` | Extracted images with source and dimensions |
| `structuredData` | `\[\]const StructuredData` | `\[\]` | Extracted structured data blocks |

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

##### Method Naming Convention

- `visit_*_start`: Called before entering an element (pre-order traversal)
- `visit_*_end`: Called after exiting an element (post-order traversal)
- `visit_*`: Called for specific element types (e.g., `visit_link`, `visit_image`)

##### Execution Order

For a typical element like `<div><p>text</p></div>`:

1. `visit_element_start` for `<div>`
2. `visit_element_start` for `<p>`
3. `visit_text` for "text"
4. `visit_element_end` for `<p>`
5. `visit_element_end` for `</div>`

##### Performance Notes

- `visit_text` is the most frequently called method (~100+ times per document)
- Return `Continue` quickly for elements you don't need to customize
- Avoid heavy computation in visitor methods; consider caching if needed

##### Methods

###### visitText()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```zig
pub fn visitText(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitText(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitElementStart()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```zig
pub fn visitElementStart(self: *const HtmlVisitor, ctx: NodeContext) VisitResult
```

**Example:**

```zig
const result = instance.visitElementStart(.{});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitElementEnd()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```zig
pub fn visitElementEnd(self: *const HtmlVisitor, ctx: NodeContext, output: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitElementEnd(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `output` | `\[:0\]const u8` | Yes | The  output |

**Returns:** `VisitResult`

###### visitLink()

Visit anchor links `<a href="...">`.

**Signature:**

```zig
pub fn visitLink(self: *const HtmlVisitor, ctx: NodeContext, href: [:0]const u8, text: [:0]const u8, title: ?[:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitLink(.{}, "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `href` | `\[:0\]const u8` | Yes | The  href |
| `text` | `\[:0\]const u8` | Yes | The  text |
| `title` | `\[:0\]const u8?` | No | The  title |

**Returns:** `VisitResult`

###### visitImage()

Visit images `<img src="...">`.

**Signature:**

```zig
pub fn visitImage(self: *const HtmlVisitor, ctx: NodeContext, src: [:0]const u8, alt: [:0]const u8, title: ?[:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitImage(.{}, "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `src` | `\[:0\]const u8` | Yes | The  src |
| `alt` | `\[:0\]const u8` | Yes | The  alt |
| `title` | `\[:0\]const u8?` | No | The  title |

**Returns:** `VisitResult`

###### visitHeading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```zig
pub fn visitHeading(self: *const HtmlVisitor, ctx: NodeContext, level: u32, text: [:0]const u8, id: ?[:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitHeading(.{}, 42, "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `level` | `u32` | Yes | The  level |
| `text` | `\[:0\]const u8` | Yes | The  text |
| `id` | `\[:0\]const u8?` | No | The  id |

**Returns:** `VisitResult`

###### visitCodeBlock()

Visit code blocks `<pre><code>`.

**Signature:**

```zig
pub fn visitCodeBlock(self: *const HtmlVisitor, ctx: NodeContext, lang: ?[:0]const u8, code: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitCodeBlock(.{}, "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `lang` | `\[:0\]const u8?` | No | The  lang |
| `code` | `\[:0\]const u8` | Yes | The  code |

**Returns:** `VisitResult`

###### visitCodeInline()

Visit inline code `<code>`.

**Signature:**

```zig
pub fn visitCodeInline(self: *const HtmlVisitor, ctx: NodeContext, code: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitCodeInline(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `code` | `\[:0\]const u8` | Yes | The  code |

**Returns:** `VisitResult`

###### visitListItem()

Visit list items `<li>`.

**Signature:**

```zig
pub fn visitListItem(self: *const HtmlVisitor, ctx: NodeContext, ordered: bool, marker: [:0]const u8, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitListItem(.{}, true, "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `ordered` | `bool` | Yes | The  ordered |
| `marker` | `\[:0\]const u8` | Yes | The  marker |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitListStart()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```zig
pub fn visitListStart(self: *const HtmlVisitor, ctx: NodeContext, ordered: bool) VisitResult
```

**Example:**

```zig
const result = instance.visitListStart(.{}, true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `ordered` | `bool` | Yes | The  ordered |

**Returns:** `VisitResult`

###### visitListEnd()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```zig
pub fn visitListEnd(self: *const HtmlVisitor, ctx: NodeContext, ordered: bool, output: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitListEnd(.{}, true, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `ordered` | `bool` | Yes | The  ordered |
| `output` | `\[:0\]const u8` | Yes | The  output |

**Returns:** `VisitResult`

###### visitTableStart()

Called before processing a table `<table>`.

**Signature:**

```zig
pub fn visitTableStart(self: *const HtmlVisitor, ctx: NodeContext) VisitResult
```

**Example:**

```zig
const result = instance.visitTableStart(.{});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitTableRow()

Visit table rows `<tr>`.

**Signature:**

```zig
pub fn visitTableRow(self: *const HtmlVisitor, ctx: NodeContext, cells: []const [:0]const u8, is_header: bool) VisitResult
```

**Example:**

```zig
const result = instance.visitTableRow(.{}, &[_]u8{}, true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `cells` | `\[\]const \[:0\]const u8` | Yes | The  cells |
| `isHeader` | `bool` | Yes | The  is header |

**Returns:** `VisitResult`

###### visitTableEnd()

Called after processing a table `</table>`.

**Signature:**

```zig
pub fn visitTableEnd(self: *const HtmlVisitor, ctx: NodeContext, output: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitTableEnd(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `output` | `\[:0\]const u8` | Yes | The  output |

**Returns:** `VisitResult`

###### visitBlockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```zig
pub fn visitBlockquote(self: *const HtmlVisitor, ctx: NodeContext, content: [:0]const u8, depth: u64) VisitResult
```

**Example:**

```zig
const result = instance.visitBlockquote(.{}, "value", 42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `content` | `\[:0\]const u8` | Yes | The  content |
| `depth` | `u64` | Yes | The  depth |

**Returns:** `VisitResult`

###### visitStrong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```zig
pub fn visitStrong(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitStrong(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitEmphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```zig
pub fn visitEmphasis(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitEmphasis(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitStrikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```zig
pub fn visitStrikethrough(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitStrikethrough(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitUnderline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```zig
pub fn visitUnderline(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitUnderline(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitSubscript()

Visit subscript elements `<sub>`.

**Signature:**

```zig
pub fn visitSubscript(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitSubscript(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitSuperscript()

Visit superscript elements `<sup>`.

**Signature:**

```zig
pub fn visitSuperscript(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitSuperscript(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitMark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```zig
pub fn visitMark(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitMark(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitLineBreak()

Visit line break elements `<br>`.

**Signature:**

```zig
pub fn visitLineBreak(self: *const HtmlVisitor, ctx: NodeContext) VisitResult
```

**Example:**

```zig
const result = instance.visitLineBreak(.{});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitHorizontalRule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```zig
pub fn visitHorizontalRule(self: *const HtmlVisitor, ctx: NodeContext) VisitResult
```

**Example:**

```zig
const result = instance.visitHorizontalRule(.{});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitCustomElement()

Visit custom elements (web components) or unknown tags.

**Signature:**

```zig
pub fn visitCustomElement(self: *const HtmlVisitor, ctx: NodeContext, tag_name: [:0]const u8, html: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitCustomElement(.{}, "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `tagName` | `\[:0\]const u8` | Yes | The  tag name |
| `html` | `\[:0\]const u8` | Yes | The  html |

**Returns:** `VisitResult`

###### visitDefinitionListStart()

Visit definition list `<dl>`.

**Signature:**

```zig
pub fn visitDefinitionListStart(self: *const HtmlVisitor, ctx: NodeContext) VisitResult
```

**Example:**

```zig
const result = instance.visitDefinitionListStart(.{});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitDefinitionTerm()

Visit definition term `<dt>`.

**Signature:**

```zig
pub fn visitDefinitionTerm(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitDefinitionTerm(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitDefinitionDescription()

Visit definition description `<dd>`.

**Signature:**

```zig
pub fn visitDefinitionDescription(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitDefinitionDescription(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitDefinitionListEnd()

Called after processing a definition list `</dl>`.

**Signature:**

```zig
pub fn visitDefinitionListEnd(self: *const HtmlVisitor, ctx: NodeContext, output: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitDefinitionListEnd(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `output` | `\[:0\]const u8` | Yes | The  output |

**Returns:** `VisitResult`

###### visitForm()

Visit form elements `<form>`.

**Signature:**

```zig
pub fn visitForm(self: *const HtmlVisitor, ctx: NodeContext, action: ?[:0]const u8, method: ?[:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitForm(.{}, "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `action` | `\[:0\]const u8?` | No | The  action |
| `method` | `\[:0\]const u8?` | No | The  method |

**Returns:** `VisitResult`

###### visitInput()

Visit input elements `<input>`.

**Signature:**

```zig
pub fn visitInput(self: *const HtmlVisitor, ctx: NodeContext, input_type: [:0]const u8, name: ?[:0]const u8, value: ?[:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitInput(.{}, "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `inputType` | `\[:0\]const u8` | Yes | The  input type |
| `name` | `\[:0\]const u8?` | No | The  name |
| `value` | `\[:0\]const u8?` | No | The  value |

**Returns:** `VisitResult`

###### visitButton()

Visit button elements `<button>`.

**Signature:**

```zig
pub fn visitButton(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitButton(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitAudio()

Visit audio elements `<audio>`.

**Signature:**

```zig
pub fn visitAudio(self: *const HtmlVisitor, ctx: NodeContext, src: ?[:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitAudio(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `src` | `\[:0\]const u8?` | No | The  src |

**Returns:** `VisitResult`

###### visitVideo()

Visit video elements `<video>`.

**Signature:**

```zig
pub fn visitVideo(self: *const HtmlVisitor, ctx: NodeContext, src: ?[:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitVideo(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `src` | `\[:0\]const u8?` | No | The  src |

**Returns:** `VisitResult`

###### visitIframe()

Visit iframe elements `<iframe>`.

**Signature:**

```zig
pub fn visitIframe(self: *const HtmlVisitor, ctx: NodeContext, src: ?[:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitIframe(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `src` | `\[:0\]const u8?` | No | The  src |

**Returns:** `VisitResult`

###### visitDetails()

Visit details elements `<details>`.

**Signature:**

```zig
pub fn visitDetails(self: *const HtmlVisitor, ctx: NodeContext, open: bool) VisitResult
```

**Example:**

```zig
const result = instance.visitDetails(.{}, true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `open` | `bool` | Yes | The  open |

**Returns:** `VisitResult`

###### visitSummary()

Visit summary elements `<summary>`.

**Signature:**

```zig
pub fn visitSummary(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitSummary(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitFigureStart()

Visit figure elements `<figure>`.

**Signature:**

```zig
pub fn visitFigureStart(self: *const HtmlVisitor, ctx: NodeContext) VisitResult
```

**Example:**

```zig
const result = instance.visitFigureStart(.{});
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitFigcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```zig
pub fn visitFigcaption(self: *const HtmlVisitor, ctx: NodeContext, text: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitFigcaption(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `\[:0\]const u8` | Yes | The  text |

**Returns:** `VisitResult`

###### visitFigureEnd()

Called after processing a figure `</figure>`.

**Signature:**

```zig
pub fn visitFigureEnd(self: *const HtmlVisitor, ctx: NodeContext, output: [:0]const u8) VisitResult
```

**Example:**

```zig
const result = instance.visitFigureEnd(.{}, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `output` | `\[:0\]const u8` | Yes | The  output |

**Returns:** `VisitResult`

---

#### ImageDimensions

Image dimensions in pixels.

Binding-safe replacement for `(u32, u32)` tuples, which degrade to
`[]const []const []const u8` when sanitized for cross-language binding generation.
Used by both `ImageMetadata` and
`InlineImage`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `width` | `u32` | — | Width in pixels. |
| `height` | `u32` | — | Height in pixels. |

---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `\[:0\]const u8` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `\[:0\]const u8?` | `null` | Alternative text from alt attribute (for accessibility) |
| `title` | `\[:0\]const u8?` | `null` | Title attribute (often shown as tooltip) |
| `dimensions` | `ImageDimensions?` | `null` | Image dimensions in pixels, if available. |
| `imageType` | `ImageType` | — | Image type classification |
| `attributes` | `std.StringHashMap(\[:0\]const u8)` | — | Additional HTML attributes |

---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `\[:0\]const u8` | — | The href URL value |
| `text` | `\[:0\]const u8` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `\[:0\]const u8?` | `null` | Optional title attribute (often shown as tooltip) |
| `linkType` | `LinkType` | — | Link type classification |
| `rel` | `\[\]const \[:0\]const u8` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `std.StringHashMap(\[:0\]const u8)` | — | Additional HTML attributes |

---

#### MetadataEntry

A single key-value metadata entry from `<head>` meta tags.

Binding-safe replacement for `(String, String)` tuples used in
`NodeContent.MetadataBlock`. Tuple pairs cannot be represented
across language boundaries without lossy degradation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `key` | `\[:0\]const u8` | — | Metadata key (e.g. `"title"`, `"description"`, `"og:title"`). |
| `value` | `\[:0\]const u8` | — | Metadata value. |

---

#### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, tag name, position in the DOM tree, and parent context.

##### Attributes

Access attributes via `NodeContext.attributes`, which returns
`std.StringHashMap([]const u8)`. When the context was built with
lazy attribute extraction (the hot path inside the converter),
the map is only materialized on the first call — if the visitor never reads
attributes, the allocation is skipped.

##### Lifetimes

String fields use `Cow<'_, str>` so the converter can pass slices directly
out of the parsed DOM without allocating. Visitor implementations that need
to outlive the callback should call `NodeContext.into_owned`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodeType` | `NodeType` | — | Coarse-grained node type classification |
| `tagName` | `\[:0\]const u8` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `depth` | `u64` | — | Depth in the DOM tree (0 = root) |
| `indexInParent` | `u64` | — | Index among siblings (0-based) |
| `parentTag` | `\[:0\]const u8?` | `null` | Parent element's tag name (None if root) |
| `isInline` | `bool` | — | Whether this element is treated as inline vs block |

##### Methods

###### attributes()

Return a reference to the attribute map.

If the context was built with lazy attribute extraction, the
map is materialized on the first call and cached for subsequent calls.
If this method is never called, no allocation occurs for attributes.

**Signature:**

```zig
pub fn attributes(self: *const NodeContext) std.StringHashMap([:0]const u8)
```

**Example:**

```zig
const result = instance.attributes();
```

**Returns:** `std.StringHashMap([:0]const u8)`

###### withOwnedAttributes()

Construct a `NodeContext` with an owned attribute map.

Use this when the caller already has materialized attributes.

**Signature:**

```zig
pub fn withOwnedAttributes(node_type: NodeType, tag_name: [:0]const u8, attributes: std.StringHashMap([:0]const u8), depth: u64, index_in_parent: u64, parent_tag: ?[:0]const u8, is_inline: bool) NodeContext
```

**Example:**

```zig
const result = NodeContext.withOwnedAttributes(.{}, "value", .{}, 42, 42, "value", true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeType` | `NodeType` | Yes | The node type |
| `tagName` | `\[:0\]const u8` | Yes | The tag name |
| `attributes` | `std.StringHashMap(\[:0\]const u8)` | Yes | The attributes |
| `depth` | `u64` | Yes | The depth |
| `indexInParent` | `u64` | Yes | The index in parent |
| `parentTag` | `\[:0\]const u8?` | No | The parent tag |
| `isInline` | `bool` | Yes | The is inline |

**Returns:** `NodeContext`

###### intoOwned()

Promote any borrowed fields into owned storage so the context can outlive `'a`.

**Signature:**

```zig
pub fn intoOwned(self: *const NodeContext) NodeContext
```

**Example:**

```zig
const result = instance.intoOwned();
```

**Returns:** `NodeContext`

---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.Standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `removeNavigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `removeForms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### default()

**Signature:**

```zig
pub fn default() PreprocessingOptions
```

**Example:**

```zig
const result = PreprocessingOptions.default();
```

**Returns:** `PreprocessingOptions`

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
| `message` | `\[:0\]const u8` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |

---

#### StructuredData

Structured data block (JSON-LD, Microdata, or `RDFa`).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, `RDFa`) |
| `rawJson` | `\[:0\]const u8` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schemaType` | `\[:0\]const u8?` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |

---

#### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `\[:0\]const u8` | — | The markdown rendering of this table. |

---

#### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `u32` | — | Number of rows. |
| `cols` | `u32` | — | Number of columns. |
| `cells` | `\[\]const GridCell` | `\[\]` | All cells in the table as a flat, sparse list. The list is ordered by `(row, col)` but is **not** a dense `rows × cols` matrix: cells that are covered by a spanning cell (via `row_span > 1` or `col_span > 1`) do not appear in the list. Only the top-left "origin" cell of a span is present, with its `row_span` and `col_span` fields set accordingly. To reconstruct the full visual grid, iterate over all cells and mark the rectangular region `\[row .. row+row_span, col .. col+col_span\]` as occupied by that cell. Any `(row, col)` position that is not the origin of any cell is covered by a span from an earlier cell. The length of this list is `≤ rows * cols`. An empty table (`rows == 0 \|\| cols == 0`) produces an empty list. |

---

#### TextAnnotation

A styling or semantic annotation that applies to a byte range within a node's text.

Unlike `DocumentNode`, which captures block-level structure (headings, paragraphs, etc.),
a `TextAnnotation` describes inline-level markup — bold, italic, links, code spans, and
similar — that spans a contiguous run of bytes inside `DocumentNode.content`'s text field.

Byte offsets (`start`..`end`) are into the UTF-8 encoded text of the parent node. The range
is half-open: `start` is inclusive and `end` is exclusive.

Multiple annotations on the same node can overlap (e.g. bold-italic text), and they are
stored in the order they are encountered during DOM traversal.

See `AnnotationKind` for the full list of supported annotation types.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `u32` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `u32` | — | End byte offset (exclusive) into the parent node's text. |
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
| `RDFa` | RDF in Attributes (`RDFa`) markup |

---

#### TierStrategy

Controls which conversion tier is used.

| Value | Description |
|-------|-------------|
| `Auto` | Automatically pick the best tier for the input (default). Runs the classifier against the prescan report and uses Tier-1 when eligible; falls back to Tier-2 on bail or when the classifier routes to Tier-2. |
| `Tier2` | Always use the Tier-2 (`tl.parse` + walk) path, skipping Tier-1. |
| `Tier1` | Force the Tier-1 byte scanner; if it bails, fall back to Tier-2. Testkit-only; not stable API. |

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
| `Inline` | Inline links: `\[text\](url)`. Default. |
| `Reference` | Reference-style links: `\[text\]\[1\]` with `\[1\]: url` at end of document. |

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
| `Markdown` | Standard Markdown (`CommonMark` compatible). Default. |
| `Djot` | Djot lightweight markup language. |
| `Plain` | Plain text output (no markup, visible text only). |

---

#### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `Heading` | A heading element (h1-h6). — Fields: `level`: `u8`, `text`: `\[:0\]const u8` |
| `Paragraph` | A paragraph of text. — Fields: `text`: `\[:0\]const u8` |
| `List` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool` |
| `ListItem` | A single list item. — Fields: `text`: `\[:0\]const u8` |
| `Table` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `Image` | An image element. — Fields: `description`: `\[:0\]const u8`, `src`: `\[:0\]const u8`, `imageIndex`: `u32` |
| `Code` | A code block or inline code. — Fields: `text`: `\[:0\]const u8`, `language`: `\[:0\]const u8` |
| `Quote` | A block quote container. |
| `DefinitionList` | A definition list container. |
| `DefinitionItem` | A definition list entry with term and description. — Fields: `term`: `\[:0\]const u8`, `definition`: `\[:0\]const u8` |
| `RawBlock` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `\[:0\]const u8`, `content`: `\[:0\]const u8` |
| `MetadataBlock` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `\[\]const MetadataEntry` |
| `Group` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `\[:0\]const u8`, `headingLevel`: `u8`, `headingText`: `\[:0\]const u8` |

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
| `Link` | A hyperlink sourced from an `<a href="...">` element. — Fields: `url`: `\[:0\]const u8`, `title`: `\[:0\]const u8` |

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
| `DepthLimitExceeded` | DOM traversal was truncated because `max_depth` was exceeded. |

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
| `Custom` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `\[:0\]const u8` |
| `Skip` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `Error` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `\[:0\]const u8` |

---

### Errors

#### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `ParseError` | HTML parsing error |
| `SanitizationError` | HTML sanitization error |
| `ConfigError` | Invalid configuration |
| `IoError` | I/O error — stores the error message string so the variant is FFI-safe. Use `ConversionError.from(io_error)` to convert from an operating-system I/O error. |
| `Panic` | Internal error caught during conversion |
| `InvalidInput` | Invalid input data |
| `Other` | Generic conversion error |

---
