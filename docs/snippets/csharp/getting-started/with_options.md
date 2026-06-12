```csharp
using HtmlToMarkdown;

var options = new ConversionOptions
{
    HeadingStyle = "atx",
    Wrap = true,
    WrapWidth = 80,
    ListIndentWidth = 4,
};

var html = "<h1>Hello</h1><p>This is <strong>formatted</strong> content.</p>";
var result = HtmlToMarkdownRs.Convert(html, options);
Console.WriteLine(result.Content);
```
