---
title: "Ruby API Reference"
---

## Ruby API Reference <span class="version-badge">v3.1.0</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```ruby
def self.convert(html, options: nil)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The HTML string to convert |
| `options` | `ConversionOptions?` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `ConversionResult`

**Errors:** Raises `Error`.


---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `heading_style` | `HeadingStyle` | `:atx` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `list_indent_type` | `ListIndentType` | `:spaces` | How to indent nested list items (spaces or tab). |
| `list_indent_width` | `Integer` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `String` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strong_em_symbol` | `String` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escape_asterisks` | `Boolean` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escape_underscores` | `Boolean` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escape_misc` | `Boolean` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escape_ascii` | `Boolean` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `code_language` | `String` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `Boolean` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `default_title` | `Boolean` | `false` | Emit a default title when no `<title>` tag is present. |
| `br_in_tables` | `Boolean` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlight_style` | `HighlightStyle` | `:double_equal` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extract_metadata` | `Boolean` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespace_mode` | `WhitespaceMode` | `:normalized` | Controls how whitespace is normalised during conversion. |
| `strip_newlines` | `Boolean` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `Boolean` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrap_width` | `Integer` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convert_as_inline` | `Boolean` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `sub_symbol` | `String` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `sup_symbol` | `String` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newline_style` | `NewlineStyle` | `:spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `code_block_style` | `CodeBlockStyle` | `:backticks` | Style used for fenced code blocks (backticks or tilde). |
| `keep_inline_images_in` | `Array<String>` | `[]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | `nil` | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `String` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `Boolean` | `false` | Emit debug information during conversion. |
| `strip_tags` | `Array<String>` | `[]` | HTML tag names whose content is stripped from the output entirely. |
| `preserve_tags` | `Array<String>` | `[]` | HTML tag names that are preserved verbatim in the output. |
| `skip_images` | `Boolean` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `link_style` | `LinkStyle` | `:inline` | Link rendering style (inline or reference). |
| `output_format` | `OutputFormat` | `:markdown` | Target output format (Markdown, plain text, etc.). |
| `include_document_structure` | `Boolean` | `false` | Include structured document tree in result. |
| `extract_images` | `Boolean` | `false` | Extract inline images from data URIs and SVGs. |
| `max_image_size` | `Integer` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `capture_svg` | `Boolean` | `false` | Capture SVG elements as images. |
| `infer_dimensions` | `Boolean` | `true` | Infer image dimensions from data. |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

###### builder()

Create a new builder with default values.

**Signature:**

```ruby
def self.builder()
```


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String?` | `nil` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `DocumentStructure?` | `nil` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HtmlMetadata` | `nil` | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `Array<TableData>` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `Array<String>` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `Array<ProcessingWarning>` | `[]` | Non-fatal processing warnings. |


---

#### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### strip_tags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```ruby
def strip_tags(tags)
```

###### preserve_tags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```ruby
def preserve_tags(tags)
```

###### keep_inline_images_in()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```ruby
def keep_inline_images_in(tags)
```

###### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```ruby
def preprocessing(preprocessing)
```

###### build()

Build the final `ConversionOptions`.

**Signature:**

```ruby
def build()
```


---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String?` | `nil` | Document title from `<title>` tag |
| `description` | `String?` | `nil` | Document description from `<meta name="description">` tag |
| `keywords` | `Array<String>` | `[]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `String?` | `nil` | Document author from `<meta name="author">` tag |
| `canonical_url` | `String?` | `nil` | Canonical URL from `<link rel="canonical">` tag |
| `base_href` | `String?` | `nil` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `String?` | `nil` | Document language from `lang` attribute |
| `text_direction` | `TextDirection?` | `:left_to_right` | Document text direction from `dir` attribute |
| `open_graph` | `Hash{String=>String}` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitter_card` | `Hash{String=>String}` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags` | `Hash{String=>String}` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `Integer?` | `nil` | Index of the parent node (None for root nodes). |
| `children` | `Array<Integer>` | — | Indices of child nodes in reading order. |
| `annotations` | `Array<TextAnnotation>` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `Hash{String=>String}?` | `nil` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `Array<DocumentNode>` | — | All nodes in document reading order. |
| `source_format` | `String?` | `nil` | The source format (always "html" for this library). |


---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The text content of the cell. |
| `row` | `Integer` | — | 0-indexed row position. |
| `col` | `Integer` | — | 0-indexed column position. |
| `row_span` | `Integer` | — | Number of rows this cell spans (default 1). |
| `col_span` | `Integer` | — | Number of columns this cell spans (default 1). |
| `is_header` | `Boolean` | — | Whether this is a header cell (`<th>`). |


---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `Integer` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `String` | — | Normalized text content of the header |
| `id` | `String?` | `nil` | HTML id attribute if present |
| `depth` | `Integer` | — | Document tree depth at the header element |
| `html_offset` | `Integer` | — | Byte offset in original HTML document |

##### Methods

###### is_valid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```ruby
def is_valid()
```


---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | `nil` | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `Array<HeaderMetadata>` | `[]` | Extracted header elements with hierarchy |
| `links` | `Array<LinkMetadata>` | `[]` | Extracted hyperlinks with type classification |
| `images` | `Array<ImageMetadata>` | `[]` | Extracted images with source and dimensions |
| `structured_data` | `Array<StructuredData>` | `[]` | Extracted structured data blocks |


---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `String` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `String?` | `nil` | Alternative text from alt attribute (for accessibility) |
| `title` | `String?` | `nil` | Title attribute (often shown as tooltip) |
| `dimensions` | `String?` | `nil` | Image dimensions as (width, height) if available |
| `image_type` | `ImageType` | — | Image type classification |
| `attributes` | `Hash{String=>String}` | — | Additional HTML attributes |


---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `String` | — | The href URL value |
| `text` | `String` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `String?` | `nil` | Optional title attribute (often shown as tooltip) |
| `link_type` | `LinkType` | — | Link type classification |
| `rel` | `Array<String>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `Hash{String=>String}` | — | Additional HTML attributes |

