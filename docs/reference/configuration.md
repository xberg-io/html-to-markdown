---
title: "Configuration Reference"
---

## Configuration Reference

This page documents all configuration types and their defaults across all languages.

### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `str \| None` | `None` | Document title from `<title>` tag |
| `description` | `str \| None` | `None` | Document description from `<meta name="description">` tag |
| `keywords` | `list[str]` | `[]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `str \| None` | `None` | Document author from `<meta name="author">` tag |
| `canonical_url` | `str \| None` | `None` | Canonical URL from `<link rel="canonical">` tag |
| `base_href` | `str \| None` | `None` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `str \| None` | `None` | Document language from `lang` attribute |
| `text_direction` | `TextDirection \| None` | `None` | Document text direction from `dir` attribute |
| `open_graph` | `dict[str, str]` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitter_card` | `dict[str, str]` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags` | `dict[str, str]` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |

---

### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `list[HeaderMetadata]` | `[]` | Extracted header elements with hierarchy |
| `links` | `list[LinkMetadata]` | `[]` | Extracted hyperlinks with type classification |
| `images` | `list[ImageMetadata]` | `[]` | Extracted images with source and dimensions |
| `structured_data` | `list[StructuredData]` | `[]` | Extracted structured data blocks |

---

### ConversionOptions

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
| `compact_tables` | `bool` | `False` | Emit tables without column padding (compact GFM format). When `True`, column widths are not computed and cells are emitted with no trailing spaces. Separator rows use exactly `---` per column. Produces token-efficient output suitable for RAG / LLM contexts. Default `False` (aligned padding preserved). |
| `highlight_style` | `HighlightStyle` | `HighlightStyle.DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extract_metadata` | `bool` | `True` | Populate `result.metadata` with `<head>` / `<meta>` extraction (title, description, Open Graph, Twitter Card, JSON-LD, …). Default `True`. Disabling skips the metadata pass only — table extraction into `result.tables` runs unconditionally. |
| `whitespace_mode` | `WhitespaceMode` | `WhitespaceMode.NORMALIZED` | Controls how whitespace sequences are normalised in the converted output. - `WhitespaceMode.Normalized` (default) — collapses consecutive whitespace characters (spaces, tabs, newlines) to a single space, matching browser rendering behaviour. - `WhitespaceMode.Strict` — preserves all whitespace exactly as it appears in the source HTML, including runs of spaces and embedded newlines. Choose `Strict` only when the source HTML uses deliberate whitespace (e.g. pre-formatted content outside `<pre>` tags). For most documents `Normalized` produces cleaner output. |
| `strip_newlines` | `bool` | `False` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `False` | Wrap long lines at `wrap_width` characters. |
| `wrap_width` | `int` | `80` | Maximum output line width in characters when `wrap` is `True` (default `80`). Lines are broken at word boundaries so that no line exceeds this length. A value of `0` is treated as "no limit" — equivalent to leaving `wrap` disabled. Has no effect when `wrap` is `False`. |
| `convert_as_inline` | `bool` | `False` | Treat the entire document as inline content (no block-level wrappers). |
| `sub_symbol` | `str` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `sup_symbol` | `str` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newline_style` | `NewlineStyle` | `NewlineStyle.SPACES` | How to encode hard line breaks (`<br>`) in Markdown. |
| `code_block_style` | `CodeBlockStyle` | `CodeBlockStyle.BACKTICKS` | Style used for fenced code blocks (backticks or tilde). |
| `keep_inline_images_in` | `list[str]` | `[]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Options for the HTML pre-processing pass applied before conversion begins. Pre-processing runs before the HTML is handed to the converter and can perform operations such as unwrapping redundant wrapper elements, removing tracking pixels, and normalising vendor-specific markup. See `PreprocessingOptions` for the full set of knobs. Defaults to `PreprocessingOptions.default()`, which enables the standard cleaning passes. Set individual fields on `PreprocessingOptions` (or construct via `ConversionOptions.builder`) to opt in or out of specific passes. |
| `encoding` | `str` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `False` | Emit debug information during conversion. |
| `strip_tags` | `list[str]` | `[]` | HTML tag names whose content is stripped from the output entirely. |
| `preserve_tags` | `list[str]` | `[]` | HTML tag names that are preserved verbatim in the output. |
| `skip_images` | `bool` | `False` | Skip conversion of `<img>` elements (omit images from output). |
| `url_escape_style` | `UrlEscapeStyle` | `UrlEscapeStyle.ANGLE` | URL encoding strategy for link and image destinations. Controls how special characters in URL destinations are escaped: - `UrlEscapeStyle.Angle` (default) — wraps the destination in angle brackets when it contains spaces or newlines. Some parsers misinterpret `>` inside such a destination. - `UrlEscapeStyle.Percent` — percent-encodes every character that is not an RFC 3986 unreserved character or `/`, producing a destination that all Markdown parsers handle correctly even when the URL contains `<`, `>`, spaces, or parentheses. |
| `link_style` | `LinkStyle` | `LinkStyle.INLINE` | Link rendering style (inline or reference). |
| `output_format` | `OutputFormat` | `OutputFormat.MARKDOWN` | Target output format (Markdown, plain text, etc.). |
| `include_document_structure` | `bool` | `False` | Include structured document tree in result. |
| `extract_images` | `bool` | `False` | Extract inline images from data URIs and SVGs. |
| `max_image_size` | `int` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `capture_svg` | `bool` | `False` | Capture SVG elements as images. |
| `infer_dimensions` | `bool` | `True` | Infer image dimensions from data. |
| `max_depth` | `int \| None` | `None` | Maximum DOM traversal depth. `None` means unlimited. When set, subtrees beyond this depth are silently truncated. |
| `exclude_selectors` | `list[str]` | `[]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]` |
| `visitor` | `VisitorHandle \| None` | `None` | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`. |

