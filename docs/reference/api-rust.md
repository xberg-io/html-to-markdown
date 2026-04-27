---
title: "Rust API Reference"
---

## Rust API Reference <span class="version-badge">v3.4.0-rc.3</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```rust
pub fn convert(html: &str, options: Option<ConversionOptions>, visitor: Option<String>) -> Result<ConversionResult, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The HTML string to convert |
| `options` | `Option<ConversionOptions>` | No | Optional conversion options (defaults to `default options`) |
| `visitor` | `Option<String>` | No | The visitor |

**Returns:** `ConversionResult`

**Errors:** Returns `Err(Error)`.


---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `heading_style` | `HeadingStyle` | `HeadingStyle::Atx` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `list_indent_type` | `ListIndentType` | `ListIndentType::Spaces` | How to indent nested list items (spaces or tab). |
| `list_indent_width` | `usize` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `String` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strong_em_symbol` | `String` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escape_asterisks` | `bool` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escape_underscores` | `bool` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escape_misc` | `bool` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escape_ascii` | `bool` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `code_language` | `String` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `bool` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `default_title` | `bool` | `false` | Emit a default title when no `<title>` tag is present. |
| `br_in_tables` | `bool` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlight_style` | `HighlightStyle` | `HighlightStyle::DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extract_metadata` | `bool` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespace_mode` | `WhitespaceMode` | `WhitespaceMode::Normalized` | Controls how whitespace is normalised during conversion. |
| `strip_newlines` | `bool` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrap_width` | `usize` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convert_as_inline` | `bool` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `sub_symbol` | `String` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `sup_symbol` | `String` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newline_style` | `NewlineStyle` | `NewlineStyle::Spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `code_block_style` | `CodeBlockStyle` | `CodeBlockStyle::Backticks` | Style used for fenced code blocks (backticks or tilde). |
| `keep_inline_images_in` | `Vec<String>` | `vec![]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `String` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `false` | Emit debug information during conversion. |
| `strip_tags` | `Vec<String>` | `vec![]` | HTML tag names whose content is stripped from the output entirely. |
| `preserve_tags` | `Vec<String>` | `vec![]` | HTML tag names that are preserved verbatim in the output. |
| `skip_images` | `bool` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `link_style` | `LinkStyle` | `LinkStyle::Inline` | Link rendering style (inline or reference). |
| `output_format` | `OutputFormat` | `OutputFormat::Markdown` | Target output format (Markdown, plain text, etc.). |
| `include_document_structure` | `bool` | `false` | Include structured document tree in result. |
| `extract_images` | `bool` | `false` | Extract inline images from data URIs and SVGs. |
| `max_image_size` | `u64` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `capture_svg` | `bool` | `false` | Capture SVG elements as images. |
| `infer_dimensions` | `bool` | `true` | Infer image dimensions from data. |
| `max_depth` | `Option<usize>` | `None` | Maximum DOM traversal depth. `None` means unlimited. When set, subtrees beyond this depth are silently truncated. |
| `exclude_selectors` | `Vec<String>` | `vec![]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]` |

##### Methods

###### default()

**Signature:**

```rust
pub fn default() -> ConversionOptions
```

###### builder()

Create a new builder with default values.

**Signature:**

```rust
pub fn builder() -> ConversionOptionsBuilder
```

###### apply_update()

Apply a partial update to these conversion options.

**Signature:**

```rust
pub fn apply_update(&self, update: ConversionOptionsUpdate)
```

###### from_update()

Create from a partial update, applying to defaults.

**Signature:**

```rust
pub fn from_update(update: ConversionOptionsUpdate) -> ConversionOptions
```

###### from()

**Signature:**

