---
title: "Configuration Reference"
---

## Configuration Reference

This page documents all configuration types and their defaults across all languages.

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
