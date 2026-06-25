# html-to-markdown

{% include 'partials/_badges.md' %}

Fast, robust HTML → Markdown for 16 languages. A tiered converter that picks the safest, fastest path per input without losing content.

## What and Why?

html-to-markdown converts real-world HTML — unclosed tags, CDATA, custom elements, malformed entities, nested tables, mixed encodings — into clean CommonMark (or Djot) without losing content, from one Rust core with native bindings for 16 languages.

It routes each input through three tiers: a single-pass byte scanner for clean HTML, a tolerant DOM walker for complex inputs, and an `html5ever` repair pass for malformed HTML — with byte-identical output across tiers, enforced by a 116-snapshot oracle and per-group performance gates in CI. The dispatcher is invisible: the same `convert()` call works regardless of which tier runs.

### Features

| Feature | Description |
| ------- | ----------- |
| **16 languages, one Rust core** | Rust, Python, Node.js, WASM, Java, Go, C#, PHP, Ruby, Elixir, R, Dart, Kotlin (Android), Swift, Zig, and a C ABI |
| **Tiered dispatch** | Byte scanner → DOM walker → `html5ever` repair, with byte-equal output across tiers |
| **Real-HTML robust** | Unclosed tags, CDATA, custom elements, malformed entities, nested tables, mixed encodings — handled without losing content |
| **GFM tables** | Padded cells, alignment, and pipe escaping |
| **Djot output** | Set `output_format = "djot"` to emit Djot instead of Markdown |
| **Metadata extraction** | Parse `<head>` into structured metadata (Open Graph, Twitter, JSON-LD, microdata, RDFa, header hierarchy) |
| **Inline images** | Opt-in mirroring of data URIs and remote image references |
| **Visitor API** | Feature-gated traversal to transform the converted Markdown AST |
| **Configurable preprocessing** | Standard, strict, and lenient presets — or build your own |
| **Fast** | 19–116 MB/s on the Wikipedia/mdream corpus; per-group regression thresholds enforced on every PR |

<div align="center">
  <a href="https://github.com/xberg-io/html-to-markdown/stargazers">
    <img src="docs/assets/star.gif" alt="Star html-to-markdown on GitHub" width="640">
  </a>
</div>

<p align="center"><strong>⭐ Star this repo to show your support — it helps others discover html-to-markdown.</strong></p>

## Quick Start

`convert()` is the single entry point — it returns a structured result with `content`, `warnings`, and optional `metadata`.

### Language Packages

<details open>
<summary><strong>Rust</strong></summary>

```sh
cargo add html-to-markdown-rs
```

