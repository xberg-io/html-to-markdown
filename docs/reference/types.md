---
title: "Types Reference"
---

## Types Reference

All types defined by the library, grouped by category. Types are shown using Rust as the canonical representation.

### Result Types

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field      | Type                        | Default              | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | --------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content`  | `Option<String>`            | `Default::default()` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `document` | `Option<DocumentStructure>` | `Default::default()` | Structured document tree with semantic elements. Populated when `ConversionOptions.include_document_structure` is `true`. `None` otherwise (the default), which avoids the overhead of building the tree. When present, the tree mirrors the converted document: headings open `Group` sections, paragraphs and list items carry inline `TextAnnotation`s, and tables reference the same `TableGrid` data exposed in `Self.tables`. Note: this field is independent of the `metadata` feature flag. Document structure collection is always available at runtime; it is gated only by the runtime option, not by a compile-time feature. |
| `metadata` | `HtmlMetadata`              | —                    | Extracted HTML metadata (title, OG, links, images, structured data).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `tables`   | `Vec<TableData>`            | `vec![]`             | Extracted tables with structured cell data and markdown representation.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `images`   | `Vec<String>`               | `vec![]`             | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `true` in options.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `warnings` | `Vec<ProcessingWarning>`    | `vec![]`             | Non-fatal processing warnings.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

---

### Configuration Types

See [Configuration Reference](configuration.md) for detailed defaults and language-specific representations.

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field                        | Type                    | Default                       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------------- | ----------------------- | ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `heading_style`              | `HeadingStyle`          | `HeadingStyle::Atx`           | Heading style to use in Markdown output (ATX `#` or Setext underline).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `list_indent_type`           | `ListIndentType`        | `ListIndentType::Spaces`      | How to indent nested list items (spaces or tab).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `list_indent_width`          | `usize`                 | `2`                           | Number of spaces (or tabs) to use for each level of list indentation.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `bullets`                    | `String`                | `"-*+"`                       | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `strong_em_symbol`           | `String`                | `"*"`                         | Character used for bold/italic emphasis markers (`*` or `_`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `escape_asterisks`           | `bool`                  | `false`                       | Escape `*` characters in plain text to avoid unintended bold/italic.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `escape_underscores`         | `bool`                  | `false`                       | Escape `_` characters in plain text to avoid unintended bold/italic.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `escape_misc`                | `bool`                  | `false`                       | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `escape_ascii`               | `bool`                  | `false`                       | Escape ASCII characters that have special meaning in certain Markdown dialects.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `code_language`              | `String`                | `""`                          | Default language annotation for fenced code blocks that have no language hint.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `autolinks`                  | `bool`                  | `true`                        | Automatically convert bare URLs into Markdown autolinks.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `default_title`              | `bool`                  | `false`                       | Emit a default title when no `<title>` tag is present.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `br_in_tables`               | `bool`                  | `false`                       | Render `<br>` elements inside table cells as literal line breaks.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `compact_tables`             | `bool`                  | `false`                       | Emit tables without column padding (compact GFM format). When `true`, column widths are not computed and cells are emitted with no trailing spaces. Separator rows use exactly `---` per column. Produces token-efficient output suitable for RAG / LLM contexts. Default `false` (aligned padding preserved).                                                                                                                                                                                                                                                                    |
| `highlight_style`            | `HighlightStyle`        | `HighlightStyle::DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `extract_metadata`           | `bool`                  | `true`                        | Populate `result.metadata` with `<head>` / `<meta>` extraction (title, description, Open Graph, Twitter Card, JSON-LD, …). Default `true`. Disabling skips the metadata pass only — table extraction into `result.tables` runs unconditionally.                                                                                                                                                                                                                                                                                                                                   |
| `whitespace_mode`            | `WhitespaceMode`        | `WhitespaceMode::Normalized`  | Controls how whitespace sequences are normalised in the converted output. - `WhitespaceMode.Normalized` (default) — collapses consecutive whitespace characters (spaces, tabs, newlines) to a single space, matching browser rendering behaviour. - `WhitespaceMode.Strict` — preserves all whitespace exactly as it appears in the source HTML, including runs of spaces and embedded newlines. Choose `Strict` only when the source HTML uses deliberate whitespace (e.g. pre-formatted content outside `<pre>` tags). For most documents `Normalized` produces cleaner output. |
| `strip_newlines`             | `bool`                  | `false`                       | Strip all newlines from the output, producing a single-line result.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `wrap`                       | `bool`                  | `false`                       | Wrap long lines at `wrap_width` characters.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `wrap_width`                 | `usize`                 | `80`                          | Maximum output line width in characters when `wrap` is `true` (default `80`). Lines are broken at word boundaries so that no line exceeds this length. A value of `0` is treated as "no limit" — equivalent to leaving `wrap` disabled. Has no effect when `wrap` is `false`.                                                                                                                                                                                                                                                                                                     |
| `convert_as_inline`          | `bool`                  | `false`                       | Treat the entire document as inline content (no block-level wrappers).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sub_symbol`                 | `String`                | `""`                          | Markdown notation for subscript text (e.g. `"~"`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `sup_symbol`                 | `String`                | `""`                          | Markdown notation for superscript text (e.g. `"^"`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `newline_style`              | `NewlineStyle`          | `NewlineStyle::Spaces`        | How to encode hard line breaks (`<br>`) in Markdown.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `code_block_style`           | `CodeBlockStyle`        | `CodeBlockStyle::Backticks`   | Style used for fenced code blocks (backticks or tilde).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `keep_inline_images_in`      | `Vec<String>`           | `vec![]`                      | HTML tag names whose `<img>` children are kept inline instead of block.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `preprocessing`              | `PreprocessingOptions`  | —                             | Options for the HTML pre-processing pass applied before conversion begins. Pre-processing runs before the HTML is handed to the converter and can perform operations such as unwrapping redundant wrapper elements, removing tracking pixels, and normalising vendor-specific markup. See `PreprocessingOptions` for the full set of knobs. Defaults to `PreprocessingOptions.default()`, which enables the standard cleaning passes. Set individual fields on `PreprocessingOptions` (or construct via `ConversionOptions.builder`) to opt in or out of specific passes.         |
| `encoding`                   | `String`                | `"utf-8"`                     | Expected character encoding of the input HTML (default `"utf-8"`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `debug`                      | `bool`                  | `false`                       | Emit debug information during conversion.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `strip_tags`                 | `Vec<String>`           | `vec![]`                      | HTML tag names whose content is stripped from the output entirely.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `preserve_tags`              | `Vec<String>`           | `vec![]`                      | HTML tag names that are preserved verbatim in the output.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `skip_images`                | `bool`                  | `false`                       | Skip conversion of `<img>` elements (omit images from output).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `link_style`                 | `LinkStyle`             | `LinkStyle::Inline`           | Link rendering style (inline or reference).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `output_format`              | `OutputFormat`          | `OutputFormat::Markdown`      | Target output format (Markdown, plain text, etc.).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `include_document_structure` | `bool`                  | `false`                       | Include structured document tree in result.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `extract_images`             | `bool`                  | `false`                       | Extract inline images from data URIs and SVGs.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `max_image_size`             | `u64`                   | `5242880`                     | Maximum decoded image size in bytes (default 5MB).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `capture_svg`                | `bool`                  | `false`                       | Capture SVG elements as images.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `infer_dimensions`           | `bool`                  | `true`                        | Infer image dimensions from data.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `max_depth`                  | `Option<usize>`         | `None`                        | Maximum DOM traversal depth. `None` means unlimited. When set, subtrees beyond this depth are silently truncated.                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `exclude_selectors`          | `Vec<String>`           | `vec![]`                      | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]`                                                                                               |
| `visitor`                    | `Option<VisitorHandle>` | `None`                        | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`.                                                                                                                                                                                                                                                                                                                                                               |

---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field               | Type                  | Default                         | Description                                                    |
| ------------------- | --------------------- | ------------------------------- | -------------------------------------------------------------- |
| `enabled`           | `bool`                | `true`                          | Enable HTML preprocessing globally                             |
| `preset`            | `PreprocessingPreset` | `PreprocessingPreset::Standard` | Preprocessing preset level (Minimal, Standard, Aggressive)     |
| `remove_navigation` | `bool`                | `true`                          | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms`      | `bool`                | `true`                          | Remove form elements (forms, inputs, buttons, etc.)            |

---

#### TableGrid

A structured table grid with cell-level data including spans.

| Field   | Type            | Default  | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------- | --------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `rows`  | `u32`           | —        | Number of rows.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `cols`  | `u32`           | —        | Number of columns.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `cells` | `Vec<GridCell>` | `vec![]` | All cells in the table as a flat, sparse list. The list is ordered by `(row, col)` but is **not** a dense `rows × cols` matrix: cells that are covered by a spanning cell (via `row_span > 1` or `col_span > 1`) do not appear in the list. Only the top-left "origin" cell of a span is present, with its `row_span` and `col_span` fields set accordingly. To reconstruct the full visual grid, iterate over all cells and mark the rectangular region `[row .. row+row_span, col .. col+col_span]` as occupied by that cell. Any `(row, col)` position that is not the origin of any cell is covered by a span from an earlier cell. The length of this vec is `≤ rows * cols`. An empty table (`rows == 0 \|\| cols == 0`) produces an empty vec. |

---

### Metadata Types

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field            | Type                      | Default              | Description                                                                                                              |
| ---------------- | ------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `title`          | `Option<String>`          | `Default::default()` | Document title from `<title>` tag                                                                                        |
| `description`    | `Option<String>`          | `Default::default()` | Document description from `<meta name="description">` tag                                                                |
| `keywords`       | `Vec<String>`             | `vec![]`             | Document keywords from `<meta name="keywords">` tag, split on commas                                                     |
| `author`         | `Option<String>`          | `Default::default()` | Document author from `<meta name="author">` tag                                                                          |
| `canonical_url`  | `Option<String>`          | `Default::default()` | Canonical URL from `<link rel="canonical">` tag                                                                          |
| `base_href`      | `Option<String>`          | `Default::default()` | Base URL from `<base href="">` tag for resolving relative URLs                                                           |
| `language`       | `Option<String>`          | `Default::default()` | Document language from `lang` attribute                                                                                  |
| `text_direction` | `Option<TextDirection>`   | `Default::default()` | Document text direction from `dir` attribute                                                                             |
| `open_graph`     | `HashMap<String, String>` | `HashMap::new()`     | Open Graph metadata (og:\* properties) for social media Keys like "title", "description", "image", "url", etc.           |
| `twitter_card`   | `HashMap<String, String>` | `HashMap::new()`     | Twitter Card metadata (twitter:\* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags`      | `HashMap<String, String>` | `HashMap::new()`     | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content           |

---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field         | Type             | Default | Description                               |
| ------------- | ---------------- | ------- | ----------------------------------------- |
| `level`       | `u8`             | —       | Header level: 1 (h1) through 6 (h6)       |
| `text`        | `String`         | —       | Normalized text content of the header     |
| `id`          | `Option<String>` | `None`  | HTML id attribute if present              |
| `depth`       | `usize`          | —       | Document tree depth at the header element |
| `html_offset` | `usize`          | —       | Byte offset in original HTML document     |

---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field        | Type                      | Default | Description                                                         |
| ------------ | ------------------------- | ------- | ------------------------------------------------------------------- |
| `href`       | `String`                  | —       | The href URL value                                                  |
| `text`       | `String`                  | —       | Link text content (normalized, concatenated if mixed with elements) |
| `title`      | `Option<String>`          | `None`  | Optional title attribute (often shown as tooltip)                   |
| `link_type`  | `LinkType`                | —       | Link type classification                                            |
| `rel`        | `Vec<String>`             | —       | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical")  |
| `attributes` | `HashMap<String, String>` | —       | Additional HTML attributes                                          |

---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field        | Type                      | Default | Description                                             |
| ------------ | ------------------------- | ------- | ------------------------------------------------------- |
| `src`        | `String`                  | —       | Image source (URL, data URI, or SVG content identifier) |
| `alt`        | `Option<String>`          | `None`  | Alternative text from alt attribute (for accessibility) |
| `title`      | `Option<String>`          | `None`  | Title attribute (often shown as tooltip)                |
| `dimensions` | `Vec<u32>`                | `None`  | Image dimensions as (width, height) if available        |
| `image_type` | `ImageType`               | —       | Image type classification                               |
| `attributes` | `HashMap<String, String>` | —       | Additional HTML attributes                              |

---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field             | Type                  | Default  | Description                                                   |
| ----------------- | --------------------- | -------- | ------------------------------------------------------------- |
| `document`        | `DocumentMetadata`    | —        | Document-level metadata (title, description, canonical, etc.) |
| `headers`         | `Vec<HeaderMetadata>` | `vec![]` | Extracted header elements with hierarchy                      |
| `links`           | `Vec<LinkMetadata>`   | `vec![]` | Extracted hyperlinks with type classification                 |
| `images`          | `Vec<ImageMetadata>`  | `vec![]` | Extracted images with source and dimensions                   |
| `structured_data` | `Vec<StructuredData>` | `vec![]` | Extracted structured data blocks                              |

---

### Document Structure

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field           | Type                | Default | Description                                         |
| --------------- | ------------------- | ------- | --------------------------------------------------- |
| `nodes`         | `Vec<DocumentNode>` | —       | All nodes in document reading order.                |
| `source_format` | `Option<String>`    | `None`  | The source format (always "html" for this library). |

---

#### DocumentNode

A single node in the document tree.

| Field         | Type                      | Default                | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------- | ------------------------- | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | `String`                  | —                      | Deterministic node identifier.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `content`     | `NodeContent`             | —                      | The semantic content of this node.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `parent`      | `Option<u32>`             | `None`                 | Index of the parent node (None for root nodes).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `children`    | `Vec<u32>`                | `/* serde(default) */` | Indices of child nodes in reading order.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `annotations` | `Vec<TextAnnotation>`     | `/* serde(default) */` | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `attributes`  | `HashMap<String, String>` | `None`                 | Format-specific attributes preserved from the source HTML element. Keys are lowercased attribute names as they appear in the HTML (e.g. `"class"`, `"id"`, `"data-foo"`). Values are the raw attribute strings, copied verbatim from the source — no HTML entity decoding is applied here. The map is `None` when no attributes are present (omitted entirely in serialized output). Not every HTML attribute is preserved: only attributes that carry semantic or structural significance for the node type are collected. For example, heading nodes capture the `"id"` attribute for anchor linking; other element-level attributes may be silently dropped. |

---

#### GridCell

A single cell in a table grid.

| Field       | Type     | Default                | Description                                    |
| ----------- | -------- | ---------------------- | ---------------------------------------------- |
| `content`   | `String` | —                      | The text content of the cell.                  |
| `row`       | `u32`    | —                      | 0-indexed row position.                        |
| `col`       | `u32`    | —                      | 0-indexed column position.                     |
| `row_span`  | `u32`    | `/* serde(default) */` | Number of rows this cell spans (default 1).    |
| `col_span`  | `u32`    | `/* serde(default) */` | Number of columns this cell spans (default 1). |
| `is_header` | `bool`   | `/* serde(default) */` | Whether this is a header cell (`<th>`).        |

---

#### TableData

A top-level extracted table with both structured data and markdown representation.

| Field      | Type        | Default | Description                           |
| ---------- | ----------- | ------- | ------------------------------------- |
| `grid`     | `TableGrid` | —       | The structured table grid.            |
| `markdown` | `String`    | —       | The markdown rendering of this table. |

---

#### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field             | Type                      | Default | Description                                             |
| ----------------- | ------------------------- | ------- | ------------------------------------------------------- |
| `node_type`       | `NodeType`                | —       | Coarse-grained node type classification                 |
| `tag_name`        | `String`                  | —       | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes`      | `HashMap<String, String>` | —       | All HTML attributes as key-value pairs                  |
| `depth`           | `usize`                   | —       | Depth in the DOM tree (0 = root)                        |
| `index_in_parent` | `usize`                   | —       | Index among siblings (0-based)                          |
| `parent_tag`      | `Option<String>`          | `None`  | Parent element's tag name (None if root)                |
| `is_inline`       | `bool`                    | —       | Whether this element is treated as inline vs block      |

---

### Other Types

#### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field         | Type                 | Default | Description                                                     |
| ------------- | -------------------- | ------- | --------------------------------------------------------------- |
| `data_type`   | `StructuredDataType` | —       | Type of structured data (JSON-LD, Microdata, RDFa)              |
| `raw_json`    | `String`             | —       | Raw JSON string (for JSON-LD) or serialized representation      |
| `schema_type` | `Option<String>`     | `None`  | Schema type if detectable (e.g., "Article", "Event", "Product") |

---

#### TextAnnotation

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

| Field   | Type             | Default | Description                                                |
| ------- | ---------------- | ------- | ---------------------------------------------------------- |
| `start` | `u32`            | —       | Start byte offset (inclusive) into the parent node's text. |
| `end`   | `u32`            | —       | End byte offset (exclusive) into the parent node's text.   |
| `kind`  | `AnnotationKind` | —       | The type of annotation.                                    |

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

| Field     | Type          | Default | Description                     |
| --------- | ------------- | ------- | ------------------------------- |
| `message` | `String`      | —       | Human-readable warning message. |
| `kind`    | `WarningKind` | —       | The category of warning.        |

---

#### VisitorHandle

Shareable, thread-safe handle to a user-provided HTML visitor implementation.

Pass an instance wrapped in this handle to `ConversionOptions` to
customise how the HTML document is traversed and converted to Markdown.
The handle may be cloned and shared across threads without additional
synchronisation on the caller's side.

_Opaque type — fields are not directly accessible._

---

#### HtmlVisitor

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

_Opaque type — fields are not directly accessible._

---

### Enums

#### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Variant         | Wire value      | Description                                                                                        |
| --------------- | --------------- | -------------------------------------------------------------------------------------------------- |
| `Bold`          | `bold`          | Bold / strong emphasis.                                                                            |
| `Italic`        | `italic`        | Italic / emphasis.                                                                                 |
| `Underline`     | `underline`     | Underline.                                                                                         |
| `Strikethrough` | `strikethrough` | Strikethrough / deleted text.                                                                      |
| `Code`          | `code`          | Inline code.                                                                                       |
| `Subscript`     | `subscript`     | Subscript text.                                                                                    |
| `Superscript`   | `superscript`   | Superscript text.                                                                                  |
| `Highlight`     | `highlight`     | Highlighted / marked text.                                                                         |
| `Link`          | `link`          | A hyperlink sourced from an `<a href="...">` element. — Fields: `url`: `String`, `title`: `String` |

---

#### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Variant     | Description                                                                      |
| ----------- | -------------------------------------------------------------------------------- |
| `Indented`  | Indented code blocks (4 spaces). `CommonMark` standard.                          |
| `Backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `Tildes`    | Fenced code blocks with tildes (~~~). Supports language hints.                   |

---

#### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Variant      | Description                                        |
| ------------ | -------------------------------------------------- |
| `Underlined` | Underlined style (=== for h1, --- for h2).         |
| `Atx`        | ATX style (# for h1, ## for h2, etc.). Default.    |
| `AtxClosed`  | ATX closed style (# title #, with closing hashes). |

---

#### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Variant       | Description                                                  |
| ------------- | ------------------------------------------------------------ |
| `DoubleEqual` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `Html`        | Preserve as HTML (==text==). Original HTML tag.              |
| `Bold`        | Render as bold (**text**). Uses strong emphasis.             |
| `None`        | Strip formatting, render as plain text. No markup.           |

---

#### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Variant     | Wire value   | Description                                        |
| ----------- | ------------ | -------------------------------------------------- |
| `DataUri`   | `data_uri`   | Data URI embedded image (base64 or other encoding) |
| `InlineSvg` | `inline_svg` | Inline SVG element                                 |
| `External`  | `external`   | External image URL (http/https)                    |
| `Relative`  | `relative`   | Relative image path                                |

---

#### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Variant     | Description                                                            |
| ----------- | ---------------------------------------------------------------------- |
| `Inline`    | Inline links: `[text](url)`. Default.                                  |
| `Reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |

---

#### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Variant    | Wire value | Description                                           |
| ---------- | ---------- | ----------------------------------------------------- |
| `Anchor`   | `anchor`   | Anchor link within same document (href starts with #) |
| `Internal` | `internal` | Internal link within same domain                      |
| `External` | `external` | External link to different domain                     |
| `Email`    | `email`    | Email link (mailto:)                                  |
| `Phone`    | `phone`    | Phone link (tel:)                                     |
| `Other`    | `other`    | Other protocol or unclassifiable                      |

---

#### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Variant  | Description                                                                   |
| -------- | ----------------------------------------------------------------------------- |
| `Spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `Tabs`   | Use tabs for indentation.                                                     |

---

#### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Variant     | Description                                                            |
| ----------- | ---------------------------------------------------------------------- |
| `Spaces`    | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `Backslash` | Backslash at end of line. Alternative Markdown syntax.                 |

---

#### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Variant          | Wire value        | Description                                                                                                                                        |
| ---------------- | ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Heading`        | `heading`         | A heading element (h1-h6). — Fields: `level`: `u8`, `text`: `String`                                                                               |
| `Paragraph`      | `paragraph`       | A paragraph of text. — Fields: `text`: `String`                                                                                                    |
| `List`           | `list`            | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool`                                                |
| `ListItem`       | `list_item`       | A single list item. — Fields: `text`: `String`                                                                                                     |
| `Table`          | `table`           | A table with structured cell data. — Fields: `grid`: `TableGrid`                                                                                   |
| `Image`          | `image`           | An image element. — Fields: `description`: `String`, `src`: `String`, `image_index`: `u32`                                                         |
| `Code`           | `code`            | A code block or inline code. — Fields: `text`: `String`, `language`: `String`                                                                      |
| `Quote`          | `quote`           | A block quote container.                                                                                                                           |
| `DefinitionList` | `definition_list` | A definition list container.                                                                                                                       |
| `DefinitionItem` | `definition_item` | A definition list entry with term and description. — Fields: `term`: `String`, `definition`: `String`                                              |
| `RawBlock`       | `raw_block`       | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `String`, `content`: `String`                                |
| `MetadataBlock`  | `metadata_block`  | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `Vec<Vec<String>>`                                             |
| `Group`          | `group`           | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `String`, `heading_level`: `u8`, `heading_text`: `String` |

---

#### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Variant                 | Description                                    |
| ----------------------- | ---------------------------------------------- |
| `Text`                  | Text node (most frequent - 100+ per document)  |
| `Element`               | Generic element node                           |
| `Heading`               | Heading elements (h1-h6)                       |
| `Paragraph`             | Paragraph element                              |
| `Div`                   | Generic div container                          |
| `Blockquote`            | Blockquote element                             |
| `Pre`                   | Preformatted text block                        |
| `Hr`                    | Horizontal rule                                |
| `List`                  | Ordered or unordered list (ul, ol)             |
| `ListItem`              | List item (li)                                 |
| `DefinitionList`        | Definition list (dl)                           |
| `DefinitionTerm`        | Definition term (dt)                           |
| `DefinitionDescription` | Definition description (dd)                    |
| `Table`                 | Table element                                  |
| `TableRow`              | Table row (tr)                                 |
| `TableCell`             | Table cell (td, th)                            |
| `TableHeader`           | Table header cell (th)                         |
| `TableBody`             | Table body (tbody)                             |
| `TableHead`             | Table head (thead)                             |
| `TableFoot`             | Table foot (tfoot)                             |
| `Link`                  | Anchor link (a)                                |
| `Image`                 | Image (img)                                    |
| `Strong`                | Strong/bold (strong, b)                        |
| `Em`                    | Emphasis/italic (em, i)                        |
| `Code`                  | Inline code (code)                             |
| `Strikethrough`         | Strikethrough (s, del, strike)                 |
| `Underline`             | Underline (u, ins)                             |
| `Subscript`             | Subscript (sub)                                |
| `Superscript`           | Superscript (sup)                              |
| `Mark`                  | Mark/highlight (mark)                          |
| `Small`                 | Small text (small)                             |
| `Br`                    | Line break (br)                                |
| `Span`                  | Span element                                   |
| `Article`               | Article element                                |
| `Section`               | Section element                                |
| `Nav`                   | Navigation element                             |
| `Aside`                 | Aside element                                  |
| `Header`                | Header element                                 |
| `Footer`                | Footer element                                 |
| `Main`                  | Main element                                   |
| `Figure`                | Figure element                                 |
| `Figcaption`            | Figure caption                                 |
| `Time`                  | Time element                                   |
| `Details`               | Details element                                |
| `Summary`               | Summary element                                |
| `Form`                  | Form element                                   |
| `Input`                 | Input element                                  |
| `Select`                | Select element                                 |
| `Option`                | Option element                                 |
| `Button`                | Button element                                 |
| `Textarea`              | Textarea element                               |
| `Label`                 | Label element                                  |
| `Fieldset`              | Fieldset element                               |
| `Legend`                | Legend element                                 |
| `Audio`                 | Audio element                                  |
| `Video`                 | Video element                                  |
| `Picture`               | Picture element                                |
| `Source`                | Source element                                 |
| `Iframe`                | Iframe element                                 |
| `Svg`                   | SVG element                                    |
| `Canvas`                | Canvas element                                 |
| `Ruby`                  | Ruby annotation                                |
| `Rt`                    | Ruby text                                      |
| `Rp`                    | Ruby parenthesis                               |
| `Abbr`                  | Abbreviation                                   |
| `Kbd`                   | Keyboard input                                 |
| `Samp`                  | Sample output                                  |
| `Var`                   | Variable                                       |
| `Cite`                  | Citation                                       |
| `Q`                     | Quote                                          |
| `Del`                   | Deleted text                                   |
| `Ins`                   | Inserted text                                  |
| `Data`                  | Data element                                   |
| `Meter`                 | Meter element                                  |
| `Progress`              | Progress element                               |
| `Output`                | Output element                                 |
| `Template`              | Template element                               |
| `Slot`                  | Slot element                                   |
| `Html`                  | HTML root element                              |
| `Head`                  | Head element                                   |
| `Body`                  | Body element                                   |
| `Title`                 | Title element                                  |
| `Meta`                  | Meta element                                   |
| `LinkTag`               | Link element (not anchor)                      |
| `Style`                 | Style element                                  |
| `Script`                | Script element                                 |
| `Base`                  | Base element                                   |
| `Custom`                | Custom element (web components) or unknown tag |

---

#### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Variant    | Description                                         |
| ---------- | --------------------------------------------------- |
| `Markdown` | Standard Markdown (CommonMark compatible). Default. |
| `Djot`     | Djot lightweight markup language.                   |
| `Plain`    | Plain text output (no markup, visible text only).   |

---

#### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Variant      | Description                                                                        |
| ------------ | ---------------------------------------------------------------------------------- |
| `Minimal`    | Minimal cleanup. Remove only essential noise (scripts, styles).                    |
| `Standard`   | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `Aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure.           |

---

#### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Variant     | Wire value  | Description                                                |
| ----------- | ----------- | ---------------------------------------------------------- |
| `JsonLd`    | `json_ld`   | JSON-LD (JSON for Linking Data) script blocks              |
| `Microdata` | `microdata` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFa`      | `rdfa`      | RDF in Attributes (RDFa) markup                            |

---

#### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Variant       | Wire value | Description                                          |
| ------------- | ---------- | ---------------------------------------------------- |
| `LeftToRight` | `ltr`      | Left-to-right text flow (default for Latin scripts)  |
| `RightToLeft` | `rtl`      | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `Auto`        | `auto`     | Automatic directionality detection                   |

---

#### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Variant        | Description                                                                                                                                                       |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Continue`     | Continue with default conversion behavior                                                                                                                         |
| `Custom`       | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `_0`: `String` |
| `Skip`         | Skip this element entirely (don't output anything) The element and all its children are ignored in the output.                                                    |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output.                                                     |
| `Error`        | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `_0`: `String`                                               |

---

#### WarningKind

Categories of processing warnings.

| Variant                 | Wire value                | Description                                                                  |
| ----------------------- | ------------------------- | ---------------------------------------------------------------------------- |
| `ImageExtractionFailed` | `image_extraction_failed` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `EncodingFallback`      | `encoding_fallback`       | The input encoding was not recognized; fell back to UTF-8.                   |
| `TruncatedInput`        | `truncated_input`         | The input was truncated due to size limits.                                  |
| `MalformedHtml`         | `malformed_html`          | The HTML was malformed but processing continued with best effort.            |
| `SanitizationApplied`   | `sanitization_applied`    | Sanitization was applied to remove potentially unsafe content.               |
| `DepthLimitExceeded`    | `depth_limit_exceeded`    | DOM traversal was truncated because max_depth was exceeded.                  |

---

#### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Variant      | Description                                                                                  |
| ------------ | -------------------------------------------------------------------------------------------- |
| `Normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `Strict`     | Preserve all whitespace exactly as it appears in the HTML.                                   |

---
