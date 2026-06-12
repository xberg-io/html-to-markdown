```csharp
using HtmlToMarkdown;

var html = @"<html><head><title>My Page</title></head>
<body><h1>Hello</h1><a href=""https://example.com"">Link</a></body></html>";

var options = new ConversionOptions { ExtractMetadata = true };
var result = HtmlToMarkdownRs.Convert(html, options);
Console.WriteLine($"Markdown: {result.Content}");
Console.WriteLine($"Title: {result.Metadata?.Title}");
Console.WriteLine($"Links: {string.Join(", ", result.Metadata?.Links ?? [])}");
```