```rust
pub fn from(update: ConversionOptionsUpdate) -> ConversionOptions
```


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Option<String>` | `Default::default()` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `Option<DocumentStructure>` | `Default::default()` | Structured document tree with semantic elements. Populated when `include_document_structure` is `true` in options. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `Vec<TableData>` | `vec![]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `Vec<String>` | `vec![]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `true` in options. |
| `warnings` | `Vec<ProcessingWarning>` | `vec![]` | Non-fatal processing warnings. |


---

#### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### strip_tags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```rust
pub fn strip_tags(&self, tags: Vec<String>) -> ConversionOptionsBuilder
```

###### preserve_tags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```rust
pub fn preserve_tags(&self, tags: Vec<String>) -> ConversionOptionsBuilder
```

###### keep_inline_images_in()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```rust
pub fn keep_inline_images_in(&self, tags: Vec<String>) -> ConversionOptionsBuilder
```

###### exclude_selectors()

Set the list of CSS selectors for elements to exclude entirely from output.

**Signature:**

```rust
pub fn exclude_selectors(&self, selectors: Vec<String>) -> ConversionOptionsBuilder
```

###### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```rust
pub fn preprocessing(&self, preprocessing: PreprocessingOptions) -> ConversionOptionsBuilder
```

###### build()

Build the final `ConversionOptions`.

**Signature:**

```rust
pub fn build(&self) -> ConversionOptions
```


---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `Option<String>` | `Default::default()` | Document title from `<title>` tag |
| `description` | `Option<String>` | `Default::default()` | Document description from `<meta name="description">` tag |
| `keywords` | `Vec<String>` | `vec![]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `Option<String>` | `Default::default()` | Document author from `<meta name="author">` tag |
| `canonical_url` | `Option<String>` | `Default::default()` | Canonical URL from `<link rel="canonical">` tag |
| `base_href` | `Option<String>` | `Default::default()` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `Option<String>` | `Default::default()` | Document language from `lang` attribute |
| `text_direction` | `Option<TextDirection>` | `Default::default()` | Document text direction from `dir` attribute |
| `open_graph` | `HashMap<String, String>` | `HashMap::new()` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitter_card` | `HashMap<String, String>` | `HashMap::new()` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags` | `HashMap<String, String>` | `HashMap::new()` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `Option<u32>` | `None` | Index of the parent node (None for root nodes). |
| `children` | `Vec<u32>` | — | Indices of child nodes in reading order. |
| `annotations` | `Vec<TextAnnotation>` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `Option<HashMap<String, String>>` | `None` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `Vec<DocumentNode>` | — | All nodes in document reading order. |
| `source_format` | `Option<String>` | `None` | The source format (always "html" for this library). |


---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The text content of the cell. |
| `row` | `u32` | — | 0-indexed row position. |
| `col` | `u32` | — | 0-indexed column position. |
| `row_span` | `u32` | — | Number of rows this cell spans (default 1). |
| `col_span` | `u32` | — | Number of columns this cell spans (default 1). |
| `is_header` | `bool` | — | Whether this is a header cell (`<th>`). |


---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `u8` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `String` | — | Normalized text content of the header |
| `id` | `Option<String>` | `None` | HTML id attribute if present |
| `depth` | `usize` | — | Document tree depth at the header element |
| `html_offset` | `usize` | — | Byte offset in original HTML document |

##### Methods

