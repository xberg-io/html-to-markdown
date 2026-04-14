## Plain Text Output

Set `output_format` to `"plain"` to strip all markup and return only visible text. This bypasses the Markdown conversion pipeline entirely for maximum speed.

{% if language == 'python' %}

```python
from html_to_markdown import convert, ConversionOptions

html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

plain = convert(html, ConversionOptions(output_format="plain"))
# Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'typescript' %}

```typescript
import { convert } from '@kreuzberg/html-to-markdown';

const html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

const plain = convert(html, { outputFormat: 'plain' });
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'ruby' %}

```ruby
require 'html_to_markdown'

html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

plain = HtmlToMarkdown.convert(html, output_format: 'plain')
# Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'php' %}

```php
use HtmlToMarkdown\Converter;
use HtmlToMarkdown\ConversionOptions;

$html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

$plain = Converter::convert($html, new ConversionOptions(outputFormat: 'plain'));
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'go' %}

```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"

html := "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

plain, _ := htmltomarkdown.Convert(html, htmltomarkdown.WithOutputFormat("plain"))
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'java' %}

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.OutputFormat;

String html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

String plain = HtmlToMarkdown.convert(html,
    new ConversionOptions().setOutputFormat(OutputFormat.PLAIN));
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'csharp' %}

```csharp
using HtmlToMarkdown;

var html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

var plain = Converter.Convert(html, new ConversionOptions { OutputFormat = "plain" });
// Result: "Title\n\nThis is bold and italic text."
```

{% elif language == 'elixir' %}

```elixir
html = "<h1>Title</h1><p>This is <strong>bold</strong> and <em>italic</em> text.</p>"

{:ok, plain} = HtmlToMarkdown.convert(html, %{output_format: "plain"})
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
