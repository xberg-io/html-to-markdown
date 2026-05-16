```kotlin
import dev.kreuzberg.android.ConversionError
import dev.kreuzberg.android.HtmlToMarkdownRs

try {
    val result = HtmlToMarkdownRs.convert("<h1>Hello</h1>")
    println(result.content)
} catch (error: ConversionError.ParseError) {
    System.err.println("Parse failed: ${error.field0}")
} catch (error: ConversionError) {
    System.err.println("Conversion failed: ${error.message}")
}
```
