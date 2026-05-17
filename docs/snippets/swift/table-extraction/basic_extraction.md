```swift
import HtmlToMarkdown

let html = """
<table>
  <tr><th>Name</th><th>Age</th></tr>
  <tr><td>Alice</td><td>30</td></tr>
  <tr><td>Bob</td><td>25</td></tr>
</table>
"""

let result = try convert(html, nil)

for table in result.tables() {
    print("Markdown:", table.markdown().toString())
    let grid = table.grid()
    print("Grid: \(grid.rows()) rows x \(grid.cols()) cols")
    for cell in grid.cells() {
        let kind = cell.is_header() ? "Header" : "Cell"
        print("  \(kind) (r\(cell.row()),c\(cell.col())): \(cell.content().toString())")
    }
}
```
