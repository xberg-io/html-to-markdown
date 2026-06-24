---
description: "html-to-markdown — Convert HTML to Markdown, Djot, or plain text. Rust core plus 15 generated packages, identical output on every runtime."
---

# html-to-markdown

Convert HTML to Markdown, Djot, or plain text. One Rust core plus 15 generated packages, identical output on every runtime. Part of the [xberg.io](https://xberg.io) document intelligence ecosystem.

<div class="hero-badges" markdown>

[:material-rocket-launch: Get Started](installation.md){ .md-button .md-button--primary }
[:material-package-variant: Installation](installation.md){ .md-button }
[:material-github: GitHub](https://github.com/xberg-io/html-to-markdown){ .md-button }
[:fontawesome-brands-discord: Discord](https://discord.gg/xt9WY3GnKR){ .md-button }

</div>

---

## Why html-to-markdown

<div class="grid cards" markdown>

- :material-flash:{ .lg .middle } **Rust core**

  Single-pass DOM walk written in Rust. The same code path runs from Python, the browser, and the CLI — no per-language conversion logic.

- :material-translate:{ .lg .middle } **16 language surfaces**

  Rust plus generated packages for Python, TypeScript, Go, Ruby, PHP, Java, C#, Elixir, R, C, WebAssembly, Swift, Dart, Kotlin Android, and Zig.

- :material-file-document-outline:{ .lg .middle } **Three output formats** <span class="version-badge">Available by v3.6</span>

  Markdown (CommonMark) by default, plus Djot and plain text via `output_format`. The same options apply to every format.

- :material-tag-multiple:{ .lg .middle } **Metadata extraction** <span class="version-badge">Available by v3.6</span>

  Document title, Open Graph, Twitter Card, JSON-LD, links, and images in one pass. Enabled by default — disable with `extract_metadata: false`.

- :material-table:{ .lg .middle } **Table extraction** <span class="version-badge">Available by v3.6</span>

  HTML tables into `result.tables` with structured cells, row/column spans, and header flags, alongside the rendered Markdown.

- :material-puzzle:{ .lg .middle } **Visitor pattern** <span class="version-badge">Available by v3.6</span>

  40 callbacks on the `HtmlVisitor` trait to skip, replace, or preserve nodes. Kotlin Android excludes visitor support. Zero cost when unused.

</div>

---

## Language Support

| Language              | Install                                                           | API Reference                            |
| :-------------------- | :---------------------------------------------------------------- | :--------------------------------------- |
| **Rust**              | `cargo add html-to-markdown-rs`                                   | [Reference](reference/api-rust.md)       |
| **Python**            | `pip install html-to-markdown`                                    | [Reference](reference/api-python.md)     |
| **TypeScript / Node** | `npm install @kreuzberg/html-to-markdown`                         | [Reference](reference/api-typescript.md) |
| **Go**                | `go get github.com/xberg-io/html-to-markdown/packages/go/v3` | [Reference](reference/api-go.md)         |
| **Ruby**              | `gem install html-to-markdown`                                    | [Reference](reference/api-ruby.md)       |
| **PHP**               | `pie install xberg-io/html-to-markdown`                      | [Reference](reference/api-php.md)        |
| **Java**              | Maven `dev.kreuzberg:html-to-markdown`                            | [Reference](reference/api-java.md)       |
| **C#**                | `dotnet add package KreuzbergDev.HtmlToMarkdown`                  | [Reference](reference/api-csharp.md)     |
| **Elixir**            | `{:html_to_markdown, "~> 3.4"}`                                   | [Reference](reference/api-elixir.md)     |
| **R**                 | `install.packages("htmltomarkdown")`                              | [Reference](reference/api-r.md)          |
| **C (FFI)**           | Shared library + header                                           | [Reference](reference/api-c.md)          |
| **WebAssembly**       | `npm install @kreuzberg/html-to-markdown-wasm`                    | [Reference](reference/api-wasm.md)       |
| **Swift**             | Swift Package `HtmlToMarkdown`                                    | [Reference](reference/api-swift.md)      |
| **Dart**              | `dart pub add h2m`                                                | [Reference](reference/api-dart.md)       |
| **Kotlin Android**    | Maven `dev.kreuzberg:html-to-markdown-android`                    | [Reference](reference/api-kotlin-android.md) |
| **Zig**               | Zig package `html_to_markdown_rs`                                 | [Reference](reference/api-zig.md)        |
| **CLI**               | `cargo install html-to-markdown-cli`                              | [CLI Guide](cli.md)                      |

---

## Quick Example

=== "Python"

    ```python title="main.py"
    from html_to_markdown import convert

    result = convert("<h1>Hello</h1><p>World</p>")
    print(result.content)
    print(result.metadata)
    ```

=== "TypeScript"

    ```typescript title="index.ts"
    import { convert } from "@kreuzberg/html-to-markdown";

    const result = convert("<h1>Hello</h1><p>World</p>");
    console.log(result.content);
    console.log(result.metadata);
    ```

=== "Rust"

    ```rust title="src/main.rs"
    use html_to_markdown_rs::convert;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let result = convert("<h1>Hello</h1><p>World</p>", None)?;
        println!("{}", result.content.unwrap_or_default());
        Ok(())
    }
    ```

---

## Part of Kreuzberg.dev

html-to-markdown ships as a standalone library and as the HTML pipeline inside the [Kreuzberg](https://docs.xberg.io) document intelligence stack.

<div class="grid cards" markdown>

- :material-file-document-multiple:{ .lg .middle } **[Kreuzberg](https://docs.xberg.io)**

  Document intelligence core — text, tables, and metadata from 91+ file formats. Uses html-to-markdown for every HTML input.

- :material-cloud:{ .lg .middle } **[Kreuzberg Cloud](https://docs.kreuzberg.cloud)**

  Managed SaaS API for document extraction. Same engine, no infrastructure.

- :material-spider-web:{ .lg .middle } **[kreuzcrawl](https://docs.kreuzcrawl.xberg.io)**

  Web crawler that pairs with html-to-markdown for crawl-then-convert pipelines.

- :material-robot:{ .lg .middle } **[liter-llm](https://docs.liter-llm.xberg.io)**

  Universal LLM client — feed it the Markdown out of html-to-markdown.

- :material-code-tags:{ .lg .middle } **[tree-sitter-language-pack](https://docs.tree-sitter-language-pack.xberg.io)**

  306 Tree-sitter grammars on demand for code intelligence.

- :fontawesome-brands-discord:{ .lg .middle } **[Discord](https://discord.gg/xt9WY3GnKR)**

  Community chat for xberg.io users and contributors.

</div>

---

## Explore the Docs

<div class="grid cards" markdown>

- :material-rocket-launch:{ .lg .middle } **Get Started**

  Install a binding and run your first `convert()` call.

  [:octicons-arrow-right-24: Installation](installation.md)

- :material-book-open-variant:{ .lg .middle } **Guides**

  Visitor pattern, table extraction, error handling.

  [:octicons-arrow-right-24: Visitor pattern](visitor.md)

- :material-puzzle-outline:{ .lg .middle } **Concepts**

  Architecture, conversion pipeline, plugin system.

  [:octicons-arrow-right-24: Architecture](concepts/architecture.md)

- :material-api:{ .lg .middle } **Reference**

  Options reference, generated API docs, per-language guides.

  [:octicons-arrow-right-24: Configuration](configuration.md)

- :material-console:{ .lg .middle } **CLI**

  Every conversion option as a command-line flag.

  [:octicons-arrow-right-24: CLI Guide](cli.md)

- :material-swap-horizontal:{ .lg .middle } **Migration**

  Upgrading from earlier versions.

  [:octicons-arrow-right-24: Migration](migration.md)

</div>

---

## Getting Help

- **Bugs and feature requests** — [Open an issue on GitHub](https://github.com/xberg-io/html-to-markdown/issues)
- **Community chat** — [Join the Discord](https://discord.gg/xt9WY3GnKR)
- **Contributing** — [Read the contributor guide](contributing.md)
