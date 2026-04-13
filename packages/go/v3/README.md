# html-to-markdown

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/html-to-markdown-rs">
    <img src="https://img.shields.io/crates/v/html-to-markdown-rs?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/html-to-markdown/">
    <img src="https://img.shields.io/pypi/v/html-to-markdown?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node">
    <img src="https://img.shields.io/npm/v/@kreuzberg/html-to-markdown-node?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/html-to-markdown-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/html-to-markdown?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/html-to-markdown?label=Go&color=007ec6&filter=v3.1.0" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown/">
    <img src="https://img.shields.io/nuget/v/KreuzbergDev.HtmlToMarkdown?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg-dev/html-to-markdown">
    <img src="https://img.shields.io/packagist/v/kreuzberg-dev/html-to-markdown?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/html-to-markdown">
    <img src="https://img.shields.io/gem/v/html-to-markdown?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://hex.pm/packages/html_to_markdown">
    <img src="https://img.shields.io/hexpm/v/html_to_markdown?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://kreuzberg-dev.r-universe.dev/htmltomarkdown">
    <img src="https://img.shields.io/cran/v/htmltomarkdown?label=R&color=007ec6" alt="R">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C">
  </a>

  <!-- Project Info -->
  <a href="https://docs.html-to-markdown.kreuzberg.dev">
    <img src="https://img.shields.io/badge/Docs-kreuzberg.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  </a>
</div>

<img width="1128" height="191" alt="html-to-markdown" src="https://github.com/user-attachments/assets/419fc06c-8313-4324-b159-4b4d3cfce5c0" />

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/pXxagNK2zN">
      <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>


High-performance HTML to Markdown converter with Go bindings to the Rust core library.
Supports automatic downloading of prebuilt FFI libraries for Linux, macOS, and Windows with customizable caching.


## Installation

```bash
go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown
```



Requires Go 1.25+. After installing the package, run `go generate` to automatically download the platform-specific FFI library:

```bash
go generate
```

This downloads the native library from GitHub releases and generates the necessary CGO flags. The library is cached in `~/.html-to-markdown/` for subsequent builds.

Alternatively, you can manually set `CGO_CFLAGS` and `CGO_LDFLAGS` environment variables if you prefer to manage the FFI library yourself.






## Performance Snapshot

Apple M4 • Real Wikipedia documents • `Convert()` (Go)

| Document | Size | Latency | Throughput |
| -------- | ---- | ------- | ---------- |
| Lists (Timeline) | 129KB | 0.46ms | 277.5 MB/s |
| Tables (Countries) | 360KB | 1.37ms | 262.1 MB/s |
| Mixed (Python wiki) | 656KB | 2.75ms | 237.9 MB/s |




## Quick Start

Basic conversion:

```go
package main

import (
    "fmt"
    "log"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown"
)

func main() {
    html := "<h1>Hello World</h1><p>This is a paragraph.</p>"

    result, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatal(err)
    }

    if result.Content != nil {
        fmt.Println(*result.Content)
    }
}
```



With conversion options:

```go
package main

import (
    "fmt"
    "log"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown"
)

func main() {
    // Check library version
    version := htmltomarkdown.Version()
    fmt.Printf("html-to-markdown version: %s\n", version)

    html := "<h1>Hello</h1><p>Welcome</p>"

    // Convert with error handling
    result, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatalf("Conversion failed: %v", err)
    }

    if result.Content != nil {
        fmt.Println(*result.Content)
    }
}
```




## API Reference

### Core Function


**`Convert(html string, options ...ConversionOptions) (ConversionResult, error)`**

Converts HTML to Markdown. Returns a `ConversionResult` struct with all results in a single call.

```go
result, err := htmltomarkdown.Convert(html)
markdown  := result.Content    // *string — converted Markdown
metadata  := result.Metadata   // *Metadata — when ExtractMetadata: true
tables    := result.Tables     // []TableData — when ExtractTables: true
```



### Options

**`ConversionOptions`** – Key configuration fields:

- `heading_style`: Heading format (`"underlined"` | `"atx"` | `"atx_closed"`) — default: `"underlined"`
- `list_indent_width`: Spaces per indent level — default: `2`
- `bullets`: Bullet characters cycle — default: `"*+-"`
- `wrap`: Enable text wrapping — default: `false`
- `wrap_width`: Wrap at column — default: `80`
- `code_language`: Default fenced code block language — default: none
- `extract_metadata`: Enable metadata extraction into `result.metadata` — default: `false`
- `extract_tables`: Enable structured table extraction into `result.tables` — default: `false`
- `output_format`: Output markup format (`"markdown"` | `"djot"` | `"plain"`) — default: `"markdown"`


## Djot Output Format

The library supports converting HTML to [Djot](https://djot.net/), a lightweight markup language similar to Markdown but with a different syntax for some elements. Set `output_format` to `"djot"` to use this format.

### Syntax Differences

| Element | Markdown | Djot |
|---------|----------|------|
| Strong | `**text**` | `*text*` |
| Emphasis | `*text*` | `_text_` |
| Strikethrough | `~~text~~` | `{-text-}` |
| Inserted/Added | N/A | `{+text+}` |
| Highlighted | N/A | `{=text=}` |
| Subscript | N/A | `~text~` |
| Superscript | N/A | `^text^` |

### Example Usage


```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"

html := "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

// Default Markdown output
markdown, _ := htmltomarkdown.Convert(html)
// Result: "This is **bold** and *italic* text."

// Note: Djot output format configuration is not yet supported in Go bindings
```


Djot's extended syntax allows you to express more semantic meaning in lightweight text, making it useful for documents that require strikethrough, insertion tracking, or mathematical notation.


## Plain Text Output

Set `output_format` to `"plain"` to strip all markup and return only visible text. This bypasses the Markdown conversion pipeline entirely for maximum speed.


```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"

html := "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

plain, _ := htmltomarkdown.Convert(html, htmltomarkdown.WithOutputFormat("plain"))
// Result: "Title\n\nThis is bold and italic text."
```


Plain text mode is useful for search indexing, text extraction, and feeding content to LLMs.






## Examples


## Links

- **GitHub:** [github.com/kreuzberg-dev/html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown)

- **Go Packages:** [pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2](https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2)

- **Kreuzberg Ecosystem:** [kreuzberg.dev](https://kreuzberg.dev)
- **Discord:** [discord.gg/pXxagNK2zN](https://discord.gg/pXxagNK2zN)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/CONTRIBUTING.md) for details on:

- Setting up the development environment
- Running tests locally
- Submitting pull requests
- Reporting issues

All contributions must follow our code quality standards (enforced via pre-commit hooks):

- Proper test coverage (Rust 95%+, language bindings 80%+)
- Formatting and linting checks
- Documentation for public APIs

## License

MIT License – see [LICENSE](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE).

## Support

If you find this library useful, consider [sponsoring the project](https://github.com/sponsors/kreuzberg-dev).

Have questions or run into issues? We're here to help:

- **GitHub Issues:** [github.com/kreuzberg-dev/html-to-markdown/issues](https://github.com/kreuzberg-dev/html-to-markdown/issues)
- **Discussions:** [github.com/kreuzberg-dev/html-to-markdown/discussions](https://github.com/kreuzberg-dev/html-to-markdown/discussions)
- **Discord Community:** [discord.gg/pXxagNK2zN](https://discord.gg/pXxagNK2zN)
