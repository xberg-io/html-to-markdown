```swift
import HtmlToMarkdown

do {
    let result = try convert(html: "<h1>Hello</h1>")
    print(result.content ?? "")
} catch let ConversionError.parseError(message, _) {
    print("Parse failed: \(message)")
} catch let error as ConversionError {
    print("Conversion failed: \(error)")
}
```
