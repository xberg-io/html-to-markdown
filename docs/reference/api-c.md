---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v3.5.0</span>

### Functions

#### htm_convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```c
HtmConversionResult* htm_convert(const char* html, HtmConversionOptions options);
```

**Parameters:**

| Name      | Type                    | Required | Description        |
| --------- | ----------------------- | -------- | ------------------ |
| `html`    | `const char*`           | Yes      | The html           |
| `options` | `HtmConversionOptions*` | No       | The options to use |

**Returns:** `HtmConversionResult`
**Errors:** Returns `NULL` on error.

---

### Types

#### HtmConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field                        | Type                      | Default                | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------------- | ------------------------- | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `heading_style`              | `HtmHeadingStyle`         | `HTM_HTM_ATX`          | Heading style to use in Markdown output (ATX `#` or Setext underline).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `list_indent_type`           | `HtmListIndentType`       | `HTM_HTM_SPACES`       | How to indent nested list items (spaces or tab).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `list_indent_width`          | `uintptr_t`               | `2`                    | Number of spaces (or tabs) to use for each level of list indentation.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `bullets`                    | `const char*`             | `"-*+"`                | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `strong_em_symbol`           | `const char*`             | `"*"`                  | Character used for bold/italic emphasis markers (`*` or `_`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `escape_asterisks`           | `bool`                    | `false`                | Escape `*` characters in plain text to avoid unintended bold/italic.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `escape_underscores`         | `bool`                    | `false`                | Escape `_` characters in plain text to avoid unintended bold/italic.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `escape_misc`                | `bool`                    | `false`                | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `escape_ascii`               | `bool`                    | `false`                | Escape ASCII characters that have special meaning in certain Markdown dialects.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `code_language`              | `const char*`             | `""`                   | Default language annotation for fenced code blocks that have no language hint.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `autolinks`                  | `bool`                    | `true`                 | Automatically convert bare URLs into Markdown autolinks.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `default_title`              | `bool`                    | `false`                | Emit a default title when no `<title>` tag is present.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `br_in_tables`               | `bool`                    | `false`                | Render `<br>` elements inside table cells as literal line breaks.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `compact_tables`             | `bool`                    | `false`                | Emit tables without column padding (compact GFM format). When `true`, column widths are not computed and cells are emitted with no trailing spaces. Separator rows use exactly `---` per column. Produces token-efficient output suitable for RAG / LLM contexts. Default `false` (aligned padding preserved).                                                                                                                                                                                                                                                                    |
| `highlight_style`            | `HtmHighlightStyle`       | `HTM_HTM_DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `extract_metadata`           | `bool`                    | `true`                 | Populate `result.metadata` with `<head>` / `<meta>` extraction (title, description, Open Graph, Twitter Card, JSON-LD, …). Default `true`. Disabling skips the metadata pass only — table extraction into `result.tables` runs unconditionally.                                                                                                                                                                                                                                                                                                                                   |
| `whitespace_mode`            | `HtmWhitespaceMode`       | `HTM_HTM_NORMALIZED`   | Controls how whitespace sequences are normalised in the converted output. - `WhitespaceMode.Normalized` (default) — collapses consecutive whitespace characters (spaces, tabs, newlines) to a single space, matching browser rendering behaviour. - `WhitespaceMode.Strict` — preserves all whitespace exactly as it appears in the source HTML, including runs of spaces and embedded newlines. Choose `Strict` only when the source HTML uses deliberate whitespace (e.g. pre-formatted content outside `<pre>` tags). For most documents `Normalized` produces cleaner output. |
| `strip_newlines`             | `bool`                    | `false`                | Strip all newlines from the output, producing a single-line result.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `wrap`                       | `bool`                    | `false`                | Wrap long lines at `wrap_width` characters.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `wrap_width`                 | `uintptr_t`               | `80`                   | Maximum output line width in characters when `wrap` is `true` (default `80`). Lines are broken at word boundaries so that no line exceeds this length. A value of `0` is treated as "no limit" — equivalent to leaving `wrap` disabled. Has no effect when `wrap` is `false`.                                                                                                                                                                                                                                                                                                     |
| `convert_as_inline`          | `bool`                    | `false`                | Treat the entire document as inline content (no block-level wrappers).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sub_symbol`                 | `const char*`             | `""`                   | Markdown notation for subscript text (e.g. `"~"`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `sup_symbol`                 | `const char*`             | `""`                   | Markdown notation for superscript text (e.g. `"^"`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `newline_style`              | `HtmNewlineStyle`         | `HTM_HTM_SPACES`       | How to encode hard line breaks (`<br>`) in Markdown.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `code_block_style`           | `HtmCodeBlockStyle`       | `HTM_HTM_BACKTICKS`    | Style used for fenced code blocks (backticks or tilde).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `keep_inline_images_in`      | `const char**`            | `NULL`                 | HTML tag names whose `<img>` children are kept inline instead of block.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `preprocessing`              | `HtmPreprocessingOptions` | —                      | Options for the HTML pre-processing pass applied before conversion begins. Pre-processing runs before the HTML is handed to the converter and can perform operations such as unwrapping redundant wrapper elements, removing tracking pixels, and normalising vendor-specific markup. See `PreprocessingOptions` for the full set of knobs. Defaults to `PreprocessingOptions.default()`, which enables the standard cleaning passes. Set individual fields on `PreprocessingOptions` (or construct via `ConversionOptions.builder`) to opt in or out of specific passes.         |
| `encoding`                   | `const char*`             | `"utf-8"`              | Expected character encoding of the input HTML (default `"utf-8"`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `debug`                      | `bool`                    | `false`                | Emit debug information during conversion.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `strip_tags`                 | `const char**`            | `NULL`                 | HTML tag names whose content is stripped from the output entirely.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `preserve_tags`              | `const char**`            | `NULL`                 | HTML tag names that are preserved verbatim in the output.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `skip_images`                | `bool`                    | `false`                | Skip conversion of `<img>` elements (omit images from output).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `link_style`                 | `HtmLinkStyle`            | `HTM_HTM_INLINE`       | Link rendering style (inline or reference).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `output_format`              | `HtmOutputFormat`         | `HTM_HTM_MARKDOWN`     | Target output format (Markdown, plain text, etc.).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `include_document_structure` | `bool`                    | `false`                | Include structured document tree in result.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `extract_images`             | `bool`                    | `false`                | Extract inline images from data URIs and SVGs.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `max_image_size`             | `uint64_t`                | `5242880`              | Maximum decoded image size in bytes (default 5MB).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `capture_svg`                | `bool`                    | `false`                | Capture SVG elements as images.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `infer_dimensions`           | `bool`                    | `true`                 | Infer image dimensions from data.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `max_depth`                  | `uintptr_t*`              | `NULL`                 | Maximum DOM traversal depth. `NULL` means unlimited. When set, subtrees beyond this depth are silently truncated.                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `exclude_selectors`          | `const char**`            | `NULL`                 | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]`                                                                                               |
| `visitor`                    | `HtmVisitorHandle*`       | `NULL`                 | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`.                                                                                                                                                                                                                                                                                                                                                               |

### Methods

#### htm_default()

**Signature:**

```c
HtmConversionOptions htm_default();
```

---

#### HtmConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field      | Type                    | Default | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | ----------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content`  | `const char**`          | `NULL`  | Converted text output (markdown, djot, or plain text). `NULL` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `document` | `HtmDocumentStructure*` | `NULL`  | Structured document tree with semantic elements. Populated when `ConversionOptions.include_document_structure` is `true`. `NULL` otherwise (the default), which avoids the overhead of building the tree. When present, the tree mirrors the converted document: headings open `Group` sections, paragraphs and list items carry inline `TextAnnotation`s, and tables reference the same `TableGrid` data exposed in `Self.tables`. Note: this field is independent of the `metadata` feature flag. Document structure collection is always available at runtime; it is gated only by the runtime option, not by a compile-time feature. |
| `metadata` | `HtmHtmlMetadata`       | —       | Extracted HTML metadata (title, OG, links, images, structured data).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `tables`   | `HtmTableData*`         | `NULL`  | Extracted tables with structured cell data and markdown representation.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `images`   | `const char**`          | `NULL`  | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `true` in options.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `warnings` | `HtmProcessingWarning*` | `NULL`  | Non-fatal processing warnings.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

---

#### HtmDocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field            | Type                | Default | Description                                                                                                              |
| ---------------- | ------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------ |
| `title`          | `const char**`      | `NULL`  | Document title from `<title>` tag                                                                                        |
| `description`    | `const char**`      | `NULL`  | Document description from `<meta name="description">` tag                                                                |
| `keywords`       | `const char**`      | `NULL`  | Document keywords from `<meta name="keywords">` tag, split on commas                                                     |
| `author`         | `const char**`      | `NULL`  | Document author from `<meta name="author">` tag                                                                          |
| `canonical_url`  | `const char**`      | `NULL`  | Canonical URL from `<link rel="canonical">` tag                                                                          |
| `base_href`      | `const char**`      | `NULL`  | Base URL from `<base href="">` tag for resolving relative URLs                                                           |
| `language`       | `const char**`      | `NULL`  | Document language from `lang` attribute                                                                                  |
| `text_direction` | `HtmTextDirection*` | `NULL`  | Document text direction from `dir` attribute                                                                             |
| `open_graph`     | `void*`             | `NULL`  | Open Graph metadata (og:\* properties) for social media Keys like "title", "description", "image", "url", etc.           |
| `twitter_card`   | `void*`             | `NULL`  | Twitter Card metadata (twitter:\* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags`      | `void*`             | `NULL`  | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content           |

---

#### HtmDocumentNode

A single node in the document tree.

| Field         | Type                 | Default                | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------- | -------------------- | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | `const char*`        | —                      | Deterministic node identifier.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `content`     | `HtmNodeContent`     | —                      | The semantic content of this node.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `parent`      | `uint32_t*`          | `NULL`                 | Index of the parent node (None for root nodes).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `children`    | `uint32_t*`          | `/* serde(default) */` | Indices of child nodes in reading order.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `annotations` | `HtmTextAnnotation*` | `/* serde(default) */` | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `attributes`  | `void**`             | `NULL`                 | Format-specific attributes preserved from the source HTML element. Keys are lowercased attribute names as they appear in the HTML (e.g. `"class"`, `"id"`, `"data-foo"`). Values are the raw attribute strings, copied verbatim from the source — no HTML entity decoding is applied here. The map is `NULL` when no attributes are present (omitted entirely in serialized output). Not every HTML attribute is preserved: only attributes that carry semantic or structural significance for the node type are collected. For example, heading nodes capture the `"id"` attribute for anchor linking; other element-level attributes may be silently dropped. |

---

#### HtmDocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field           | Type               | Default | Description                                         |
| --------------- | ------------------ | ------- | --------------------------------------------------- |
| `nodes`         | `HtmDocumentNode*` | —       | All nodes in document reading order.                |
| `source_format` | `const char**`     | `NULL`  | The source format (always "html" for this library). |

---

#### HtmGridCell

A single cell in a table grid.

| Field       | Type          | Default                | Description                                    |
| ----------- | ------------- | ---------------------- | ---------------------------------------------- |
| `content`   | `const char*` | —                      | The text content of the cell.                  |
| `row`       | `uint32_t`    | —                      | 0-indexed row position.                        |
| `col`       | `uint32_t`    | —                      | 0-indexed column position.                     |
| `row_span`  | `uint32_t`    | —                      | Number of rows this cell spans (default 1).    |
| `col_span`  | `uint32_t`    | —                      | Number of columns this cell spans (default 1). |
| `is_header` | `bool`        | `/* serde(default) */` | Whether this is a header cell (`<th>`).        |

---

#### HtmHeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field         | Type           | Default | Description                               |
| ------------- | -------------- | ------- | ----------------------------------------- |
| `level`       | `uint8_t`      | —       | Header level: 1 (h1) through 6 (h6)       |
| `text`        | `const char*`  | —       | Normalized text content of the header     |
| `id`          | `const char**` | `NULL`  | HTML id attribute if present              |
| `depth`       | `uintptr_t`    | —       | Document tree depth at the header element |
| `html_offset` | `uintptr_t`    | —       | Byte offset in original HTML document     |

### Methods

#### htm_is_valid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```c
bool htm_is_valid();
```

---

#### HtmHtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field             | Type                  | Default | Description                                                   |
| ----------------- | --------------------- | ------- | ------------------------------------------------------------- |
| `document`        | `HtmDocumentMetadata` | —       | Document-level metadata (title, description, canonical, etc.) |
| `headers`         | `HtmHeaderMetadata*`  | `NULL`  | Extracted header elements with hierarchy                      |
| `links`           | `HtmLinkMetadata*`    | `NULL`  | Extracted hyperlinks with type classification                 |
| `images`          | `HtmImageMetadata*`   | `NULL`  | Extracted images with source and dimensions                   |
| `structured_data` | `HtmStructuredData*`  | `NULL`  | Extracted structured data blocks                              |

---

#### HtmHtmlVisitor

Visitor trait for HTML→Markdown conversion.

Implement this trait to customize the conversion behavior for any HTML element type.
All methods have default implementations that return `VisitResult.Continue`, allowing
selective override of only the elements you care about.

### Method Naming Convention

- `visit_*_start`: Called before entering an element (pre-order traversal)
- `visit_*_end`: Called after exiting an element (post-order traversal)
- `visit_*`: Called for specific element types (e.g., `visit_link`, `visit_image`)

### Execution Order

For a typical element like `<div><p>text</p></div>`:

1. `visit_element_start` for `<div>`
2. `visit_element_start` for `<p>`
3. `visit_text` for "text"
4. `visit_element_end` for `<p>`
5. `visit_element_end` for `</div>`

### Performance Notes

- `visit_text` is the most frequently called method (~100+ times per document)
- Return `VisitResult.Continue` quickly for elements you don't need to customize
- Avoid heavy computation in visitor methods; consider caching if needed

### Methods

#### htm_visit_text()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```c
HtmVisitResult htm_visit_text(HtmNodeContext ctx, const char* text);
```

#### htm_visit_element_start()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```c
HtmVisitResult htm_visit_element_start(HtmNodeContext ctx);
```

#### htm_visit_element_end()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```c
HtmVisitResult htm_visit_element_end(HtmNodeContext ctx, const char* output);
```

#### htm_visit_link()

Visit anchor links `<a href="...">`.

**Signature:**

```c
HtmVisitResult htm_visit_link(HtmNodeContext ctx, const char* href, const char* text, const char* title);
```

#### htm_visit_image()

Visit images `<img src="...">`.

**Signature:**

```c
HtmVisitResult htm_visit_image(HtmNodeContext ctx, const char* src, const char* alt, const char* title);
```

#### htm_visit_heading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```c
HtmVisitResult htm_visit_heading(HtmNodeContext ctx, uint32_t level, const char* text, const char* id);
```

#### htm_visit_code_block()

Visit code blocks `<pre><code>`.

**Signature:**

```c
HtmVisitResult htm_visit_code_block(HtmNodeContext ctx, const char* lang, const char* code);
```

#### htm_visit_code_inline()

Visit inline code `<code>`.

**Signature:**

```c
HtmVisitResult htm_visit_code_inline(HtmNodeContext ctx, const char* code);
```

#### htm_visit_list_item()

Visit list items `<li>`.

**Signature:**

```c
HtmVisitResult htm_visit_list_item(HtmNodeContext ctx, bool ordered, const char* marker, const char* text);
```

#### htm_visit_list_start()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```c
HtmVisitResult htm_visit_list_start(HtmNodeContext ctx, bool ordered);
```

#### htm_visit_list_end()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```c
HtmVisitResult htm_visit_list_end(HtmNodeContext ctx, bool ordered, const char* output);
```

#### htm_visit_table_start()

Called before processing a table `<table>`.

**Signature:**

```c
HtmVisitResult htm_visit_table_start(HtmNodeContext ctx);
```

#### htm_visit_table_row()

Visit table rows `<tr>`.

**Signature:**

```c
HtmVisitResult htm_visit_table_row(HtmNodeContext ctx, const char** cells, bool is_header);
```

#### htm_visit_table_end()

Called after processing a table `</table>`.

**Signature:**

```c
HtmVisitResult htm_visit_table_end(HtmNodeContext ctx, const char* output);
```

#### htm_visit_blockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```c
HtmVisitResult htm_visit_blockquote(HtmNodeContext ctx, const char* content, uintptr_t depth);
```

#### htm_visit_strong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```c
HtmVisitResult htm_visit_strong(HtmNodeContext ctx, const char* text);
```

#### htm_visit_emphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```c
HtmVisitResult htm_visit_emphasis(HtmNodeContext ctx, const char* text);
```

#### htm_visit_strikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```c
HtmVisitResult htm_visit_strikethrough(HtmNodeContext ctx, const char* text);
```

#### htm_visit_underline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```c
HtmVisitResult htm_visit_underline(HtmNodeContext ctx, const char* text);
```

#### htm_visit_subscript()

Visit subscript elements `<sub>`.

**Signature:**

```c
HtmVisitResult htm_visit_subscript(HtmNodeContext ctx, const char* text);
```

#### htm_visit_superscript()

Visit superscript elements `<sup>`.

**Signature:**

```c
HtmVisitResult htm_visit_superscript(HtmNodeContext ctx, const char* text);
```

#### htm_visit_mark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```c
HtmVisitResult htm_visit_mark(HtmNodeContext ctx, const char* text);
```

#### htm_visit_line_break()

Visit line break elements `<br>`.

**Signature:**

```c
HtmVisitResult htm_visit_line_break(HtmNodeContext ctx);
```

#### htm_visit_horizontal_rule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```c
HtmVisitResult htm_visit_horizontal_rule(HtmNodeContext ctx);
```

#### htm_visit_custom_element()

Visit custom elements (web components) or unknown tags.

**Signature:**

```c
HtmVisitResult htm_visit_custom_element(HtmNodeContext ctx, const char* tag_name, const char* html);
```

#### htm_visit_definition_list_start()

Visit definition list `<dl>`.

**Signature:**

```c
HtmVisitResult htm_visit_definition_list_start(HtmNodeContext ctx);
```

#### htm_visit_definition_term()

Visit definition term `<dt>`.

**Signature:**

```c
HtmVisitResult htm_visit_definition_term(HtmNodeContext ctx, const char* text);
```

#### htm_visit_definition_description()

Visit definition description `<dd>`.

**Signature:**

```c
HtmVisitResult htm_visit_definition_description(HtmNodeContext ctx, const char* text);
```

#### htm_visit_definition_list_end()

Called after processing a definition list `</dl>`.

**Signature:**

```c
HtmVisitResult htm_visit_definition_list_end(HtmNodeContext ctx, const char* output);
```

#### htm_visit_form()

Visit form elements `<form>`.

**Signature:**

```c
HtmVisitResult htm_visit_form(HtmNodeContext ctx, const char* action, const char* method);
```

#### htm_visit_input()

Visit input elements `<input>`.

**Signature:**

```c
HtmVisitResult htm_visit_input(HtmNodeContext ctx, const char* input_type, const char* name, const char* value);
```

#### htm_visit_button()

Visit button elements `<button>`.

**Signature:**

```c
HtmVisitResult htm_visit_button(HtmNodeContext ctx, const char* text);
```

#### htm_visit_audio()

Visit audio elements `<audio>`.

**Signature:**

```c
HtmVisitResult htm_visit_audio(HtmNodeContext ctx, const char* src);
```

#### htm_visit_video()

Visit video elements `<video>`.

**Signature:**

```c
HtmVisitResult htm_visit_video(HtmNodeContext ctx, const char* src);
```

#### htm_visit_iframe()

Visit iframe elements `<iframe>`.

**Signature:**

```c
HtmVisitResult htm_visit_iframe(HtmNodeContext ctx, const char* src);
```

#### htm_visit_details()

Visit details elements `<details>`.

**Signature:**

```c
HtmVisitResult htm_visit_details(HtmNodeContext ctx, bool open);
```

#### htm_visit_summary()

Visit summary elements `<summary>`.

**Signature:**

```c
HtmVisitResult htm_visit_summary(HtmNodeContext ctx, const char* text);
```

#### htm_visit_figure_start()

Visit figure elements `<figure>`.

**Signature:**

```c
HtmVisitResult htm_visit_figure_start(HtmNodeContext ctx);
```

#### htm_visit_figcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```c
HtmVisitResult htm_visit_figcaption(HtmNodeContext ctx, const char* text);
```

#### htm_visit_figure_end()

Called after processing a figure `</figure>`.

**Signature:**

```c
HtmVisitResult htm_visit_figure_end(HtmNodeContext ctx, const char* output);
```

---

#### HtmImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field        | Type           | Default | Description                                             |
| ------------ | -------------- | ------- | ------------------------------------------------------- |
| `src`        | `const char*`  | —       | Image source (URL, data URI, or SVG content identifier) |
| `alt`        | `const char**` | `NULL`  | Alternative text from alt attribute (for accessibility) |
| `title`      | `const char**` | `NULL`  | Title attribute (often shown as tooltip)                |
| `dimensions` | `uint32_t**`   | `NULL`  | Image dimensions as (width, height) if available        |
| `image_type` | `HtmImageType` | —       | Image type classification                               |
| `attributes` | `void*`        | —       | Additional HTML attributes                              |

---

#### HtmLinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field        | Type           | Default | Description                                                         |
| ------------ | -------------- | ------- | ------------------------------------------------------------------- |
| `href`       | `const char*`  | —       | The href URL value                                                  |
| `text`       | `const char*`  | —       | Link text content (normalized, concatenated if mixed with elements) |
| `title`      | `const char**` | `NULL`  | Optional title attribute (often shown as tooltip)                   |
| `link_type`  | `HtmLinkType`  | —       | Link type classification                                            |
| `rel`        | `const char**` | —       | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical")  |
| `attributes` | `void*`        | —       | Additional HTML attributes                                          |

---

#### HtmNodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field             | Type           | Default | Description                                             |
| ----------------- | -------------- | ------- | ------------------------------------------------------- |
| `node_type`       | `HtmNodeType`  | —       | Coarse-grained node type classification                 |
| `tag_name`        | `const char*`  | —       | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes`      | `void*`        | —       | All HTML attributes as key-value pairs                  |
| `depth`           | `uintptr_t`    | —       | Depth in the DOM tree (0 = root)                        |
| `index_in_parent` | `uintptr_t`    | —       | Index among siblings (0-based)                          |
| `parent_tag`      | `const char**` | `NULL`  | Parent element's tag name (None if root)                |
| `is_inline`       | `bool`         | —       | Whether this element is treated as inline vs block      |

---

#### HtmPreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field               | Type                     | Default            | Description                                                    |
| ------------------- | ------------------------ | ------------------ | -------------------------------------------------------------- |
| `enabled`           | `bool`                   | `true`             | Enable HTML preprocessing globally                             |
| `preset`            | `HtmPreprocessingPreset` | `HTM_HTM_STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive)     |
| `remove_navigation` | `bool`                   | `true`             | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms`      | `bool`                   | `true`             | Remove form elements (forms, inputs, buttons, etc.)            |

### Methods

#### htm_default()

**Signature:**

```c
HtmPreprocessingOptions htm_default();
```

---

#### HtmProcessingWarning

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

| Field     | Type             | Default | Description                     |
| --------- | ---------------- | ------- | ------------------------------- |
| `message` | `const char*`    | —       | Human-readable warning message. |
| `kind`    | `HtmWarningKind` | —       | The category of warning.        |

---

#### HtmStructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field         | Type                    | Default | Description                                                     |
| ------------- | ----------------------- | ------- | --------------------------------------------------------------- |
| `data_type`   | `HtmStructuredDataType` | —       | Type of structured data (JSON-LD, Microdata, RDFa)              |
| `raw_json`    | `const char*`           | —       | Raw JSON string (for JSON-LD) or serialized representation      |
| `schema_type` | `const char**`          | `NULL`  | Schema type if detectable (e.g., "Article", "Event", "Product") |

---

#### HtmTableData

A top-level extracted table with both structured data and markdown representation.

| Field      | Type           | Default | Description                           |
| ---------- | -------------- | ------- | ------------------------------------- |
| `grid`     | `HtmTableGrid` | —       | The structured table grid.            |
| `markdown` | `const char*`  | —       | The markdown rendering of this table. |

---

#### HtmTableGrid

A structured table grid with cell-level data including spans.

| Field   | Type           | Default | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------- | -------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `rows`  | `uint32_t`     | —       | Number of rows.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `cols`  | `uint32_t`     | —       | Number of columns.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `cells` | `HtmGridCell*` | `NULL`  | All cells in the table as a flat, sparse list. The list is ordered by `(row, col)` but is **not** a dense `rows × cols` matrix: cells that are covered by a spanning cell (via `row_span > 1` or `col_span > 1`) do not appear in the list. Only the top-left "origin" cell of a span is present, with its `row_span` and `col_span` fields set accordingly. To reconstruct the full visual grid, iterate over all cells and mark the rectangular region `[row .. row+row_span, col .. col+col_span]` as occupied by that cell. Any `(row, col)` position that is not the origin of any cell is covered by a span from an earlier cell. The length of this vec is `≤ rows * cols`. An empty table (`rows == 0 \|\| cols == 0`) produces an empty vec. |

---

#### HtmTextAnnotation

A styling or semantic annotation that applies to a byte range within a node's text.

Unlike `DocumentNode`, which captures block-level structure (headings, paragraphs, etc.),
a `TextAnnotation` describes inline-level markup — bold, italic, links, code spans, and
similar — that spans a contiguous run of bytes inside `DocumentNode.content`'s text field.

Byte offsets (`start`..`end`) are into the UTF-8 encoded text of the parent node. The range
follows Rust slice conventions: `start` is inclusive and `end` is exclusive, so the annotated
text is `text[start as usize..end as usize]`.

Multiple annotations on the same node can overlap (e.g. bold-italic text), and they are
stored in the order they are encountered during DOM traversal.

See `AnnotationKind` for the full list of supported annotation types.

| Field   | Type                | Default | Description                                                |
| ------- | ------------------- | ------- | ---------------------------------------------------------- |
| `start` | `uint32_t`          | —       | Start byte offset (inclusive) into the parent node's text. |
| `end`   | `uint32_t`          | —       | End byte offset (exclusive) into the parent node's text.   |
| `kind`  | `HtmAnnotationKind` | —       | The type of annotation.                                    |

---

#### HtmVisitorHandle

Shareable, thread-safe handle to a user-provided HTML visitor implementation.

Pass an instance wrapped in this handle to `ConversionOptions` to
customise how the HTML document is traversed and converted to Markdown.
The handle may be cloned and shared across threads without additional
synchronisation on the caller's side.

---

### Enums

#### HtmTextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value               | Description                                          |
| ------------------- | ---------------------------------------------------- |
| `HTM_LEFT_TO_RIGHT` | Left-to-right text flow (default for Latin scripts)  |
| `HTM_RIGHT_TO_LEFT` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `HTM_AUTO`          | Automatic directionality detection                   |

---

#### HtmLinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value          | Description                                           |
| -------------- | ----------------------------------------------------- |
| `HTM_ANCHOR`   | Anchor link within same document (href starts with #) |
| `HTM_INTERNAL` | Internal link within same domain                      |
| `HTM_EXTERNAL` | External link to different domain                     |
| `HTM_EMAIL`    | Email link (mailto:)                                  |
| `HTM_PHONE`    | Phone link (tel:)                                     |
| `HTM_OTHER`    | Other protocol or unclassifiable                      |

---

#### HtmImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value            | Description                                        |
| ---------------- | -------------------------------------------------- |
| `HTM_DATA_URI`   | Data URI embedded image (base64 or other encoding) |
| `HTM_INLINE_SVG` | Inline SVG element                                 |
| `HTM_EXTERNAL`   | External image URL (http/https)                    |
| `HTM_RELATIVE`   | Relative image path                                |

---

#### HtmStructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value           | Description                                                |
| --------------- | ---------------------------------------------------------- |
| `HTM_JSON_LD`   | JSON-LD (JSON for Linking Data) script blocks              |
| `HTM_MICRODATA` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `HTM_RDFA`      | RDF in Attributes (RDFa) markup                            |

---

#### HtmPreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value            | Description                                                                        |
| ---------------- | ---------------------------------------------------------------------------------- |
| `HTM_MINIMAL`    | Minimal cleanup. Remove only essential noise (scripts, styles).                    |
| `HTM_STANDARD`   | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `HTM_AGGRESSIVE` | Aggressive cleanup. Remove extensive non-content elements and structure.           |

---

#### HtmHeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value            | Description                                        |
| ---------------- | -------------------------------------------------- |
| `HTM_UNDERLINED` | Underlined style (=== for h1, --- for h2).         |
| `HTM_ATX`        | ATX style (# for h1, ## for h2, etc.). Default.    |
| `HTM_ATX_CLOSED` | ATX closed style (# title #, with closing hashes). |

---

#### HtmListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value        | Description                                                                   |
| ------------ | ----------------------------------------------------------------------------- |
| `HTM_SPACES` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `HTM_TABS`   | Use tabs for indentation.                                                     |

---

#### HtmWhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value            | Description                                                                                  |
| ---------------- | -------------------------------------------------------------------------------------------- |
| `HTM_NORMALIZED` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `HTM_STRICT`     | Preserve all whitespace exactly as it appears in the HTML.                                   |

---

#### HtmNewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value           | Description                                                            |
| --------------- | ---------------------------------------------------------------------- |
| `HTM_SPACES`    | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `HTM_BACKSLASH` | Backslash at end of line. Alternative Markdown syntax.                 |

---

#### HtmCodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value           | Description                                                                      |
| --------------- | -------------------------------------------------------------------------------- |
| `HTM_INDENTED`  | Indented code blocks (4 spaces). `CommonMark` standard.                          |
| `HTM_BACKTICKS` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `HTM_TILDES`    | Fenced code blocks with tildes (~~~). Supports language hints.                   |

---

#### HtmHighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value              | Description                                                  |
| ------------------ | ------------------------------------------------------------ |
| `HTM_DOUBLE_EQUAL` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `HTM_HTML`         | Preserve as HTML (==text==). Original HTML tag.              |
| `HTM_BOLD`         | Render as bold (**text**). Uses strong emphasis.             |
| `HTM_NONE`         | Strip formatting, render as plain text. No markup.           |

---

#### HtmLinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value           | Description                                                            |
| --------------- | ---------------------------------------------------------------------- |
| `HTM_INLINE`    | Inline links: `[text](url)`. Default.                                  |
| `HTM_REFERENCE` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |

---

#### HtmOutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value          | Description                                         |
| -------------- | --------------------------------------------------- |
| `HTM_MARKDOWN` | Standard Markdown (CommonMark compatible). Default. |
| `HTM_DJOT`     | Djot lightweight markup language.                   |
| `HTM_PLAIN`    | Plain text output (no markup, visible text only).   |

---

#### HtmNodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value                 | Description                                                                                                                                                       |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `HTM_HEADING`         | A heading element (h1-h6). — Fields: `level`: `uint8_t`, `text`: `const char*`                                                                                    |
| `HTM_PARAGRAPH`       | A paragraph of text. — Fields: `text`: `const char*`                                                                                                              |
| `HTM_LIST`            | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool`                                                               |
| `HTM_LIST_ITEM`       | A single list item. — Fields: `text`: `const char*`                                                                                                               |
| `HTM_TABLE`           | A table with structured cell data. — Fields: `grid`: `HtmTableGrid`                                                                                               |
| `HTM_IMAGE`           | An image element. — Fields: `description`: `const char*`, `src`: `const char*`, `image_index`: `uint32_t`                                                         |
| `HTM_CODE`            | A code block or inline code. — Fields: `text`: `const char*`, `language`: `const char*`                                                                           |
| `HTM_QUOTE`           | A block quote container.                                                                                                                                          |
| `HTM_DEFINITION_LIST` | A definition list container.                                                                                                                                      |
| `HTM_DEFINITION_ITEM` | A definition list entry with term and description. — Fields: `term`: `const char*`, `definition`: `const char*`                                                   |
| `HTM_RAW_BLOCK`       | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `const char*`, `content`: `const char*`                                     |
| `HTM_METADATA_BLOCK`  | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `const char**`                                                                |
| `HTM_GROUP`           | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `const char*`, `heading_level`: `uint8_t`, `heading_text`: `const char*` |

---

#### HtmAnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value               | Description                                                                                                  |
| ------------------- | ------------------------------------------------------------------------------------------------------------ |
| `HTM_BOLD`          | Bold / strong emphasis.                                                                                      |
| `HTM_ITALIC`        | Italic / emphasis.                                                                                           |
| `HTM_UNDERLINE`     | Underline.                                                                                                   |
| `HTM_STRIKETHROUGH` | Strikethrough / deleted text.                                                                                |
| `HTM_CODE`          | Inline code.                                                                                                 |
| `HTM_SUBSCRIPT`     | Subscript text.                                                                                              |
| `HTM_SUPERSCRIPT`   | Superscript text.                                                                                            |
| `HTM_HIGHLIGHT`     | Highlighted / marked text.                                                                                   |
| `HTM_LINK`          | A hyperlink sourced from an `<a href="...">` element. — Fields: `url`: `const char*`, `title`: `const char*` |

---

#### HtmWarningKind

Categories of processing warnings.

| Value                         | Description                                                                  |
| ----------------------------- | ---------------------------------------------------------------------------- |
| `HTM_IMAGE_EXTRACTION_FAILED` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `HTM_ENCODING_FALLBACK`       | The input encoding was not recognized; fell back to UTF-8.                   |
| `HTM_TRUNCATED_INPUT`         | The input was truncated due to size limits.                                  |
| `HTM_MALFORMED_HTML`          | The HTML was malformed but processing continued with best effort.            |
| `HTM_SANITIZATION_APPLIED`    | Sanitization was applied to remove potentially unsafe content.               |
| `HTM_DEPTH_LIMIT_EXCEEDED`    | DOM traversal was truncated because max_depth was exceeded.                  |

---

#### HtmNodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value                        | Description                                    |
| ---------------------------- | ---------------------------------------------- |
| `HTM_TEXT`                   | Text node (most frequent - 100+ per document)  |
| `HTM_ELEMENT`                | Generic element node                           |
| `HTM_HEADING`                | Heading elements (h1-h6)                       |
| `HTM_PARAGRAPH`              | Paragraph element                              |
| `HTM_DIV`                    | Generic div container                          |
| `HTM_BLOCKQUOTE`             | Blockquote element                             |
| `HTM_PRE`                    | Preformatted text block                        |
| `HTM_HR`                     | Horizontal rule                                |
| `HTM_LIST`                   | Ordered or unordered list (ul, ol)             |
| `HTM_LIST_ITEM`              | List item (li)                                 |
| `HTM_DEFINITION_LIST`        | Definition list (dl)                           |
| `HTM_DEFINITION_TERM`        | Definition term (dt)                           |
| `HTM_DEFINITION_DESCRIPTION` | Definition description (dd)                    |
| `HTM_TABLE`                  | Table element                                  |
| `HTM_TABLE_ROW`              | Table row (tr)                                 |
| `HTM_TABLE_CELL`             | Table cell (td, th)                            |
| `HTM_TABLE_HEADER`           | Table header cell (th)                         |
| `HTM_TABLE_BODY`             | Table body (tbody)                             |
| `HTM_TABLE_HEAD`             | Table head (thead)                             |
| `HTM_TABLE_FOOT`             | Table foot (tfoot)                             |
| `HTM_LINK`                   | Anchor link (a)                                |
| `HTM_IMAGE`                  | Image (img)                                    |
| `HTM_STRONG`                 | Strong/bold (strong, b)                        |
| `HTM_EM`                     | Emphasis/italic (em, i)                        |
| `HTM_CODE`                   | Inline code (code)                             |
| `HTM_STRIKETHROUGH`          | Strikethrough (s, del, strike)                 |
| `HTM_UNDERLINE`              | Underline (u, ins)                             |
| `HTM_SUBSCRIPT`              | Subscript (sub)                                |
| `HTM_SUPERSCRIPT`            | Superscript (sup)                              |
| `HTM_MARK`                   | Mark/highlight (mark)                          |
| `HTM_SMALL`                  | Small text (small)                             |
| `HTM_BR`                     | Line break (br)                                |
| `HTM_SPAN`                   | Span element                                   |
| `HTM_ARTICLE`                | Article element                                |
| `HTM_SECTION`                | Section element                                |
| `HTM_NAV`                    | Navigation element                             |
| `HTM_ASIDE`                  | Aside element                                  |
| `HTM_HEADER`                 | Header element                                 |
| `HTM_FOOTER`                 | Footer element                                 |
| `HTM_MAIN`                   | Main element                                   |
| `HTM_FIGURE`                 | Figure element                                 |
| `HTM_FIGCAPTION`             | Figure caption                                 |
| `HTM_TIME`                   | Time element                                   |
| `HTM_DETAILS`                | Details element                                |
| `HTM_SUMMARY`                | Summary element                                |
| `HTM_FORM`                   | Form element                                   |
| `HTM_INPUT`                  | Input element                                  |
| `HTM_SELECT`                 | Select element                                 |
| `HTM_OPTION`                 | Option element                                 |
| `HTM_BUTTON`                 | Button element                                 |
| `HTM_TEXTAREA`               | Textarea element                               |
| `HTM_LABEL`                  | Label element                                  |
| `HTM_FIELDSET`               | Fieldset element                               |
| `HTM_LEGEND`                 | Legend element                                 |
| `HTM_AUDIO`                  | Audio element                                  |
| `HTM_VIDEO`                  | Video element                                  |
| `HTM_PICTURE`                | Picture element                                |
| `HTM_SOURCE`                 | Source element                                 |
| `HTM_IFRAME`                 | Iframe element                                 |
| `HTM_SVG`                    | SVG element                                    |
| `HTM_CANVAS`                 | Canvas element                                 |
| `HTM_RUBY`                   | Ruby annotation                                |
| `HTM_RT`                     | Ruby text                                      |
| `HTM_RP`                     | Ruby parenthesis                               |
| `HTM_ABBR`                   | Abbreviation                                   |
| `HTM_KBD`                    | Keyboard input                                 |
| `HTM_SAMP`                   | Sample output                                  |
| `HTM_VAR`                    | Variable                                       |
| `HTM_CITE`                   | Citation                                       |
| `HTM_Q`                      | Quote                                          |
| `HTM_DEL`                    | Deleted text                                   |
| `HTM_INS`                    | Inserted text                                  |
| `HTM_DATA`                   | Data element                                   |
| `HTM_METER`                  | Meter element                                  |
| `HTM_PROGRESS`               | Progress element                               |
| `HTM_OUTPUT`                 | Output element                                 |
| `HTM_TEMPLATE`               | Template element                               |
| `HTM_SLOT`                   | Slot element                                   |
| `HTM_HTML`                   | HTML root element                              |
| `HTM_HEAD`                   | Head element                                   |
| `HTM_BODY`                   | Body element                                   |
| `HTM_TITLE`                  | Title element                                  |
| `HTM_META`                   | Meta element                                   |
| `HTM_LINK_TAG`               | Link element (not anchor)                      |
| `HTM_STYLE`                  | Style element                                  |
| `HTM_SCRIPT`                 | Script element                                 |
| `HTM_BASE`                   | Base element                                   |
| `HTM_CUSTOM`                 | Custom element (web components) or unknown tag |

---

#### HtmVisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value               | Description                                                                                                                                                           |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `HTM_CONTINUE`      | Continue with default conversion behavior                                                                                                                             |
| `HTM_CUSTOM`        | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `const char*` |
| `HTM_SKIP`          | Skip this element entirely (don't output anything) The element and all its children are ignored in the output.                                                        |
| `HTM_PRESERVE_HTML` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output.                                                         |
| `HTM_ERROR`         | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `const char*`                                               |

---

### Errors

#### HtmConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant                  | Description                             |
| ------------------------ | --------------------------------------- |
| `HTM_PARSE_ERROR`        | HTML parsing error                      |
| `HTM_SANITIZATION_ERROR` | HTML sanitization error                 |
| `HTM_CONFIG_ERROR`       | Invalid configuration                   |
| `HTM_IO_ERROR`           | I/O error                               |
| `HTM_PANIC`              | Internal error caught during conversion |
| `HTM_INVALID_INPUT`      | Invalid input data                      |
| `HTM_OTHER`              | Generic conversion error                |

---
