```swift
import HtmlToMarkdown

final class CustomVisitor: HtmlVisitorProtocol {
    func visitLink(_ ctx: NodeContext, _ href: String, _ text: String, _ title: String?) -> VisitResult {
        // Replace links with a bracketed custom format
        return .custom(field0: "[\(text)](\(href))")
    }

    func visitHeading(_ ctx: NodeContext, _ level: UInt32, _ text: String, _ id: String?) -> VisitResult {
        // Keep default rendering for headings
        return .continue_
    }
}

let visitorHandle = makeHtmlVisitorHandle(CustomVisitor())
let options = try conversionOptionsFromJsonWithVisitor("{}", visitorHandle)

let html = "<h1>Title</h1><p>See <a href=\"https://example.com\">example</a>.</p>"
let result = try convert(html, options)
let markdown = result.content()?.toString() ?? ""
print(markdown)
```
