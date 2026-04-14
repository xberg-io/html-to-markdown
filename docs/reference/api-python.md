---
title: "Python API Reference"
---

## Python API Reference <span class="version-badge">v3.1.0</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```python
def convert(html: str, options: ConversionOptions = None) -> ConversionResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `str` | Yes | The HTML string to convert |
| `options` | `ConversionOptions | None` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `ConversionResult`

**Errors:** Raises `Error`.


---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `heading_style` | `HeadingStyle` | `HeadingStyle.ATX` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `list_indent_type` | `ListIndentType` | `ListIndentType.SPACES` | How to indent nested list items (spaces or tab). |
| `list_indent_width` | `int` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `str` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strong_em_symbol` | `str` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escape_asterisks` | `bool` | `False` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escape_underscores` | `bool` | `False` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escape_misc` | `bool` | `False` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escape_ascii` | `bool` | `False` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `code_language` | `str` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `bool` | `True` | Automatically convert bare URLs into Markdown autolinks. |
| `default_title` | `bool` | `False` | Emit a default title when no `<title>` tag is present. |
| `br_in_tables` | `bool` | `False` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlight_style` | `HighlightStyle` | `HighlightStyle.DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extract_metadata` | `bool` | `True` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespace_mode` | `WhitespaceMode` | `WhitespaceMode.NORMALIZED` | Controls how whitespace is normalised during conversion. |
| `strip_newlines` | `bool` | `False` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `False` | Wrap long lines at `wrap_width` characters. |
| `wrap_width` | `int` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convert_as_inline` | `bool` | `False` | Treat the entire document as inline content (no block-level wrappers). |
| `sub_symbol` | `str` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `sup_symbol` | `str` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newline_style` | `NewlineStyle` | `NewlineStyle.SPACES` | How to encode hard line breaks (`<br>`) in Markdown. |
| `code_block_style` | `CodeBlockStyle` | `CodeBlockStyle.BACKTICKS` | Style used for fenced code blocks (backticks or tilde). |
| `keep_inline_images_in` | `list[str]` | `[]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | `None` | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `str` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `False` | Emit debug information during conversion. |
| `strip_tags` | `list[str]` | `[]` | HTML tag names whose content is stripped from the output entirely. |
| `preserve_tags` | `list[str]` | `[]` | HTML tag names that are preserved verbatim in the output. |
| `skip_images` | `bool` | `False` | Skip conversion of `<img>` elements (omit images from output). |
| `link_style` | `LinkStyle` | `LinkStyle.INLINE` | Link rendering style (inline or reference). |
| `output_format` | `OutputFormat` | `OutputFormat.MARKDOWN` | Target output format (Markdown, plain text, etc.). |
| `include_document_structure` | `bool` | `False` | Include structured document tree in result. |
| `extract_images` | `bool` | `False` | Extract inline images from data URIs and SVGs. |
| `max_image_size` | `int` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `capture_svg` | `bool` | `False` | Capture SVG elements as images. |
| `infer_dimensions` | `bool` | `True` | Infer image dimensions from data. |

##### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> ConversionOptions
```

###### builder()

Create a new builder with default values.

**Signature:**

```python
@staticmethod
def builder() -> ConversionOptionsBuilder
```


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str | None` | `None` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `DocumentStructure | None` | `None` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HtmlMetadata` | `None` | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `list[TableData]` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `list[str]` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `list[ProcessingWarning]` | `[]` | Non-fatal processing warnings. |


---

#### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### strip_tags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```python
def strip_tags(self, tags: list[str]) -> ConversionOptionsBuilder
```

###### preserve_tags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```python
def preserve_tags(self, tags: list[str]) -> ConversionOptionsBuilder
```

###### keep_inline_images_in()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```python
def keep_inline_images_in(self, tags: list[str]) -> ConversionOptionsBuilder
```

###### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```python
def preprocessing(self, preprocessing: PreprocessingOptions) -> ConversionOptionsBuilder
```

###### build()

Build the final `ConversionOptions`.

**Signature:**

```python
def build(self) -> ConversionOptions
```


---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `str | None` | `None` | Document title from `<title>` tag |
| `description` | `str | None` | `None` | Document description from `<meta name="description">` tag |
| `keywords` | `list[str]` | `[]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `str | None` | `None` | Document author from `<meta name="author">` tag |
| `canonical_url` | `str | None` | `None` | Canonical URL from `<link rel="canonical">` tag |
| `base_href` | `str | None` | `None` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `str | None` | `None` | Document language from `lang` attribute |
| `text_direction` | `TextDirection | None` | `TextDirection.LEFT_TO_RIGHT` | Document text direction from `dir` attribute |
| `open_graph` | `dict[str, str]` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitter_card` | `dict[str, str]` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags` | `dict[str, str]` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `int | None` | `None` | Index of the parent node (None for root nodes). |
| `children` | `list[int]` | — | Indices of child nodes in reading order. |
| `annotations` | `list[TextAnnotation]` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `dict[str, str] | None` | `None` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `list[DocumentNode]` | — | All nodes in document reading order. |
| `source_format` | `str | None` | `None` | The source format (always "html" for this library). |


---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The text content of the cell. |
| `row` | `int` | — | 0-indexed row position. |
| `col` | `int` | — | 0-indexed column position. |
| `row_span` | `int` | — | Number of rows this cell spans (default 1). |
| `col_span` | `int` | — | Number of columns this cell spans (default 1). |
| `is_header` | `bool` | — | Whether this is a header cell (`<th>`). |


---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `int` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `str` | — | Normalized text content of the header |
| `id` | `str | None` | `None` | HTML id attribute if present |
| `depth` | `int` | — | Document tree depth at the header element |
| `html_offset` | `int` | — | Byte offset in original HTML document |

