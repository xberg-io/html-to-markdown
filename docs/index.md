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

### Part of the Kreuzberg Ecosystem

html-to-markdown powers the HTML conversion pipeline in [kreuzberg](https://docs.kreuzberg.dev), a document intelligence library for extracting text and structured data from PDFs, DOCX, images, and other document formats.
