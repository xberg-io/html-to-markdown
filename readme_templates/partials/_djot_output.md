## Djot Output Format

The library supports converting HTML to [Djot](https://djot.net/), a lightweight markup language similar to Markdown but with a different syntax for some elements. Set `output_format` to `"djot"` to use this format.

### Syntax Differences

| Element        | Markdown   | Djot       |
| -------------- | ---------- | ---------- |
| Strong         | `**text**` | `*text*`   |
| Emphasis       | `*text*`   | `_text_`   |
| Strikethrough  | `~~text~~` | `{-text-}` |
| Inserted/Added | N/A        | `{+text+}` |
| Highlighted    | N/A        | `{=text=}` |
| Subscript      | N/A        | `~text~`   |
| Superscript    | N/A        | `^text^`   |

### Example Usage

{% if language == 'python' %}

```python
from html_to_markdown import ConversionOptions, convert

html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

# Default Markdown output
markdown_result = convert(html)
markdown = markdown_result.content
# Result: "This is **bold** and *italic* text."

# Djot output
djot_result = convert(html, ConversionOptions(output_format="djot"))
djot = djot_result.content
# Result: "This is *bold* and _italic_ text."
```

{% elif language == 'typescript' %}

```typescript
import { convert, OutputFormat } from "@kreuzberg/html-to-markdown";

const html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
const markdownResult = convert(html);
const markdown = markdownResult.content;
// Result: "This is **bold** and *italic* text."

// Djot output
const djotResult = convert(html, { outputFormat: OutputFormat.Djot });
const djot = djotResult.content;
// Result: "This is *bold* and _italic_ text."
```

{% elif language == 'ruby' %}

```ruby
require 'html_to_markdown'

html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

# Default Markdown output
markdown_result = HtmlToMarkdown.convert(html)
markdown = markdown_result[:content]
# Result: "This is **bold** and *italic* text."

# Djot output
djot_result = HtmlToMarkdown.convert(html, output_format: 'djot')
djot = djot_result[:content]
# Result: "This is *bold* and _italic_ text."
```

{% elif language == 'php' %}

Set `ConversionOptions.outputFormat` to `OutputFormat::Djot` and pass the options object to
`HtmlToMarkdownApi::convert($html, $options)`. The PHP API reference lists the full options constructor.

{% elif language == 'go' %}

```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3"

html := "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

// Default Markdown output
markdownResult, _ := htmltomarkdown.Convert(html, nil)
markdown := *markdownResult.Content
// Result: "This is **bold** and *italic* text."

// Djot output
djotResult, _ := htmltomarkdown.Convert(html, &htmltomarkdown.ConversionOptions{
    OutputFormat: htmltomarkdown.OutputFormatDjot,
})
djot := *djotResult.Content
// Result: "This is *bold* and _italic_ text."
```

{% elif language == 'java' %}

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.OutputFormat;

String html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
String markdown = HtmlToMarkdown.convert(html).content();
// Result: "This is **bold** and *italic* text."

// Djot output
String djot = HtmlToMarkdown.convert(html,
    ConversionOptions.builder()
        .withOutputFormat(OutputFormat.Djot)
        .build()
).content();
// Result: "This is *bold* and _italic_ text."
```

{% elif language == 'csharp' %}

```csharp
using HtmlToMarkdown;

var html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
var markdown = Converter.Convert(html).Content;
// Result: "This is **bold** and *italic* text."

// Djot output
var djot = Converter.Convert(html, new ConversionOptions { OutputFormat = OutputFormat.Djot }).Content;
// Result: "This is *bold* and _italic_ text."
```

{% elif language == 'elixir' %}

```elixir
html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

# Default Markdown output
{:ok, markdown_result} = HtmlToMarkdown.convert(html)
markdown = markdown_result.content
# Result: "This is **bold** and *italic* text."

# Djot output
{:ok, djot_result} = HtmlToMarkdown.convert(html, %{output_format: "Djot"})
djot = djot_result.content
# Result: "This is *bold* and _italic_ text."
```

{% endif %}

Djot's extended syntax allows you to express more semantic meaning in lightweight text, making it useful for documents that require strikethrough, insertion tracking, or mathematical notation.
