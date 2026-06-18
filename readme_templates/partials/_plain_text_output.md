## Plain Text Output

Set `output_format` to `"plain"` to strip all markup and return only visible text. This bypasses the Markdown conversion pipeline entirely for maximum speed.

{% if language == 'python' %}

```python
from html_to_markdown import ConversionOptions, convert

html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

result = convert(html, ConversionOptions(output_format="plain"))
plain = result.content
# Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'typescript' %}

```typescript
import { convert, OutputFormat } from "@kreuzberg/html-to-markdown";

const html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

const result = convert(html, { outputFormat: OutputFormat.Plain });
const plain = result.content;
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'ruby' %}

```ruby
require 'html_to_markdown'

html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

result = HtmlToMarkdown.convert(html, output_format: 'plain')
plain = result[:content]
# Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'php' %}

Set `ConversionOptions.outputFormat` to `OutputFormat::Plain` and pass the options object to
`HtmlToMarkdownApi::convert($html, $options)`. The PHP API reference lists the full options constructor.

{% elif language == 'go' %}

```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3"

html := "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

result, _ := htmltomarkdown.Convert(html, &htmltomarkdown.ConversionOptions{
    OutputFormat: htmltomarkdown.OutputFormatPlain,
})
plain := *result.Content
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'java' %}

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.OutputFormat;

String html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

String plain = HtmlToMarkdown.convert(html,
    ConversionOptions.builder()
        .withOutputFormat(OutputFormat.Plain)
        .build()
).content();
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'csharp' %}

```csharp
using HtmlToMarkdown;

var html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

var plain = Converter.Convert(html, new ConversionOptions { OutputFormat = OutputFormat.Plain }).Content;
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'elixir' %}

```elixir
html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

{:ok, result} = HtmlToMarkdown.convert(html, %{output_format: "Plain"})
plain = result.content
# Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'r' %}

```r
html <- "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

result <- convert(html, options = list(output_format = "plain"))
plain <- result$content
# Result: "Title\n\nThis is bold and italic text."
```

{% endif %}

Plain text mode is useful for search indexing, text extraction, and feeding content to LLMs.
