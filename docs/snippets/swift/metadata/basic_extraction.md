```swift
import HtmlToMarkdown

let options = try conversionOptionsFromJson(
    "{\"extract_metadata\":true,\"extract_images\":true}"
)

let html = """
<html>
  <head>
    <title>My Page</title>
    <meta name="description" content="A short description.">
    <meta name="author" content="Jane Doe">
  </head>
  <body>
    <h1>Welcome</h1>
    <p>See <a href="https://example.com">example</a>.</p>
  </body>
</html>
"""

let result = try convert(html, options)
let markdown = result.content()?.toString() ?? ""

let metadata = result.metadata()
let document = metadata.document()
print("title:", document.title()?.toString() ?? "")
print("description:", document.description()?.toString() ?? "")
print("author:", document.author()?.toString() ?? "")
print("headers:", metadata.headers().count)
print("links:", metadata.links().count)
print("images:", metadata.images().count)
print(markdown)
```
