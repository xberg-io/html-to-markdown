# Architecture

html-to-markdown has a Rust core with 12 language bindings on top: Python, TypeScript, Go, Ruby, PHP, Java, C#, Elixir, R, C, WASM, and a pre-built CLI. All 12 call the same `convert()` function. There is no separate conversion logic per language.

## Crate Layout

The repository root is a **[Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)**: the top-level `Cargo.toml` lists every Rust package that belongs to the project, they share one **`Cargo.lock`**, and you build or test them from the repo root with `cargo build`, `cargo test -p <crate-name>`, or `cargo build --workspace`. That keeps the core library and all first-party bindings on the same version line and guarantees they compile against each other.

Under **`crates/`** there are **eight packages** (each is its own crate with its own `Cargo.toml`):

- **Core and shipping:** `html-to-markdown` (the library published as *html-to-markdown-rs*), `html-to-markdown-cli`, and `html-to-markdown-ffi` (`libhtml_to_markdown` for Go, Java, C#, C, and similar consumers).
- **Native bindings compiled from this repo:** `html-to-markdown-py`, `html-to-markdown-node`, `html-to-markdown-php`, and `html-to-markdown-wasm` (each embeds or wraps the core differently—PyO3, NAPI-RS, ext-php-rs, wasm-bindgen).
- **Shared glue:** `html-to-markdown-bindings-common` holds option marshalling and other bits reused by those bindings so they do not duplicate the same types.

The workspace also includes **`tools/`** crates (for example snippet generation and checks); they are not part of the public layout below but are built with the same workspace commands.

```
crates/
├── html-to-markdown/          # Core library (html-to-markdown-rs on crates.io)
├── html-to-markdown-cli/      # CLI binary
├── html-to-markdown-ffi/      # C FFI shared library (libhtml_to_markdown)
├── html-to-markdown-node/     # Node.js binding (NAPI-RS)
├── html-to-markdown-py/       # Python binding (PyO3)
├── html-to-markdown-php/      # PHP extension (ext-php-rs)
├── html-to-markdown-wasm/     # WebAssembly (wasm-bindgen)
└── html-to-markdown-bindings-common/  # Shared option marshalling
```

Go, Ruby, Java, C#, Elixir, and R all live under `packages/` and call into `libhtml_to_markdown` (the C FFI crate) via their language's native FFI mechanism.

## Core Library

`crates/html-to-markdown/` is the only place where conversion logic lives. Its public API is one function:

```rust
pub fn convert(html: &str, options: Option<ConversionOptions>) -> Result<ConversionResult>
```

Internally, the conversion happens in five phases:

1. **Preprocessing.** Scripts and styles are stripped with a fast byte-scanner pass. If `preprocess` is true, navigation elements are also removed before parsing.
2. **Parsing.** The HTML is parsed by [astral-tl](https://crates.io/crates/astral-tl) (`tl` in the Cargo manifest), a high-performance single-pass HTML parser that targets modern HTML5.
3. **DOM walk.** The library traverses the parse tree in pre-order. For every node it calls element-handler functions, accumulating Markdown into a string buffer. If a visitor is registered, each handler calls the corresponding `HtmlVisitor` method and uses the returned `VisitResult` to either proceed, substitute custom output, skip, or abort.
4. **Post-processing.** Whitespace normalization, trailing newlines, reference-style link definitions, and Markdown dialect adjustments (Djot or plain text) are applied to the accumulated output.
5. **Extraction.** Tables, inline images, document structure, and metadata are collected during the walk and assembled into `ConversionResult`.

## Binding Mechanisms

Each language reaches Rust via a different mechanism. All paths fall into one of two categories: direct embedding (the Rust code compiles into the host language's native extension module) or the C FFI shared library (the host language loads `libhtml_to_markdown` at runtime).

### Direct embedding

| Language | Mechanism | Crate |
|----------|-----------|-------|
| Python | [PyO3](https://pyo3.rs) extension module | `html-to-markdown-py` |
| TypeScript / Node.js | [NAPI-RS](https://napi.rs) Node-API addon | `html-to-markdown-node` |
| PHP | [ext-php-rs](https://davidcole1340.github.io/ext-php-rs/) PHP extension | `html-to-markdown-php` |
| Ruby | [Magnus](https://github.com/matsadler/magnus) gem extension | `packages/ruby` |
| Elixir | [Rustler](https://github.com/rusterlium/rustler) NIF | `packages/elixir` |
| R | [extendr](https://extendr.github.io) R extension | `packages/r` |
| WebAssembly | [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) | `html-to-markdown-wasm` |

For these, the conversion function is compiled directly into the native extension. The host language never sees the C ABI.

### Via libhtml_to_markdown

| Language | Mechanism | Notes |
|----------|-----------|-------|
| Go | cgo | Bundles `libhtml_to_markdown.a` in the Go module. |
| Java | Foreign Function & Memory API (Java 22+) | Extracts the shared library from the JAR at startup. |
| C# | P/Invoke (`DllImport`) | Loads `libhtml_to_markdown` via `NativeLibrary`. |
| C | Direct link | Consumer links against `html-to-markdown-ffi`. |

The C FFI crate exposes a minimal, null-safe C API and catches any Rust panics at the boundary, converting them to `ConversionError::Panic` rather than letting them unwind across the ABI.

## Feature Flags

The core library compiles with six Cargo features. Bindings enable the subset they expose.

| Feature | What it gates |
|---------|---------------|
| `metadata` | All metadata extraction code and `HtmlMetadata` types (on by default) |
| `visitor` | `HtmlVisitor` trait and `ConversionError::Visitor` |
| `inline-images` | Data-URI decoder and inline SVG extractor |
| `serde` | `Serialize`/`Deserialize` on option and result types |
| `full` | All four optional features combined |

The WASM build omits `inline-images` (no filesystem, no image decoder in the browser sandbox).

## Thread Safety and Allocation

The core library is stateless and does not use global state. `convert()` is safe to call from multiple threads simultaneously. Each call allocates and frees its own buffers independently.

Bindings that cross the C FFI boundary use arena-managed allocation: the caller passes a pointer and the library writes into caller-controlled memory, minimising back-and-forth across the ABI. Memory returned across the FFI must be freed with `html_to_markdown_free_result`; the Go and Java bindings handle this internally.

## HTML Parser

[astral-tl](https://crates.io/crates/astral-tl) is a fork of the `tl` crate maintained by the Kreuzberg team. It is a streaming HTML tokenizer and DOM builder optimised for correctness on real-world HTML. It handles malformed HTML by following browser-compatible recovery rules, so conversion degrades gracefully on broken markup rather than aborting.

The parser produces an in-memory tree. The library does not stream: the entire HTML is parsed before the DOM walk starts. For very large documents (multi-MB) this is the dominant memory cost.

## Single-Pass Traversal

The DOM walk is pre-order and single-pass. Metadata collection, visitor dispatch, table extraction, and Markdown serialisation all happen in the same traversal. There is no separate analysis phase and no second pass over the tree.

This design keeps memory low (the output buffer grows steadily; there is no intermediate AST) and makes adding a new output format a matter of adding a new handler, not restructuring the pipeline.

--8<-- "snippets/feedback.md"
