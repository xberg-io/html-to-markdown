---
title: html-to-markdown
description: High-performance HTML to Markdown conversion powered by Rust
---

## html-to-markdown

High-performance HTML to Markdown conversion powered by Rust. A single Rust core with native bindings for 12 language ecosystems, delivering identical output across every runtime.

### Quick Example

=== "Rust"
    ```rust
    use html_to_markdown_rs::convert;

    let result = convert("<h1>Hello</h1><p>This is <strong>fast</strong>!</p>", None)?;
    println!("{}", result.content.unwrap_or_default());
    // # Hello
    //
    // This is **fast**!
    ```

=== "Python"
    ```python
    from html_to_markdown import convert

    result = convert("<h1>Hello</h1><p>This is <strong>fast</strong>!</p>")
    print(result["content"])
    # # Hello
    #
    # This is **fast**!
    ```

### Features

- 150–280 MB/s throughput, 10–80x faster than pure-Python alternatives
- 12 native language bindings: Rust, Python, TypeScript, Go, Ruby, PHP, Java, C#, Elixir, R, C, WASM
- `convert()` returns a `ConversionResult` with `content`, `metadata`, `tables`, `images`, and `warnings`
- Metadata extraction: title, description, Open Graph, Twitter Card, JSON-LD, links, images
- Structured document tree extraction (`DocumentStructure`)
- Visitor pattern for content filtering, URL rewriting, and custom dialects
- Output formats: Markdown (CommonMark), Djot, plain text
- Built-in HTML sanitization via [ammonia](https://github.com/rust-ammonia/ammonia)
- CLI tool with full flag coverage

### Get Started

- [Installation](installation.md) — package manager commands for all 12 languages
- [Usage](usage.md) — convert HTML, access result fields, extract metadata
- [Configuration](configuration.md) — all options with types, defaults, and descriptions
- [CLI](cli.md) — command-line reference

<<<<<<< HEAD
    Rust-native parsing and a single-pass DOM walk. 10–80x faster than pure-language alternatives. No JVM, no interpreter overhead.

- :material-translate:{ .lg .middle } **12 language bindings**

    Rust, Python, TypeScript, Go, Ruby, PHP, Java, C#, Elixir, R, C, and WebAssembly. All wrap the same core — no separate conversion logic per language.

- :material-format-text:{ .lg .middle } **Three output formats**

    Markdown (CommonMark), Djot, and plain text. Switch with `output_format`. All formatting options apply to every format.

- :material-tag-multiple:{ .lg .middle } **Metadata extraction**

    Title, description, Open Graph, Twitter Card, JSON-LD, links, and images — all in one pass. Enable with `extract_metadata: true`.

- :material-table:{ .lg .middle } **Table extraction**

    Every `<table>` lands in `result.tables` as a structured cell grid with colspan, rowspan, and header data, plus a rendered Markdown string.

- :material-filter:{ .lg .middle } **Visitor pattern**

    40 callbacks to rewrite links, filter elements, or emit custom Markdown for any HTML tag. Zero overhead when not used.

</div>

---

### Explore the Docs

<div class="grid cards" markdown>

- :material-download:{ .lg .middle } **Installation**

    Package manager commands for all 12 languages, plus Cargo feature flags for Rust.

    [:octicons-arrow-right-24: Installation](installation.md)

- :material-play:{ .lg .middle } **Usage**

    Convert HTML, read result fields, extract metadata, and work with document structure.

    [:octicons-arrow-right-24: Usage](usage.md)

- :material-cog:{ .lg .middle } **Configuration**

    All 34 options with types, defaults, and descriptions. Output formats, preprocessing, link and image handling.

    [:octicons-arrow-right-24: Configuration](configuration.md)

- :material-table:{ .lg .middle } **Table Extraction**

    Access cell-level grid data with colspan, rowspan, and header info alongside rendered Markdown.

    [:octicons-arrow-right-24: Tables](tables.md)

- :material-filter:{ .lg .middle } **Visitor Pattern**

    Filter, rewrite, or replace any element during conversion with 40 typed callbacks.

    [:octicons-arrow-right-24: Visitor](visitor.md)

- :material-api:{ .lg .middle } **API Reference**

    Every public type: `ConversionResult`, `DocumentNode`, `HtmlMetadata`, `TableData`, `InlineImage`, and more.

    [:octicons-arrow-right-24: API Reference](api-reference.md)

</div>

---

### Getting Help

- **Bugs and feature requests** — [Open an issue on GitHub](https://github.com/kreuzberg-dev/html-to-markdown/issues)
- **Contributing** — [Read the contributor guide](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/CONTRIBUTING.md)

<div class="home-kreuzberg" markdown="1">

### Part of Kreuzberg

html-to-markdown powers the HTML conversion pipeline in [Kreuzberg](https://docs.kreuzberg.dev), a document intelligence library for extracting text and structured data from PDFs, DOCX, images, and other document formats.

</div>
=======
### Part of the Kreuzberg Ecosystem

html-to-markdown powers the HTML conversion pipeline in [kreuzberg](https://docs.kreuzberg.dev), a document intelligence library for extracting text and structured data from PDFs, DOCX, images, and other document formats.
>>>>>>> 76547739 (feat!: alef adoption — full polyglot codegen migration (v3.2.0))
