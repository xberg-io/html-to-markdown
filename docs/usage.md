# Usage

## Basic Conversion

`convert()` accepts an HTML string and returns a `ConversionResult`.

=== "Rust"
    --8<-- "snippets/rust/getting-started/basic_usage.md"

=== "Python"
    --8<-- "snippets/python/getting-started/basic_usage.md"

=== "TypeScript"
    --8<-- "snippets/typescript/getting-started/basic_usage.md"

=== "Go"
    --8<-- "snippets/go/getting-started/basic_usage.md"

=== "Ruby"
    --8<-- "snippets/ruby/getting-started/basic_usage.md"

=== "PHP"
    --8<-- "snippets/php/getting-started/basic_usage.md"

=== "Java"
    --8<-- "snippets/java/getting-started/basic_usage.md"

=== "C#"
    --8<-- "snippets/csharp/getting-started/basic_usage.md"

=== "Elixir"
    --8<-- "snippets/elixir/getting-started/basic_usage.md"

=== "R"
    --8<-- "snippets/r/getting-started/basic_usage.md"

=== "C"
    --8<-- "snippets/c/getting-started/basic_usage.md"

=== "WASM"
    --8<-- "snippets/wasm/getting-started/basic_usage.md"

## ConversionResult Fields

Every call to `convert()` returns a `ConversionResult` with the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `content` | `Optional<String>` | The converted text (Markdown, Djot, or plain). `None`/`null` when `output_format` is `"none"`. |
| `document` | `Optional<DocumentStructure>` | Structured document tree (headings, paragraphs, lists, tables). Only populated when `include_document_structure` is `true`. |
| `metadata` | `HtmlMetadata` | Extracted HTML metadata (title, description, Open Graph, Twitter Card, JSON-LD, links, images). |
| `tables` | `Vec<TableData>` | Extracted tables with full grid data (headers, rows, colspan/rowspan). |
| `images` | `Vec<ExtractedImage>` | Extracted inline images (data URIs, embedded SVGs). Only populated when `extract_images` is `true`. |
| `warnings` | `Vec<ProcessingWarning>` | Non-fatal warnings raised during conversion. |

## Using Options

Control output style, metadata extraction, and more via `ConversionOptions`.

=== "Rust"
    --8<-- "snippets/rust/getting-started/with_options.md"

=== "Python"
    --8<-- "snippets/python/getting-started/with_options.md"

=== "TypeScript"
    --8<-- "snippets/typescript/getting-started/with_options.md"

=== "Go"
    --8<-- "snippets/go/getting-started/with_options.md"

=== "Ruby"
    --8<-- "snippets/ruby/getting-started/with_options.md"

=== "PHP"
    --8<-- "snippets/php/getting-started/with_options.md"

=== "Java"
    --8<-- "snippets/java/getting-started/with_options.md"

=== "C#"
    --8<-- "snippets/csharp/getting-started/with_options.md"

=== "Elixir"
    --8<-- "snippets/elixir/getting-started/with_options.md"

=== "R"
    --8<-- "snippets/r/getting-started/with_options.md"

## Metadata Extraction

Enable `extract_metadata` to populate the `metadata` field with structured data parsed from the HTML `<head>` and document body.

=== "Rust"
    --8<-- "snippets/rust/metadata/basic_extraction.md"

=== "Python"
    --8<-- "snippets/python/metadata/basic_extraction.md"

=== "TypeScript"
    --8<-- "snippets/typescript/metadata/basic_extraction.md"

=== "Go"
    --8<-- "snippets/go/metadata/basic_extraction.md"

=== "Ruby"
    --8<-- "snippets/ruby/metadata/basic_extraction.md"

=== "PHP"
    --8<-- "snippets/php/metadata/basic_extraction.md"

=== "Java"
    --8<-- "snippets/java/metadata/basic_extraction.md"

=== "C#"
    --8<-- "snippets/csharp/metadata/basic_extraction.md"

=== "Elixir"
    --8<-- "snippets/elixir/metadata/basic_extraction.md"

=== "R"
    --8<-- "snippets/r/metadata/basic_extraction.md"

### Metadata Fields

| Field | Description |
|-------|-------------|
| `document.title` | Page title from `<title>` tag |
| `document.description` | Content of `<meta name="description">` |
| `document.language` | `lang` attribute of `<html>` tag |
| `document.charset` | Character encoding declaration |
| `document.open_graph` | Open Graph tags (`og:title`, `og:description`, etc.) |
| `document.twitter_card` | Twitter Card tags |
| `document.json_ld` | JSON-LD structured data blocks |
| `headers` | All `<h1>`–`<h6>` elements with level, text, and id |
| `links` | All `<a>` tags with href, text, rel, and link type |
| `images` | All `<img>` tags with src, alt, width, height |

## Document Structure Extraction

Enable `include_document_structure` to get a parsed tree of the document's structural elements.

=== "Rust"
    ```rust
    use html_to_markdown_rs::{convert, ConversionOptions};

    let options = ConversionOptions::builder()
        .include_document_structure(true)
        .build();
    let result = convert("<h1>Title</h1><p>Paragraph</p>", Some(options))?;

    if let Some(doc) = &result.document {
        for node in &doc.nodes {
            println!("{:?}", node);
        }
    }
    ```

=== "Python"
    ```python
    from html_to_markdown import ConversionOptions, convert

    options = ConversionOptions(include_document_structure=True)
    result = convert("<h1>Title</h1><p>Paragraph</p>", options)
    doc = result["document"]
    for node in doc["nodes"]:
        print(node)
    ```

=== "TypeScript"
    ```typescript
    import { convert, ConversionOptions } from '@kreuzberg/html-to-markdown';

    const options: ConversionOptions = { includeDocumentStructure: true };
    const result = convert('<h1>Title</h1><p>Paragraph</p>', options);
    const nodes = result.document?.nodes ?? [];
    for (const node of nodes) {
      console.log(node);
    }
    ```
