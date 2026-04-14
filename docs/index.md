---
title: Home
description: "html-to-markdown — Convert HTML to Markdown, Djot, or plain text. Rust core, 12 native language bindings, 150–280 MB/s."
---

<div class="home-hero" markdown="1">

## html-to-markdown

<p class="home-lead" markdown="1">
Convert HTML to Markdown, Djot, or plain text. One Rust core, 12 native language bindings, identical output on every runtime.
</p>

<div class="home-instruction" markdown="1">

### Start here

1. **[Install a binding](installation.md)** — add the package to your project (versions, feature flags, and verify steps are on that page).
2. **[Run a minimal `convert()`](usage.md#basic-conversion)** — open *Usage → Basic conversion*, choose your language tab, and copy the hello-world snippet.

</div>

</div>

---

### What It Does

<div class="grid cards" markdown>

- :material-lightning-bolt:{ .lg .middle } **150–280 MB/s throughput**

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
