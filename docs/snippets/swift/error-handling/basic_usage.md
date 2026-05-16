```swift
import HtmlToMarkdown

do {
    let result = try convert("<h1>Hello</h1>", nil)
    print(result.content()?.toString() ?? "")
} catch let ConversionError.parseError(message, _) {
    print("Parse failed: \(message)")
} catch let error as ConversionError {
    print("Conversion failed: \(error)")
}
```
