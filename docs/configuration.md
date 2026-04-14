# Configuration

All options are passed via `ConversionOptions` (builder pattern in Rust, keyword arguments in Python/Ruby/Elixir/R, object literal in TypeScript, struct in Go/Java/C#, constructor in PHP).

## Options Reference

### Output Format

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `output_format` | `"markdown"` \| `"djot"` \| `"plain"` \| `"none"` | `"markdown"` | Target output format. Use `"none"` to skip conversion and only extract metadata/structure. |

### Headings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `heading_style` | `"atx"` \| `"underlined"` \| `"atx_closed"` | `"atx"` | ATX uses `#` prefixes (`# H1`). Underlined uses `===`/`---` for h1/h2. ATX closed adds trailing hashes (`# H1 #`). |

### Lists

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `list_indent_type` | `"spaces"` \| `"tab"` | `"spaces"` | Indentation character for nested lists. |
| `list_indent_width` | `int` (1–8) | `2` | Number of spaces per nesting level (when using spaces). |
| `bullets` | `string` | `"-"` | Characters to cycle through for unordered list markers. For example `"*+-"` uses `*` at level 1, `+` at level 2, `-` at level 3. |

### Text Formatting

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `strong_em_symbol` | `"*"` \| `"_"` | `"*"` | Symbol used for bold (`**text**`) and italic (`*text*`). |
| `newline_style` | `"backslash"` \| `"spaces"` | `"backslash"` | How to render `<br>` tags: backslash at end of line or two trailing spaces. |
| `sub_symbol` | `string` | `""` | Symbol to wrap `<sub>` content (e.g. `"~"` → `~text~`). |
| `sup_symbol` | `string` | `""` | Symbol to wrap `<sup>` content (e.g. `"^"` → `^text^`). |
| `highlight_style` | `"double-equal"` \| `"html"` \| `"bold"` \| `"none"` | `"double-equal"` | Rendering of `<mark>` elements. |

### Escaping

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `escape_asterisks` | `bool` | `false` | Escape `*` characters in text. |
| `escape_underscores` | `bool` | `false` | Escape `_` characters in text. |
| `escape_misc` | `bool` | `false` | Escape characters like `[`, `]`, `<`, `>`, `#`, etc. |
| `escape_ascii` | `bool` | `false` | Escape all ASCII punctuation (strict CommonMark compliance). |

### Code Blocks

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `code_block_style` | `"indented"` \| `"backticks"` \| `"tildes"` | `"indented"` | How to format multi-line code blocks. |
| `code_language` | `string` | `""` | Default language tag for fenced code blocks without an explicit language. |

### Links

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `autolinks` | `bool` | `false` | When link text equals the href, emit `<url>` instead of `[url](url)`. |
| `default_title` | `bool` | `false` | Use the href as link title when no `title` attribute is present. |
| `link_style` | `"inline"` \| `"reference"` | `"inline"` | `inline` emits `[text](url)`. `reference` emits `[text][1]` with numbered definitions collected at the end of the document. |

### Images

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `keep_inline_images_in` | `list[str]` | `[]` | Element names where images should be kept as Markdown `![alt](src)` rather than converted to alt text. |
| `extract_images` | `bool` | `false` | Extract data URIs and embedded SVGs into the `images` field of `ConversionResult`. |
| `skip_images` | `bool` | `false` | Drop image elements entirely. No `![alt](src)` output, no alt-text fallback. |
| `max_image_size` | `int` (bytes) | `5242880` | Maximum byte size for an extracted inline image. Larger images are skipped. 5 MB default. |
| `capture_svg` | `bool` | `false` | Include inline `<svg>` elements in `result.images` when `extract_images` is enabled. |
| `infer_dimensions` | `bool` | `true` | Infer missing `width` and `height` from decoded image bytes when extracting inline images. |

### Tables

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `br_in_tables` | `bool` | `false` | Preserve line breaks in table cells as `<br>` rather than converting to spaces. |

### Whitespace

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `whitespace_mode` | `"normalized"` \| `"strict"` | `"normalized"` | `normalized` cleans excess whitespace; `strict` preserves whitespace as-is. |
| `strip_newlines` | `bool` | `false` | Remove all newlines from input HTML before processing (useful for minified HTML). |

### Wrapping

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `wrap` | `bool` | `false` | Enable line wrapping. |
| `wrap_width` | `int` (20–500) | `80` | Column width for line wrapping when `wrap` is enabled. |

### Element Handling

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `convert_as_inline` | `bool` | `false` | Treat block-level elements as inline (no paragraph breaks). |
| `strip_tags` | `list[str]` | `[]` | Tags to strip entirely (only text content is preserved, no Markdown conversion). |
| `preserve_tags` | `list[str]` | `[]` | Tags to emit verbatim as HTML instead of converting to Markdown. Counterpart to `strip_tags`. |

### Parsing

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `encoding` | `string` | `"utf-8"` | **CLI only.** Character encoding of the input file or stdin. The value must be a label that the WHATWG Encoding Standard recognises (`"windows-1252"`, `"shift_jis"`, `"iso-8859-1"`, etc.). The core library stores but does not use this field; decoding happens in the CLI before the string reaches `convert()`. |

### Debugging

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `debug` | `bool` | `false` | **CLI only.** When true, the CLI prints diagnostic lines to stderr after each conversion (e.g. `"Generated 1234 bytes of markdown"`). The core library stores but does not act on this field. |

### Metadata Extraction

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `extract_metadata` | `bool` | `false` | Populate `result.metadata` with title, headers, links, images, and structured data. |
| `extract_document` | `bool` | `false` | Extract document-level metadata (title, description, charset, language, Open Graph, etc.). Requires `extract_metadata`. |
| `extract_headers` | `bool` | `false` | Extract heading elements with level, text, and id. Requires `extract_metadata`. |
| `extract_links` | `bool` | `false` | Extract anchor tags with href, text, rel, and link type. Requires `extract_metadata`. |
| `extract_structured_data` | `bool` | `false` | Extract JSON-LD, Microdata, and RDFa blocks. Requires `extract_metadata`. |
| `max_structured_data_size` | `int` | `100000` | Maximum byte size of structured data blocks to extract. |

### Document Structure

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `include_document_structure` | `bool` | `false` | Populate `result.document` with a parsed tree of headings, paragraphs, lists, and tables. |

### Preprocessing

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `preprocess` | `bool` | `false` | Clean up HTML before conversion. Required for any of the options below to have an effect. |
| `preset` | `"minimal"` \| `"standard"` \| `"aggressive"` | `"standard"` | Preset level carried through for forward compatibility. Current releases honour the boolean flags below and do not branch on preset. |
| `keep_navigation` | `bool` | `false` | Keep `<nav>`, and keep `<header>`/`<footer>`/`<aside>` that otherwise look like navigation. |
| `keep_forms` | `bool` | `false` | Accepted and stored. Current releases do not drop form elements during preprocessing regardless of this flag. |

When `preprocess` is `true` and `keep_navigation` is `false`, the preprocessor drops:

- every `<nav>` element
- `<header>` elements outside a semantic content ancestor (`<article>`, `<main>`, etc.)
- `<header>`, `<footer>`, and `<aside>` that carry navigation hints in their class or id attributes (`menu`, `sidebar`, `breadcrumb`, and similar)

Script and style tags are always stripped before the DOM walk starts, independent of `preprocess`.

## Output Format Comparison

Given this HTML:

```html
<h1>Report</h1><p>See <a href="https://example.com"><strong>example</strong></a>.</p>
```

=== "markdown"
    ```markdown
    # Report

    See [**example**](https://example.com).
    ```

=== "djot"
    ```djot
    # Report

    See [*example*](https://example.com).
    ```

=== "plain"
    ```text
    Report

    See example.
    ```

`markdown` and `djot` both preserve structure and link targets. `djot` uses single-asterisk strong emphasis; `markdown` uses double asterisks. `plain` strips all formatting, link targets, and list markers, returning readable text only.

## Builder Examples

=== "Rust"
    ```rust
    use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle};

    let options = ConversionOptions::builder()
        .heading_style(HeadingStyle::Atx)
        .code_block_style("backticks")
        .wrap(true)
        .wrap_width(80)
        .extract_metadata(true)
        .build();

    let result = convert(html, Some(options))?;
    ```

=== "Python"
    ```python
    from html_to_markdown import ConversionOptions, convert

    options = ConversionOptions(
        heading_style="atx",
        code_block_style="backticks",
        wrap=True,
        wrap_width=80,
        extract_metadata=True,
    )
    result = convert(html, options)
    ```

=== "TypeScript"
    ```typescript
    import { convert, ConversionOptions } from '@kreuzberg/html-to-markdown';

    const options: ConversionOptions = {
      headingStyle: 'atx',
      codeBlockStyle: 'backticks',
      wrap: true,
      wrapWidth: 80,
      extractMetadata: true,
    };

    const result = convert(html, options);
    ```

=== "Go"
    ```go
    opts := htmltomarkdown.ConversionOptions{
        HeadingStyle:    "atx",
        CodeBlockStyle:  "backticks",
        Wrap:            true,
        WrapWidth:       80,
        ExtractMetadata: true,
    }
    result, err := htmltomarkdown.Convert(html, opts)```

=== "Ruby"
    ```ruby
    result = HtmlToMarkdown.convert(
      html,
      heading_style: :atx,
      code_block_style: :fenced,
      wrap: true,
      wrap_width: 80,
      extract_metadata: true,
    )```

=== "PHP"
    ```php
    $options = new ConversionOptions(
        headingStyle: 'Atx',
        codeBlockStyle: 'Backticks',
        wrap: true,
        wrapWidth: 80,
        extractMetadata: true,
    );
    $result = $converter->convert($html, $options);```

=== "Java"
    ```java
    ConversionOptions options = ConversionOptions.builder()
        .headingStyle("atx")
        .codeBlockStyle("backticks")
        .wrap(true)
        .wrapWidth(80)
        .extractMetadata(true)
        .build();
    ConversionResult result = HtmlToMarkdown.convert(html, options);```

=== "C#"
    ```csharp
    var options = new ConversionOptions
    {
        HeadingStyle = "atx",
        CodeBlockStyle = "backticks",
        Wrap = true,
        WrapWidth = 80,
        ExtractMetadata = true,
    };
    var result = HtmlToMarkdownConverter.Convert(html, options);```

=== "Elixir"
    ```elixir
    opts = %HtmlToMarkdown.Options{
      heading_style: :atx,
      code_block_style: :backticks,
      wrap: true,
      wrap_width: 80,
      extract_metadata: true,
    }
    {:ok, result} = HtmlToMarkdown.convert(html, opts)```

=== "R"
    ```r
    opts <- conversion_options(
      heading_style = "atx",
      code_block_style = "backticks",
      wrap = TRUE,
      wrap_width = 80L,
      extract_metadata = TRUE
    )
    result <- convert(html, opts)```

--8<-- "snippets/feedback.md"