##### Methods

###### classify_link()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```ruby
def self.classify_link(href)
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
| `extract_document` | `Boolean` | `true` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `extract_headers` | `Boolean` | `true` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `extract_links` | `Boolean` | `true` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `extract_images` | `Boolean` | `true` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `extract_structured_data` | `Boolean` | `true` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `max_structured_data_size` | `Integer` | `nil` | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

##### Methods

###### default()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```ruby
def self.default()
```

###### any_enabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```ruby
def any_enabled()
```


---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `Boolean` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `:standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `Boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `Boolean` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```


---

#### ProcessingWarning

A non-fatal warning generated during HTML processing.

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
| `data_type` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `raw_json` | `String` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schema_type` | `String?` | `nil` | Schema type if detectable (e.g., "Article", "Event", "Product") |


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
| `rows` | `Integer` | `nil` | Number of rows. |
| `cols` | `Integer` | `nil` | Number of columns. |
| `cells` | `Array<GridCell>` | `[]` | All cells in the table (may be fewer than rows*cols due to spans). |


---

#### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `Integer` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `Integer` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `AnnotationKind` | — | The type of annotation. |


---

### Enums

#### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `minimal` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `standard` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

#### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `underlined` | Underlined style (=== for h1, --- for h2). |
| `atx` | ATX style (# for h1, ## for h2, etc.). Default. |
| `atx_closed` | ATX closed style (# title #, with closing hashes). |


---

#### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `tabs` | Use tabs for indentation. |


---

#### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `strict` | Preserve all whitespace exactly as it appears in the HTML. |


---

#### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `spaces` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `backslash` | Backslash at end of line. Alternative Markdown syntax. |


---

#### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `indented` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `tildes` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

#### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `double_equal` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `html` | Preserve as HTML (==text==). Original HTML tag. |
| `bold` | Render as bold (**text**). Uses strong emphasis. |
| `none` | Strip formatting, render as plain text. No markup. |


---

#### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `inline` | Inline links: `[text](url)`. Default. |
| `reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

#### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `markdown` | Standard Markdown (CommonMark compatible). Default. |
| `djot` | Djot lightweight markup language. |
| `plain` | Plain text output (no markup, visible text only). |


---

#### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `heading` | A heading element (h1-h6). |
| `paragraph` | A paragraph of text. |
| `list` | A list container (ordered or unordered). Children are `ListItem` nodes. |
| `list_item` | A single list item. |
| `table` | A table with structured cell data. |
| `image` | An image element. |
| `code` | A code block or inline code. |
| `quote` | A block quote container. |
| `definition_list` | A definition list container. |
| `definition_item` | A definition list entry with term and description. |
| `raw_block` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). |
| `metadata_block` | A block of key-value metadata pairs (from `<head>` meta tags). |
| `group` | A section grouping container (auto-generated from heading hierarchy). |


---

#### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `bold` | Bold / strong emphasis. |
| `italic` | Italic / emphasis. |
| `underline` | Underline. |
| `strikethrough` | Strikethrough / deleted text. |
| `code` | Inline code. |
| `subscript` | Subscript text. |
| `superscript` | Superscript text. |
| `highlight` | Highlighted / marked text. |
| `link` | A hyperlink. |


---

#### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `image_extraction_failed` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `encoding_fallback` | The input encoding was not recognized; fell back to UTF-8. |
| `truncated_input` | The input was truncated due to size limits. |
| `malformed_html` | The HTML was malformed but processing continued with best effort. |
| `sanitization_applied` | Sanitization was applied to remove potentially unsafe content. |


---

#### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `left_to_right` | Left-to-right text flow (default for Latin scripts) |
| `right_to_left` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `auto` | Automatic directionality detection |


---

#### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value | Description |
|-------|-------------|
| `anchor` | Anchor link within same document (href starts with #) |
| `internal` | Internal link within same domain |
| `external` | External link to different domain |
| `email` | Email link (mailto:) |
| `phone` | Phone link (tel:) |
| `other` | Other protocol or unclassifiable |


---

#### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `data_uri` | Data URI embedded image (base64 or other encoding) |
| `inline_svg` | Inline SVG element |
| `external` | External image URL (http/https) |
| `relative` | Relative image path |


---

#### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `json_ld` | JSON-LD (JSON for Linking Data) script blocks |
| `microdata` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `rd_fa` | RDF in Attributes (RDFa) markup |


---

### Errors

#### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `parse_error` | HTML parsing error |
| `sanitization_error` | HTML sanitization error |
| `config_error` | Invalid configuration |
| `io_error` | I/O error |
| `panic` | Internal error caught during conversion |
| `invalid_input` | Invalid input data |
| `other` | Generic conversion error |


---
