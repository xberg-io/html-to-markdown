```python
from html_to_markdown import ConversionOptions, convert

class CustomVisitor:
    def visit_link(self, ctx, href, text, title):
        # Custom link handling
        return {"type": "continue"}

    def visit_image(self, ctx, src, alt, title):
        # Custom image handling
        return {"type": "continue"}

options = ConversionOptions(visitor=CustomVisitor())
result = convert(html, options)
markdown = result.content
```
