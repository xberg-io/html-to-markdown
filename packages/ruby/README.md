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


Blazing-fast HTML to Markdown conversion for Ruby, powered by the same Rust engine used by our Python, Node.js, WebAssembly, and PHP packages.
Ship identical Markdown across every runtime while enjoying native extension performance with Magnus bindings.


## Installation

```bash
gem install html-to-markdown
```



Requires Ruby 3.2+ with Magnus native extension bindings. Published for Linux, macOS.






## Performance Snapshot

Apple M4 • Real Wikipedia documents • `convert()` (Ruby)

| Document | Size | Latency | Throughput |
| -------- | ---- | ------- | ---------- |
| Lists (Timeline) | 129KB | 0.71ms | 182 MB/s |
| Tables (Countries) | 360KB | 2.15ms | 167 MB/s |
| Mixed (Python wiki) | 656KB | 4.89ms | 134 MB/s |




## Quick Start

Basic conversion:

```ruby
require 'html_to_markdown'

html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>"
result = HtmlToMarkdown.convert(html)
markdown = result[:content]
```



With conversion options:

```ruby
require 'html_to_markdown'

html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>"
result = HtmlToMarkdown.convert(html, heading_style: :atx, code_block_style: :fenced)
markdown = result[:content]
```




## API Reference

### Core Function


**`convert(html, options: nil, visitor: nil) -> ConversionResult`**

Converts HTML to Markdown. Returns a `ConversionResult` hash with all results in a single call.

```ruby
require 'html_to_markdown'

result = HtmlToMarkdown.convert(html)
markdown = result[:content]       # Converted Markdown string
metadata = result[:metadata]      # Metadata (when extract_metadata: true)
tables   = result[:tables]        # Structured table data (when extract_tables: true)
document = result[:document]      # Document-level info
images   = result[:images]        # Extracted images
warnings = result[:warnings]      # Any conversion warnings
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


```ruby
require 'html_to_markdown'

html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

# Default Markdown output
markdown = HtmlToMarkdown.convert(html)
# Result: "This is **bold** and *italic* text."

# Djot output
djot = HtmlToMarkdown.convert(html, output_format: 'djot')
# Result: "This is *bold* and _italic_ text."
```


Djot's extended syntax allows you to express more semantic meaning in lightweight text, making it useful for documents that require strikethrough, insertion tracking, or mathematical notation.


## Plain Text Output

Set `output_format` to `"plain"` to strip all markup and return only visible text. This bypasses the Markdown conversion pipeline entirely for maximum speed.


```ruby
require 'html_to_markdown'

html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

plain = HtmlToMarkdown.convert(html, output_format: 'plain')
# Result: "Title\n\nThis is bold and italic text."
```


Plain text mode is useful for search indexing, text extraction, and feeding content to LLMs.



## Metadata Extraction

The metadata extraction feature enables comprehensive document analysis during conversion. Extract document properties, headers, links, images, and structured data in a single pass — all via the standard `convert()` function.

**Use Cases:**

- **SEO analysis** – Extract title, description, Open Graph tags, Twitter cards
- **Table of contents generation** – Build structured outlines from heading hierarchy
- **Content migration** – Document all external links and resources
- **Accessibility audits** – Check for images without alt text, empty links, invalid heading hierarchy
- **Link validation** – Classify and validate anchor, internal, external, email, and phone links

**Zero Overhead When Disabled:** Metadata extraction adds negligible overhead and happens during the HTML parsing pass. Pass `extract_metadata: true` in `ConversionOptions` to enable it; the result is available at `result.metadata`.

### Example: Quick Start


```ruby
require 'html_to_markdown'

html = '<h1>Article</h1><img src="test.jpg" alt="test">'
result = HtmlToMarkdown.convert(html, extract_metadata: true)

puts result[:content]                             # Converted Markdown
puts result[:metadata][:document][:title]         # Document title
puts result[:metadata][:headers]                  # All h1-h6 elements
puts result[:metadata][:links]                    # All hyperlinks
puts result[:metadata][:images]                   # All images with alt text
puts result[:metadata][:structured_data]          # JSON-LD, Microdata, RDFa
```






## Visitor Pattern

The visitor pattern enables custom HTML→Markdown conversion logic by providing callbacks for specific HTML elements during traversal. Pass a visitor as the third argument to `convert()`.

**Use Cases:**

- **Custom Markdown dialects** – Convert to Obsidian, Notion, or other flavors
- **Content filtering** – Remove tracking pixels, ads, or unwanted elements
- **URL rewriting** – Rewrite CDN URLs, add query parameters, validate links
- **Accessibility validation** – Check alt text, heading hierarchy, link text
- **Analytics** – Track element usage, link destinations, image sources

**Supported Visitor Methods:** 40+ callbacks for text, inline elements, links, images, headings, lists, blocks, and tables.

### Example: Quick Start


```ruby
require 'html_to_markdown'

class MyVisitor
  def visit_link(ctx, href, text, title = nil)
    # Rewrite CDN URLs
    if href.start_with?('https://old-cdn.com')
      href = href.sub('https://old-cdn.com', 'https://new-cdn.com')
    end
    { type: :custom, output: "[#{text}](#{href})" }
  end

  def visit_image(ctx, src, alt = nil, title = nil)
    # Skip tracking pixels
    src.include?('tracking') ? { type: :skip } : { type: :continue }
  end
end

html = '<a href="https://old-cdn.com/file.pdf">Download</a>'
result = HtmlToMarkdown.convert(html, visitor: MyVisitor.new)
markdown = result[:content]
```





## Examples


## Links

- **GitHub:** [github.com/kreuzberg-dev/html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown)

- **RubyGems:** [rubygems.org/gems/html-to-markdown](https://rubygems.org/gems/html-to-markdown)

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
