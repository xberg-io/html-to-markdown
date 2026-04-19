---
name: html-to-markdown
description: >-
  Convert HTML to Markdown, Djot, or plain text with structured extraction.
  Use when writing code that calls html-to-markdown APIs in Rust, Python,
  TypeScript, Go, Ruby, PHP, Java, C#, Elixir, R, C, or WASM.
  Covers installation, conversion, configuration, metadata extraction,
  document structure, and CLI usage.
license: MIT
metadata:
  author: kreuzberg-dev
  version: "3.2.0"
  repository: https://github.com/kreuzberg-dev/html-to-markdown
---

# html-to-markdown

html-to-markdown is a high-performance HTML to Markdown converter with a Rust core and 12 native language bindings. It converts HTML to CommonMark Markdown, Djot, or plain text in a single pass, optionally extracting metadata, tables, inline images, and a structured document tree.

Use this skill when writing code that:

- Converts HTML strings or files to Markdown, Djot, or plain text
- Extracts metadata (title, OG tags, headers, links, images, structured data) from HTML
- Extracts structured table data from HTML
- Extracts inline images (data URIs, SVGs) from HTML
- Uses preprocessing to clean noisy HTML (ads, navigation, forms) before conversion
- Implements custom element conversion with the visitor pattern

## Installation

### Python

```bash
pip install html-to-markdown
```

### Node.js / TypeScript

```bash
npm install @kreuzberg/html-to-markdown
```

### Rust

```toml
# Cargo.toml
[dependencies]
html-to-markdown-rs = "3"
# Default features: ["metadata"]
# Optional: features = ["metadata", "inline-images", "visitor", "async-visitor", "serde"]
# Full: features = ["full"]
```

### Go

```bash
go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v3
```

Import path: `github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown`

### Ruby

```bash
gem install html-to-markdown
```

### PHP

```bash
composer require kreuzberg-dev/html-to-markdown
```

### Java (Maven)

```xml
<dependency>
  <groupId>dev.kreuzberg</groupId>
  <artifactId>html-to-markdown</artifactId>
  <version>3.2.0</version>
</dependency>
```

### C# (.NET)

```bash
dotnet add package KreuzbergDev.HtmlToMarkdown
```

### Elixir

```elixir
# mix.exs
{:html_to_markdown, "~> 3.0"}
```

### R

```r
install.packages("htmltomarkdown")
```

### CLI

```bash
# Install via cargo
cargo install html-to-markdown-cli

# Basic usage
html-to-markdown input.html                          # convert file to stdout
html-to-markdown input.html -o output.md             # convert file to output file
cat file.html | html-to-markdown                     # read from stdin
html-to-markdown --url https://example.com           # fetch and convert URL

# JSON output (ConversionResult as JSON with content, tables, metadata, images, warnings)
html-to-markdown --json input.html
html-to-markdown --json --include-structure input.html   # include document structure

# Key flags
html-to-markdown --extract-inline-images input.html  # include inline image data in JSON output
html-to-markdown --show-warnings input.html          # print non-fatal warnings to stderr
html-to-markdown --no-content --json input.html      # extract only (no markdown text, just metadata/tables)
html-to-markdown --heading-style atx input.html      # set heading style
html-to-markdown --preprocess input.html             # preprocess HTML before converting
```

### WASM

```bash
npm install @kreuzberg/html-to-markdown-wasm
```

## Quick Start

### Rust

```rust
use html_to_markdown_rs::{convert, ConversionOptions};

let html = "<h1>Hello World</h1><p>This is a paragraph.</p>";
let result = convert(html, None)?;
println!("{}", result.content.unwrap_or_default());
// Output: # Hello World\n\nThis is a paragraph.\n
```

### Python

```python
from html_to_markdown import convert

html = "<h1>Hello World</h1><p>This is a paragraph.</p>"
result = convert(html)
print(result.content)
# Output: # Hello World\n\nThis is a paragraph.\n
```

### TypeScript / Node.js

```typescript
import { convert } from '@kreuzberg/html-to-markdown';

const html = '<h1>Hello World</h1><p>This is a paragraph.</p>';
const result = JSON.parse(convert(html));
console.log(result.content);
// Output: # Hello World\n\nThis is a paragraph.\n
```

### CLI

```bash
# From file
html-to-markdown input.html

# From file, save to output
html-to-markdown input.html -o output.md

# From stdin
cat file.html | html-to-markdown

# JSON output (ConversionResult as JSON)
html-to-markdown --json input.html

# JSON output with full structure (tables, metadata, images)
html-to-markdown --json --include-structure input.html

# Show warnings
html-to-markdown --show-warnings input.html

# Fetch URL
html-to-markdown --url https://example.com > output.md
```

## ConversionResult Fields

All languages return the same data structure (as a dict, object, or struct).

| Field | Rust Type | Python | TypeScript | Description |
|-------|-----------|--------|------------|-------------|
| `content` | `Option<String>` | `str \| None` | `string \| null` | Converted text (Markdown/Djot/plain). `None` only in extraction-only mode. |
| `document` | `Option<DocumentStructure>` | `None` (not yet wired) | `null` (not yet wired) | Structured document tree when `include_document_structure=true` |
| `metadata` | `HtmlMetadata` | `HtmlMetadata \| None` | JSON object or null | Extracted HTML metadata (title, OG, headers, links, images, structured data). Requires `metadata` feature. |
| `tables` | `Vec<TableData>` | `list[TableData]` | `array` | Extracted tables with `grid` (structured cells) and `markdown` fields |
| `images` | `Vec<InlineImage>` | `list` | `array` | Extracted inline images (data URIs, SVGs). Requires `inline-images` feature. |
| `warnings` | `Vec<ProcessingWarning>` | `list[ProcessingWarning]` | `array` | Non-fatal processing warnings with `message` and `kind` fields |