See [Rust README](https://github.com/xberg-io/html-to-markdown/tree/main/crates/html-to-markdown) for full documentation.

</details>

<details>
<summary><strong>Python</strong></summary>

```sh
pip install html-to-markdown
```

See [Python README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/python) for full documentation.

</details>

<details>
<summary><strong>Node.js</strong></summary>

```sh
npm install @kreuzberg/html-to-markdown
```

See [Node.js README](https://github.com/xberg-io/html-to-markdown/tree/main/crates/html-to-markdown-node) for full documentation.

</details>

<details>
<summary><strong>Go</strong></summary>

```sh
go get github.com/xberg-io/html-to-markdown/packages/go/v3
```

See [Go README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/go) for full documentation.

</details>

<details>
<summary><strong>Java</strong></summary>

Available on Maven Central as `dev.kreuzberg:html-to-markdown`. See [Java README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/java) for the dependency snippet and current version.

</details>

<details>
<summary><strong>C#</strong></summary>

```sh
dotnet add package KreuzbergDev.HtmlToMarkdown
```

See [C# README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/csharp) for full documentation.

</details>

<details>
<summary><strong>Ruby</strong></summary>

```sh
gem install html-to-markdown
```

See [Ruby README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/ruby) for full documentation.

</details>

<details>
<summary><strong>PHP</strong></summary>

This is a native PHP extension (Rust `ext-php-rs`), so install it with [PIE](https://github.com/php/pie) — not `composer require`:

```sh
pie install xberg-io/html-to-markdown
```

See [PHP README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/php) for full documentation.

</details>

<details>
<summary><strong>Elixir</strong></summary>

Add `{:html_to_markdown, "~> 3.6"}` to your `mix.exs` dependencies. See [Elixir README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/elixir) for full documentation.

</details>

<details>
<summary><strong>R</strong></summary>

```r
install.packages("htmltomarkdown", repos = "https://xberg-io.r-universe.dev")
```

See [R README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/r) for full documentation.

</details>

<details>
<summary><strong>Dart / Flutter</strong></summary>

```sh
dart pub add h2m
```

See [Dart README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/dart) for full documentation.

</details>

<details>
<summary><strong>Kotlin (Android)</strong></summary>

Available on Maven Central as `dev.kreuzberg:html-to-markdown-android`. See [Kotlin README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/kotlin-android) for the dependency snippet and current version.

</details>

<details>
<summary><strong>Swift</strong></summary>

Add via Swift Package Manager. See [Swift README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/swift) for full documentation.

</details>

<details>
<summary><strong>Zig</strong></summary>

See [Zig README](https://github.com/xberg-io/html-to-markdown/tree/main/packages/zig) for installation and usage.

</details>

<details>
<summary><strong>WebAssembly</strong></summary>

```sh
npm install @kreuzberg/html-to-markdown-wasm
```

See [WebAssembly README](https://github.com/xberg-io/html-to-markdown/tree/main/crates/html-to-markdown-wasm) for full documentation.

</details>

<details>
<summary><strong>C/C++ (FFI)</strong></summary>

Pre-built `.so` / `.dll` / `.dylib` from [GitHub Releases](https://github.com/xberg-io/html-to-markdown/releases). See [FFI crate](https://github.com/xberg-io/html-to-markdown/tree/main/crates/html-to-markdown-ffi) for full documentation.

</details>

<details>
<summary><strong>CLI</strong></summary>

```sh
cargo install html-to-markdown-cli
```

```sh
brew install xberg-io/tap/html-to-markdown
```

See [CLI usage](https://docs.html-to-markdown.xberg.io) for full documentation.

</details>

### AI Coding Assistants

Install the html-to-markdown plugin from the [`xberg-io/plugins`](https://github.com/xberg-io/plugins) marketplace. It ships the html-to-markdown agent skills and works with every major coding agent — expand your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add xberg-io/plugins
/plugin install html-to-markdown@kreuzberg
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/xberg-io/plugins
```

Then search for `html-to-markdown` and select **Install Plugin**.

</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/xberg-io/plugins`, then select **html-to-markdown**.

</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/xberg-io/plugins
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/xberg-io/plugins
droid plugin install html-to-markdown@kreuzberg
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/xberg-io/plugins
copilot plugin install html-to-markdown@kreuzberg
```

</details>

<details>
<summary><strong>opencode</strong></summary>

Add the package to `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@kreuzberg/opencode-html-to-markdown"]
}
```

</details>

## Documentation

Full guides, the `convert()` API for every binding, tier architecture, the metadata and visitor APIs, and performance benchmarks live at **[docs.html-to-markdown.xberg.io](https://docs.html-to-markdown.xberg.io)**.

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/xberg-io/kreuzberg) — document intelligence: text, tables, metadata from 91+ formats with optional OCR.
- [Xberg Enterprise](https://github.com/xberg-io/xberg-enterprise) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/xberg-io/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/xberg-io/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/xberg-io/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [tree-sitter-language-pack](https://github.com/xberg-io/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/xberg-io/alef) — the polyglot binding generator that produces every per-language binding across the 5 polyglot repos.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and guidelines.

## License

MIT License — see [LICENSE](LICENSE) for details.
