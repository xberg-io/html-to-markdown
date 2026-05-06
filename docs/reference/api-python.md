---
title: "Python API Reference"
---

## Python API Reference <span class="version-badge">v3.4.0-rc.25</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

When the `visitor` feature is enabled, a custom `HtmlVisitor` can be
attached via the `visitor` field on `ConversionOptions`.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```python
def convert(html: str, options: ConversionOptions = None) -> ConversionResult
```

**Parameters:**

| Name      | Type               | Required | Description |
| --------- | ------------------ | -------- | ----------- |
| `html`    | `str`              | Yes      | The html    |
| `options` | `ConversionOptions | None`    | No          | The options to use |

**Returns:** `ConversionResult`

**Errors:** Raises `Error`.

---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field                        | Type                   | Default                       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ---------------------------- | ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `heading_style`              | `HeadingStyle`         | `HeadingStyle.ATX`            | Heading style to use in Markdown output (ATX `#` or Setext underline).                                                                                                                                                                                                                                                                                                                                                                                                              |
| `list_indent_type`           | `ListIndentType`       | `ListIndentType.SPACES`       | How to indent nested list items (spaces or tab).                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `list_indent_width`          | `int`                  | `2`                           | Number of spaces (or tabs) to use for each level of list indentation.                                                                                                                                                                                                                                                                                                                                                                                                               |
| `bullets`                    | `str`                  | `"-*+"`                       | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`).                                                                                                                                                                                                                                                                                                                                                                                                            |
| `strong_em_symbol`           | `str`                  | `"*"`                         | Character used for bold/italic emphasis markers (`*` or `_`).                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `escape_asterisks`           | `bool`                 | `False`                       | Escape `*` characters in plain text to avoid unintended bold/italic.                                                                                                                                                                                                                                                                                                                                                                                                                |
| `escape_underscores`         | `bool`                 | `False`                       | Escape `_` characters in plain text to avoid unintended bold/italic.                                                                                                                                                                                                                                                                                                                                                                                                                |
| `escape_misc`                | `bool`                 | `False`                       | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text.                                                                                                                                                                                                                                                                                                                                                                                                          |
| `escape_ascii`               | `bool`                 | `False`                       | Escape ASCII characters that have special meaning in certain Markdown dialects.                                                                                                                                                                                                                                                                                                                                                                                                     |
| `code_language`              | `str`                  | `""`                          | Default language annotation for fenced code blocks that have no language hint.                                                                                                                                                                                                                                                                                                                                                                                                      |
| `autolinks`                  | `bool`                 | `True`                        | Automatically convert bare URLs into Markdown autolinks.                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `default_title`              | `bool`                 | `False`                       | Emit a default title when no `<title>` tag is present.                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `br_in_tables`               | `bool`                 | `False`                       | Render `<br>` elements inside table cells as literal line breaks.                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `highlight_style`            | `HighlightStyle`       | `HighlightStyle.DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`).                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `extract_metadata`           | `bool`                 | `True`                        | Extract `<meta>` and `<head>` information into the result metadata.                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `whitespace_mode`            | `WhitespaceMode`       | `WhitespaceMode.NORMALIZED`   | Controls how whitespace is normalised during conversion.                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `strip_newlines`             | `bool`                 | `False`                       | Strip all newlines from the output, producing a single-line result.                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `wrap`                       | `bool`                 | `False`                       | Wrap long lines at `wrap_width` characters.                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `wrap_width`                 | `int`                  | `80`                          | Maximum line width when `wrap` is enabled (default `80`).                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `convert_as_inline`          | `bool`                 | `False`                       | Treat the entire document as inline content (no block-level wrappers).                                                                                                                                                                                                                                                                                                                                                                                                              |
| `sub_symbol`                 | `str`                  | `""`                          | Markdown notation for subscript text (e.g. `"~"`).                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `sup_symbol`                 | `str`                  | `""`                          | Markdown notation for superscript text (e.g. `"^"`).                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `newline_style`              | `NewlineStyle`         | `NewlineStyle.SPACES`         | How to encode hard line breaks (`<br>`) in Markdown.                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `code_block_style`           | `CodeBlockStyle`       | `CodeBlockStyle.BACKTICKS`    | Style used for fenced code blocks (backticks or tilde).                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `keep_inline_images_in`      | `list[str]`            | `[]`                          | HTML tag names whose `<img>` children are kept inline instead of block.                                                                                                                                                                                                                                                                                                                                                                                                             |
| `preprocessing`              | `PreprocessingOptions` | —                             | Pre-processing options applied to the HTML before conversion.                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `encoding`                   | `str`                  | `"utf-8"`                     | Expected character encoding of the input HTML (default `"utf-8"`).                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `debug`                      | `bool`                 | `False`                       | Emit debug information during conversion.                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `strip_tags`                 | `list[str]`            | `[]`                          | HTML tag names whose content is stripped from the output entirely.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `preserve_tags`              | `list[str]`            | `[]`                          | HTML tag names that are preserved verbatim in the output.                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `skip_images`                | `bool`                 | `False`                       | Skip conversion of `<img>` elements (omit images from output).                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `link_style`                 | `LinkStyle`            | `LinkStyle.INLINE`            | Link rendering style (inline or reference).                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `output_format`              | `OutputFormat`         | `OutputFormat.MARKDOWN`       | Target output format (Markdown, plain text, etc.).                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `include_document_structure` | `bool`                 | `False`                       | Include structured document tree in result.                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `extract_images`             | `bool`                 | `False`                       | Extract inline images from data URIs and SVGs.                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `max_image_size`             | `int`                  | `5242880`                     | Maximum decoded image size in bytes (default 5MB).                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `capture_svg`                | `bool`                 | `False`                       | Capture SVG elements as images.                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `infer_dimensions`           | `bool`                 | `True`                        | Infer image dimensions from data.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `max_depth`                  | `int                   | None`                         | `None`                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Maximum DOM traversal depth. `None` means unlimited. When set, subtrees beyond this depth are silently truncated.                                                                                                   |
| `exclude_selectors`          | `list[str]`            | `[]`                          | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]` |
| `visitor`                    | `VisitorHandle         | None`                         | `None`                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`. |

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

###### apply_update()

Apply a partial update to these conversion options.

**Signature:**

```python
def apply_update(self, update: ConversionOptionsUpdate) -> None
```

###### from_update()

Create from a partial update, applying to defaults.

**Signature:**

```python
@staticmethod
def from_update(update: ConversionOptionsUpdate) -> ConversionOptions
```

###### from()

**Signature:**

```python
@staticmethod
def from(update: ConversionOptionsUpdate) -> ConversionOptions
```

---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field      | Type                      | Default | Description                                                                                         |
| ---------- | ------------------------- | ------- | --------------------------------------------------------------------------------------------------- |
| `content`  | `str                      | None`   | `None`                                                                                              | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `DocumentStructure        | None`   | `None`                                                                                              | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options.                                 |
| `metadata` | `HtmlMetadata`            | —       | Extracted HTML metadata (title, OG, links, images, structured data).                                |
| `tables`   | `list[TableData]`         | `[]`    | Extracted tables with structured cell data and markdown representation.                             |
| `images`   | `list[str]`               | `[]`    | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `list[ProcessingWarning]` | `[]`    | Non-fatal processing warnings.                                                                      |

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

###### exclude_selectors()

Set the list of CSS selectors for elements to exclude entirely from output.

**Signature:**

```python
def exclude_selectors(self, selectors: list[str]) -> ConversionOptionsBuilder
```

###### visitor()

Set the visitor used during conversion.

**Signature:**

```python
def visitor(self, visitor: VisitorHandle) -> ConversionOptionsBuilder
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

