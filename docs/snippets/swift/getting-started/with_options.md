```swift
import HtmlToMarkdown

let options = try conversionOptionsFromJson(
    "{\"heading_style\":\"atx\",\"list_indent_width\":2,\"wrap\":true}"
)

let html = "<h1>Hello</h1><p>This is <strong>formatted</strong> content.</p>"
let result = try convert(html, options)
let markdown = result.content()?.toString() ?? ""
print(markdown)
```
