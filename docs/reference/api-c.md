---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v3.1.0</span>

### Functions

#### htm_convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```c
HTMConversionResult* htm_convert(const char* html, HTMConversionOptions options);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `const char*` | Yes | The HTML string to convert |
| `options` | `HTMConversionOptions*` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `HTMConversionResult`

**Errors:** Returns `NULL` on error.


---

### Types

#### HTMConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `heading_style` | `HTMHeadingStyle` | `HTM_HTM_ATX` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `list_indent_type` | `HTMListIndentType` | `HTM_HTM_SPACES` | How to indent nested list items (spaces or tab). |
| `list_indent_width` | `uintptr_t` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `const char*` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strong_em_symbol` | `const char*` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escape_asterisks` | `bool` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escape_underscores` | `bool` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escape_misc` | `bool` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escape_ascii` | `bool` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `code_language` | `const char*` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `bool` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `default_title` | `bool` | `false` | Emit a default title when no `<title>` tag is present. |
| `br_in_tables` | `bool` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlight_style` | `HTMHighlightStyle` | `HTM_HTM_DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extract_metadata` | `bool` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespace_mode` | `HTMWhitespaceMode` | `HTM_HTM_NORMALIZED` | Controls how whitespace is normalised during conversion. |
| `strip_newlines` | `bool` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrap_width` | `uintptr_t` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convert_as_inline` | `bool` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `sub_symbol` | `const char*` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `sup_symbol` | `const char*` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newline_style` | `HTMNewlineStyle` | `HTM_HTM_SPACES` | How to encode hard line breaks (`<br>`) in Markdown. |
| `code_block_style` | `HTMCodeBlockStyle` | `HTM_HTM_BACKTICKS` | Style used for fenced code blocks (backticks or tilde). |
| `keep_inline_images_in` | `const char**` | `NULL` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `HTMPreprocessingOptions` | `NULL` | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `const char*` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `false` | Emit debug information during conversion. |
| `strip_tags` | `const char**` | `NULL` | HTML tag names whose content is stripped from the output entirely. |
| `preserve_tags` | `const char**` | `NULL` | HTML tag names that are preserved verbatim in the output. |
| `skip_images` | `bool` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `link_style` | `HTMLinkStyle` | `HTM_HTM_INLINE` | Link rendering style (inline or reference). |
| `output_format` | `HTMOutputFormat` | `HTM_HTM_MARKDOWN` | Target output format (Markdown, plain text, etc.). |
| `include_document_structure` | `bool` | `false` | Include structured document tree in result. |
| `extract_images` | `bool` | `false` | Extract inline images from data URIs and SVGs. |
| `max_image_size` | `uint64_t` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `capture_svg` | `bool` | `false` | Capture SVG elements as images. |
| `infer_dimensions` | `bool` | `true` | Infer image dimensions from data. |

##### Methods

###### htm_default()

**Signature:**

```c
HTMConversionOptions htm_default();
```

###### htm_builder()

Create a new builder with default values.

**Signature:**

```c
HTMConversionOptionsBuilder htm_builder();
```


---

#### HTMConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char**` | `NULL` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `HTMDocumentStructure*` | `NULL` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HTMHtmlMetadata` | `NULL` | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `HTMTableData*` | `NULL` | Extracted tables with structured cell data and markdown representation. |
| `images` | `const char**` | `NULL` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `HTMProcessingWarning*` | `NULL` | Non-fatal processing warnings. |


---

#### HTMConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### htm_strip_tags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```c
HTMConversionOptionsBuilder htm_strip_tags(const char** tags);
```

###### htm_preserve_tags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```c
HTMConversionOptionsBuilder htm_preserve_tags(const char** tags);
```

###### htm_keep_inline_images_in()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```c
HTMConversionOptionsBuilder htm_keep_inline_images_in(const char** tags);
```

###### htm_preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```c
HTMConversionOptionsBuilder htm_preprocessing(HTMPreprocessingOptions preprocessing);
```

###### htm_build()

Build the final `ConversionOptions`.

**Signature:**

```c
HTMConversionOptions htm_build();
```


---

#### HTMDocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `const char**` | `NULL` | Document title from `<title>` tag |
| `description` | `const char**` | `NULL` | Document description from `<meta name="description">` tag |
| `keywords` | `const char**` | `NULL` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `const char**` | `NULL` | Document author from `<meta name="author">` tag |
| `canonical_url` | `const char**` | `NULL` | Canonical URL from `<link rel="canonical">` tag |
| `base_href` | `const char**` | `NULL` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `const char**` | `NULL` | Document language from `lang` attribute |
| `text_direction` | `HTMTextDirection*` | `HTM_HTM_LEFT_TO_RIGHT` | Document text direction from `dir` attribute |
| `open_graph` | `void*` | `NULL` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitter_card` | `void*` | `NULL` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags` | `void*` | `NULL` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### HTMDocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char*` | — | Deterministic node identifier. |
| `content` | `HTMNodeContent` | — | The semantic content of this node. |
| `parent` | `uint32_t*` | `NULL` | Index of the parent node (None for root nodes). |
| `children` | `uint32_t*` | — | Indices of child nodes in reading order. |
| `annotations` | `HTMTextAnnotation*` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `void**` | `NULL` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### HTMDocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `HTMDocumentNode*` | — | All nodes in document reading order. |
| `source_format` | `const char**` | `NULL` | The source format (always "html" for this library). |


