```swift
import HtmlToMarkdown

let options = try conversionOptionsFromJson(
    "{\"include_document_structure\":true}"
)

let html = """
<table>
  <tr><th>Name</th><th>Age</th></tr>
  <tr><td>Alice</td><td>30</td></tr>
  <tr><td>Bob</td><td>25</td></tr>
</table>
"""

let result = try convert(html: html, options: options)

for table in result.tables {
    print("Markdown:", table.markdown)
    let grid = table.grid
    print("Grid: \(grid.rows) rows x \(grid.cols) cols")
    for cell in grid.cells {
        let kind = cell.isHeader ? "Header" : "Cell"
        print("  \(kind) (r\(cell.row),c\(cell.col)): \(cell.content)")
    }
}
```
