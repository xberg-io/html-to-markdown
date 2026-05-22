# html-to-markdown

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://github.com/kreuzberg-dev/alef">
    <img src="https://img.shields.io/badge/bindings%20by-alef%20%D7%90-007ec6" alt="Bindings by alef">
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
    <img src="https://img.shields.io/badge/docs-kreuzberg.dev-007ec6" alt="Documentation">
  </a>
</div>

<div align="center" style="margin: 24px 0 0;">
  <a href="https://kreuzberg.dev">
    <img width="3384" height="573" alt="html-to-markdown" src="https://github.com/user-attachments/assets/478a83da-237b-446b-b3a8-e564c13e00a8" />
  </a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px;">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="32" src="https://img.shields.io/badge/Discord-Join%20our%20community-007ec6?logo=discord&logoColor=white" alt="Join Discord">
  </a>
  <a href="https://docs.html-to-markdown.kreuzberg.dev/demo/">
    <img height="32" src="https://img.shields.io/badge/Live%20Demo-Open-007ec6?logo=webassembly&logoColor=white" alt="Live Demo">
  </a>
</div>

High-performance HTML to Markdown conversion powered by Rust. Ships as native bindings for **Rust, Python, TypeScript/Node.js, Ruby, PHP, Go, Java, C#, Elixir, R, C (FFI), and WebAssembly** with identical rendering across all runtimes.

**[Documentation](https://docs.html-to-markdown.kreuzberg.dev)** | **[API Reference](https://docs.rs/html-to-markdown-rs/)**

## Highlights

- **Rust-native throughput** with html5ever parsing
- **12 language bindings** with consistent output across all runtimes
- **Structured result** — `convert()` returns `ConversionResult` with `content`, `metadata`, `tables`, `images`, and `warnings`
- **Metadata extraction** — title, headers, links, images, structured data (JSON-LD, Microdata, RDFa)
- **Visitor pattern** — custom callbacks for content filtering, URL rewriting, domain-specific dialects
- **Table extraction** — extract structured table data (cells, headers, rendered markdown) during conversion
- **Secure by default** — built-in HTML sanitization via ammonia

## Quick Start

```bash
# Rust
cargo add html-to-markdown-rs

# Python
pip install html-to-markdown

# TypeScript / Node.js
npm install @kreuzberg/html-to-markdown-node

# Ruby
gem install html-to-markdown

# CLI
cargo install html-to-markdown-cli
# or
brew install kreuzberg-dev/tap/html-to-markdown
```

See the package READMEs for all languages including PHP, Go, Java, C#, Elixir, R, and WASM.

### Usage

`convert()` is the single entry point. It returns a structured `ConversionResult`:

```python
# Python
from html_to_markdown import convert

result = convert("<h1>Hello</h1><p>World</p>")
print(result.content)        # # Hello\n\nWorld
print(result.metadata)       # title, links, headings, …
```

```typescript
// TypeScript / Node.js
import { convert } from "@kreuzberg/html-to-markdown";

const result = convert("<h1>Hello</h1><p>World</p>");
console.log(result.content); // # Hello\n\nWorld
console.log(result.metadata); // title, links, headings, …
```

```rust
// Rust
use html_to_markdown_rs::convert;

let result = convert("<h1>Hello</h1><p>World</p>", None)?;
println!("{}", result.content.unwrap_or_default());
```

## Language Bindings

| Language             | Package                                                                                                      | Install                                                           |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------- |
| Rust                 | [html-to-markdown-rs](https://crates.io/crates/html-to-markdown-rs)                                          | `cargo add html-to-markdown-rs`                                   |
| Python               | [html-to-markdown](https://pypi.org/project/html-to-markdown/)                                               | `pip install html-to-markdown`                                    |
| TypeScript / Node.js | [@kreuzberg/html-to-markdown-node](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node)           | `npm install @kreuzberg/html-to-markdown-node`                    |
| WebAssembly          | [@kreuzberg/html-to-markdown-wasm](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm)           | `npm install @kreuzberg/html-to-markdown-wasm`                    |
| Ruby                 | [html-to-markdown](https://rubygems.org/gems/html-to-markdown)                                               | `gem install html-to-markdown`                                    |
| PHP                  | [kreuzberg-dev/html-to-markdown](https://packagist.org/packages/kreuzberg-dev/html-to-markdown)              | `composer require kreuzberg-dev/html-to-markdown`                 |
| Go                   | [htmltomarkdown](https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown) | `go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v3` |
| Java                 | [dev.kreuzberg:html-to-markdown](https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown)       | Maven / Gradle                                                    |
| C#                   | [KreuzbergDev.HtmlToMarkdown](https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown/)                   | `dotnet add package KreuzbergDev.HtmlToMarkdown`                  |
| Elixir               | [html_to_markdown](https://hex.pm/packages/html_to_markdown)                                                 | `mix deps.get html_to_markdown`                                   |
| R                    | [htmltomarkdown](https://kreuzberg-dev.r-universe.dev/htmltomarkdown)                                        | `install.packages("htmltomarkdown")`                              |
| C (FFI)              | [releases](https://github.com/kreuzberg-dev/html-to-markdown/releases)                                       | Pre-built `.so` / `.dll` / `.dylib`                               |

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — document intelligence: text, tables, metadata from 91+ formats with optional OCR.
- [Kreuzberg Cloud](https://github.com/kreuzberg-dev/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/kreuzberg-dev/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [liter-llm](https://github.com/kreuzberg-dev/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [tree-sitter-language-pack](https://github.com/kreuzberg-dev/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/kreuzberg-dev/alef) — the polyglot binding generator that produces all per-language bindings.
- [Discord](https://discord.gg/xt9WY3GnKR) — community, roadmap, announcements.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and guidelines.

## License

MIT License — see [LICENSE](LICENSE) for details.