---

#### HTMGridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | The text content of the cell. |
| `row` | `uint32_t` | — | 0-indexed row position. |
| `col` | `uint32_t` | — | 0-indexed column position. |
| `row_span` | `uint32_t` | — | Number of rows this cell spans (default 1). |
| `col_span` | `uint32_t` | — | Number of columns this cell spans (default 1). |
| `is_header` | `bool` | — | Whether this is a header cell (`<th>`). |


---

#### HTMHeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `uint8_t` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `const char*` | — | Normalized text content of the header |
| `id` | `const char**` | `NULL` | HTML id attribute if present |
| `depth` | `uintptr_t` | — | Document tree depth at the header element |
| `html_offset` | `uintptr_t` | — | Byte offset in original HTML document |

##### Methods

###### htm_is_valid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```c
bool htm_is_valid();
```


---

#### HTMHtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `HTMDocumentMetadata` | `NULL` | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `HTMHeaderMetadata*` | `NULL` | Extracted header elements with hierarchy |
| `links` | `HTMLinkMetadata*` | `NULL` | Extracted hyperlinks with type classification |
| `images` | `HTMImageMetadata*` | `NULL` | Extracted images with source and dimensions |
| `structured_data` | `HTMStructuredData*` | `NULL` | Extracted structured data blocks |


---

#### HTMImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `const char*` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `const char**` | `NULL` | Alternative text from alt attribute (for accessibility) |
| `title` | `const char**` | `NULL` | Title attribute (often shown as tooltip) |
| `dimensions` | `const char**` | `NULL` | Image dimensions as (width, height) if available |
| `image_type` | `HTMImageType` | — | Image type classification |
| `attributes` | `void*` | — | Additional HTML attributes |


---

#### HTMLinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `const char*` | — | The href URL value |
| `text` | `const char*` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `const char**` | `NULL` | Optional title attribute (often shown as tooltip) |
| `link_type` | `HTMLinkType` | — | Link type classification |
| `rel` | `const char**` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `void*` | — | Additional HTML attributes |

##### Methods

###### htm_classify_link()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```c
HTMLinkType htm_classify_link(const char* href);
```


---

#### HTMMetadataConfig

Configuration for metadata extraction granularity.

Controls which metadata types are extracted and size limits for safety.
Enables selective extraction of different metadata categories from HTML documents,
allowing fine-grained control over which types of information to collect during
the HTML-to-Markdown conversion process.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `extract_document` | `bool` | `true` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `extract_headers` | `bool` | `true` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `extract_links` | `bool` | `true` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `extract_images` | `bool` | `true` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `extract_structured_data` | `bool` | `true` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `max_structured_data_size` | `uintptr_t` | `NULL` | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

##### Methods

###### htm_default()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```c
HTMMetadataConfig htm_default();
```

###### htm_any_enabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```c
bool htm_any_enabled();
```


---

#### HTMPreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `preset` | `HTMPreprocessingPreset` | `HTM_HTM_STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### htm_default()

**Signature:**

```c
HTMPreprocessingOptions htm_default();
```


---

#### HTMProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `const char*` | — | Human-readable warning message. |
| `kind` | `HTMWarningKind` | — | The category of warning. |


---

#### HTMStructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data_type` | `HTMStructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `raw_json` | `const char*` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schema_type` | `const char**` | `NULL` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

#### HTMTableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `HTMTableGrid` | — | The structured table grid. |
| `markdown` | `const char*` | — | The markdown rendering of this table. |


---

#### HTMTableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `uint32_t` | `NULL` | Number of rows. |
| `cols` | `uint32_t` | `NULL` | Number of columns. |
| `cells` | `HTMGridCell*` | `NULL` | All cells in the table (may be fewer than rows*cols due to spans). |


---

#### HTMTextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `uint32_t` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `uint32_t` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `HTMAnnotationKind` | — | The type of annotation. |


---

### Enums

#### HTMPreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `HTM_MINIMAL` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `HTM_STANDARD` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `HTM_AGGRESSIVE` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

#### HTMHeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `HTM_UNDERLINED` | Underlined style (=== for h1, --- for h2). |
| `HTM_ATX` | ATX style (# for h1, ## for h2, etc.). Default. |
| `HTM_ATX_CLOSED` | ATX closed style (# title #, with closing hashes). |


---

#### HTMListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `HTM_SPACES` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `HTM_TABS` | Use tabs for indentation. |


---

#### HTMWhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `HTM_NORMALIZED` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `HTM_STRICT` | Preserve all whitespace exactly as it appears in the HTML. |


---

#### HTMNewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `HTM_SPACES` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `HTM_BACKSLASH` | Backslash at end of line. Alternative Markdown syntax. |


---

#### HTMCodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `HTM_INDENTED` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `HTM_BACKTICKS` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `HTM_TILDES` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

#### HTMHighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `HTM_DOUBLE_EQUAL` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `HTM_HTML` | Preserve as HTML (==text==). Original HTML tag. |
| `HTM_BOLD` | Render as bold (**text**). Uses strong emphasis. |
| `HTM_NONE` | Strip formatting, render as plain text. No markup. |


---

#### HTMLinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `HTM_INLINE` | Inline links: `[text](url)`. Default. |
| `HTM_REFERENCE` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

#### HTMOutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `HTM_MARKDOWN` | Standard Markdown (CommonMark compatible). Default. |
| `HTM_DJOT` | Djot lightweight markup language. |
| `HTM_PLAIN` | Plain text output (no markup, visible text only). |


---

#### HTMNodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `HTM_HEADING` | A heading element (h1-h6). |
| `HTM_PARAGRAPH` | A paragraph of text. |
| `HTM_LIST` | A list container (ordered or unordered). Children are `ListItem` nodes. |
| `HTM_LIST_ITEM` | A single list item. |
| `HTM_TABLE` | A table with structured cell data. |
| `HTM_IMAGE` | An image element. |
| `HTM_CODE` | A code block or inline code. |
| `HTM_QUOTE` | A block quote container. |
| `HTM_DEFINITION_LIST` | A definition list container. |
| `HTM_DEFINITION_ITEM` | A definition list entry with term and description. |
| `HTM_RAW_BLOCK` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). |
| `HTM_METADATA_BLOCK` | A block of key-value metadata pairs (from `<head>` meta tags). |
| `HTM_GROUP` | A section grouping container (auto-generated from heading hierarchy). |


---

#### HTMAnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `HTM_BOLD` | Bold / strong emphasis. |
| `HTM_ITALIC` | Italic / emphasis. |
| `HTM_UNDERLINE` | Underline. |
| `HTM_STRIKETHROUGH` | Strikethrough / deleted text. |
| `HTM_CODE` | Inline code. |
| `HTM_SUBSCRIPT` | Subscript text. |
| `HTM_SUPERSCRIPT` | Superscript text. |
| `HTM_HIGHLIGHT` | Highlighted / marked text. |
| `HTM_LINK` | A hyperlink. |


---

#### HTMWarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `HTM_IMAGE_EXTRACTION_FAILED` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `HTM_ENCODING_FALLBACK` | The input encoding was not recognized; fell back to UTF-8. |
| `HTM_TRUNCATED_INPUT` | The input was truncated due to size limits. |
| `HTM_MALFORMED_HTML` | The HTML was malformed but processing continued with best effort. |
| `HTM_SANITIZATION_APPLIED` | Sanitization was applied to remove potentially unsafe content. |


---

#### HTMTextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `HTM_LEFT_TO_RIGHT` | Left-to-right text flow (default for Latin scripts) |
| `HTM_RIGHT_TO_LEFT` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `HTM_AUTO` | Automatic directionality detection |


---

#### HTMLinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value | Description |
|-------|-------------|
| `HTM_ANCHOR` | Anchor link within same document (href starts with #) |
| `HTM_INTERNAL` | Internal link within same domain |
| `HTM_EXTERNAL` | External link to different domain |
| `HTM_EMAIL` | Email link (mailto:) |
| `HTM_PHONE` | Phone link (tel:) |
| `HTM_OTHER` | Other protocol or unclassifiable |


---

#### HTMImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `HTM_DATA_URI` | Data URI embedded image (base64 or other encoding) |
| `HTM_INLINE_SVG` | Inline SVG element |
| `HTM_EXTERNAL` | External image URL (http/https) |
| `HTM_RELATIVE` | Relative image path |


---

#### HTMStructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `HTM_JSON_LD` | JSON-LD (JSON for Linking Data) script blocks |
| `HTM_MICRODATA` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `HTM_RD_FA` | RDF in Attributes (RDFa) markup |


---

### Errors

#### HTMConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `HTM_PARSE_ERROR` | HTML parsing error |
| `HTM_SANITIZATION_ERROR` | HTML sanitization error |
| `HTM_CONFIG_ERROR` | Invalid configuration |
| `HTM_IO_ERROR` | I/O error |
| `HTM_PANIC` | Internal error caught during conversion |
| `HTM_INVALID_INPUT` | Invalid input data |
| `HTM_OTHER` | Generic conversion error |


---
