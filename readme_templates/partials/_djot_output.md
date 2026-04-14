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

{% if language == 'python' %}

```python
from html_to_markdown import convert, ConversionOptions

html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

# Default Markdown output
markdown = convert(html)
# Result: "This is **bold** and *italic* text."

# Djot output
djot = convert(html, ConversionOptions(output_format="djot"))
# Result: "This is *bold* and _italic_ text."
```

{% elif language == 'typescript' %}

```typescript
import { convert, ConversionOptions } from '@kreuzberg/html-to-markdown';

const html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
const markdown = convert(html);
// Result: "This is **bold** and *italic* text."

// Djot output
const djot = convert(html, { outputFormat: 'djot' });
// Result: "This is *bold* and _italic_ text."
```

{% elif language == 'ruby' %}

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

{% elif language == 'php' %}

```php
use HtmlToMarkdown\Converter;
use HtmlToMarkdown\ConversionOptions;

$html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
$markdown = Converter::convert($html);
// Result: "This is **bold** and *italic* text."

// Djot output
$djot = Converter::convert($html, new ConversionOptions(outputFormat: 'djot'));
// Result: "This is *bold* and _italic_ text."
```

{% elif language == 'go' %}

```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"

html := "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

// Default Markdown output
markdown, _ := htmltomarkdown.Convert(html)
// Result: "This is **bold** and *italic* text."

// Note: Djot output format configuration is not yet supported in Go bindings
```

{% elif language == 'java' %}

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

{% elif language == 'csharp' %}

```csharp
using HtmlToMarkdown;

var html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
var markdown = Converter.Convert(html);
// Result: "This is **bold** and *italic* text."

// Djot output
var djot = Converter.Convert(html, new ConversionOptions { OutputFormat = "djot" });
// Result: "This is *bold* and _italic_ text."
```

{% elif language == 'elixir' %}

```elixir
html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

# Default Markdown output
{:ok, markdown} = HtmlToMarkdown.convert(html)
# Result: "This is **bold** and *italic* text."

# Djot output
{:ok, djot} = HtmlToMarkdown.convert(html, %{output_format: "djot"})
# Result: "This is *bold* and _italic_ text."
```

{% endif %}

Djot's extended syntax allows you to express more semantic meaning in lightweight text, making it useful for documents that require strikethrough, insertion tracking, or mathematical notation.
