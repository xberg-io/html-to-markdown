# Installation

Install **html-to-markdown** for your language using the commands below. Each binding wraps the same Rust core, so behavior matches across runtimes. Pick a tab under **Language bindings** for copy-paste snippets, feature flags, and a quick verification step where applicable.

## Requirements

| Language             | Min version       | Package                                                    |
| -------------------- | ----------------- | ---------------------------------------------------------- |
| Rust                 | 1.85              | `html-to-markdown-rs` on crates.io                         |
| Python               | 3.10              | `html-to-markdown` on PyPI                                 |
| TypeScript / Node.js | Node.js 18        | `@kreuzberg/html-to-markdown` on npm                       |
| Go                   | 1.26              | `github.com/kreuzberg-dev/html-to-markdown/packages/go/v3` |
| Ruby                 | 3.2               | `html-to-markdown` on RubyGems                             |
| PHP                  | 8.2               | `kreuzberg-dev/html-to-markdown` on Packagist              |
| Java                 | 25                | `dev.kreuzberg:html-to-markdown` on Maven Central          |
| C#                   | .NET 10           | `KreuzbergDev.HtmlToMarkdown` on NuGet                     |
| Elixir               | 1.14              | `html_to_markdown` on Hex                                  |
| R                    | 4.2               | `htmltomarkdown` on CRAN                                   |
| C                    | —                 | `libhtml_to_markdown` (GitHub Releases)                    |
| WebAssembly          | —                 | `@kreuzberg/html-to-markdown-wasm` on npm                  |
| Swift                | Swift 6.0         | `HtmlToMarkdown` SwiftPM package                           |
| Dart                 | 3.11              | `h2m` on pub.dev                                           |
| Kotlin Android       | Android minSdk 21 | `dev.kreuzberg:html-to-markdown-android` on Maven Central  |
| Zig                  | 0.16              | `html_to_markdown_rs` Zig package                          |

---

## Language bindings

=== "Rust"

    Add to `Cargo.toml`:

    ```toml
    [dependencies]
    html-to-markdown-rs = "3.6"
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

    | Feature | Enables | Default | Availability |
    |---------|---------|---------|--------------|
    | `metadata` | `HtmlMetadata` extraction and all `extract_*` options | yes | <span class="version-badge">Available by v3.6</span> |
    | `visitor` | `HtmlVisitor` trait and `ConversionError::Visitor` | no | <span class="version-badge">Available by v3.6</span> |
    | `inline-images` | data-URI decoding and inline SVG extraction | no | <span class="version-badge">Available by v3.6</span> |
    | `serde` | `Serialize`/`Deserialize` on all public option and result types | no | <span class="version-badge">Available by v3.6</span> |
    | `full` | all of the above | no | <span class="version-badge">Available by v3.6</span> |

    ```toml
    # With visitor support
    html-to-markdown-rs = { version = "3.6", features = ["visitor"] }

    # Everything
    html-to-markdown-rs = { version = "3.6", features = ["full"] }
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
    python -c "from html_to_markdown import convert; print(convert('<h1>Hello</h1>').content)"
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
    gem 'html-to-markdown', '~> 3.6'
    ```

    **Verify:**

    ```bash
    ruby -e "require 'html_to_markdown'; puts HtmlToMarkdown.convert('<h1>Hello</h1>')[:content]"
    # # Hello
    ```

    Requires Ruby 3.2+. Precompiled native gems are available for Linux and macOS. On unsupported platforms `bundle install` will compile the extension from source, which requires a Rust toolchain.

=== "PHP"

    This is a native PHP extension (Rust `ext-php-rs`), installed with [PIE](https://github.com/php/pie) — not `composer require` (Composer cannot load a native extension):

    ```bash
    pie install kreuzberg-dev/html-to-markdown
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

    Requires PHP 8.2+. PIE downloads the prebuilt extension binary matching your PHP version and platform from the GitHub release (PHP 8.2–8.5 on Linux x86_64/arm64, macOS arm64, and Windows x86_64).

