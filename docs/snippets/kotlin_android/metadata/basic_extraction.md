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
    "{\"extract_metadata\":true}",
    ConversionOptions::class.java,
)

val html = """
    <html>
      <head>
        <title>My Page</title>
        <meta name="description" content="A short description">
      </head>
      <body><h1>Hello</h1><a href="https://example.com">Link</a></body>
    </html>
""".trimIndent()

val result = HtmlToMarkdownRs.convert(html, options)
println("Markdown: ${result.content}")
println("Title: ${result.metadata.document.title}")
println("Description: ${result.metadata.document.description}")
println("Headers: ${result.metadata.headers}")
println("Links: ${result.metadata.links}")
println("Images: ${result.metadata.images}")
```
