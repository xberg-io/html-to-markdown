# Migrating to v3

## Background

html-to-markdown began as a Python library — a fork of
[markdownify](https://github.com/matthewwithanm/python-markdownify) —
focused on converting HTML to Markdown. Over time, the core was
rewritten in Rust for performance, and native bindings were added for
12 programming languages: Python, TypeScript, Ruby, Go, PHP, Java,
C#, Elixir, R, C, and WebAssembly.

v3.0.0 rationalises the library's API surface. Instead of multiple
specialised functions, there is now a single `convert()` function that
returns a structured `ConversionResult`. This simplifies the API while
making all extraction capabilities available through one call.

### hOCR support moved to kreuzberg

hOCR (OCR-generated HTML) processing was removed from html-to-markdown
and moved into [kreuzberg](https://github.com/kreuzberg-dev/kreuzberg),
the document extraction library in the kreuzberg.dev ecosystem. hOCR is
a document-level concern — it belongs in the extraction pipeline, not
in an HTML-to-Markdown converter.

## API changes

### Before (v2)

v2 had multiple functions for different extraction needs:

- `convert(html)` → `String`
- `convert_with_metadata(html, options, config)` → `(String, Metadata)`
- `convert_with_inline_images(html, options, config)` → `Extraction`
- `convert_with_tables(html, options)` → `TablesResult`
- `convert_with_visitor(html, options, visitor)` → `String`

### After (v3)

v3 has one function:

- `convert(html, options?, visitor?)` → `ConversionResult`

`ConversionResult` contains all data:

- `content` — the converted text (Markdown, Djot, or plain text)
- `metadata` — HTML metadata (title, OG, links, images, structured data)
- `tables` — extracted tables with cell data
- `document` — structured document tree (when enabled)
- `images` — extracted inline images (when enabled)
- `warnings` — non-fatal processing warnings

### Migration table

| v2 | v3 |
|----|-----|
| `convert(html)` | `convert(html).content` |
| `convert_with_metadata(html, opts, cfg)` | `convert(html, opts).metadata` |
| `convert_with_inline_images(html, opts, cfg)` | `convert(html, opts).images` (set `extract_images: true`) |
| `convert_with_tables(html, opts)` | `convert(html, opts).tables` |
| `convert_with_visitor(html, opts, v)` | `convert(html, opts, v).content` |
| `convert_to_string(html)` | `convert(html).content` |
| `hocr_spatial_tables: true` | Use [kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) |
| `ExtendedMetadata` | `HtmlMetadata` |
| `start_profiling()` | Removed |
| `markdownify(html)` (Python) | `convert(html).content` |

## Examples by language

### Rust

```rust
// v2
let markdown = convert(html, None)?;
let (md, meta) = convert_with_metadata(html, None, cfg, None)?;

// v3
let result = convert(html, None)?;
let markdown = result.content.unwrap_or_default();
let metadata = result.metadata; // always available
```

### Python

```python
# v2
markdown = convert(html)
markdown, metadata = convert_with_metadata(html)

# v3
result = convert(html)
markdown = result["content"]
metadata = result["metadata"]
```

### TypeScript

```typescript
// v2
const markdown = convert(html);
const { markdown, metadata } = convertWithMetadata(html);

// v3
const result = convert(html);
const markdown = result.content;
const metadata = result.metadata;
```

### Go

```go
// v2
markdown, err := htmltomarkdown.Convert(html, nil)
markdown, meta, err := htmltomarkdown.ConvertWithMetadata(html, nil)

// v3
result, err := htmltomarkdown.Convert(html, nil)
markdown := result.Content
metadata := result.Metadata
```

### Ruby

```ruby
# v2
markdown = HtmlToMarkdown.convert(html)
markdown, metadata = HtmlToMarkdown.convert_with_metadata(html)

# v3
result = HtmlToMarkdown.convert(html)
markdown = result[:content]
metadata = result[:metadata]
```

### PHP

```php
// v2
$markdown = HtmlToMarkdown\convert($html);
[$markdown, $metadata] = HtmlToMarkdown\convert_with_metadata($html);

// v3
$result = HtmlToMarkdown\convert($html);
$markdown = $result->content;
$metadata = $result->metadata;
```

### Java

```java
// v2
String markdown = HtmlToMarkdown.convert(html);
ConversionPair pair = HtmlToMarkdown.convertWithMetadata(html);

// v3
ConversionResult result = HtmlToMarkdown.convert(html);
String markdown = result.getContent();
HtmlMetadata metadata = result.getMetadata();
```

### C

```csharp
// v2
string markdown = HtmlToMarkdown.Convert(html);
(string md, Metadata meta) = HtmlToMarkdown.ConvertWithMetadata(html);

// v3
ConversionResult result = HtmlToMarkdown.Convert(html);
string markdown = result.Content;
HtmlMetadata metadata = result.Metadata;
```

### Elixir

```elixir
# v2
{:ok, markdown} = HtmlToMarkdown.convert(html)
{:ok, markdown, metadata} = HtmlToMarkdown.convert_with_metadata(html)

# v3
{:ok, result} = HtmlToMarkdown.convert(html)
markdown = result.content
metadata = result.metadata
```

### R

```r
# v2
markdown <- html_to_markdown(html)
result   <- html_to_markdown_with_metadata(html)

# v3
result   <- convert(html)
markdown <- result$content
metadata <- result$metadata
```

### C (FFI)

```c
/* v2 */
char *markdown = htm_convert(html, NULL);
htm_free(markdown);

/* v3 */
HtmResult *result = htm_convert(html, NULL);
const char *markdown = htm_result_content(result);
htm_result_free(result);
```