=== "Java"

    **Maven** — add to `pom.xml`:

    ```xml
    <dependency>
        <groupId>dev.kreuzberg</groupId>
        <artifactId>html-to-markdown</artifactId>
        <version>3.6.8</version>
    </dependency>
    ```

    **Gradle** — add to `build.gradle`:

    ```groovy
    implementation 'dev.kreuzberg:html-to-markdown:3.6.8'
    ```

    Or Kotlin DSL (`build.gradle.kts`):

    ```kotlin
    implementation("dev.kreuzberg:html-to-markdown:3.6.8")
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

    Requires Java 25+. The JAR extracts the native `libhtml_to_markdown` shared library at startup. No separate install step is needed — the library is bundled for Linux x86_64, macOS arm64/x86_64, and Windows x64.

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

    Targets .NET 10. The package bundles native binaries for Linux, macOS, and Windows via `NativeLibrary` P/Invoke.

=== "Elixir"

    Add to `mix.exs`:

    ```elixir
    def deps do
      [
        {:html_to_markdown, "~> 3.6"}
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

    Requires Elixir 1.14+. Uses Rustler NIFs — precompiled NIF binaries are fetched automatically via `RustlerPrecompiled`. Set `RUSTLER_PRECOMPILED_GLOBAL_CACHE_PATH` to control cache location.

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

    Requires R 4.2+. Available on CRAN for Linux (x86_64), macOS (arm64, x86_64), and Windows.

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
        HTMConversionResult *result = htm_convert("<h1>Hello</h1>", NULL);
        if (result == NULL) {
            fprintf(stderr, "convert failed: %s\n", htm_last_error_context());
            return 1;
        }
        char *content = htm_conversion_result_content(result);
        if (content != NULL) {
            printf("%s\n", content);
            htm_free_string(content);
        }
        htm_conversion_result_free(result);
        return 0;
    }
    ```

    Compile with:

    ```bash
    gcc main.c -lhtml_to_markdown -L./target/release -o main
    ```

    Every heap-allocated string returned across the FFI must be released with `htm_free_string`, and every result/options handle must be released with the matching `htm_*_free` function. The memory belongs to the Rust allocator — do not call `free()` directly.

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

    `init()` must complete before any `convert()` call. After that, `convert` is synchronous. The WASM build exposes inline image extraction when built with `inline-images`; generated native bindings may omit the Rust-only image payload. Works in browsers, Cloudflare Workers, Deno, and Bun.

=== "Swift"

    Add the package URL in SwiftPM:

    ```text
    https://github.com/kreuzberg-dev/html-to-markdown
    ```

    **Verify:**

    ```swift
    import HtmlToMarkdown

    let result = try convert("<h1>Hello</h1>", nil)
    print(result.content()?.toString() ?? "")
    ```

    Requires Swift 6.0+. The package currently targets macOS 13+ and iOS 16+.

=== "Dart"

    ```bash
    dart pub add h2m
    ```

    **Verify:**

    ```dart
    import 'package:h2m/h2m.dart';
    import 'package:h2m/src/html_to_markdown_rs_bridge_generated/frb_generated.dart'
        show RustLib;

    await RustLib.init();
    final result = await H2mBridge.convert('<h1>Hello</h1>');
    print(result.content);
    ```

    Requires Dart 3.11+. The generated package is pure Dart from the caller's point of view and uses flutter_rust_bridge under the hood.

=== "Kotlin Android"

    Add the Android artifact:

    ```kotlin
    implementation("dev.kreuzberg:html-to-markdown-android:3.6.8")
    ```

    Requires Android minSdk 21. This is an Android AAR with bundled JNI libraries; Kotlin/JVM users should use the Java artifact (`dev.kreuzberg:html-to-markdown`) directly.

=== "Zig"

    Use the Zig package `html_to_markdown_rs` from the release artifacts or the repository package metadata.

    ```zig
    const html_to_markdown = @import("html_to_markdown");
    ```

    Requires Zig 0.16+. The package wraps the generated C FFI library.

---

## Troubleshooting on Windows

If `pip install html-to-markdown` fails on Windows 11 with Python 3.14 or later, try these steps:

1. **Force the prebuilt wheel:**

   ```bash
   pip install --only-binary=:all: html-to-markdown
   ```

   This prevents fallback to source build. If this fails, a wheel resolution issue exists — try updating `pip` or `uv`.

2. **Enable MSVC compiler (if sdist build is required):** Install Microsoft Visual Studio Build Tools with the "Desktop development with C++" workload. Then ensure MSVC's `link.exe` precedes any GNU variants in `PATH`. Check with `where link` — the first result should be a path under `VC\Tools\MSVC`.

3. **Update pip or uv:** Python 3.14 wheel tags are recent. Run `pip install --upgrade pip` or `uv self update` to ensure correct wheel matching.

---

## CLI

Install via Cargo:

```bash
cargo install html-to-markdown-cli
```

Or via Homebrew:

Homebrew 6.0+ requires explicit trust for third-party taps. Trust once, then install:

```bash
brew trust kreuzberg-dev/tap
brew install kreuzberg-dev/tap/html-to-markdown
```

**Verify:**

```bash
echo "<h1>Hello</h1>" | html-to-markdown
# # Hello
```

See [CLI reference](cli.md) for all flags and options.

--8<-- "snippets/feedback.md"
