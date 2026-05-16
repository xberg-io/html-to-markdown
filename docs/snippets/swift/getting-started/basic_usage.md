```swift
import HtmlToMarkdown

let html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>"
let result = try convert(html, nil)
let markdown = result.content()?.toString() ?? ""
print(markdown)
```