## Configuration

All languages expose the same options. See [references/configuration.md](references/configuration.md) for the complete options table.

### Rust (builder pattern)

```rust
use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle, CodeBlockStyle, OutputFormat};

let options = ConversionOptions::builder()
    .heading_style(HeadingStyle::Atx)
    .code_block_style(CodeBlockStyle::Backticks)
    .autolinks(true)
    .wrap(true)
    .wrap_width(100)
    .output_format(OutputFormat::Markdown)
    .build();

let result = convert(html, Some(options))?;
```

### Python (dataclass)

```python
from html_to_markdown import convert, ConversionOptions, PreprocessingOptions

options = ConversionOptions(
    heading_style="atx",
    code_block_style="backticks",
    autolinks=True,
    wrap=True,
    wrap_width=100,
)
preprocessing = PreprocessingOptions(
    enabled=True,
    preset="aggressive",
)

result = convert(html, options, preprocessing)
print(result.content)
```

### TypeScript / Node.js (object)

```typescript
import { convert } from '@kreuzberg/html-to-markdown';

const options = {
    headingStyle: 'Atx',
    codeBlockStyle: 'Backticks',
    autolinks: true,
    wrap: true,
    wrapWidth: 100,
};

const result = JSON.parse(convert(html, options));
```

### Go (JSON options)

```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown"

result, err := htmltomarkdown.Convert(html)
// result.Content contains the markdown string
// result.Tables contains structured table data
```

### Ruby (hash)

```ruby
require 'html_to_markdown'

result = HtmlToMarkdown.convert(html)
puts result[:content]

# With options
result = HtmlToMarkdown.convert(html, {
  heading_style: "atx",
  code_block_style: "backticks",
})
```

## Metadata Extraction

The `metadata` field (requires `metadata` feature / default in Python/Node) contains:

```python
# Python example
from html_to_markdown import convert

html = """
<html lang="en">
  <head>
    <title>My Article</title>
    <meta name="description" content="An article">
    <meta property="og:title" content="OG Title">
  </head>
  <body>
    <h1>Main Heading</h1>
    <a href="https://example.com">Link</a>
    <img src="photo.jpg" alt="A photo">
  </body>
</html>
"""
result = convert(html)
meta = result.metadata

# Document-level metadata
print(meta.document.title)           # "My Article"
print(meta.document.description)     # "An article"
print(meta.document.language)        # "en"
print(meta.document.open_graph)      # {"title": "OG Title"}

# Headers
print(meta.headers[0].level)         # 1
print(meta.headers[0].text)          # "Main Heading"

# Links
print(meta.links[0].href)            # "https://example.com"
print(meta.links[0].link_type)       # "external"

# Images
print(meta.images[0].alt)            # "A photo"
print(meta.images[0].image_type)     # "external"
```

All metadata is available in `result.metadata` (Python/Rust/Go/Ruby/Elixir) or `JSON.parse(convert(html)).metadata` (TypeScript) from the single `convert()` call.

## Document Structure Extraction

Enable `include_document_structure=True` to get a structured semantic tree:

```python
from html_to_markdown import convert, ConversionOptions

result = convert(html, ConversionOptions(include_document_structure=True))
doc = result.document  # DocumentStructure with nodes array
# Each node has: id, content (node_type + fields), parent, children, annotations
```

Node types include: `heading`, `paragraph`, `list`, `list_item`, `table`, `image`, `code`, `quote`, `group`, `metadata_block`.

## Common Pitfalls

1. **`convert()` returns a result object, not a string.** Access `.content` (Rust/Python) or `JSON.parse()` the return (Node.js) to get the Markdown string.
2. **Node.js `convert()` returns a JSON string.** Always `JSON.parse(convert(html))` — the native NAPI binding returns serialized JSON to avoid type conversion overhead.
3. **Python `convert()` returns a `ConversionResult` object.** Use `result.content` for the Markdown text, not `str(result)` or `result["content"]`.
4. **Rust builder pattern required.** Use `ConversionOptions::builder().field(value).build()` — direct struct construction omits future fields silently.
5. **`metadata` requires the `metadata` feature.** In Rust, the `metadata` feature is included in `default`. In bindings (Python, Node, etc.), metadata is always available.
6. **Python `PreprocessingOptions` is a separate parameter.** Pass it as the second argument to `convert()`, not inside `ConversionOptions`.
7. **`include_document_structure` must be enabled explicitly.** The `document` field is `None` by default to avoid overhead.
8. **Inline image extraction requires `extract_images=True`.** The `images` field is empty unless `ConversionOptions(extract_images=True)` is set.
9. **CLI `--json` outputs JSON, not Markdown.** When `--json` is used, output is the full `ConversionResult` JSON. Use `html-to-markdown input.html` (without `--json`) for plain Markdown output.
10. **Go `Convert()` returns `ExtractionResult` struct.** Access `.Content` (string) not the struct itself.

## Additional Resources

- **[Rust API Reference](references/rust-api.md)** — Complete Rust function signatures, types, feature flags
- **[Python API Reference](references/python-api.md)** — All Python functions, dataclasses, type hints
- **[TypeScript API Reference](references/typescript-api.md)** — All TypeScript functions, interfaces, Buffer support
- **[Other Bindings](references/other-bindings.md)** — Go, Ruby, PHP, Java, C#, Elixir, R, WASM, C FFI
- **[Configuration Reference](references/configuration.md)** — All 30+ ConversionOptions fields with defaults
- **[CLI Reference](references/cli-reference.md)** — All CLI flags and usage examples

GitHub: <https://github.com/kreuzberg-dev/html-to-markdown>
