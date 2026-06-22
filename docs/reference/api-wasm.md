---
title: "WebAssembly API Reference"
---

## WebAssembly API Reference <span class="version-badge">v3.7.0</span>

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

```typescript
function convert(html: string, options?: ConversionOptions): ConversionResult
```

**Example:**

```typescript
const result = convert("value", new ConversionOptions());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `string` | Yes | The html |
| `options` | `ConversionOptions \| null` | No | The options to use |

**Returns:** `ConversionResult`

**Errors:** Throws `Error` with a descriptive message.

---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `headingStyle` | `HeadingStyle` | `HeadingStyle.Atx` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `listIndentType` | `ListIndentType` | `ListIndentType.Spaces` | How to indent nested list items (spaces or tab). |
| `listIndentWidth` | `number` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `string` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strongEmSymbol` | `string` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escapeAsterisks` | `boolean` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escapeUnderscores` | `boolean` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escapeMisc` | `boolean` | `false` | Escape miscellaneous Markdown metacharacters (`\[\]()#` etc.) in plain text. |
| `escapeAscii` | `boolean` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `codeLanguage` | `string` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `boolean` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `defaultTitle` | `boolean` | `false` | Emit a default title when no `<title>` tag is present. |
| `brInTables` | `boolean` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `compactTables` | `boolean` | `false` | Emit tables without column padding (compact GFM format). When `true`, column widths are not computed and cells are emitted with no trailing spaces. Separator rows use exactly `---` per column. Produces token-efficient output suitable for RAG / LLM contexts. Default `false` (aligned padding preserved). |
| `highlightStyle` | `HighlightStyle` | `HighlightStyle.DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extractMetadata` | `boolean` | `true` | Populate `result.metadata` with `<head>` / `<meta>` extraction (title, description, Open Graph, Twitter Card, JSON-LD, …). Default `true`. Disabling skips the metadata pass only — table extraction into `result.tables` runs unconditionally. |
| `whitespaceMode` | `WhitespaceMode` | `WhitespaceMode.Normalized` | Controls how whitespace sequences are normalised in the converted output. - `WhitespaceMode.Normalized` (default) — collapses consecutive whitespace characters (spaces, tabs, newlines) to a single space, matching browser rendering behaviour. - `WhitespaceMode.Strict` — preserves all whitespace exactly as it appears in the source HTML, including runs of spaces and embedded newlines. Choose `Strict` only when the source HTML uses deliberate whitespace (e.g. pre-formatted content outside `<pre>` tags). For most documents `Normalized` produces cleaner output. |
| `stripNewlines` | `boolean` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `boolean` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrapWidth` | `number` | `80` | Maximum output line width in characters when `wrap` is `true` (default `80`). Lines are broken at word boundaries so that no line exceeds this length. A value of `0` is treated as "no limit" — equivalent to leaving `wrap` disabled. Has no effect when `wrap` is `false`. |
| `convertAsInline` | `boolean` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `subSymbol` | `string` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `supSymbol` | `string` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newlineStyle` | `NewlineStyle` | `NewlineStyle.Spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `codeBlockStyle` | `CodeBlockStyle` | `CodeBlockStyle.Backticks` | Style used for fenced code blocks (backticks or tilde). |
| `keepInlineImagesIn` | `Array<string>` | `\[\]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Options for the HTML pre-processing pass applied before conversion begins. Pre-processing runs before the HTML is handed to the converter and can perform operations such as unwrapping redundant wrapper elements, removing tracking pixels, and normalising vendor-specific markup. See `PreprocessingOptions` for the full set of knobs. Defaults to the standard preprocessing options, which enables the standard cleaning passes. Set individual fields on `PreprocessingOptions` (or construct via `ConversionOptions.builder`) to opt in or out of specific passes. |
| `encoding` | `string` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `boolean` | `false` | Emit debug information during conversion. |
| `stripTags` | `Array<string>` | `\[\]` | HTML tag names whose content is stripped from the output entirely. |
| `preserveTags` | `Array<string>` | `\[\]` | HTML tag names that are preserved verbatim in the output. |
| `skipImages` | `boolean` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `urlEscapeStyle` | `UrlEscapeStyle` | `UrlEscapeStyle.Angle` | URL encoding strategy for link and image destinations. Controls how special characters in URL destinations are escaped: - `UrlEscapeStyle.Angle` (default) — wraps the destination in angle brackets when it contains spaces or newlines. Some parsers misinterpret `>` inside such a destination. - `UrlEscapeStyle.Percent` — percent-encodes every character that is not an RFC 3986 unreserved character or `/`, producing a destination that all Markdown parsers handle correctly even when the URL contains `<`, `>`, spaces, or parentheses. |
| `linkStyle` | `LinkStyle` | `LinkStyle.Inline` | Link rendering style (inline or reference). |
| `outputFormat` | `OutputFormat` | `OutputFormat.Markdown` | Target output format (Markdown, plain text, etc.). |
| `includeDocumentStructure` | `boolean` | `false` | Include structured document tree in result. |
| `extractImages` | `boolean` | `false` | Extract inline images from data URIs and SVGs. |
| `maxImageSize` | `number` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `captureSvg` | `boolean` | `false` | Capture SVG elements as images. |
| `inferDimensions` | `boolean` | `true` | Infer image dimensions from data. |
| `maxDepth` | `number \| null` | `null` | Maximum DOM traversal depth. `null` means unlimited. When set, subtrees beyond this depth are silently truncated. |
| `excludeSelectors` | `Array<string>` | `\[\]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `\[attribute\]`, etc. Invalid selectors are silently skipped at conversion time. Example: `\[".cookie-banner", "#ad-container", "\[role='complementary'\]"\]` |
| `tierStrategy` | `TierStrategy` | `TierStrategy.Auto` | Which conversion tier to use. - `TierStrategy.Auto` (default) — automatically choose the best path. - `TierStrategy.Tier2` — always use the Tier-2 DOM-walk path. - `TierStrategy.Tier1` — always attempt Tier-1 (testkit only). |
| `visitor` | `VisitorHandle \| null` | `null` | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`. |

