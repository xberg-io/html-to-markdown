# html-to-markdown

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
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
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/html-to-markdown?label=Go&color=007ec6&filter=v3.1.0" alt="Go">
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
    <img src="https://img.shields.io/cran/v/htmltomarkdown?label=R&color=007ec6" alt="R">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C">
  </a>

  <!-- Project Info -->
  <a href="https://docs.html-to-markdown.kreuzberg.dev">
    <img src="https://img.shields.io/badge/Docs-kreuzberg.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  </a>
</div>

<img width="1128" height="191" alt="html-to-markdown" src="https://github.com/user-attachments/assets/419fc06c-8313-4324-b159-4b4d3cfce5c0" />

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/pXxagNK2zN">
      <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>


High-performance HTML to Markdown converter with Java Panama FFI bindings to the Rust core.
Uses Foreign Function & Memory API for zero-dependency, thread-safe conversion with full metadata extraction support.


## Installation

```bash
<dependency>
    <groupId>dev.kreuzberg</groupId>
    <artifactId>html-to-markdown</artifactId>
    <version>3.1.0</version>
    <classifier>linux</classifier> <!-- or macos, windows -->
</dependency>

```



Requires Java 25+ with Panama FFI support.

**Maven:**

```xml
<dependency>
    <groupId>dev.kreuzberg</groupId>
    <artifactId>html-to-markdown</artifactId>
    <version>3.1.0</version>
</dependency>
```

**Gradle (Kotlin DSL):**

```kotlin
implementation("dev.kreuzberg:html-to-markdown:3.1.0")
```






## Performance Snapshot

Apple M4 • Real Wikipedia documents • `convert()` (Java)

| Document | Size | Ops/sec | Throughput |
| -------- | ---- | ------- | ---------- |
| Lists (Timeline) | 129KB | 2,308 | 291.5 MB/s |
| Tables (Countries) | 360KB | 773 | 272.0 MB/s |
| Mixed (Python) | 656KB | 403 | 258.5 MB/s |




## Quick Start

Basic conversion:

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionResult;

public class Example {
    public static void main(String[] args) {
        String html = "<h1>Hello World</h1><p>This is a <strong>test</strong>.</p>";
        ConversionResult result = HtmlToMarkdown.convert(html);
        System.out.println(result.content());
    }
}
```



With conversion options:

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.ConversionResult;

public class MetadataExample {
    public static void main(String[] args) {
        String html = "<html><head><title>My Page</title></head>"
            + "<body><h1>Welcome</h1><a href=\"https://example.com\">Link</a></body></html>";

        ConversionOptions options = ConversionOptions.builder()
            .extractMetadata(true)
            .build();
        ConversionResult result = HtmlToMarkdown.convert(html, options);

        System.out.println("Markdown: " + result.content());
        System.out.println("Title: " + result.metadata().document().title());
        System.out.println("Headers: " + result.metadata().headers().size());
        System.out.println("Links: " + result.metadata().links().size());
    }
}
```




## API Reference

### Core Function


**`HtmlToMarkdown.convert(String html) : ConversionResult`**
**`HtmlToMarkdown.convert(String html, ConversionOptions options) : ConversionResult`**

Converts HTML to Markdown. Returns a `ConversionResult` record with all results in a single call.

```java
ConversionResult result = HtmlToMarkdown.convert(html);
String   markdown = result.content();   // Converted Markdown string
Metadata metadata = result.metadata();  // null unless extractMetadata(true)
List<?>  tables   = result.tables();    // empty unless extractTables(true)
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

| Element | Markdown | Djot |
|---------|----------|------|
| Strong | `**text**` | `*text*` |
| Emphasis | `*text*` | `_text_` |
| Strikethrough | `~~text~~` | `{-text-}` |
| Inserted/Added | N/A | `{+text+}` |
| Highlighted | N/A | `{=text=}` |
| Subscript | N/A | `~text~` |
| Superscript | N/A | `^text^` |

### Example Usage


```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.OutputFormat;

String html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
String markdown = HtmlToMarkdown.convert(html);
// Result: "This is **bold** and *italic* text."

// Djot output
String djot = HtmlToMarkdown.convert(html,
    new ConversionOptions().setOutputFormat(OutputFormat.DJOT));
// Result: "This is *bold* and _italic_ text."
```


Djot's extended syntax allows you to express more semantic meaning in lightweight text, making it useful for documents that require strikethrough, insertion tracking, or mathematical notation.


## Plain Text Output

Set `output_format` to `"plain"` to strip all markup and return only visible text. This bypasses the Markdown conversion pipeline entirely for maximum speed.


```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.OutputFormat;

String html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

String plain = HtmlToMarkdown.convert(html,
    new ConversionOptions().setOutputFormat(OutputFormat.PLAIN));
// Result: "Title\n\nThis is bold and italic text."
```


Plain text mode is useful for search indexing, text extraction, and feeding content to LLMs.






## Examples


## Links

- **GitHub:** [github.com/kreuzberg-dev/html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown)

- **Maven Central:** [central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown](https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown)

- **Kreuzberg Ecosystem:** [kreuzberg.dev](https://kreuzberg.dev)
- **Discord:** [discord.gg/pXxagNK2zN](https://discord.gg/pXxagNK2zN)

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

MIT License – see [LICENSE](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE).

## Support

If you find this library useful, consider [sponsoring the project](https://github.com/sponsors/kreuzberg-dev).

Have questions or run into issues? We're here to help:

- **GitHub Issues:** [github.com/kreuzberg-dev/html-to-markdown/issues](https://github.com/kreuzberg-dev/html-to-markdown/issues)
- **Discussions:** [github.com/kreuzberg-dev/html-to-markdown/discussions](https://github.com/kreuzberg-dev/html-to-markdown/discussions)
- **Discord Community:** [discord.gg/pXxagNK2zN](https://discord.gg/pXxagNK2zN)