###### is_valid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```rust
pub fn is_valid(&self) -> bool
```


---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `Vec<HeaderMetadata>` | `vec![]` | Extracted header elements with hierarchy |
| `links` | `Vec<LinkMetadata>` | `vec![]` | Extracted hyperlinks with type classification |
| `images` | `Vec<ImageMetadata>` | `vec![]` | Extracted images with source and dimensions |
| `structured_data` | `Vec<StructuredData>` | `vec![]` | Extracted structured data blocks |


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

```rust
pub fn visit_element_start(&self, ctx: NodeContext) -> VisitResult
```

##### visit_element_end()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```rust
pub fn visit_element_end(&self, ctx: NodeContext, output: &str) -> VisitResult
```

###### visit_text()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```rust
pub fn visit_text(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_link()

Visit anchor links `<a href="...">`.

**Signature:**

```rust
pub fn visit_link(&self, ctx: NodeContext, href: &str, text: &str, title: Option<String>) -> VisitResult
```

###### visit_image()

Visit images `<img src="...">`.

**Signature:**

```rust
pub fn visit_image(&self, ctx: NodeContext, src: &str, alt: &str, title: Option<String>) -> VisitResult
```

###### visit_heading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```rust
pub fn visit_heading(&self, ctx: NodeContext, level: u32, text: &str, id: Option<String>) -> VisitResult
```

###### visit_code_block()

Visit code blocks `<pre><code>`.

**Signature:**

```rust
pub fn visit_code_block(&self, ctx: NodeContext, lang: Option<String>, code: &str) -> VisitResult
```

###### visit_code_inline()

Visit inline code `<code>`.

**Signature:**

```rust
pub fn visit_code_inline(&self, ctx: NodeContext, code: &str) -> VisitResult
```

###### visit_list_item()

Visit list items `<li>`.

**Signature:**

```rust
pub fn visit_list_item(&self, ctx: NodeContext, ordered: bool, marker: &str, text: &str) -> VisitResult
```

###### visit_list_start()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```rust
pub fn visit_list_start(&self, ctx: NodeContext, ordered: bool) -> VisitResult
```

###### visit_list_end()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```rust
pub fn visit_list_end(&self, ctx: NodeContext, ordered: bool, output: &str) -> VisitResult
```

###### visit_table_start()

Called before processing a table `<table>`.

**Signature:**

```rust
pub fn visit_table_start(&self, ctx: NodeContext) -> VisitResult
```

###### visit_table_row()

Visit table rows `<tr>`.

**Signature:**

```rust
pub fn visit_table_row(&self, ctx: NodeContext, cells: Vec<String>, is_header: bool) -> VisitResult
```

###### visit_table_end()

Called after processing a table `</table>`.

**Signature:**

```rust
pub fn visit_table_end(&self, ctx: NodeContext, output: &str) -> VisitResult
```

###### visit_blockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```rust
pub fn visit_blockquote(&self, ctx: NodeContext, content: &str, depth: usize) -> VisitResult
```

###### visit_strong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```rust
pub fn visit_strong(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_emphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```rust
pub fn visit_emphasis(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_strikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```rust
pub fn visit_strikethrough(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_underline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```rust
pub fn visit_underline(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_subscript()

Visit subscript elements `<sub>`.

**Signature:**

```rust
pub fn visit_subscript(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_superscript()

Visit superscript elements `<sup>`.

**Signature:**

```rust
pub fn visit_superscript(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_mark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```rust
pub fn visit_mark(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_line_break()

Visit line break elements `<br>`.

**Signature:**

```rust
pub fn visit_line_break(&self, ctx: NodeContext) -> VisitResult
```

###### visit_horizontal_rule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```rust
pub fn visit_horizontal_rule(&self, ctx: NodeContext) -> VisitResult
```

###### visit_custom_element()

Visit custom elements (web components) or unknown tags.

**Signature:**

```rust
pub fn visit_custom_element(&self, ctx: NodeContext, tag_name: &str, html: &str) -> VisitResult
```

###### visit_definition_list_start()

Visit definition list `<dl>`.

**Signature:**

```rust
pub fn visit_definition_list_start(&self, ctx: NodeContext) -> VisitResult
```

###### visit_definition_term()

Visit definition term `<dt>`.

**Signature:**

```rust
pub fn visit_definition_term(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_definition_description()

Visit definition description `<dd>`.

**Signature:**

```rust
pub fn visit_definition_description(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_definition_list_end()

Called after processing a definition list `</dl>`.

**Signature:**

```rust
pub fn visit_definition_list_end(&self, ctx: NodeContext, output: &str) -> VisitResult
```

###### visit_form()

Visit form elements `<form>`.

**Signature:**

```rust
pub fn visit_form(&self, ctx: NodeContext, action: Option<String>, method: Option<String>) -> VisitResult
```

###### visit_input()

Visit input elements `<input>`.

**Signature:**

```rust
pub fn visit_input(&self, ctx: NodeContext, input_type: &str, name: Option<String>, value: Option<String>) -> VisitResult
```

###### visit_button()

Visit button elements `<button>`.

**Signature:**

```rust
pub fn visit_button(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_audio()

Visit audio elements `<audio>`.

**Signature:**

```rust
pub fn visit_audio(&self, ctx: NodeContext, src: Option<String>) -> VisitResult
```

###### visit_video()

Visit video elements `<video>`.

**Signature:**

```rust
pub fn visit_video(&self, ctx: NodeContext, src: Option<String>) -> VisitResult
```

###### visit_iframe()

Visit iframe elements `<iframe>`.

**Signature:**

```rust
pub fn visit_iframe(&self, ctx: NodeContext, src: Option<String>) -> VisitResult
```

###### visit_details()

Visit details elements `<details>`.

**Signature:**

```rust
pub fn visit_details(&self, ctx: NodeContext, open: bool) -> VisitResult
```

###### visit_summary()

Visit summary elements `<summary>`.

**Signature:**

```rust
pub fn visit_summary(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_figure_start()

Visit figure elements `<figure>`.

**Signature:**

```rust
pub fn visit_figure_start(&self, ctx: NodeContext) -> VisitResult
```

###### visit_figcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```rust
pub fn visit_figcaption(&self, ctx: NodeContext, text: &str) -> VisitResult
```

###### visit_figure_end()

Called after processing a figure `</figure>`.

**Signature:**

```rust
pub fn visit_figure_end(&self, ctx: NodeContext, output: &str) -> VisitResult
```


---

##### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `String` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `Option<String>` | `None` | Alternative text from alt attribute (for accessibility) |
| `title` | `Option<String>` | `None` | Title attribute (often shown as tooltip) |
| `dimensions` | `Option<Vec<u32>>` | `None` | Image dimensions as (width, height) if available |
| `image_type` | `ImageType` | — | Image type classification |
| `attributes` | `HashMap<String, String>` | — | Additional HTML attributes |


---

##### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `String` | — | The href URL value |
| `text` | `String` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `Option<String>` | `None` | Optional title attribute (often shown as tooltip) |
| `link_type` | `LinkType` | — | Link type classification |
| `rel` | `Vec<String>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `HashMap<String, String>` | — | Additional HTML attributes |

###### Methods

###### classify_link()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```rust
pub fn classify_link(href: &str) -> LinkType
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
| `attributes` | `HashMap<String, String>` | — | All HTML attributes as key-value pairs |
| `depth` | `usize` | — | Depth in the DOM tree (0 = root) |
| `index_in_parent` | `usize` | — | Index among siblings (0-based) |
| `parent_tag` | `Option<String>` | `None` | Parent element's tag name (None if root) |
| `is_inline` | `bool` | — | Whether this element is treated as inline vs block |


---

##### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset::Standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

###### Methods

###### default()

**Signature:**

```rust
pub fn default() -> PreprocessingOptions
```

###### apply_update()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```rust
pub fn apply_update(&self, update: PreprocessingOptionsUpdate)
```

###### from_update()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```rust
pub fn from_update(update: PreprocessingOptionsUpdate) -> PreprocessingOptions
```

###### from()

**Signature:**

```rust
pub fn from(update: PreprocessingOptionsUpdate) -> PreprocessingOptions
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
| `schema_type` | `Option<String>` | `None` | Schema type if detectable (e.g., "Article", "Event", "Product") |


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
| `rows` | `u32` | — | Number of rows. |
| `cols` | `u32` | — | Number of columns. |
| `cells` | `Vec<GridCell>` | `vec![]` | All cells in the table (may be fewer than rows*cols due to spans). |


---

##### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `u32` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `u32` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `AnnotationKind` | — | The type of annotation. |


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
| `Heading` | A heading element (h1-h6). — Fields: `level`: `u8`, `text`: `String` |
| `Paragraph` | A paragraph of text. — Fields: `text`: `String` |
| `List` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool` |
| `ListItem` | A single list item. — Fields: `text`: `String` |
| `Table` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `Image` | An image element. — Fields: `description`: `String`, `src`: `String`, `image_index`: `u32` |
| `Code` | A code block or inline code. — Fields: `text`: `String`, `language`: `String` |
| `Quote` | A block quote container. |
| `DefinitionList` | A definition list container. |
| `DefinitionItem` | A definition list entry with term and description. — Fields: `term`: `String`, `definition`: `String` |
| `RawBlock` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `String`, `content`: `String` |
| `MetadataBlock` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `Vec<String>` |
| `Group` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `String`, `heading_level`: `u8`, `heading_text`: `String` |


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
| `Link` | A hyperlink. — Fields: `url`: `String`, `title`: `String` |


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
| `Custom` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `String` |
| `Skip` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `Error` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `String` |


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
