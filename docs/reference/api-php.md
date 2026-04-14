---
title: "PHP API Reference"
---

## PHP API Reference <span class="version-badge">v3.1.0</span>

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
| `html` | `string` | Yes | The HTML string to convert |
| `options` | `?ConversionOptions` | No | Optional conversion options (defaults to `default options`) |

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
| `extractMetadata` | `bool` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
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
| `preprocessing` | `PreprocessingOptions` | `null` | Pre-processing options applied to the HTML before conversion. |
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


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `?string` | `null` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `?DocumentStructure` | `null` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HtmlMetadata` | `null` | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `array<TableData>` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `array<string>` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
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
| `textDirection` | `?TextDirection` | `TextDirection::LeftToRight` | Document text direction from `dir` attribute |
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
| `document` | `DocumentMetadata` | `null` | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `array<HeaderMetadata>` | `[]` | Extracted header elements with hierarchy |
| `links` | `array<LinkMetadata>` | `[]` | Extracted hyperlinks with type classification |
| `images` | `array<ImageMetadata>` | `[]` | Extracted images with source and dimensions |
| `structuredData` | `array<StructuredData>` | `[]` | Extracted structured data blocks |


---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `string` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `?string` | `null` | Alternative text from alt attribute (for accessibility) |
| `title` | `?string` | `null` | Title attribute (often shown as tooltip) |
| `dimensions` | `?string` | `null` | Image dimensions as (width, height) if available |
| `imageType` | `ImageType` | — | Image type classification |
| `attributes` | `array<string, string>` | — | Additional HTML attributes |


---

#### LinkMetadata

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

##### Methods

###### classifyLink()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```php
public static function classifyLink(string $href): LinkType
```


---

#### MetadataConfig

Configuration for metadata extraction granularity.

Controls which metadata types are extracted and size limits for safety.
Enables selective extraction of different metadata categories from HTML documents,
allowing fine-grained control over which types of information to collect during
the HTML-to-Markdown conversion process.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `extractDocument` | `bool` | `true` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `extractHeaders` | `bool` | `true` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `extractLinks` | `bool` | `true` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `extractImages` | `bool` | `true` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `extractStructuredData` | `bool` | `true` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `maxStructuredDataSize` | `int` | `null` | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

##### Methods

###### default()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```php
public static function default(): MetadataConfig
```

###### anyEnabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```php
public function anyEnabled(): bool
```


---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset::Standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `removeNavigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `removeForms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### default()

**Signature:**

```php
public static function default(): PreprocessingOptions
```


---

#### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `string` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |


---

#### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `rawJson` | `string` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schemaType` | `?string` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |


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
| `rows` | `int` | `null` | Number of rows. |
| `cols` | `int` | `null` | Number of columns. |
| `cells` | `array<GridCell>` | `[]` | All cells in the table (may be fewer than rows*cols due to spans). |


---

#### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `int` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `int` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `AnnotationKind` | — | The type of annotation. |


---

### Enums

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
| `Heading` | A heading element (h1-h6). |
| `Paragraph` | A paragraph of text. |
| `List` | A list container (ordered or unordered). Children are `ListItem` nodes. |
| `ListItem` | A single list item. |
| `Table` | A table with structured cell data. |
| `Image` | An image element. |
| `Code` | A code block or inline code. |
| `Quote` | A block quote container. |
| `DefinitionList` | A definition list container. |
| `DefinitionItem` | A definition list entry with term and description. |
| `RawBlock` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). |
| `MetadataBlock` | A block of key-value metadata pairs (from `<head>` meta tags). |
| `Group` | A section grouping container (auto-generated from heading hierarchy). |


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
| `Link` | A hyperlink. |


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


---

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
| `RdFa` | RDF in Attributes (RDFa) markup |


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
