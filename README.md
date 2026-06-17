# html-to-markdown

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://github.com/kreuzberg-dev/alef">
    <img src="https://img.shields.io/badge/built%20with-alef%20%D7%90-007ec6" alt="Built with alef">
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
  <a href="https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v3">
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

Fast, robust HTML → Markdown for 16 languages. A tiered converter that picks the safest fastest path per input without losing content.

**[Documentation](https://docs.html-to-markdown.kreuzberg.dev)** | **[API Reference](https://docs.rs/html-to-markdown-rs/)**

## Highlights

- **16 languages, one Rust core.** Rust, Python, Node.js, WASM, Java, Go, C#, PHP, Ruby, Elixir, R, Dart, Kotlin (Android), Swift, Zig, C ABI.
- **Tiered dispatch.** Byte scanner for clean HTML → DOM walker for complex inputs → `html5ever` repair for malformed HTML. Byte-equal output across tiers.
- **Real-HTML robust.** Unclosed tags, CDATA, custom elements, malformed entities, nested tables, mixed encodings — handled without losing content.
- **GFM tables, Djot output, metadata extraction, visitor API, inline images, configurable preprocessing presets.**
- **CommonMark-compatible Markdown** with GFM-style tables. `output_format = "djot"` switches to Djot.
- **116-snapshot oracle + per-group regression gates in CI.** Performance and correctness both enforced.

## Architecture

The converter routes each input through one of three tiers based on a fast prescan of the byte stream:

1. **Tier-1 — Single-pass byte scanner.** Walks `html.as_bytes()` once and emits Markdown directly. Handles 110+ HTML tags including paragraphs, headings, lists, GFM tables, links, images, inline emphasis, blockquotes, indented code blocks. Bails (returns a structured error) on any construct it cannot prove byte-equivalent to Tier-2 — custom elements, CDATA, malformed entities, nested tables, mixed table sections, multi-line table cells, etc.

2. **Tier-2 — `tl::parse` DOM walker.** Picks up Tier-1's bails and inputs the classifier rejected up front (non-default style options, non-Markdown output, etc.). Handles the full HTML5 spec via a tolerant DOM walk.

3. **Tier-3 — `html5ever` standards-conformant parser.** Engaged when Tier-2 detects HTML requiring full HTML5 repair (custom elements, structural recovery, weird namespace transitions).

The dispatcher is invisible to the caller. The same `convert()` call works regardless of which tier handled the input; the output is byte-identical across tiers (enforced by a 116-snapshot oracle).

## Performance

Best-of-3 measurements on the harness corpus (Apple Silicon, `cargo build --release`):

| Fixture | Size | ms (best of 3) | Throughput |
|---|---:|---:|---:|
| `wikipedia/medium_python.html` | 1.24 MB | 62.58 ms | 19.0 MB/s |
| `wikipedia/large_rust.html` | 1.07 MB | 37.17 ms | 27.3 MB/s |
| `wikipedia/small_html.html` | 973 KB | 29.32 ms | 31.6 MB/s |
| `wikipedia/tables_countries.html` | 756 KB | 18.95 ms | 38.1 MB/s |
| `mdream/github-markdown-complete.html` | 430 KB | 10.57 ms | 38.7 MB/s |
| `mdream/react-learn.html` | 265 KB | 12.11 ms | 20.9 MB/s |
| `mdream/wikipedia-small.html` | 166 KB | 5.63 ms | 28.1 MB/s |
| `issues/gh-121-hacker-news.html` | 57 KB | 1.08 ms | 50.3 MB/s |
| `mdream/nuxt-example.html` | 3.6 KB | 0.029 ms | 116.1 MB/s |

Corpus: 29 fixtures totalling 6.4 MB across `clean_small`, `clean_medium`, `clean_large`, `spec_rules`, `adversarial`, and `fallthrough_*` groups. Per-group regression thresholds (5–30%) are enforced on every PR via `task bench:compare`. Run `task bench:run` to reproduce on your hardware.

## Capabilities

- **HTML element coverage**: 110+ tags handled natively in Tier-1; full HTML5 coverage via Tier-2/Tier-3 fallback.
- **GFM-style tables** with padded cells, alignment, and pipe escaping.
- **Djot output**: set `ConversionOptions { output_format: OutputFormat::Djot, .. }` to emit Djot instead of Markdown.
- **Metadata extraction**: parse `<head>` into structured `HtmlMetadata` (open-graph, twitter, JSON-LD, microdata, RDFa, header hierarchy).
- **Inline images**: opt-in via `inline-images` feature; mirrors data URIs and remote image references.
- **Visitor API**: feature-gated traversal that lets callers transform the converted Markdown AST (`visitor` feature).
- **Configurable preprocessing**: standard, strict, lenient presets — or build your own.
- **Tiered fallback**: Tier-3 (`html5ever`) handles inputs the other tiers cannot, so the converter never silently corrupts malformed HTML.

## Quick Start

```bash
# Rust
cargo add html-to-markdown-rs

# Python
pip install html-to-markdown

# TypeScript / Node.js
npm install @kreuzberg/html-to-markdown

# Ruby
gem install html-to-markdown

# CLI
cargo install html-to-markdown-cli
# or (Homebrew 6.0+ requires explicit trust for third-party taps)
brew trust kreuzberg-dev/tap
brew install kreuzberg-dev/tap/html-to-markdown
```

See the package READMEs for the full list: PHP, Go, Java, C#, Elixir, R, Dart, Kotlin (Android), Swift, Zig, WASM, and a C ABI for everything else.

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
| TypeScript / Node.js | [@kreuzberg/html-to-markdown](https://www.npmjs.com/package/@kreuzberg/html-to-markdown)                     | `npm install @kreuzberg/html-to-markdown`                         |
| WebAssembly          | [@kreuzberg/html-to-markdown-wasm](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm)           | `npm install @kreuzberg/html-to-markdown-wasm`                    |
| Ruby                 | [html-to-markdown](https://rubygems.org/gems/html-to-markdown)                                               | `gem install html-to-markdown`                                    |
| PHP                  | [kreuzberg-dev/html-to-markdown](https://packagist.org/packages/kreuzberg-dev/html-to-markdown)              | `composer require kreuzberg-dev/html-to-markdown`                 |
| Go                   | [htmltomarkdown](https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v3)                | `go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v3` |
| Java                 | [dev.kreuzberg:html-to-markdown](https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown)       | Maven / Gradle                                                    |
| C#                   | [KreuzbergDev.HtmlToMarkdown](https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown/)                   | `dotnet add package KreuzbergDev.HtmlToMarkdown`                  |
| Elixir               | [html_to_markdown](https://hex.pm/packages/html_to_markdown)                                                 | add `{:html_to_markdown, "~> 3.6"}` to `mix.exs`                  |
| R                    | [htmltomarkdown](https://kreuzberg-dev.r-universe.dev/htmltomarkdown)                                        | `install.packages("htmltomarkdown", repos = "https://kreuzberg-dev.r-universe.dev")` |
| C (FFI)              | [releases](https://github.com/kreuzberg-dev/html-to-markdown/releases)                                       | Pre-built `.so` / `.dll` / `.dylib`                               |

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — document intelligence: text, tables, metadata from 91+ formats with optional OCR.
- [Kreuzberg Cloud](https://github.com/kreuzberg-dev/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/kreuzberg-dev/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/kreuzberg-dev/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [tree-sitter-language-pack](https://github.com/kreuzberg-dev/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/kreuzberg-dev/alef) — the polyglot binding generator that produces every per-language binding across the 5 polyglot repos.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and guidelines.

## License

MIT License — see [LICENSE](LICENSE) for details.
