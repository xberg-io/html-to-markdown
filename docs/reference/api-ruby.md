---
title: "Ruby API Reference"
---

## Ruby API Reference <span class="version-badge">v3.4.0-rc.17</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```ruby
def self.convert(html, options: nil, visitor: nil)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The HTML string to convert |
| `options` | `ConversionOptions?` | No | Optional conversion options (defaults to `default options`) |
| `visitor` | `String?` | No | The visitor |

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
| `preprocessing` | `PreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
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
| `max_depth` | `Integer?` | `nil` | Maximum DOM traversal depth. `nil` means unlimited. When set, subtrees beyond this depth are silently truncated. |
| `exclude_selectors` | `Array<String>` | `[]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]` |

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

###### apply_update()

Apply a partial update to these conversion options.

**Signature:**

```ruby
def apply_update(update)
```

###### from_update()

Create from a partial update, applying to defaults.

**Signature:**

```ruby
def self.from_update(update)
```

###### from()

**Signature:**

```ruby
def self.from(update)
```


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String?` | `nil` | Converted text output (markdown, djot, or plain text). `nil` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `DocumentStructure?` | `nil` | Structured document tree with semantic elements. Populated when `include_document_structure` is `true` in options. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `Array<TableData>` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `Array<String>` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `true` in options. |
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

###### exclude_selectors()

Set the list of CSS selectors for elements to exclude entirely from output.

**Signature:**