| Field            | Type             | Default | Description                                                                                                              |
| ---------------- | ---------------- | ------- | ------------------------------------------------------------------------------------------------------------------------ |
| `title`          | `str             | None`   | `None`                                                                                                                   | Document title from `<title>` tag                              |
| `description`    | `str             | None`   | `None`                                                                                                                   | Document description from `<meta name="description">` tag      |
| `keywords`       | `list[str]`      | `[]`    | Document keywords from `<meta name="keywords">` tag, split on commas                                                     |
| `author`         | `str             | None`   | `None`                                                                                                                   | Document author from `<meta name="author">` tag                |
| `canonical_url`  | `str             | None`   | `None`                                                                                                                   | Canonical URL from `<link rel="canonical">` tag                |
| `base_href`      | `str             | None`   | `None`                                                                                                                   | Base URL from `<base href="">` tag for resolving relative URLs |
| `language`       | `str             | None`   | `None`                                                                                                                   | Document language from `lang` attribute                        |
| `text_direction` | `TextDirection   | None`   | `None`                                                                                                                   | Document text direction from `dir` attribute                   |
| `open_graph`     | `dict[str, str]` | `{}`    | Open Graph metadata (og:\* properties) for social media Keys like "title", "description", "image", "url", etc.           |
| `twitter_card`   | `dict[str, str]` | `{}`    | Twitter Card metadata (twitter:\* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags`      | `dict[str, str]` | `{}`    | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content           |

---

#### DocumentNode

A single node in the document tree.

| Field         | Type                   | Default | Description                                                                                |
| ------------- | ---------------------- | ------- | ------------------------------------------------------------------------------------------ |
| `id`          | `str`                  | —       | Deterministic node identifier.                                                             |
| `content`     | `NodeContent`          | —       | The semantic content of this node.                                                         |
| `parent`      | `int                   | None`   | `None`                                                                                     | Index of the parent node (None for root nodes).                  |
| `children`    | `list[int]`            | —       | Indices of child nodes in reading order.                                                   |
| `annotations` | `list[TextAnnotation]` | —       | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes`  | `dict[str, str]        | None`   | `None`                                                                                     | Format-specific attributes (e.g. class, id, data-\* attributes). |

---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field           | Type                 | Default | Description                          |
| --------------- | -------------------- | ------- | ------------------------------------ |
| `nodes`         | `list[DocumentNode]` | —       | All nodes in document reading order. |
| `source_format` | `str                 | None`   | `None`                               | The source format (always "html" for this library). |

---

#### GridCell

A single cell in a table grid.

| Field       | Type   | Default | Description                                    |
| ----------- | ------ | ------- | ---------------------------------------------- |
| `content`   | `str`  | —       | The text content of the cell.                  |
| `row`       | `int`  | —       | 0-indexed row position.                        |
| `col`       | `int`  | —       | 0-indexed column position.                     |
| `row_span`  | `int`  | —       | Number of rows this cell spans (default 1).    |
| `col_span`  | `int`  | —       | Number of columns this cell spans (default 1). |
| `is_header` | `bool` | —       | Whether this is a header cell (`<th>`).        |

---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field         | Type  | Default | Description                               |
| ------------- | ----- | ------- | ----------------------------------------- |
| `level`       | `int` | —       | Header level: 1 (h1) through 6 (h6)       |
| `text`        | `str` | —       | Normalized text content of the header     |
| `id`          | `str  | None`   | `None`                                    | HTML id attribute if present |
| `depth`       | `int` | —       | Document tree depth at the header element |
| `html_offset` | `int` | —       | Byte offset in original HTML document     |

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

| Field             | Type                   | Default | Description                                                   |
| ----------------- | ---------------------- | ------- | ------------------------------------------------------------- |
| `document`        | `DocumentMetadata`     | —       | Document-level metadata (title, description, canonical, etc.) |
| `headers`         | `list[HeaderMetadata]` | `[]`    | Extracted header elements with hierarchy                      |
| `links`           | `list[LinkMetadata]`   | `[]`    | Extracted hyperlinks with type classification                 |
| `images`          | `list[ImageMetadata]`  | `[]`    | Extracted images with source and dimensions                   |
| `structured_data` | `list[StructuredData]` | `[]`    | Extracted structured data blocks                              |

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

```python
def visit_element_start(self, ctx: NodeContext) -> VisitResult
```

##### visit_element_end()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```python
def visit_element_end(self, ctx: NodeContext, output: str) -> VisitResult
```

###### visit_text()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```python
def visit_text(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_link()

Visit anchor links `<a href="...">`.

**Signature:**

```python
def visit_link(self, ctx: NodeContext, href: str, text: str, title: str) -> VisitResult
```

###### visit_image()

Visit images `<img src="...">`.

**Signature:**

```python
def visit_image(self, ctx: NodeContext, src: str, alt: str, title: str) -> VisitResult
```

###### visit_heading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```python
def visit_heading(self, ctx: NodeContext, level: int, text: str, id: str) -> VisitResult
```

###### visit_code_block()

Visit code blocks `<pre><code>`.

**Signature:**

```python
def visit_code_block(self, ctx: NodeContext, lang: str, code: str) -> VisitResult
```

###### visit_code_inline()

Visit inline code `<code>`.

**Signature:**

```python
def visit_code_inline(self, ctx: NodeContext, code: str) -> VisitResult
```

###### visit_list_item()

Visit list items `<li>`.

**Signature:**

```python
def visit_list_item(self, ctx: NodeContext, ordered: bool, marker: str, text: str) -> VisitResult
```

###### visit_list_start()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```python
def visit_list_start(self, ctx: NodeContext, ordered: bool) -> VisitResult
```

###### visit_list_end()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```python
def visit_list_end(self, ctx: NodeContext, ordered: bool, output: str) -> VisitResult
```

###### visit_table_start()

Called before processing a table `<table>`.

**Signature:**

```python
def visit_table_start(self, ctx: NodeContext) -> VisitResult
```

###### visit_table_row()

Visit table rows `<tr>`.

**Signature:**

```python
def visit_table_row(self, ctx: NodeContext, cells: list[str], is_header: bool) -> VisitResult
```

###### visit_table_end()

Called after processing a table `</table>`.

**Signature:**

```python
def visit_table_end(self, ctx: NodeContext, output: str) -> VisitResult
```

###### visit_blockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```python
def visit_blockquote(self, ctx: NodeContext, content: str, depth: int) -> VisitResult
```

###### visit_strong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```python
def visit_strong(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_emphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```python
def visit_emphasis(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_strikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```python
def visit_strikethrough(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_underline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```python
def visit_underline(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_subscript()

Visit subscript elements `<sub>`.

**Signature:**

```python
def visit_subscript(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_superscript()

Visit superscript elements `<sup>`.

**Signature:**

```python
def visit_superscript(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_mark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```python
def visit_mark(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_line_break()

Visit line break elements `<br>`.

**Signature:**

```python
def visit_line_break(self, ctx: NodeContext) -> VisitResult
```

###### visit_horizontal_rule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```python
def visit_horizontal_rule(self, ctx: NodeContext) -> VisitResult
```

###### visit_custom_element()

Visit custom elements (web components) or unknown tags.

**Signature:**

```python
def visit_custom_element(self, ctx: NodeContext, tag_name: str, html: str) -> VisitResult
```

###### visit_definition_list_start()

Visit definition list `<dl>`.

**Signature:**

```python
def visit_definition_list_start(self, ctx: NodeContext) -> VisitResult
```

###### visit_definition_term()

Visit definition term `<dt>`.

**Signature:**

```python
def visit_definition_term(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_definition_description()

Visit definition description `<dd>`.

**Signature:**

```python
def visit_definition_description(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_definition_list_end()

Called after processing a definition list `</dl>`.

**Signature:**

```python
def visit_definition_list_end(self, ctx: NodeContext, output: str) -> VisitResult
```

###### visit_form()

Visit form elements `<form>`.

**Signature:**

```python
def visit_form(self, ctx: NodeContext, action: str, method: str) -> VisitResult
```

###### visit_input()

Visit input elements `<input>`.

**Signature:**

```python
def visit_input(self, ctx: NodeContext, input_type: str, name: str, value: str) -> VisitResult
```

###### visit_button()

Visit button elements `<button>`.

**Signature:**

```python
def visit_button(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_audio()

Visit audio elements `<audio>`.

**Signature:**

```python
def visit_audio(self, ctx: NodeContext, src: str) -> VisitResult
```

###### visit_video()

Visit video elements `<video>`.

**Signature:**

```python
def visit_video(self, ctx: NodeContext, src: str) -> VisitResult
```

###### visit_iframe()

Visit iframe elements `<iframe>`.

**Signature:**

```python
def visit_iframe(self, ctx: NodeContext, src: str) -> VisitResult
```

###### visit_details()

Visit details elements `<details>`.

**Signature:**

```python
def visit_details(self, ctx: NodeContext, open: bool) -> VisitResult
```

###### visit_summary()

Visit summary elements `<summary>`.

**Signature:**

```python
def visit_summary(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_figure_start()

Visit figure elements `<figure>`.

**Signature:**

```python
def visit_figure_start(self, ctx: NodeContext) -> VisitResult
```

###### visit_figcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```python
def visit_figcaption(self, ctx: NodeContext, text: str) -> VisitResult
```

###### visit_figure_end()

Called after processing a figure `</figure>`.

**Signature:**

```python
def visit_figure_end(self, ctx: NodeContext, output: str) -> VisitResult
```

---

##### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field        | Type             | Default | Description                                             |
| ------------ | ---------------- | ------- | ------------------------------------------------------- |
| `src`        | `str`            | —       | Image source (URL, data URI, or SVG content identifier) |
| `alt`        | `str             | None`   | `None`                                                  | Alternative text from alt attribute (for accessibility) |
| `title`      | `str             | None`   | `None`                                                  | Title attribute (often shown as tooltip)                |
| `dimensions` | `list[int]       | None`   | `None`                                                  | Image dimensions as (width, height) if available        |
| `image_type` | `ImageType`      | —       | Image type classification                               |
| `attributes` | `dict[str, str]` | —       | Additional HTML attributes                              |

---

##### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field        | Type             | Default | Description                                                         |
| ------------ | ---------------- | ------- | ------------------------------------------------------------------- |
| `href`       | `str`            | —       | The href URL value                                                  |
| `text`       | `str`            | —       | Link text content (normalized, concatenated if mixed with elements) |
| `title`      | `str             | None`   | `None`                                                              | Optional title attribute (often shown as tooltip) |
| `link_type`  | `LinkType`       | —       | Link type classification                                            |
| `rel`        | `list[str]`      | —       | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical")  |
| `attributes` | `dict[str, str]` | —       | Additional HTML attributes                                          |

###### Methods

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

##### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field             | Type             | Default | Description                                             |
| ----------------- | ---------------- | ------- | ------------------------------------------------------- |
| `node_type`       | `NodeType`       | —       | Coarse-grained node type classification                 |
| `tag_name`        | `str`            | —       | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes`      | `dict[str, str]` | —       | All HTML attributes as key-value pairs                  |
| `depth`           | `int`            | —       | Depth in the DOM tree (0 = root)                        |
| `index_in_parent` | `int`            | —       | Index among siblings (0-based)                          |
| `parent_tag`      | `str             | None`   | `None`                                                  | Parent element's tag name (None if root) |
| `is_inline`       | `bool`           | —       | Whether this element is treated as inline vs block      |

---

##### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field               | Type                  | Default                        | Description                                                    |
| ------------------- | --------------------- | ------------------------------ | -------------------------------------------------------------- |
| `enabled`           | `bool`                | `True`                         | Enable HTML preprocessing globally                             |
| `preset`            | `PreprocessingPreset` | `PreprocessingPreset.STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive)     |
| `remove_navigation` | `bool`                | `True`                         | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms`      | `bool`                | `True`                         | Remove form elements (forms, inputs, buttons, etc.)            |

###### Methods

###### default()

**Signature:**

```python
@staticmethod
def default() -> PreprocessingOptions
```

###### apply_update()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```python
def apply_update(self, update: PreprocessingOptionsUpdate) -> None
```

###### from_update()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```python
@staticmethod
def from_update(update: PreprocessingOptionsUpdate) -> PreprocessingOptions
```

###### from()

**Signature:**

```python
@staticmethod
def from(update: PreprocessingOptionsUpdate) -> PreprocessingOptions
```

---

##### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field     | Type          | Default | Description                     |
| --------- | ------------- | ------- | ------------------------------- |
| `message` | `str`         | —       | Human-readable warning message. |
| `kind`    | `WarningKind` | —       | The category of warning.        |

---

##### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field         | Type                 | Default | Description                                                |
| ------------- | -------------------- | ------- | ---------------------------------------------------------- |
| `data_type`   | `StructuredDataType` | —       | Type of structured data (JSON-LD, Microdata, RDFa)         |
| `raw_json`    | `str`                | —       | Raw JSON string (for JSON-LD) or serialized representation |
| `schema_type` | `str                 | None`   | `None`                                                     | Schema type if detectable (e.g., "Article", "Event", "Product") |

---

##### TableData

A top-level extracted table with both structured data and markdown representation.

| Field      | Type        | Default | Description                           |
| ---------- | ----------- | ------- | ------------------------------------- |
| `grid`     | `TableGrid` | —       | The structured table grid.            |
| `markdown` | `str`       | —       | The markdown rendering of this table. |

---

##### TableGrid

A structured table grid with cell-level data including spans.

| Field   | Type             | Default | Description                                                         |
| ------- | ---------------- | ------- | ------------------------------------------------------------------- |
| `rows`  | `int`            | —       | Number of rows.                                                     |
| `cols`  | `int`            | —       | Number of columns.                                                  |
| `cells` | `list[GridCell]` | `[]`    | All cells in the table (may be fewer than rows\*cols due to spans). |

---

##### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field   | Type             | Default | Description                                                |
| ------- | ---------------- | ------- | ---------------------------------------------------------- |
| `start` | `int`            | —       | Start byte offset (inclusive) into the parent node's text. |
| `end`   | `int`            | —       | End byte offset (exclusive) into the parent node's text.   |
| `kind`  | `AnnotationKind` | —       | The type of annotation.                                    |

---

##### VisitorHandle

Type alias for a visitor handle (Rc-wrapped `RefCell` for interior mutability).

This allows visitors to be passed around and shared while still being mutable.

---

#### Enums

##### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value           | Description                                          |
| --------------- | ---------------------------------------------------- |
| `LEFT_TO_RIGHT` | Left-to-right text flow (default for Latin scripts)  |
| `RIGHT_TO_LEFT` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `AUTO`          | Automatic directionality detection                   |

---

##### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value      | Description                                           |
| ---------- | ----------------------------------------------------- |
| `ANCHOR`   | Anchor link within same document (href starts with #) |
| `INTERNAL` | Internal link within same domain                      |
| `EXTERNAL` | External link to different domain                     |
| `EMAIL`    | Email link (mailto:)                                  |
| `PHONE`    | Phone link (tel:)                                     |
| `OTHER`    | Other protocol or unclassifiable                      |

---

##### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value        | Description                                        |
| ------------ | -------------------------------------------------- |
| `DATA_URI`   | Data URI embedded image (base64 or other encoding) |
| `INLINE_SVG` | Inline SVG element                                 |
| `EXTERNAL`   | External image URL (http/https)                    |
| `RELATIVE`   | Relative image path                                |

---

##### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value       | Description                                                |
| ----------- | ---------------------------------------------------------- |
| `JSON_LD`   | JSON-LD (JSON for Linking Data) script blocks              |
| `MICRODATA` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFA`      | RDF in Attributes (RDFa) markup                            |

---

##### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value        | Description                                                                        |
| ------------ | ---------------------------------------------------------------------------------- |
| `MINIMAL`    | Minimal cleanup. Remove only essential noise (scripts, styles).                    |
| `STANDARD`   | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `AGGRESSIVE` | Aggressive cleanup. Remove extensive non-content elements and structure.           |

---

##### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value        | Description                                        |
| ------------ | -------------------------------------------------- |
| `UNDERLINED` | Underlined style (=== for h1, --- for h2).         |
| `ATX`        | ATX style (# for h1, ## for h2, etc.). Default.    |
| `ATX_CLOSED` | ATX closed style (# title #, with closing hashes). |

---

##### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value    | Description                                                                   |
| -------- | ----------------------------------------------------------------------------- |
| `SPACES` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `TABS`   | Use tabs for indentation.                                                     |

---

##### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value        | Description                                                                                  |
| ------------ | -------------------------------------------------------------------------------------------- |
| `NORMALIZED` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `STRICT`     | Preserve all whitespace exactly as it appears in the HTML.                                   |

---

##### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value       | Description                                                            |
| ----------- | ---------------------------------------------------------------------- |
| `SPACES`    | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `BACKSLASH` | Backslash at end of line. Alternative Markdown syntax.                 |

---

##### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value       | Description                                                                      |
| ----------- | -------------------------------------------------------------------------------- |
| `INDENTED`  | Indented code blocks (4 spaces). `CommonMark` standard.                          |
| `BACKTICKS` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `TILDES`    | Fenced code blocks with tildes (~~~). Supports language hints.                   |

---

##### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value          | Description                                                  |
| -------------- | ------------------------------------------------------------ |
| `DOUBLE_EQUAL` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `HTML`         | Preserve as HTML (==text==). Original HTML tag.              |
| `BOLD`         | Render as bold (**text**). Uses strong emphasis.             |
| `NONE`         | Strip formatting, render as plain text. No markup.           |

---

##### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value       | Description                                                            |
| ----------- | ---------------------------------------------------------------------- |
| `INLINE`    | Inline links: `[text](url)`. Default.                                  |
| `REFERENCE` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |

---

##### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value      | Description                                         |
| ---------- | --------------------------------------------------- |
| `MARKDOWN` | Standard Markdown (CommonMark compatible). Default. |
| `DJOT`     | Djot lightweight markup language.                   |
| `PLAIN`    | Plain text output (no markup, visible text only).   |

---

##### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value             | Description                                                                                                                                   |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `HEADING`         | A heading element (h1-h6). — Fields: `level`: `int`, `text`: `str`                                                                            |
| `PARAGRAPH`       | A paragraph of text. — Fields: `text`: `str`                                                                                                  |
| `LIST`            | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool`                                           |
| `LIST_ITEM`       | A single list item. — Fields: `text`: `str`                                                                                                   |
| `TABLE`           | A table with structured cell data. — Fields: `grid`: `TableGrid`                                                                              |
| `IMAGE`           | An image element. — Fields: `description`: `str`, `src`: `str`, `image_index`: `int`                                                          |
| `CODE`            | A code block or inline code. — Fields: `text`: `str`, `language`: `str`                                                                       |
| `QUOTE`           | A block quote container.                                                                                                                      |
| `DEFINITION_LIST` | A definition list container.                                                                                                                  |
| `DEFINITION_ITEM` | A definition list entry with term and description. — Fields: `term`: `str`, `definition`: `str`                                               |
| `RAW_BLOCK`       | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `str`, `content`: `str`                                 |
| `METADATA_BLOCK`  | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `list[str]`                                               |
| `GROUP`           | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `str`, `heading_level`: `int`, `heading_text`: `str` |

---

##### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value           | Description                                         |
| --------------- | --------------------------------------------------- |
| `BOLD`          | Bold / strong emphasis.                             |
| `ITALIC`        | Italic / emphasis.                                  |
| `UNDERLINE`     | Underline.                                          |
| `STRIKETHROUGH` | Strikethrough / deleted text.                       |
| `CODE`          | Inline code.                                        |
| `SUBSCRIPT`     | Subscript text.                                     |
| `SUPERSCRIPT`   | Superscript text.                                   |
| `HIGHLIGHT`     | Highlighted / marked text.                          |
| `LINK`          | A hyperlink. — Fields: `url`: `str`, `title`: `str` |

---

##### WarningKind

Categories of processing warnings.

| Value                     | Description                                                                  |
| ------------------------- | ---------------------------------------------------------------------------- |
| `IMAGE_EXTRACTION_FAILED` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `ENCODING_FALLBACK`       | The input encoding was not recognized; fell back to UTF-8.                   |
| `TRUNCATED_INPUT`         | The input was truncated due to size limits.                                  |
| `MALFORMED_HTML`          | The HTML was malformed but processing continued with best effort.            |
| `SANITIZATION_APPLIED`    | Sanitization was applied to remove potentially unsafe content.               |
| `DEPTH_LIMIT_EXCEEDED`    | DOM traversal was truncated because max_depth was exceeded.                  |

---

##### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value                    | Description                                    |
| ------------------------ | ---------------------------------------------- |
| `TEXT`                   | Text node (most frequent - 100+ per document)  |
| `ELEMENT`                | Generic element node                           |
| `HEADING`                | Heading elements (h1-h6)                       |
| `PARAGRAPH`              | Paragraph element                              |
| `DIV`                    | Generic div container                          |
| `BLOCKQUOTE`             | Blockquote element                             |
| `PRE`                    | Preformatted text block                        |
| `HR`                     | Horizontal rule                                |
| `LIST`                   | Ordered or unordered list (ul, ol)             |
| `LIST_ITEM`              | List item (li)                                 |
| `DEFINITION_LIST`        | Definition list (dl)                           |
| `DEFINITION_TERM`        | Definition term (dt)                           |
| `DEFINITION_DESCRIPTION` | Definition description (dd)                    |
| `TABLE`                  | Table element                                  |
| `TABLE_ROW`              | Table row (tr)                                 |
| `TABLE_CELL`             | Table cell (td, th)                            |
| `TABLE_HEADER`           | Table header cell (th)                         |
| `TABLE_BODY`             | Table body (tbody)                             |
| `TABLE_HEAD`             | Table head (thead)                             |
| `TABLE_FOOT`             | Table foot (tfoot)                             |
| `LINK`                   | Anchor link (a)                                |
| `IMAGE`                  | Image (img)                                    |
| `STRONG`                 | Strong/bold (strong, b)                        |
| `EM`                     | Emphasis/italic (em, i)                        |
| `CODE`                   | Inline code (code)                             |
| `STRIKETHROUGH`          | Strikethrough (s, del, strike)                 |
| `UNDERLINE`              | Underline (u, ins)                             |
| `SUBSCRIPT`              | Subscript (sub)                                |
| `SUPERSCRIPT`            | Superscript (sup)                              |
| `MARK`                   | Mark/highlight (mark)                          |
| `SMALL`                  | Small text (small)                             |
| `BR`                     | Line break (br)                                |
| `SPAN`                   | Span element                                   |
| `ARTICLE`                | Article element                                |
| `SECTION`                | Section element                                |
| `NAV`                    | Navigation element                             |
| `ASIDE`                  | Aside element                                  |
| `HEADER`                 | Header element                                 |
| `FOOTER`                 | Footer element                                 |
| `MAIN`                   | Main element                                   |
| `FIGURE`                 | Figure element                                 |
| `FIGCAPTION`             | Figure caption                                 |
| `TIME`                   | Time element                                   |
| `DETAILS`                | Details element                                |
| `SUMMARY`                | Summary element                                |
| `FORM`                   | Form element                                   |
| `INPUT`                  | Input element                                  |
| `SELECT`                 | Select element                                 |
| `OPTION`                 | Option element                                 |
| `BUTTON`                 | Button element                                 |
| `TEXTAREA`               | Textarea element                               |
| `LABEL`                  | Label element                                  |
| `FIELDSET`               | Fieldset element                               |
| `LEGEND`                 | Legend element                                 |
| `AUDIO`                  | Audio element                                  |
| `VIDEO`                  | Video element                                  |
| `PICTURE`                | Picture element                                |
| `SOURCE`                 | Source element                                 |
| `IFRAME`                 | Iframe element                                 |
| `SVG`                    | SVG element                                    |
| `CANVAS`                 | Canvas element                                 |
| `RUBY`                   | Ruby annotation                                |
| `RT`                     | Ruby text                                      |
| `RP`                     | Ruby parenthesis                               |
| `ABBR`                   | Abbreviation                                   |
| `KBD`                    | Keyboard input                                 |
| `SAMP`                   | Sample output                                  |
| `VAR`                    | Variable                                       |
| `CITE`                   | Citation                                       |
| `Q`                      | Quote                                          |
| `DEL`                    | Deleted text                                   |
| `INS`                    | Inserted text                                  |
| `DATA`                   | Data element                                   |
| `METER`                  | Meter element                                  |
| `PROGRESS`               | Progress element                               |
| `OUTPUT`                 | Output element                                 |
| `TEMPLATE`               | Template element                               |
| `SLOT`                   | Slot element                                   |
| `HTML`                   | HTML root element                              |
| `HEAD`                   | Head element                                   |
| `BODY`                   | Body element                                   |
| `TITLE`                  | Title element                                  |
| `META`                   | Meta element                                   |
| `LINK_TAG`               | Link element (not anchor)                      |
| `STYLE`                  | Style element                                  |
| `SCRIPT`                 | Script element                                 |
| `BASE`                   | Base element                                   |
| `CUSTOM`                 | Custom element (web components) or unknown tag |

---

##### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value           | Description                                                                                                                                                   |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CONTINUE`      | Continue with default conversion behavior                                                                                                                     |
| `CUSTOM`        | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `str` |
| `SKIP`          | Skip this element entirely (don't output anything) The element and all its children are ignored in the output.                                                |
| `PRESERVE_HTML` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output.                                                 |
| `ERROR`         | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `str`                                               |

---

#### Errors

##### ConversionError

Errors that can occur during HTML to Markdown conversion.

**Base class:** `ConversionError(Exception)`

| Exception                            | Description                             |
| ------------------------------------ | --------------------------------------- |
| `ParseError(ConversionError)`        | HTML parsing error                      |
| `SanitizationError(ConversionError)` | HTML sanitization error                 |
| `ConfigError(ConversionError)`       | Invalid configuration                   |
| `IoError(ConversionError)`           | I/O error                               |
| `Panic(ConversionError)`             | Internal error caught during conversion |
| `InvalidInput(ConversionError)`      | Invalid input data                      |
| `Other(ConversionError)`             | Generic conversion error                |

---