---

### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `True` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `bool` | `True` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `bool` | `True` | Remove form elements (forms, inputs, buttons, etc.) |

---

### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str \| None` | `None` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `DocumentStructure \| None` | `None` | Structured document tree with semantic elements. Populated when `ConversionOptions.include_document_structure` is `True`. `None` otherwise (the default), which avoids the overhead of building the tree. When present, the tree mirrors the converted document: headings open `Group` sections, paragraphs and list items carry inline `TextAnnotation`s, and tables reference the same `TableGrid` data exposed in `Self.tables`. Note: this field is independent of the `metadata` feature flag. Document structure collection is always available at runtime; it is gated only by the runtime option, not by a compile-time feature. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `list[TableData]` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `list[str]` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `list[ProcessingWarning]` | `[]` | Non-fatal processing warnings. |

---

### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `int` | — | Number of rows. |
| `cols` | `int` | — | Number of columns. |
| `cells` | `list[GridCell]` | `[]` | All cells in the table as a flat, sparse list. The list is ordered by `(row, col)` but is **not** a dense `rows × cols` matrix: cells that are covered by a spanning cell (via `row_span > 1` or `col_span > 1`) do not appear in the list. Only the top-left "origin" cell of a span is present, with its `row_span` and `col_span` fields set accordingly. To reconstruct the full visual grid, iterate over all cells and mark the rectangular region `[row .. row+row_span, col .. col+col_span]` as occupied by that cell. Any `(row, col)` position that is not the origin of any cell is covered by a span from an earlier cell. The length of this vec is `≤ rows * cols`. An empty table (`rows == 0 \|\| cols == 0`) produces an empty vec. |

---

### Enums

#### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Variant | Description |
|---------|-------------|
| `Indented` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `Backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `Tildes` | Fenced code blocks with tildes (~~~). Supports language hints. |

---

#### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Variant | Description |
|---------|-------------|
| `Underlined` | Underlined style (=== for h1, --- for h2). |
| `Atx` | ATX style (# for h1, ## for h2, etc.). Default. |
| `AtxClosed` | ATX closed style (# title #, with closing hashes). |

---

#### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Variant | Description |
|---------|-------------|
| `DoubleEqual` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `Html` | Preserve as HTML (==text==). Original HTML tag. |
| `Bold` | Render as bold (**text**). Uses strong emphasis. |
| `None` | Strip formatting, render as plain text. No markup. |

---

#### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Variant | Description |
|---------|-------------|
| `Inline` | Inline links: `[text](url)`. Default. |
| `Reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |

---

#### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Variant | Description |
|---------|-------------|
| `Spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `Tabs` | Use tabs for indentation. |

---

#### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Variant | Description |
|---------|-------------|
| `Spaces` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `Backslash` | Backslash at end of line. Alternative Markdown syntax. |

---

#### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Variant | Description |
|---------|-------------|
| `Markdown` | Standard Markdown (CommonMark compatible). Default. |
| `Djot` | Djot lightweight markup language. |
| `Plain` | Plain text output (no markup, visible text only). |

---

#### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Variant | Description |
|---------|-------------|
| `Minimal` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `Standard` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `Aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure. |

---

#### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Variant | Wire value | Description |
|---------|------------|-------------|
| `LeftToRight` | `ltr` | Left-to-right text flow (default for Latin scripts) |
| `RightToLeft` | `rtl` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `Auto` | `auto` | Automatic directionality detection |

---

#### UrlEscapeStyle

URL encoding strategy for link and image destinations.

Controls how special characters in URL destinations are handled when they
require escaping to produce valid Markdown.

The `Angle` variant (default) wraps the destination in angle brackets:
`[text](<url with spaces>)`. This is the CommonMark-specified escape hatch
but breaks when the URL itself contains `>`.

The `Percent` variant percent-encodes every character that is not an RFC 3986
unreserved character or `/`, producing a destination safe for all Markdown
parsers: `[text](url%20with%20spaces)`.

| Variant | Description |
|---------|-------------|
| `Angle` | Wrap destinations that contain spaces or newlines in angle brackets. Default. |
| `Percent` | Percent-encode all characters that are not RFC 3986 unreserved or `/`. |

---

#### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Variant | Description |
|---------|-------------|
| `Normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `Strict` | Preserve all whitespace exactly as it appears in the HTML. |

---