##### Methods

###### is_valid()

Validate that the header level is within valid range (1-6).

**Returns:**

`True` if level is 1-6, `False` otherwise.

**Signature:**

```python
def is_valid(self) -> bool
```


---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | `None` | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `list[HeaderMetadata]` | `[]` | Extracted header elements with hierarchy |
| `links` | `list[LinkMetadata]` | `[]` | Extracted hyperlinks with type classification |
| `images` | `list[ImageMetadata]` | `[]` | Extracted images with source and dimensions |
| `structured_data` | `list[StructuredData]` | `[]` | Extracted structured data blocks |


---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `str` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `str | None` | `None` | Alternative text from alt attribute (for accessibility) |
| `title` | `str | None` | `None` | Title attribute (often shown as tooltip) |
| `dimensions` | `str | None` | `None` | Image dimensions as (width, height) if available |
| `image_type` | `ImageType` | — | Image type classification |
| `attributes` | `dict[str, str]` | — | Additional HTML attributes |


---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `str` | — | The href URL value |
| `text` | `str` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `str | None` | `None` | Optional title attribute (often shown as tooltip) |
| `link_type` | `LinkType` | — | Link type classification |
| `rel` | `list[str]` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `dict[str, str]` | — | Additional HTML attributes |

##### Methods

###### classify_link()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```python
@staticmethod
def classify_link(href: str) -> LinkType
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
| `extract_document` | `bool` | `True` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `extract_headers` | `bool` | `True` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `extract_links` | `bool` | `True` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `extract_images` | `bool` | `True` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `extract_structured_data` | `bool` | `True` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `max_structured_data_size` | `int` | `None` | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

##### Methods

###### default()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```python
@staticmethod
def default() -> MetadataConfig
```

###### any_enabled()

Check if any metadata extraction is enabled.

Returns `True` if at least one extraction category is enabled, `False` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`True` if any of the extraction flags are enabled, `False` if all are disabled.

**Signature:**

```python
def any_enabled(self) -> bool
```


---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `True` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `bool` | `True` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `bool` | `True` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> PreprocessingOptions
```


---

#### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `str` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |


---

#### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data_type` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `raw_json` | `str` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schema_type` | `str | None` | `None` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

#### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `str` | — | The markdown rendering of this table. |


---

#### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `int` | `None` | Number of rows. |
| `cols` | `int` | `None` | Number of columns. |
| `cells` | `list[GridCell]` | `[]` | All cells in the table (may be fewer than rows*cols due to spans). |


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
| `MINIMAL` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `STANDARD` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `AGGRESSIVE` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

#### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `UNDERLINED` | Underlined style (=== for h1, --- for h2). |
| `ATX` | ATX style (# for h1, ## for h2, etc.). Default. |
| `ATX_CLOSED` | ATX closed style (# title #, with closing hashes). |


---

#### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `SPACES` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `TABS` | Use tabs for indentation. |


---

#### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `NORMALIZED` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `STRICT` | Preserve all whitespace exactly as it appears in the HTML. |


---

#### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `SPACES` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `BACKSLASH` | Backslash at end of line. Alternative Markdown syntax. |


---

#### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `INDENTED` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `BACKTICKS` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `TILDES` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

#### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `DOUBLE_EQUAL` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `HTML` | Preserve as HTML (==text==). Original HTML tag. |
| `BOLD` | Render as bold (**text**). Uses strong emphasis. |
| `NONE` | Strip formatting, render as plain text. No markup. |


---

#### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `INLINE` | Inline links: `[text](url)`. Default. |
| `REFERENCE` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

#### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `MARKDOWN` | Standard Markdown (CommonMark compatible). Default. |
| `DJOT` | Djot lightweight markup language. |
| `PLAIN` | Plain text output (no markup, visible text only). |


---

#### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `HEADING` | A heading element (h1-h6). |
| `PARAGRAPH` | A paragraph of text. |
| `LIST` | A list container (ordered or unordered). Children are `ListItem` nodes. |
| `LIST_ITEM` | A single list item. |
| `TABLE` | A table with structured cell data. |
| `IMAGE` | An image element. |
| `CODE` | A code block or inline code. |
| `QUOTE` | A block quote container. |
| `DEFINITION_LIST` | A definition list container. |
| `DEFINITION_ITEM` | A definition list entry with term and description. |
| `RAW_BLOCK` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). |
| `METADATA_BLOCK` | A block of key-value metadata pairs (from `<head>` meta tags). |
| `GROUP` | A section grouping container (auto-generated from heading hierarchy). |


---

#### AnnotationKind

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
| `LINK` | A hyperlink. |


---

#### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `IMAGE_EXTRACTION_FAILED` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `ENCODING_FALLBACK` | The input encoding was not recognized; fell back to UTF-8. |
| `TRUNCATED_INPUT` | The input was truncated due to size limits. |
| `MALFORMED_HTML` | The HTML was malformed but processing continued with best effort. |
| `SANITIZATION_APPLIED` | Sanitization was applied to remove potentially unsafe content. |


---

#### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `LEFT_TO_RIGHT` | Left-to-right text flow (default for Latin scripts) |
| `RIGHT_TO_LEFT` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `AUTO` | Automatic directionality detection |


---

#### LinkType

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

#### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `DATA_URI` | Data URI embedded image (base64 or other encoding) |
| `INLINE_SVG` | Inline SVG element |
| `EXTERNAL` | External image URL (http/https) |
| `RELATIVE` | Relative image path |


---

#### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `JSON_LD` | JSON-LD (JSON for Linking Data) script blocks |
| `MICRODATA` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RD_FA` | RDF in Attributes (RDFa) markup |


---

### Errors

#### ConversionError

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
