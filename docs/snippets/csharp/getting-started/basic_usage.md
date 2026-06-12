```csharp
using HtmlToMarkdown;

var html = "<h1>Hello World</h1><p>This is a paragraph.</p>";
var result = HtmlToMarkdownRs.Convert(html);
Console.WriteLine(result.Content);
```
