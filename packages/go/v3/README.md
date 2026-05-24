# html-to-markdown

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://github.com/kreuzberg-dev/alef">
    <img src="https://img.shields.io/badge/Bindings-alef%20%D7%90-007ec6" alt="Bindings">
  </a>
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
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/html-to-markdown?label=Go&color=007ec6&filter=v3*" alt="Go">
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
    <img src="https://img.shields.io/badge/R-htmltomarkdown-007ec6" alt="R">
  </a>
  <a href="https://pub.dev/packages/h2m">
    <img src="https://img.shields.io/pub/v/h2m?label=Dart&color=007ec6" alt="Dart">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown-android">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/html-to-markdown-android?label=Kotlin&color=007ec6" alt="Kotlin">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/tree/main/packages/swift">
    <img src="https://img.shields.io/badge/Swift-SPM-007ec6" alt="Swift">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/tree/main/packages/zig">
    <img src="https://img.shields.io/badge/Zig-package-007ec6" alt="Zig">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C FFI">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://docs.html-to-markdown.kreuzberg.dev">
    <img src="https://img.shields.io/badge/Docs-html--to--markdown-007ec6" alt="Documentation">
  </a>
</div>

<div align="center" style="margin: 24px 0 0;">
  <a href="https://kreuzberg.dev">
    <img alt="html-to-markdown" src="https://github.com/user-attachments/assets/478a83da-237b-446b-b3a8-e564c13e00a8" />
  </a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px;">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="22" src="https://img.shields.io/badge/Discord-Chat-007ec6?logo=discord&logoColor=white" alt="Join Discord">
  </a>
  <a href="https://docs.html-to-markdown.kreuzberg.dev/demo/">
    <img height="22" src="https://img.shields.io/badge/Live%20Demo-Open-007ec6?logo=webassembly&logoColor=white" alt="Live Demo">
  </a>
</div>

High-performance HTML to Markdown converter with Go bindings to the Rust core library.
Supports automatic downloading of prebuilt FFI libraries for Linux, macOS, and Windows with customizable caching.

## What This Package Provides

- **Same renderer as every binding** — output matches Rust, Python, Node.js, Ruby, PHP, Go, Java, .NET, Elixir, R, Dart, Swift, Zig, C FFI, and WASM.
- **Structured conversion result** — Markdown plus metadata, links, headings, images, tables, and warnings where the binding exposes them.
- **Production defaults** — HTML is parsed with the Rust core, sanitized by default, and rendered without runtime-specific Markdown drift.
- **Go module** — cgo-backed binding with the Rust renderer behind ordinary Go functions and errors.

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

**Apple M4** · `Convert()` · Real Wikipedia documents

| Document | Size | Latency | Throughput |
|----------|------|---------|------------|
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

    htmltomarkdown "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3"
)

func main() {
    html := "<h1>Hello World</h1><p>This is a paragraph.</p>"

    result, err := htmltomarkdown.Convert(html, nil)
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

    htmltomarkdown "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3"
)

func main() {
    html := "<h1>Hello</h1><p>Welcome</p>"

    width := uint(80)
    opts := htmltomarkdown.ConversionOptions{
        Wrap:      true,
        WrapWidth: &width,
    }

    result, err := htmltomarkdown.Convert(html, &opts)
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

| Element        | Markdown   | Djot       |
| -------------- | ---------- | ---------- |
| Strong         | `**text**` | `*text*`   |
| Emphasis       | `*text*`   | `_text_`   |
| Strikethrough  | `~~text~~` | `{-text-}` |
| Inserted/Added | N/A        | `{+text+}` |
| Highlighted    | N/A        | `{=text=}` |
| Subscript      | N/A        | `~text~`   |
| Superscript    | N/A        | `^text^`   |

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

## Visitor Pattern

The visitor pattern enables custom HTML→Markdown conversion logic by providing callbacks for specific HTML elements during traversal. Pass a visitor as the third argument to `convert()`.

**Use Cases:**

- **Custom Markdown dialects** – Convert to Obsidian, Notion, or other flavors
- **Content filtering** – Remove tracking pixels, ads, or unwanted elements
- **URL rewriting** – Rewrite CDN URLs, add query parameters, validate links
- **Accessibility validation** – Check alt text, heading hierarchy, link text
- **Analytics** – Track element usage, link destinations, image sources

**Supported Visitor Methods:** 40+ callbacks for text, inline elements, links, images, headings, lists, blocks, and tables.

### Example: Quick Start

## Examples

## Links

- **GitHub:** [github.com/kreuzberg-dev/html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown)
- **Go Packages:** [pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2](https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2)
- **Discord:** [discord.gg/xt9WY3GnKR](https://discord.gg/xt9WY3GnKR)

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — document intelligence: text, tables, metadata from 90+ formats with optional OCR.
- [Kreuzberg Cloud](https://github.com/kreuzberg-dev/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/kreuzberg-dev/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [liter-llm](https://github.com/kreuzberg-dev/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [tree-sitter-language-pack](https://github.com/kreuzberg-dev/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/kreuzberg-dev/alef) — the polyglot binding generator that produces every per-language binding across the 5 polyglot repos.
- [Discord](https://discord.gg/xt9WY3GnKR) — community, roadmap, announcements.

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

MIT License – see [LICENSE](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE). Copyright © Kreuzberg, Inc.

## Support

If you find this library useful, consider [sponsoring the project](https://github.com/sponsors/kreuzberg-dev).

Have questions or run into issues? We're here to help:

- **GitHub Issues:** [github.com/kreuzberg-dev/html-to-markdown/issues](https://github.com/kreuzberg-dev/html-to-markdown/issues)
- **Issues:** [github.com/kreuzberg-dev/html-to-markdown/issues](https://github.com/kreuzberg-dev/html-to-markdown/issues)
- **Discord Community:** [discord.gg/xt9WY3GnKR](https://discord.gg/xt9WY3GnKR)