```ruby
def exclude_selectors(selectors)
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
| `text_direction` | `TextDirection?` | `nil` | Document text direction from `dir` attribute |
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
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `Array<HeaderMetadata>` | `[]` | Extracted header elements with hierarchy |
| `links` | `Array<LinkMetadata>` | `[]` | Extracted hyperlinks with type classification |
| `images` | `Array<ImageMetadata>` | `[]` | Extracted images with source and dimensions |
| `structured_data` | `Array<StructuredData>` | `[]` | Extracted structured data blocks |


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

#### visit_element_start()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```ruby
def visit_element_start(ctx)
```

##### visit_element_end()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```ruby
def visit_element_end(ctx, output)
```

###### visit_text()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```ruby
def visit_text(ctx, text)
```

###### visit_link()

Visit anchor links `<a href="...">`.

**Signature:**

```ruby
def visit_link(ctx, href, text, title)
```

###### visit_image()

Visit images `<img src="...">`.

**Signature:**

```ruby
def visit_image(ctx, src, alt, title)
```

###### visit_heading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```ruby
def visit_heading(ctx, level, text, id)
```

###### visit_code_block()

Visit code blocks `<pre><code>`.

**Signature:**

```ruby
def visit_code_block(ctx, lang, code)
```

###### visit_code_inline()

Visit inline code `<code>`.

**Signature:**

```ruby
def visit_code_inline(ctx, code)
```

###### visit_list_item()

Visit list items `<li>`.

**Signature:**

```ruby
def visit_list_item(ctx, ordered, marker, text)
```

###### visit_list_start()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```ruby
def visit_list_start(ctx, ordered)
```

###### visit_list_end()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```ruby
def visit_list_end(ctx, ordered, output)
```

###### visit_table_start()

Called before processing a table `<table>`.

**Signature:**

```ruby
def visit_table_start(ctx)
```

###### visit_table_row()

Visit table rows `<tr>`.

**Signature:**

```ruby
def visit_table_row(ctx, cells, is_header)
```

###### visit_table_end()

Called after processing a table `</table>`.

**Signature:**

```ruby
def visit_table_end(ctx, output)
```

###### visit_blockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```ruby
def visit_blockquote(ctx, content, depth)
```

###### visit_strong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```ruby
def visit_strong(ctx, text)
```

###### visit_emphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```ruby
def visit_emphasis(ctx, text)
```

###### visit_strikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```ruby
def visit_strikethrough(ctx, text)
```

###### visit_underline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```ruby
def visit_underline(ctx, text)
```

###### visit_subscript()

Visit subscript elements `<sub>`.

**Signature:**

```ruby
def visit_subscript(ctx, text)
```

###### visit_superscript()

Visit superscript elements `<sup>`.

**Signature:**

```ruby
def visit_superscript(ctx, text)
```

###### visit_mark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```ruby
def visit_mark(ctx, text)
```

###### visit_line_break()

Visit line break elements `<br>`.

**Signature:**

```ruby
def visit_line_break(ctx)
```

###### visit_horizontal_rule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```ruby
def visit_horizontal_rule(ctx)
```

###### visit_custom_element()

Visit custom elements (web components) or unknown tags.

**Signature:**

```ruby
def visit_custom_element(ctx, tag_name, html)
```

###### visit_definition_list_start()

Visit definition list `<dl>`.

**Signature:**

```ruby
def visit_definition_list_start(ctx)
```

###### visit_definition_term()

Visit definition term `<dt>`.

**Signature:**

```ruby
def visit_definition_term(ctx, text)
```

###### visit_definition_description()

Visit definition description `<dd>`.

**Signature:**

```ruby
def visit_definition_description(ctx, text)
```

###### visit_definition_list_end()

Called after processing a definition list `</dl>`.

**Signature:**

```ruby
def visit_definition_list_end(ctx, output)
```

###### visit_form()

Visit form elements `<form>`.

**Signature:**

```ruby
def visit_form(ctx, action, method)
```

###### visit_input()

Visit input elements `<input>`.

**Signature:**

```ruby
def visit_input(ctx, input_type, name, value)
```

###### visit_button()

Visit button elements `<button>`.

**Signature:**

```ruby
def visit_button(ctx, text)
```

###### visit_audio()

Visit audio elements `<audio>`.

**Signature:**

```ruby
def visit_audio(ctx, src)
```

###### visit_video()

Visit video elements `<video>`.

**Signature:**

```ruby
def visit_video(ctx, src)
```

###### visit_iframe()

Visit iframe elements `<iframe>`.

**Signature:**

```ruby
def visit_iframe(ctx, src)
```

###### visit_details()

Visit details elements `<details>`.

**Signature:**

```ruby
def visit_details(ctx, open)
```

###### visit_summary()

Visit summary elements `<summary>`.

**Signature:**

```ruby
def visit_summary(ctx, text)
```

###### visit_figure_start()

Visit figure elements `<figure>`.

**Signature:**

```ruby
def visit_figure_start(ctx)
```

###### visit_figcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```ruby
def visit_figcaption(ctx, text)
```

###### visit_figure_end()

Called after processing a figure `</figure>`.

**Signature:**

```ruby
def visit_figure_end(ctx, output)
```


---

##### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `String` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `String?` | `nil` | Alternative text from alt attribute (for accessibility) |
| `title` | `String?` | `nil` | Title attribute (often shown as tooltip) |
| `dimensions` | `Array<Integer>?` | `nil` | Image dimensions as (width, height) if available |
| `image_type` | `ImageType` | — | Image type classification |
| `attributes` | `Hash{String=>String}` | — | Additional HTML attributes |


---

##### LinkMetadata

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

###### Methods

###### classify_link()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```ruby
def self.classify_link(href)
```


---

##### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `node_type` | `NodeType` | — | Coarse-grained node type classification |
| `tag_name` | `String` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `Hash{String=>String}` | — | All HTML attributes as key-value pairs |
| `depth` | `Integer` | — | Depth in the DOM tree (0 = root) |
| `index_in_parent` | `Integer` | — | Index among siblings (0-based) |
| `parent_tag` | `String?` | `nil` | Parent element's tag name (None if root) |
| `is_inline` | `Boolean` | — | Whether this element is treated as inline vs block |


---

##### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `Boolean` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `:standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `Boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `Boolean` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

###### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

###### apply_update()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```ruby
def apply_update(update)
```

###### from_update()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```ruby
def self.from_update(update)
```

###### from()

**Signature:**

```ruby
def self.from(update)
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
| `data_type` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `raw_json` | `String` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schema_type` | `String?` | `nil` | Schema type if detectable (e.g., "Article", "Event", "Product") |


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
| `rows` | `Integer` | — | Number of rows. |
| `cols` | `Integer` | — | Number of columns. |
| `cells` | `Array<GridCell>` | `[]` | All cells in the table (may be fewer than rows*cols due to spans). |


---

##### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `Integer` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `Integer` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `AnnotationKind` | — | The type of annotation. |


---

#### Enums

##### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `left_to_right` | Left-to-right text flow (default for Latin scripts) |
| `right_to_left` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `auto` | Automatic directionality detection |


---

##### LinkType

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

##### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `data_uri` | Data URI embedded image (base64 or other encoding) |
| `inline_svg` | Inline SVG element |
| `external` | External image URL (http/https) |
| `relative` | Relative image path |


---

##### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `json_ld` | JSON-LD (JSON for Linking Data) script blocks |
| `microdata` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `rdfa` | RDF in Attributes (RDFa) markup |


---

##### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `minimal` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `standard` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

##### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `underlined` | Underlined style (=== for h1, --- for h2). |
| `atx` | ATX style (# for h1, ## for h2, etc.). Default. |
| `atx_closed` | ATX closed style (# title #, with closing hashes). |


---

##### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `tabs` | Use tabs for indentation. |


---

##### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `strict` | Preserve all whitespace exactly as it appears in the HTML. |


---

##### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `spaces` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `backslash` | Backslash at end of line. Alternative Markdown syntax. |


---

##### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `indented` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `tildes` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

##### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `double_equal` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `html` | Preserve as HTML (==text==). Original HTML tag. |
| `bold` | Render as bold (**text**). Uses strong emphasis. |
| `none` | Strip formatting, render as plain text. No markup. |


---

##### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `inline` | Inline links: `[text](url)`. Default. |
| `reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

##### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `markdown` | Standard Markdown (CommonMark compatible). Default. |
| `djot` | Djot lightweight markup language. |
| `plain` | Plain text output (no markup, visible text only). |


---

##### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `heading` | A heading element (h1-h6). — Fields: `level`: `Integer`, `text`: `String` |
| `paragraph` | A paragraph of text. — Fields: `text`: `String` |
| `list` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `Boolean` |
| `list_item` | A single list item. — Fields: `text`: `String` |
| `table` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `image` | An image element. — Fields: `description`: `String`, `src`: `String`, `image_index`: `Integer` |
| `code` | A code block or inline code. — Fields: `text`: `String`, `language`: `String` |
| `quote` | A block quote container. |
| `definition_list` | A definition list container. |
| `definition_item` | A definition list entry with term and description. — Fields: `term`: `String`, `definition`: `String` |
| `raw_block` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `String`, `content`: `String` |
| `metadata_block` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `Array<String>` |
| `group` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `String`, `heading_level`: `Integer`, `heading_text`: `String` |


---

##### AnnotationKind

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
| `link` | A hyperlink. — Fields: `url`: `String`, `title`: `String` |


---

##### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `image_extraction_failed` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `encoding_fallback` | The input encoding was not recognized; fell back to UTF-8. |
| `truncated_input` | The input was truncated due to size limits. |
| `malformed_html` | The HTML was malformed but processing continued with best effort. |
| `sanitization_applied` | Sanitization was applied to remove potentially unsafe content. |
| `depth_limit_exceeded` | DOM traversal was truncated because max_depth was exceeded. |


---

##### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value | Description |
|-------|-------------|
| `text` | Text node (most frequent - 100+ per document) |
| `element` | Generic element node |
| `heading` | Heading elements (h1-h6) |
| `paragraph` | Paragraph element |
| `div` | Generic div container |
| `blockquote` | Blockquote element |
| `pre` | Preformatted text block |
| `hr` | Horizontal rule |
| `list` | Ordered or unordered list (ul, ol) |
| `list_item` | List item (li) |
| `definition_list` | Definition list (dl) |
| `definition_term` | Definition term (dt) |
| `definition_description` | Definition description (dd) |
| `table` | Table element |
| `table_row` | Table row (tr) |
| `table_cell` | Table cell (td, th) |
| `table_header` | Table header cell (th) |
| `table_body` | Table body (tbody) |
| `table_head` | Table head (thead) |
| `table_foot` | Table foot (tfoot) |
| `link` | Anchor link (a) |
| `image` | Image (img) |
| `strong` | Strong/bold (strong, b) |
| `em` | Emphasis/italic (em, i) |
| `code` | Inline code (code) |
| `strikethrough` | Strikethrough (s, del, strike) |
| `underline` | Underline (u, ins) |
| `subscript` | Subscript (sub) |
| `superscript` | Superscript (sup) |
| `mark` | Mark/highlight (mark) |
| `small` | Small text (small) |
| `br` | Line break (br) |
| `span` | Span element |
| `article` | Article element |
| `section` | Section element |
| `nav` | Navigation element |
| `aside` | Aside element |
| `header` | Header element |
| `footer` | Footer element |
| `main` | Main element |
| `figure` | Figure element |
| `figcaption` | Figure caption |
| `time` | Time element |
| `details` | Details element |
| `summary` | Summary element |
| `form` | Form element |
| `input` | Input element |
| `select` | Select element |
| `option` | Option element |
| `button` | Button element |
| `textarea` | Textarea element |
| `label` | Label element |
| `fieldset` | Fieldset element |
| `legend` | Legend element |
| `audio` | Audio element |
| `video` | Video element |
| `picture` | Picture element |
| `source` | Source element |
| `iframe` | Iframe element |
| `svg` | SVG element |
| `canvas` | Canvas element |
| `ruby` | Ruby annotation |
| `rt` | Ruby text |
| `rp` | Ruby parenthesis |
| `abbr` | Abbreviation |
| `kbd` | Keyboard input |
| `samp` | Sample output |
| `var` | Variable |
| `cite` | Citation |
| `q` | Quote |
| `del` | Deleted text |
| `ins` | Inserted text |
| `data` | Data element |
| `meter` | Meter element |
| `progress` | Progress element |
| `output` | Output element |
| `template` | Template element |
| `slot` | Slot element |
| `html` | HTML root element |
| `head` | Head element |
| `body` | Body element |
| `title` | Title element |
| `meta` | Meta element |
| `link_tag` | Link element (not anchor) |
| `style` | Style element |
| `script` | Script element |
| `base` | Base element |
| `custom` | Custom element (web components) or unknown tag |


---

##### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value | Description |
|-------|-------------|
| `continue` | Continue with default conversion behavior |
| `custom` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `String` |
| `skip` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `preserve_html` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `error` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `String` |


---

#### Errors

##### ConversionError

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
