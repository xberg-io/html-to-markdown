```csharp
using HtmlToMarkdown;

var html = @"
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>";

var result = HtmlToMarkdownRs.Convert(html);

foreach (var table in result.Tables)
{
    foreach (var cell in table.Grid.Cells)
    {
        var kind = cell.IsHeader ? "Header" : "Cell";
        Console.WriteLine($"  {kind} (r{cell.Row},c{cell.Col}): {cell.Content}");
    }
}
```
