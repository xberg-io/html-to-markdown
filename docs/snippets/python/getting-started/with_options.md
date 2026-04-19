```python
from html_to_markdown import ConversionOptions, convert

html = "<h1>Hello</h1><p>This is <strong>formatted</strong> content.</p>"
options = ConversionOptions(
    heading_style="atx",
    list_indent_width=2,
)
result = convert(html, options)
markdown = result.content
```
