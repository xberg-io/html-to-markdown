# Installation
Install **html-to-markdown** for your language using the commands below. Each binding wraps the same Rust core, so behavior matches across runtimes. Pick a tab under **Language bindings** for copy-paste snippets, feature flags, and a quick verification step where applicable.

## Requirements

| Language | Min version | Package |
|----------|-------------|---------|
| Rust | 1.70 | `html-to-markdown-rs` on crates.io |
| Python | 3.10 | `html-to-markdown` on PyPI |
| TypeScript / Node.js | Node.js 18 | `@kreuzberg/html-to-markdown` on npm |
| Go | 1.26 | `github.com/kreuzberg-dev/html-to-markdown/packages/go/v3` |
| Ruby | 3.2 | `html-to-markdown` on RubyGems |
| PHP | 8.1 | `kreuzberg/html-to-markdown` on Packagist |
| Java | 22 | `dev.kreuzberg:html-to-markdown` on Maven Central |
| C# | .NET Standard 2.0 | `KreuzbergDev.HtmlToMarkdown` on NuGet |
| Elixir | 1.19 | `html_to_markdown` on Hex |
| R | 4.0 | `htmltomarkdown` on CRAN |
| C | — | `libhtml_to_markdown` (GitHub Releases) |
| WebAssembly | — | `@kreuzberg/html-to-markdown-wasm` on npm |

---

## Language bindings

=== "Rust"

    Add to `Cargo.toml`:

    ```toml
    [dependencies]
    html-to-markdown-rs = "3.1"
    ```

    Or via `cargo add`:

    ```bash
    cargo add html-to-markdown-rs
    ```

    **Verify:**

    ```rust
    use html_to_markdown_rs::convert;

    fn main() {
        let result = convert("<h1>Hello</h1>", None).unwrap();
        println!("{}", result.content.unwrap());
        // # Hello
    }
    ```

    ### Feature flags

    By default only the `metadata` feature is enabled. Opt in to additional features as needed:

    | Feature | Enables | Default |
    |---------|---------|---------|
    | `metadata` | `HtmlMetadata` extraction and all `extract_*` options | yes |
    | `visitor` | `HtmlVisitor` trait and `ConversionError::Visitor` | no |
    | `inline-images` | data-URI decoding and inline SVG extraction | no |
    | `serde` | `Serialize`/`Deserialize` on all public option and result types | no |
    | `full` | all of the above | no |

    ```toml
    # With visitor support
    html-to-markdown-rs = { version = "3.1", features = ["visitor"] }

    # Everything
    html-to-markdown-rs = { version = "3.1", features = ["full"] }
    ```

=== "Python"

    ```bash
    pip install html-to-markdown
    ```

    Using `uv`:

    ```bash
    uv add html-to-markdown
    ```

    **Verify:**

    ```bash
    python -c "from html_to_markdown import convert; print(convert('<h1>Hello</h1>')['content'])"
    # # Hello
    ```

    Requires Python 3.10+. The package ships precompiled wheels for Linux (x86_64, aarch64), macOS (Apple Silicon, x86_64), and Windows (x64). A source distribution is also available if no wheel matches your platform.

=== "TypeScript"

    ```bash
    npm install @kreuzberg/html-to-markdown
    ```

    Using `pnpm` or `yarn`:

    ```bash
    pnpm add @kreuzberg/html-to-markdown
    yarn add @kreuzberg/html-to-markdown
    ```

    **Verify:**

    ```typescript
    import { convert } from '@kreuzberg/html-to-markdown';

    const result = convert('<h1>Hello</h1>');
    console.log(result.content);
    // # Hello
    ```

    Requires Node.js 18+. The package ships both ESM and CJS outputs with bundled TypeScript types. No separate `@types/` package needed.

=== "Go"

    ```bash
    go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v3
    ```

    **Verify:**

    ```go
    package main

    import (
        "fmt"
        htmltomarkdown "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3"
    )

    func main() {
        result, err := htmltomarkdown.Convert("<h1>Hello</h1>", nil)
        if err != nil {
            panic(err)
        }
        fmt.Println(*result.Content)
        // # Hello
    }
    ```

    Requires Go 1.26+. The module bundles a precompiled `libhtml_to_markdown.a` static library for each supported platform — no separate Rust toolchain needed.

=== "Ruby"

    ```bash
    gem install html-to-markdown
    ```

    Or add to your `Gemfile`:

    ```ruby
    gem 'html-to-markdown', '~> 3.1'
    ```

    **Verify:**

    ```bash
    ruby -e "require 'html_to_markdown'; puts HtmlToMarkdown.convert('<h1>Hello</h1>')[:content]"
    # # Hello
    ```

    Requires Ruby 3.2+. Precompiled native gems are available for Linux and macOS. On unsupported platforms `bundle install` will compile the extension from source, which requires a Rust toolchain.

=== "PHP"

    ```bash
    composer require kreuzberg/html-to-markdown
    ```

    **Verify:**

    ```php
    <?php
    require 'vendor/autoload.php';

    use function HtmlToMarkdown\convert;

    $result = convert('<h1>Hello</h1>');
    echo $result['content'];
    // # Hello
    ```

    Requires PHP 8.1+. The package ships precompiled extensions for common PHP versions. If no prebuilt extension matches, Composer will compile from source via `cargo`.

