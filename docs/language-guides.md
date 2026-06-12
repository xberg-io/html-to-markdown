# Language Binding Guides

Every binding wraps the same Rust core. The option names and return shapes are identical across languages; only the syntax differs. This page covers per-language install notes and naming conventions.

## Rust

**Package:** `html-to-markdown-rs` on [crates.io](https://crates.io/crates/html-to-markdown-rs)

```toml
html-to-markdown-rs = "3.4"
```

Option structs follow Rust naming conventions (`snake_case`). Use the builder API via `ConversionOptions::builder()` or construct `ConversionOptions` directly.

See [Installation: Feature Flags](installation.md#feature-flags) for the six Cargo features.

## Python

**Package:** `html-to-markdown` on PyPI
**Requires:** Python ≥ 3.10

```bash
pip install html-to-markdown
```

```python
from html_to_markdown import convert, ConversionOptions

result = convert("<h1>Title</h1>", ConversionOptions(heading_style="atx"))
print(result.content)
```

Option keys match the Rust field names (`snake_case`). `ConversionOptions` accepts keyword arguments. `ConversionResult` is a class with attributes — access fields as `result.content`, `result.metadata`, `result.tables`, `result.images`, `result.document`, `result.warnings`.

## TypeScript

**Package:** `@kreuzberg/html-to-markdown` on npm
**Requires:** Node.js ≥ 18

```bash
npm install @kreuzberg/html-to-markdown
```

```typescript
import { convert } from "@kreuzberg/html-to-markdown";

const result = convert("<h1>Title</h1>", { headingStyle: "atx" });
console.log(result.content);
```

Option keys are `camelCase` (`headingStyle`, `linkStyle`, `outputFormat`). The package ships both ESM and CJS outputs. TypeScript types are bundled.

## Go

**Module:** `github.com/kreuzberg-dev/html-to-markdown/packages/go/v3`
**Requires:** Go ≥ 1.26

```bash
go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v3
```

```go
import htmltomarkdown "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3"

result, err := htmltomarkdown.Convert("<h1>Title</h1>", nil)
if err != nil {
    log.Fatal(err)
}
fmt.Println(result.Content)
```

Options use Go struct field names (`PascalCase`). `Convert` returns `(*ConversionResult, error)`. Errors are standard Go errors. Use `errors.Is`/`errors.As` to inspect them.

## Ruby

**Gem:** `html-to-markdown` on RubyGems
**Requires:** Ruby ≥ 3.2

```bash
gem install html-to-markdown
```

```ruby
require 'html_to_markdown'

result = HtmlToMarkdown.convert('<h1>Title</h1>', heading_style: :atx)
puts result[:content]
```

Options are keyword arguments with `snake_case` symbols. `result` is a hash. Errors raise `HtmlToMarkdown::ConversionError`.

## PHP

**Package:** `kreuzberg-dev/html-to-markdown` on Packagist
**Requires:** PHP ≥ 8.2

```bash
composer require kreuzberg-dev/html-to-markdown
```

```php
use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;

$options = ConversionOptions::builder()->headingStyle('atx')->build();
$result = HtmlToMarkdown::convert('<h1>Title</h1>', $options);
echo $result->content;
```

Classes live in the flat `HtmlToMarkdown\` namespace. `HtmlToMarkdown::convert` returns a `ConversionResult` object (`$result->content`, `$result->metadata`, `$result->tables`). Conversion failures throw `HtmlToMarkdown\HtmlToMarkdownException`.

## Java

**Maven:** `dev.kreuzberg:html-to-markdown`
**Requires:** Java ≥ 25

```xml
<dependency>
    <groupId>dev.kreuzberg</groupId>
    <artifactId>html-to-markdown</artifactId>
    <version>3.6.0</version>
</dependency>
```

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.ConversionResult;

ConversionOptions options = ConversionOptions.builder()
    .headingStyle("atx")
    .build();
ConversionResult result = HtmlToMarkdown.convert("<h1>Title</h1>", options);
System.out.println(result.getContent());
```

Uses a builder for options. Conversion failures throw `dev.kreuzberg.htmltomarkdown.HtmlToMarkdownRsException` and its subtypes (`ParseErrorException`, `ConfigErrorException`, `InvalidInputException`, `PanicException`, `OtherException`). Native binaries ship in the JAR for Linux x86_64, macOS arm64/x86_64, and Windows x86_64.

## C# (.NET)

**NuGet:** `KreuzbergDev.HtmlToMarkdown`
**Requires:** .NET 10

```bash
dotnet add package KreuzbergDev.HtmlToMarkdown
```

```csharp
using HtmlToMarkdown;

var options = new ConversionOptions { HeadingStyle = "atx" };
var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1>", options);
Console.WriteLine(result.Content);
```

Classes live in the `HtmlToMarkdown` namespace (the `KreuzbergDev.HtmlToMarkdown` prefix is only the NuGet package id). Properties are `PascalCase`. Errors throw `HtmlToMarkdownRsException` and subtypes.

## Elixir

**Hex:** `html_to_markdown`
**Requires:** Elixir ≥ 1.14

```elixir
{:html_to_markdown, "~> 3.4"}
```

```elixir
case HtmlToMarkdown.convert("<h1>Title</h1>", heading_style: :atx) do
  {:ok, result} -> IO.puts(result.content)
  {:error, reason} -> IO.warn("failed: #{reason}")
end
```

`convert/2` returns `{:ok, result}` or `{:error, reason}`. Options are a keyword list. The struct fields match the Rust names (`snake_case`).

## R

**CRAN:** `htmltomarkdown`

```r
install.packages("htmltomarkdown")
```

```r
library(htmltomarkdown)

result <- htmltomarkdown::convert("<h1>Title</h1>", heading_style = "atx")
cat(result$content)
```

Options are named function arguments. The returned list matches the `ConversionResult` shape with `snake_case` names. Errors stop execution with a message; wrap in `tryCatch` if you need to handle them.

## C

**Link against:** `libhtml_to_markdown`
**Header:** `html_to_markdown.h` (HTM*H, all symbols prefixed `htm*_`/`HTM_`)

Download a pre-built release archive for your platform from the [GitHub releases page](https://github.com/kreuzberg-dev/html-to-markdown/releases), or build from source with `cargo build --release -p html-to-markdown-ffi`.

```c
#include "html_to_markdown.h"

HTMConversionResult *result = htm_convert("<h1>Title</h1>", NULL);
if (result != NULL) {
    char *content = htm_conversion_result_content(result);
    if (content != NULL) {
        printf("%s\n", content);
        htm_free_string(content);
    }
    htm_conversion_result_free(result);
}
```

Every heap-allocated string must be released with `htm_free_string` and every handle with the matching `htm_*_free` function. The C API is a thin synchronous FFI layer. No async mode, no thread-local state. Conversion errors return `NULL` and set the thread-local error accessible via `htm_last_error_code` / `htm_last_error_context`.

## WASM

**npm:** `@kreuzberg/html-to-markdown-wasm`

```bash
npm install @kreuzberg/html-to-markdown-wasm
```

```javascript
import init, { convert } from "@kreuzberg/html-to-markdown-wasm";

await init();
const result = convert("<h1>Title</h1>", { headingStyle: "atx" });
console.log(result.content);
```

`init()` loads and instantiates the `.wasm` file. Call it once before any conversion. After that, `convert` is synchronous. Options use `camelCase` and have the same shape as the TypeScript binding. The WASM build omits the `inline-images` feature (no file-system access in the browser sandbox).

--8<-- "snippets/feedback.md"
