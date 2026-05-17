```kotlin
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.PropertyNamingStrategies
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import dev.kreuzberg.android.ConversionOptions
import dev.kreuzberg.android.HtmlToMarkdownRs

val mapper = ObjectMapper()
    .registerKotlinModule()
    .setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE)
val options = mapper.readValue(
    "{\"heading_style\":\"Atx\",\"list_indent_width\":2,\"wrap\":true}",
    ConversionOptions::class.java,
)

val html = "<h1>Hello</h1><p>This is <strong>formatted</strong> content.</p>"
val result = HtmlToMarkdownRs.convert(html, options)
val markdown: String? = result.content
```