=== "Java"

    **Maven** — add to `pom.xml`:

    ```xml
    <dependency>
        <groupId>dev.kreuzberg</groupId>
        <artifactId>html-to-markdown</artifactId>
        <version>3.1.0</version>
    </dependency>
    ```

    **Gradle** — add to `build.gradle`:

    ```groovy
    implementation 'dev.kreuzberg:html-to-markdown:3.1.0'
    ```

    Or Kotlin DSL (`build.gradle.kts`):

    ```kotlin
    implementation("dev.kreuzberg:html-to-markdown:3.1.0")
    ```

    **Verify:**

    ```java
    import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;

    public class Main {
        public static void main(String[] args) {
            var result = HtmlToMarkdown.convert("<h1>Hello</h1>");
            System.out.println(result.content());
            // # Hello
        }
    }
    ```

    Requires Java 22+. The JAR extracts the native `libhtml_to_markdown` shared library at startup. No separate install step is needed — the library is bundled for Linux x86_64, macOS arm64/x86_64, and Windows x64.

=== "C#"

    ```bash
    dotnet add package KreuzbergDev.HtmlToMarkdown
    ```

    Or via the NuGet Package Manager in Visual Studio — search for `KreuzbergDev.HtmlToMarkdown`.

    **Verify:**

    ```csharp
    using HtmlToMarkdown;

    var result = HtmlToMarkdownConverter.Convert("<h1>Hello</h1>");
    Console.WriteLine(result.Content);
    // # Hello
    ```

    Targets .NET Standard 2.0 and above (.NET 6+, .NET Framework 4.6.1+). The package bundles native binaries for Linux, macOS, and Windows via `NativeLibrary` P/Invoke.

=== "Elixir"

    Add to `mix.exs`:

    ```elixir
    def deps do
      [
        {:html_to_markdown, "~> 3.1"}
      ]
    end
    ```

    Then fetch:

    ```bash
    mix deps.get
    ```

    **Verify:**

    ```elixir
    {:ok, result} = HtmlToMarkdown.convert("<h1>Hello</h1>")
    IO.puts(result.content)
    # # Hello
    ```

    Requires Elixir 1.19+. Uses Rustler NIFs — precompiled NIF binaries are fetched automatically via `RustlerPrecompiled`. Set `RUSTLER_PRECOMPILED_GLOBAL_CACHE_PATH` to control cache location.

=== "R"

    ```r
    install.packages("htmltomarkdown")
    ```

    **Verify:**

    ```r
    library(htmltomarkdown)
    result <- htmltomarkdown::convert("<h1>Hello</h1>")
    cat(result$content)
    # # Hello
    ```

    Requires R 4.0+. Available on CRAN for Linux (x86_64), macOS (arm64, x86_64), and Windows.

=== "C"

    Download a prebuilt release archive for your platform from the [GitHub Releases page](https://github.com/kreuzberg-dev/html-to-markdown/releases). Each archive contains:

    - `libhtml_to_markdown.so` / `.dylib` / `.dll` — shared library
    - `libhtml_to_markdown.a` — static library
    - `html_to_markdown.h` — C header

    **Build from source** (requires Rust toolchain):

    ```bash
    git clone https://github.com/kreuzberg-dev/html-to-markdown.git
    cd html-to-markdown
    cargo build --release -p html-to-markdown-ffi
    # output: target/release/libhtml_to_markdown.{so,dylib,dll}
    ```

    **Link and verify:**

    ```c
    #include "html_to_markdown.h"
    #include <stdio.h>

    int main(void) {
        HtmlToMarkdownResult r = html_to_markdown_convert("<h1>Hello</h1>", NULL);
        if (r.error == NULL) {
            printf("%s\n", r.content);
            // # Hello
        }
        html_to_markdown_free_result(r);
        return 0;
    }
    ```

    Compile with:

    ```bash
    gcc main.c -lhtml_to_markdown -L./target/release -o main
    ```

    Always call `html_to_markdown_free_result` after every call. Memory returned across the FFI boundary is owned by the Rust allocator and must not be freed with `free()`.

=== "WASM"

    ```bash
    npm install @kreuzberg/html-to-markdown-wasm
    ```

    **Verify:**

    ```javascript
    import init, { convert } from '@kreuzberg/html-to-markdown-wasm';

    await init();  // load and instantiate the .wasm file — call once

    const result = convert('<h1>Hello</h1>');
    console.log(result.content);
    // # Hello
    ```

    `init()` must complete before any `convert()` call. After that, `convert` is synchronous. The WASM build omits the `inline-images` feature (no filesystem access in the browser sandbox). Works in browsers, Cloudflare Workers, Deno, and Bun.

---

## CLI

Install via Cargo:

```bash
cargo install html-to-markdown-cli
```

Or via Homebrew:

```bash
brew install kreuzberg-dev/tap/html-to-markdown
```

**Verify:**

```bash
echo "<h1>Hello</h1>" | html-to-markdown
# # Hello
```

See [CLI reference](cli.md) for all flags and options.

--8<-- "snippets/feedback.md"
