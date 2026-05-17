```kotlin
import dev.kreuzberg.android.ConversionOptions
import dev.kreuzberg.android.HtmlToMarkdownRs

val html = """
    <table>
      <tr><th>Name</th><th>Age</th></tr>
      <tr><td>Alice</td><td>30</td></tr>
      <tr><td>Bob</td><td>25</td></tr>
    </table>
""".trimIndent()

val result = HtmlToMarkdownRs.convert(html, ConversionOptions.builder().build())

for (table in result.tables) {
    println(table.markdown)
    for (cell in table.grid.cells) {
        val kind = if (cell.isHeader) "Header" else "Cell"
        println("  $kind (r${cell.row},c${cell.col}): ${cell.content}")
    }
}
```