##### Methods

###### default()

**Signature:**

```typescript
static default(): ConversionOptions
```

**Example:**

```typescript
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
| `content` | `string \| null` | `null` | Converted text output in the selected format: Markdown, Djot, or plain text. |
| `document` | `DocumentStructure \| null` | `null` | Structured document tree with semantic elements. Populated when the `include_document_structure` option is `true`. `null` otherwise (the default), which avoids the overhead of building the tree. When present, the tree mirrors the converted document: headings open `Group` sections, paragraphs and list items carry inline `TextAnnotation`s, and tables reference the same `TableGrid` data exposed in the result's `tables` field. Note: this field is independent of the `metadata` feature flag. Document structure collection is always available at runtime; it is gated only by the runtime option, not by a compile-time feature. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `Array<TableData>` | `\[\]` | Extracted tables with structured cell data and markdown representation. |
| `warnings` | `Array<ProcessingWarning>` | `\[\]` | Non-fatal processing warnings. |

---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `string \| null` | `null` | Document title from `<title>` tag |
| `description` | `string \| null` | `null` | Document description from `<meta name="description">` tag |
| `keywords` | `Array<string>` | `\[\]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `string \| null` | `null` | Document author from `<meta name="author">` tag |
| `canonicalUrl` | `string \| null` | `null` | Canonical URL from `<link rel="canonical">` tag |
| `baseHref` | `string \| null` | `null` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `string \| null` | `null` | Document language from `lang` attribute |
| `textDirection` | `TextDirection \| null` | `null` | Document text direction from `dir` attribute |
| `openGraph` | `Record<string, string>` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitterCard` | `Record<string, string>` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `metaTags` | `Record<string, string>` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |

---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `string` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `number \| null` | `null` | Index of the parent node (None for root nodes). |
| `children` | `Array<number>` | `/* serde(default) */` | Indices of child nodes in reading order. |
| `annotations` | `Array<TextAnnotation>` | `/* serde(default) */` | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `Record<string, string> \| null` | `null` | Format-specific attributes preserved from the source HTML element. Keys are lowercased attribute names as they appear in the HTML (e.g. `"class"`, `"id"`, `"data-foo"`). Values are the raw attribute strings, copied verbatim from the source — no HTML entity decoding is applied here. The map is `null` when no attributes are present (omitted entirely in serialized output). Not every HTML attribute is preserved: only attributes that carry semantic or structural significance for the node type are collected. For example, heading nodes capture the `"id"` attribute for anchor linking; other element-level attributes may be silently dropped. |

---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `Array<DocumentNode>` | — | All nodes in document reading order. |
| `sourceFormat` | `string \| null` | `null` | The source format (always "html" for this library). |

---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The text content of the cell. |
| `row` | `number` | — | 0-indexed row position. |
| `col` | `number` | — | 0-indexed column position. |
| `rowSpan` | `number` | `serde(default = "default_span")` | Number of rows this cell spans (default 1). |
| `colSpan` | `number` | `serde(default = "default_span")` | Number of columns this cell spans (default 1). |
| `isHeader` | `boolean` | `/* serde(default) */` | Whether this is a header cell (`<th>`). |

---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `number` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `string` | — | Normalized text content of the header |
| `id` | `string \| null` | `null` | HTML id attribute if present |
| `depth` | `number` | — | Document tree depth at the header element |
| `htmlOffset` | `number` | — | Byte offset in original HTML document |

##### Methods

###### isValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```typescript
isValid(): boolean
```

**Example:**

```typescript
const result = instance.isValid();
```

**Returns:** `boolean`

---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `Array<HeaderMetadata>` | `\[\]` | Extracted header elements with hierarchy |
| `links` | `Array<LinkMetadata>` | `\[\]` | Extracted hyperlinks with type classification |
| `images` | `Array<ImageMetadata>` | `\[\]` | Extracted images with source and dimensions |
| `structuredData` | `Array<StructuredData>` | `\[\]` | Extracted structured data blocks |

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

```typescript
visitText(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitText(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitElementStart()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```typescript
visitElementStart(ctx: NodeContext): VisitResult
```

**Example:**

```typescript
const result = instance.visitElementStart(new NodeContext());
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

```typescript
visitElementEnd(ctx: NodeContext, output: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitElementEnd(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `output` | `string` | Yes | The  output |

**Returns:** `VisitResult`

###### visitLink()

Visit anchor links `<a href="...">`.

**Signature:**

```typescript
visitLink(ctx: NodeContext, href: string, text: string, title: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitLink(new NodeContext(), "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `href` | `string` | Yes | The  href |
| `text` | `string` | Yes | The  text |
| `title` | `string \| null` | No | The  title |

**Returns:** `VisitResult`

###### visitImage()

Visit images `<img src="...">`.

**Signature:**

```typescript
visitImage(ctx: NodeContext, src: string, alt: string, title: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitImage(new NodeContext(), "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `src` | `string` | Yes | The  src |
| `alt` | `string` | Yes | The  alt |
| `title` | `string \| null` | No | The  title |

**Returns:** `VisitResult`

###### visitHeading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```typescript
visitHeading(ctx: NodeContext, level: number, text: string, id: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitHeading(new NodeContext(), 42, "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `level` | `number` | Yes | The  level |
| `text` | `string` | Yes | The  text |
| `id` | `string \| null` | No | The  id |

**Returns:** `VisitResult`

###### visitCodeBlock()

Visit code blocks `<pre><code>`.

**Signature:**

```typescript
visitCodeBlock(ctx: NodeContext, lang: string, code: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitCodeBlock(new NodeContext(), "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `lang` | `string \| null` | No | The  lang |
| `code` | `string` | Yes | The  code |

**Returns:** `VisitResult`

###### visitCodeInline()

Visit inline code `<code>`.

**Signature:**

```typescript
visitCodeInline(ctx: NodeContext, code: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitCodeInline(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `code` | `string` | Yes | The  code |

**Returns:** `VisitResult`

###### visitListItem()

Visit list items `<li>`.

**Signature:**

```typescript
visitListItem(ctx: NodeContext, ordered: boolean, marker: string, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitListItem(new NodeContext(), true, "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `ordered` | `boolean` | Yes | The  ordered |
| `marker` | `string` | Yes | The  marker |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitListStart()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```typescript
visitListStart(ctx: NodeContext, ordered: boolean): VisitResult
```

**Example:**

```typescript
const result = instance.visitListStart(new NodeContext(), true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `ordered` | `boolean` | Yes | The  ordered |

**Returns:** `VisitResult`

###### visitListEnd()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```typescript
visitListEnd(ctx: NodeContext, ordered: boolean, output: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitListEnd(new NodeContext(), true, "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `ordered` | `boolean` | Yes | The  ordered |
| `output` | `string` | Yes | The  output |

**Returns:** `VisitResult`

###### visitTableStart()

Called before processing a table `<table>`.

**Signature:**

```typescript
visitTableStart(ctx: NodeContext): VisitResult
```

**Example:**

```typescript
const result = instance.visitTableStart(new NodeContext());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitTableRow()

Visit table rows `<tr>`.

**Signature:**

```typescript
visitTableRow(ctx: NodeContext, cells: Array<string>, isHeader: boolean): VisitResult
```

**Example:**

```typescript
const result = instance.visitTableRow(new NodeContext(), [], true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `cells` | `Array<string>` | Yes | The  cells |
| `isHeader` | `boolean` | Yes | The  is header |

**Returns:** `VisitResult`

###### visitTableEnd()

Called after processing a table `</table>`.

**Signature:**

```typescript
visitTableEnd(ctx: NodeContext, output: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitTableEnd(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `output` | `string` | Yes | The  output |

**Returns:** `VisitResult`

###### visitBlockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```typescript
visitBlockquote(ctx: NodeContext, content: string, depth: number): VisitResult
```

**Example:**

```typescript
const result = instance.visitBlockquote(new NodeContext(), "value", 42);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `content` | `string` | Yes | The  content |
| `depth` | `number` | Yes | The  depth |

**Returns:** `VisitResult`

###### visitStrong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```typescript
visitStrong(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitStrong(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitEmphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```typescript
visitEmphasis(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitEmphasis(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitStrikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```typescript
visitStrikethrough(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitStrikethrough(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitUnderline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```typescript
visitUnderline(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitUnderline(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitSubscript()

Visit subscript elements `<sub>`.

**Signature:**

```typescript
visitSubscript(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitSubscript(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitSuperscript()

Visit superscript elements `<sup>`.

**Signature:**

```typescript
visitSuperscript(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitSuperscript(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitMark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```typescript
visitMark(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitMark(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitLineBreak()

Visit line break elements `<br>`.

**Signature:**

```typescript
visitLineBreak(ctx: NodeContext): VisitResult
```

**Example:**

```typescript
const result = instance.visitLineBreak(new NodeContext());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitHorizontalRule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```typescript
visitHorizontalRule(ctx: NodeContext): VisitResult
```

**Example:**

```typescript
const result = instance.visitHorizontalRule(new NodeContext());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitCustomElement()

Visit custom elements (web components) or unknown tags.

**Signature:**

```typescript
visitCustomElement(ctx: NodeContext, tagName: string, html: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitCustomElement(new NodeContext(), "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `tagName` | `string` | Yes | The  tag name |
| `html` | `string` | Yes | The  html |

**Returns:** `VisitResult`

###### visitDefinitionListStart()

Visit definition list `<dl>`.

**Signature:**

```typescript
visitDefinitionListStart(ctx: NodeContext): VisitResult
```

**Example:**

```typescript
const result = instance.visitDefinitionListStart(new NodeContext());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitDefinitionTerm()

Visit definition term `<dt>`.

**Signature:**

```typescript
visitDefinitionTerm(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitDefinitionTerm(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitDefinitionDescription()

Visit definition description `<dd>`.

**Signature:**

```typescript
visitDefinitionDescription(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitDefinitionDescription(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitDefinitionListEnd()

Called after processing a definition list `</dl>`.

**Signature:**

```typescript
visitDefinitionListEnd(ctx: NodeContext, output: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitDefinitionListEnd(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `output` | `string` | Yes | The  output |

**Returns:** `VisitResult`

###### visitForm()

Visit form elements `<form>`.

**Signature:**

```typescript
visitForm(ctx: NodeContext, action: string, method: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitForm(new NodeContext(), "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `action` | `string \| null` | No | The  action |
| `method` | `string \| null` | No | The  method |

**Returns:** `VisitResult`

###### visitInput()

Visit input elements `<input>`.

**Signature:**

```typescript
visitInput(ctx: NodeContext, inputType: string, name: string, value: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitInput(new NodeContext(), "value", "value", "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `inputType` | `string` | Yes | The  input type |
| `name` | `string \| null` | No | The  name |
| `value` | `string \| null` | No | The  value |

**Returns:** `VisitResult`

###### visitButton()

Visit button elements `<button>`.

**Signature:**

```typescript
visitButton(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitButton(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitAudio()

Visit audio elements `<audio>`.

**Signature:**

```typescript
visitAudio(ctx: NodeContext, src: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitAudio(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `src` | `string \| null` | No | The  src |

**Returns:** `VisitResult`

###### visitVideo()

Visit video elements `<video>`.

**Signature:**

```typescript
visitVideo(ctx: NodeContext, src: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitVideo(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `src` | `string \| null` | No | The  src |

**Returns:** `VisitResult`

###### visitIframe()

Visit iframe elements `<iframe>`.

**Signature:**

```typescript
visitIframe(ctx: NodeContext, src: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitIframe(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `src` | `string \| null` | No | The  src |

**Returns:** `VisitResult`

###### visitDetails()

Visit details elements `<details>`.

**Signature:**

```typescript
visitDetails(ctx: NodeContext, open: boolean): VisitResult
```

**Example:**

```typescript
const result = instance.visitDetails(new NodeContext(), true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `open` | `boolean` | Yes | The  open |

**Returns:** `VisitResult`

###### visitSummary()

Visit summary elements `<summary>`.

**Signature:**

```typescript
visitSummary(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitSummary(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitFigureStart()

Visit figure elements `<figure>`.

**Signature:**

```typescript
visitFigureStart(ctx: NodeContext): VisitResult
```

**Example:**

```typescript
const result = instance.visitFigureStart(new NodeContext());
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |

**Returns:** `VisitResult`

###### visitFigcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```typescript
visitFigcaption(ctx: NodeContext, text: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitFigcaption(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `text` | `string` | Yes | The  text |

**Returns:** `VisitResult`

###### visitFigureEnd()

Called after processing a figure `</figure>`.

**Signature:**

```typescript
visitFigureEnd(ctx: NodeContext, output: string): VisitResult
```

**Example:**

```typescript
const result = instance.visitFigureEnd(new NodeContext(), "value");
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `NodeContext` | Yes | The node context |
| `output` | `string` | Yes | The  output |

**Returns:** `VisitResult`

---

#### ImageDimensions

Image dimensions in pixels.

Binding-safe replacement for `(u32, u32)` tuples, which degrade to
`string[][]` when sanitized for cross-language binding generation.
Used by both `ImageMetadata` and
`InlineImage`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `width` | `number` | — | Width in pixels. |
| `height` | `number` | — | Height in pixels. |

---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `string` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `string \| null` | `null` | Alternative text from alt attribute (for accessibility) |
| `title` | `string \| null` | `null` | Title attribute (often shown as tooltip) |
| `dimensions` | `ImageDimensions \| null` | `null` | Image dimensions in pixels, if available. |
| `imageType` | `ImageType` | — | Image type classification |
| `attributes` | `Record<string, string>` | — | Additional HTML attributes |

---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `string` | — | The href URL value |
| `text` | `string` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `string \| null` | `null` | Optional title attribute (often shown as tooltip) |
| `linkType` | `LinkType` | — | Link type classification |
| `rel` | `Array<string>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `Record<string, string>` | — | Additional HTML attributes |

---

#### MetadataEntry

A single key-value metadata entry from `<head>` meta tags.

Binding-safe replacement for `(String, String)` tuples used in
`NodeContent.MetadataBlock`. Tuple pairs cannot be represented
across language boundaries without lossy degradation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `key` | `string` | — | Metadata key (e.g. `"title"`, `"description"`, `"og:title"`). |
| `value` | `string` | — | Metadata value. |

---

#### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, tag name, position in the DOM tree, and parent context.

##### Attributes

Access attributes via `NodeContext.attributes`, which returns
`Record<string, string>`. When the context was built with
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
| `tagName` | `string` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `depth` | `number` | — | Depth in the DOM tree (0 = root) |
| `indexInParent` | `number` | — | Index among siblings (0-based) |
| `parentTag` | `string \| null` | `null` | Parent element's tag name (None if root) |
| `isInline` | `boolean` | — | Whether this element is treated as inline vs block |

##### Methods

###### attributes()

Return a reference to the attribute map.

If the context was built with lazy attribute extraction, the
map is materialized on the first call and cached for subsequent calls.
If this method is never called, no allocation occurs for attributes.

**Signature:**

```typescript
attributes(): Record<string, string>
```

**Example:**

```typescript
const result = instance.attributes();
```

**Returns:** `Record<string, string>`

###### withOwnedAttributes()

Construct a `NodeContext` with an owned attribute map.

Use this when the caller already has materialized attributes.

**Signature:**

```typescript
static withOwnedAttributes(nodeType: NodeType, tagName: string, attributes: Record<string, string>, depth: number, indexInParent: number, parentTag: string, isInline: boolean): NodeContext
```

**Example:**

```typescript
const result = NodeContext.withOwnedAttributes(new NodeType(), "value", {}, 42, 42, "value", true);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeType` | `NodeType` | Yes | The node type |
| `tagName` | `string` | Yes | The tag name |
| `attributes` | `Record<string, string>` | Yes | The attributes |
| `depth` | `number` | Yes | The depth |
| `indexInParent` | `number` | Yes | The index in parent |
| `parentTag` | `string \| null` | No | The parent tag |
| `isInline` | `boolean` | Yes | The is inline |

**Returns:** `NodeContext`

###### intoOwned()

Promote any borrowed fields into owned storage so the context can outlive `'a`.

**Signature:**

```typescript
intoOwned(): NodeContext
```

**Example:**

```typescript
const result = instance.intoOwned();
```

**Returns:** `NodeContext`

---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.Standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `removeNavigation` | `boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `removeForms` | `boolean` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### default()

**Signature:**

```typescript
static default(): PreprocessingOptions
```

**Example:**

```typescript
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
| `message` | `string` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |

---

#### StructuredData

Structured data block (JSON-LD, Microdata, or `RDFa`).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, `RDFa`) |
| `rawJson` | `string` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schemaType` | `string \| null` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |

---

#### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `string` | — | The markdown rendering of this table. |

---

#### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `number` | — | Number of rows. |
| `cols` | `number` | — | Number of columns. |
| `cells` | `Array<GridCell>` | `\[\]` | All cells in the table as a flat, sparse list. The list is ordered by `(row, col)` but is **not** a dense `rows × cols` matrix: cells that are covered by a spanning cell (via `row_span > 1` or `col_span > 1`) do not appear in the list. Only the top-left "origin" cell of a span is present, with its `row_span` and `col_span` fields set accordingly. To reconstruct the full visual grid, iterate over all cells and mark the rectangular region `\[row .. row+row_span, col .. col+col_span\]` as occupied by that cell. Any `(row, col)` position that is not the origin of any cell is covered by a span from an earlier cell. The length of this list is `≤ rows * cols`. An empty table (`rows == 0 \|\| cols == 0`) produces an empty list. |

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
| `start` | `number` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `number` | — | End byte offset (exclusive) into the parent node's text. |
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
| `Heading` | A heading element (h1-h6). — Fields: `level`: `number`, `text`: `string` |
| `Paragraph` | A paragraph of text. — Fields: `text`: `string` |
| `List` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `boolean` |
| `ListItem` | A single list item. — Fields: `text`: `string` |
| `Table` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `Image` | An image element. — Fields: `description`: `string`, `src`: `string`, `imageIndex`: `number` |
| `Code` | A code block or inline code. — Fields: `text`: `string`, `language`: `string` |
| `Quote` | A block quote container. |
| `DefinitionList` | A definition list container. |
| `DefinitionItem` | A definition list entry with term and description. — Fields: `term`: `string`, `definition`: `string` |
| `RawBlock` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `string`, `content`: `string` |
| `MetadataBlock` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `Array<MetadataEntry>` |
| `Group` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `string`, `headingLevel`: `number`, `headingText`: `string` |

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
| `Link` | A hyperlink sourced from an `<a href="...">` element. — Fields: `url`: `string`, `title`: `string` |

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
| `Custom` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `string` |
| `Skip` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `Error` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `string` |

---

### Errors

#### ConversionError

Errors that can occur during HTML to Markdown conversion.

Errors are thrown as plain `Error` objects with descriptive messages.

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
