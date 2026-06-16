# html-to-markdown-rs

[![Built with alef](https://img.shields.io/badge/built%20with-alef%20%D7%90-007ec6)](https://github.com/kreuzberg-dev/alef)

High-performance HTML to Markdown converter built with Rust.

This crate is the core engine compiled into the Python wheels, Ruby gem, Node.js NAPI bindings, WebAssembly package, and CLI, ensuring identical Markdown output across every language.

[![Crates.io](https://img.shields.io/crates/v/html-to-markdown-rs.svg?color=007ec6)](https://crates.io/crates/html-to-markdown-rs)
[![npm version](https://img.shields.io/npm/v/@kreuzberg/html-to-markdown.svg?logo=npm&color=007ec6)](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node)
[![PyPI version](https://img.shields.io/pypi/v/html-to-markdown.svg?logo=pypi&color=007ec6)](https://pypi.org/project/html-to-markdown/)
[![Gem Version](https://img.shields.io/gem/v/html-to-markdown.svg?color=007ec6)](https://rubygems.org/gems/html-to-markdown)
[![Packagist](https://img.shields.io/packagist/v/kreuzberg-dev/html-to-markdown.svg?color=007ec6)](https://packagist.org/packages/kreuzberg-dev/html-to-markdown)
[![docs.rs](https://img.shields.io/badge/docs.rs-html--to--markdown--rs-007ec6)](https://docs.rs/html-to-markdown-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-007ec6)](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE)

Fast, reliable HTML to Markdown conversion with full CommonMark compliance. Built with `html5ever` for correctness and a DOM-based filter for safe preprocessing.

## Installation

```toml
[dependencies]
html-to-markdown-rs = "3.0"
```

## Basic Usage

`convert()` returns a structured `ConversionResult` with the converted text, metadata, tables, and more:

```rust
use html_to_markdown_rs::convert;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"
        <html lang="en">
          <head><title>Welcome</title></head>
          <body>
            <h1>Welcome</h1>
            <p>This is <strong>fast</strong> conversion!</p>
            <ul>
                <li>Built with Rust</li>
                <li>CommonMark compliant</li>
            </ul>
          </body>
        </html>
    "#;

    let result = convert(html, None)?;
    println!("{}", result.content.unwrap_or_default());

    if let Some(metadata) = &result.metadata {
        println!("Title: {:?}", metadata.document.title);
        println!("Headers: {:?}", metadata.headers);
    }

    for table in &result.tables {
        println!("Table with {} rows", table.cells.len());
    }

    Ok(())
}
```

## Error Handling

Conversion returns a `Result<ConversionResult, ConversionError>`. Inputs that look like binary data are rejected with
`ConversionError::InvalidInput` to prevent runaway allocations. Table `colspan`/`rowspan` values are also clamped
internally to keep output sizes bounded.

## Configuration

### Builder Pattern

```rust
use html_to_markdown_rs::{
    convert, ConversionOptions, HeadingStyle, CodeBlockStyle,
};

let options = ConversionOptions::builder()
    .heading_style(HeadingStyle::Atx)
    .list_indent_width(2)
    .bullets("-")
    .autolinks(true)
    .wrap(true)
    .wrap_width(80)
    .build();

let result = convert(html, Some(options))?;
println!("{}", result.content.unwrap_or_default());
```

### Struct Literal

```rust
use html_to_markdown_rs::{
    convert, ConversionOptions, HeadingStyle, ListIndentType,
};

let options = ConversionOptions {
    heading_style: HeadingStyle::Atx,
    list_indent_width: 2,
    list_indent_type: ListIndentType::Spaces,
    bullets: "-".to_string(),
    strong_em_symbol: '*',
    escape_asterisks: false,
    escape_underscores: false,
    newline_style: html_to_markdown_rs::NewlineStyle::Backslash,
    code_block_style: html_to_markdown_rs::CodeBlockStyle::Backticks,
    ..Default::default()
};

let result = convert(html, Some(options))?;
println!("{}", result.content.unwrap_or_default());
```

### Preserving HTML Tags

The `preserve_tags` option allows you to keep specific HTML tags in their original form instead of converting them to Markdown:

```rust
use html_to_markdown_rs::{convert, ConversionOptions};

let html = r#"
<p>Before table</p>
<table class="data">
    <tr><th>Name</th><th>Value</th></tr>
    <tr><td>Item 1</td><td>100</td></tr>
</table>
<p>After table</p>
"#;

let options = ConversionOptions {
    preserve_tags: vec!["table".to_string()],
    ..Default::default()
};

let result = convert(html, Some(options))?;
// result.content => "Before table\n\n<table class=\"data\">...</table>\n\nAfter table\n"
```

## Web Scraping with Preprocessing

```rust
use html_to_markdown_rs::{convert, ConversionOptions, PreprocessingOptions};

let mut options = ConversionOptions::default();
options.preprocessing.enabled = true;
options.preprocessing.preset = html_to_markdown_rs::PreprocessingPreset::Aggressive;
options.preprocessing.remove_navigation = true;
options.preprocessing.remove_forms = true;

let result = convert(scraped_html, Some(options))?;
println!("{}", result.content.unwrap_or_default());
```

## Metadata Extraction

Metadata is automatically included in the result. Configure which fields to extract via `MetadataConfig`:

```rust
use html_to_markdown_rs::{convert, ConversionOptions, MetadataConfig};

let options = ConversionOptions::builder()
    .metadata_config(MetadataConfig {
        extract_headers: true,
        extract_links: true,
        extract_images: false,
        ..Default::default()
    })
    .build();

let result = convert(html, Some(options))?;
if let Some(metadata) = &result.metadata {
    println!("Title: {:?}", metadata.document.title);
    for header in &metadata.headers {
        println!("H{}: {}", header.level, header.text);
    }
    for link in &metadata.links {
        println!("Link: {} -> {}", link.text, link.href);
    }
}
```

## Image Extraction

```rust
use html_to_markdown_rs::{convert, ConversionOptions};

let options = ConversionOptions::builder()
    .extract_images(true)
    .max_image_size(5 * 1024 * 1024) // 5 MB max
    .infer_dimensions(true)
    .build();

let result = convert(html, Some(options))?;
println!("{}", result.content.unwrap_or_default());
for img in &result.images {
    println!("Image: {} ({} bytes)", img.src, img.data.as_ref().map_or(0, |d| d.len()));
}
```

## Table Extraction

Structured table data is always included in `ConversionResult.tables`:

```rust
use html_to_markdown_rs::convert;

let html = r#"
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"#;

let result = convert(html, None)?;

println!("{}", result.content.unwrap_or_default());
for table in &result.tables {
    println!("Table with {} rows:", table.cells.len());
    for (i, row) in table.cells.iter().enumerate() {
        let prefix = if table.is_header_row[i] { "Header" } else { "Row" };
        println!("  {}: {:?}", prefix, row);
    }
}
```

## Custom Visitors

```rust
use html_to_markdown_rs::{convert, ConversionOptions};
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

struct NoImagesVisitor;

impl HtmlVisitor for NoImagesVisitor {
    fn visit_image(
        &mut self,
        _ctx: &NodeContext,
        _src: &str,
        _alt: &str,
        _title: Option<&str>,
    ) -> VisitResult {
        VisitResult::Skip
    }
}

let options = ConversionOptions::builder()
    .visitor(Box::new(NoImagesVisitor))
    .build();

let result = convert(html, Some(options))?;
println!("{}", result.content.unwrap_or_default());
```

## Other Language Bindings

This is the core Rust library. For other languages:

- **JavaScript/TypeScript**: [html-to-markdown-node](https://github.com/kreuzberg-dev/html-to-markdown/tree/main/crates/html-to-markdown-node) (NAPI-RS) or [html-to-markdown-wasm](https://github.com/kreuzberg-dev/html-to-markdown/tree/main/crates/html-to-markdown-wasm) (WebAssembly)
- **Python**: [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown/tree/main/crates/html-to-markdown-py) (PyO3)
- **PHP**: [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown/tree/main/packages/php) (PIE + Composer helpers)
- **Ruby**: [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown/tree/main/packages/ruby) (Magnus + rb-sys)
- **CLI**: [html-to-markdown-cli](https://github.com/kreuzberg-dev/html-to-markdown/tree/main/crates/html-to-markdown-cli)

## Documentation

- [Full Documentation](https://docs.html-to-markdown.kreuzberg.dev)
- [API Reference](https://docs.rs/html-to-markdown-rs)
- [Documentation](https://docs.html-to-markdown.kreuzberg.dev/)
- [Contributing Guide](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/CONTRIBUTING.md)

## Performance

10-30x faster than pure Python/JavaScript implementations, delivering 150-280 MB/s throughput.

## License

MIT
