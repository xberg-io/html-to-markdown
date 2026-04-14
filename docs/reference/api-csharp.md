---
title: "C# API Reference"
---

## C# API Reference <span class="version-badge">v3.1.0</span>

### Functions

#### Convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```csharp
public static ConversionResult Convert(string html, ConversionOptions? options = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Html` | `string` | Yes | The HTML string to convert |
| `Options` | `ConversionOptions?` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `ConversionResult`

**Errors:** Throws `Error`.


---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `HeadingStyle` | `HeadingStyle` | `HeadingStyle.Atx` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `ListIndentType` | `ListIndentType` | `ListIndentType.Spaces` | How to indent nested list items (spaces or tab). |
| `ListIndentWidth` | `nuint` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `Bullets` | `string` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `StrongEmSymbol` | `string` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `EscapeAsterisks` | `bool` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `EscapeUnderscores` | `bool` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `EscapeMisc` | `bool` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `EscapeAscii` | `bool` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `CodeLanguage` | `string` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `Autolinks` | `bool` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `DefaultTitle` | `bool` | `false` | Emit a default title when no `<title>` tag is present. |
| `BrInTables` | `bool` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `HighlightStyle` | `HighlightStyle` | `HighlightStyle.DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `ExtractMetadata` | `bool` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `WhitespaceMode` | `WhitespaceMode` | `WhitespaceMode.Normalized` | Controls how whitespace is normalised during conversion. |
| `StripNewlines` | `bool` | `false` | Strip all newlines from the output, producing a single-line result. |
| `Wrap` | `bool` | `false` | Wrap long lines at `wrap_width` characters. |
| `WrapWidth` | `nuint` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `ConvertAsInline` | `bool` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `SubSymbol` | `string` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `SupSymbol` | `string` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `NewlineStyle` | `NewlineStyle` | `NewlineStyle.Spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `CodeBlockStyle` | `CodeBlockStyle` | `CodeBlockStyle.Backticks` | Style used for fenced code blocks (backticks or tilde). |
| `KeepInlineImagesIn` | `List<string>` | `new List<string>()` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `Preprocessing` | `PreprocessingOptions` | `null` | Pre-processing options applied to the HTML before conversion. |
| `Encoding` | `string` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `Debug` | `bool` | `false` | Emit debug information during conversion. |
| `StripTags` | `List<string>` | `new List<string>()` | HTML tag names whose content is stripped from the output entirely. |
| `PreserveTags` | `List<string>` | `new List<string>()` | HTML tag names that are preserved verbatim in the output. |
| `SkipImages` | `bool` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `LinkStyle` | `LinkStyle` | `LinkStyle.Inline` | Link rendering style (inline or reference). |
| `OutputFormat` | `OutputFormat` | `OutputFormat.Markdown` | Target output format (Markdown, plain text, etc.). |
| `IncludeDocumentStructure` | `bool` | `false` | Include structured document tree in result. |
| `ExtractImages` | `bool` | `false` | Extract inline images from data URIs and SVGs. |
| `MaxImageSize` | `ulong` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `CaptureSvg` | `bool` | `false` | Capture SVG elements as images. |
| `InferDimensions` | `bool` | `true` | Infer image dimensions from data. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public ConversionOptions CreateDefault()
```

###### Builder()

Create a new builder with default values.

**Signature:**

```csharp
public ConversionOptionsBuilder Builder()
```


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string?` | `null` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `Document` | `DocumentStructure?` | `null` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `Metadata` | `HtmlMetadata` | `null` | Extracted HTML metadata (title, OG, links, images, structured data). |
| `Tables` | `List<TableData>` | `new List<TableData>()` | Extracted tables with structured cell data and markdown representation. |
| `Images` | `List<string>` | `new List<string>()` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `Warnings` | `List<ProcessingWarning>` | `new List<ProcessingWarning>()` | Non-fatal processing warnings. |


---

#### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### StripTags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```csharp
public ConversionOptionsBuilder StripTags(List<string> tags)
```

###### PreserveTags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```csharp
public ConversionOptionsBuilder PreserveTags(List<string> tags)
```

###### KeepInlineImagesIn()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```csharp
public ConversionOptionsBuilder KeepInlineImagesIn(List<string> tags)
```

###### Preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```csharp
public ConversionOptionsBuilder Preprocessing(PreprocessingOptions preprocessing)
```

###### Build()

Build the final `ConversionOptions`.

**Signature:**

```csharp
public ConversionOptions Build()
```


---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `string?` | `null` | Document title from `<title>` tag |
| `Description` | `string?` | `null` | Document description from `<meta name="description">` tag |
| `Keywords` | `List<string>` | `new List<string>()` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `Author` | `string?` | `null` | Document author from `<meta name="author">` tag |
| `CanonicalUrl` | `string?` | `null` | Canonical URL from `<link rel="canonical">` tag |
| `BaseHref` | `string?` | `null` | Base URL from `<base href="">` tag for resolving relative URLs |
| `Language` | `string?` | `null` | Document language from `lang` attribute |
| `TextDirection` | `TextDirection?` | `TextDirection.LeftToRight` | Document text direction from `dir` attribute |
| `OpenGraph` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `TwitterCard` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `MetaTags` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Deterministic node identifier. |
| `Content` | `NodeContent` | — | The semantic content of this node. |
| `Parent` | `uint?` | `null` | Index of the parent node (None for root nodes). |
| `Children` | `List<uint>` | — | Indices of child nodes in reading order. |
| `Annotations` | `List<TextAnnotation>` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `Attributes` | `Dictionary<string, string>?` | `null` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Nodes` | `List<DocumentNode>` | — | All nodes in document reading order. |
| `SourceFormat` | `string?` | `null` | The source format (always "html" for this library). |


---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The text content of the cell. |
| `Row` | `uint` | — | 0-indexed row position. |
| `Col` | `uint` | — | 0-indexed column position. |
| `RowSpan` | `uint` | — | Number of rows this cell spans (default 1). |
| `ColSpan` | `uint` | — | Number of columns this cell spans (default 1). |
| `IsHeader` | `bool` | — | Whether this is a header cell (`<th>`). |


---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Level` | `byte` | — | Header level: 1 (h1) through 6 (h6) |
| `Text` | `string` | — | Normalized text content of the header |
| `Id` | `string?` | `null` | HTML id attribute if present |
| `Depth` | `nuint` | — | Document tree depth at the header element |
| `HtmlOffset` | `nuint` | — | Byte offset in original HTML document |

##### Methods

###### IsValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```csharp
public bool IsValid()
```


---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Document` | `DocumentMetadata` | `null` | Document-level metadata (title, description, canonical, etc.) |
| `Headers` | `List<HeaderMetadata>` | `new List<HeaderMetadata>()` | Extracted header elements with hierarchy |
| `Links` | `List<LinkMetadata>` | `new List<LinkMetadata>()` | Extracted hyperlinks with type classification |
| `Images` | `List<ImageMetadata>` | `new List<ImageMetadata>()` | Extracted images with source and dimensions |
| `StructuredData` | `List<StructuredData>` | `new List<StructuredData>()` | Extracted structured data blocks |


---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Src` | `string` | — | Image source (URL, data URI, or SVG content identifier) |
| `Alt` | `string?` | `null` | Alternative text from alt attribute (for accessibility) |
| `Title` | `string?` | `null` | Title attribute (often shown as tooltip) |
| `Dimensions` | `string?` | `null` | Image dimensions as (width, height) if available |
| `ImageType` | `ImageType` | — | Image type classification |
| `Attributes` | `Dictionary<string, string>` | — | Additional HTML attributes |


---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Href` | `string` | — | The href URL value |
| `Text` | `string` | — | Link text content (normalized, concatenated if mixed with elements) |
| `Title` | `string?` | `null` | Optional title attribute (often shown as tooltip) |
| `LinkType` | `LinkType` | — | Link type classification |
| `Rel` | `List<string>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `Attributes` | `Dictionary<string, string>` | — | Additional HTML attributes |

##### Methods

###### ClassifyLink()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```csharp
public LinkType ClassifyLink(string href)
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
| `ExtractDocument` | `bool` | `true` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `ExtractHeaders` | `bool` | `true` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `ExtractLinks` | `bool` | `true` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `ExtractImages` | `bool` | `true` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `ExtractStructuredData` | `bool` | `true` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `MaxStructuredDataSize` | `nuint` | `null` | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

##### Methods

###### CreateDefault()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```csharp
public MetadataConfig CreateDefault()
```

###### AnyEnabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```csharp
public bool AnyEnabled()
```


---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `Preset` | `PreprocessingPreset` | `PreprocessingPreset.Standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `RemoveNavigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `RemoveForms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public PreprocessingOptions CreateDefault()
```


---

#### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Message` | `string` | — | Human-readable warning message. |
| `Kind` | `WarningKind` | — | The category of warning. |


---

#### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `DataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `RawJson` | `string` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `SchemaType` | `string?` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

#### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Grid` | `TableGrid` | — | The structured table grid. |
| `Markdown` | `string` | — | The markdown rendering of this table. |


---

#### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Rows` | `uint` | `null` | Number of rows. |
| `Cols` | `uint` | `null` | Number of columns. |
| `Cells` | `List<GridCell>` | `new List<GridCell>()` | All cells in the table (may be fewer than rows*cols due to spans). |


---

#### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Start` | `uint` | — | Start byte offset (inclusive) into the parent node's text. |
| `End` | `uint` | — | End byte offset (exclusive) into the parent node's text. |
| `Kind` | `AnnotationKind` | — | The type of annotation. |


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
