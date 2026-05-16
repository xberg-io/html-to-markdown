# Building HtmlToMarkdown

The Swift package wraps a Rust library via [swift-bridge](https://github.com/chinedufn/swift-bridge).
SwiftPM cannot invoke Cargo directly, so you must run the cargo build step first.

## Workflow

### 1. Build the Rust binding crate

From the **repository root**:

```sh
cargo build -p html-to-markdown-rs-swift
```

This compiles `target/debug/libhtml_to_markdown_rs_swift.a` and runs
`swift-bridge-build` in `build.rs`, which writes generated Swift and C sources
into `target/debug/build/html-to-markdown-rs-swift-*/out/`.

### 2. Copy generated sources into the SwiftPM targets

The package uses two internal targets:

- `Sources/RustBridgeC/` — pure C target with the combined C header
- `Sources/RustBridge/` — Swift bridge files that `import RustBridgeC`

```sh
OUT=$(ls -dt target/debug/build/html-to-markdown-rs-swift-*/out 2>/dev/null | head -1)

# Combine C headers into the RustBridgeC target
cat "$OUT/SwiftBridgeCore.h" "$OUT/html-to-markdown-rs-swift/html-to-markdown-rs-swift.h" \
    > packages/swift/Sources/RustBridgeC/RustBridgeC.h

# Copy Swift bridge files, prepending "import RustBridgeC" so they see the C types.
# Use `{ echo ...; cat ...; }` rather than `printf "...$(cat)..."` because printf
# interprets `%` and `\` sequences in its format string, which would corrupt the
# generated Swift sources.
{ echo "import RustBridgeC"; cat "$OUT/SwiftBridgeCore.swift"; } \
    > packages/swift/Sources/RustBridge/SwiftBridgeCore.swift
{ echo "import RustBridgeC"; cat "$OUT/html-to-markdown-rs-swift/html-to-markdown-rs-swift.swift"; } \
    > packages/swift/Sources/RustBridge/html-to-markdown-rs-swift.swift
```

If the glob `html-to-markdown-rs-swift-*/out` matches multiple directories, `ls -dt ... | head -1`
picks the most recently modified one.

### 3. Build and test the Swift package

```sh
swift build --package-path packages/swift
swift test --package-path packages/swift
```

## Release builds

Replace `target/debug` with `target/release` and pass
`--configuration release` to `swift build`:

```sh
cargo build --release -p html-to-markdown-rs-swift
OUT=$(ls -dt target/release/build/html-to-markdown-rs-swift-*/out 2>/dev/null | head -1)

cat "$OUT/SwiftBridgeCore.h" "$OUT/html-to-markdown-rs-swift/html-to-markdown-rs-swift.h" \
    > packages/swift/Sources/RustBridgeC/RustBridgeC.h
{ echo "import RustBridgeC"; cat "$OUT/SwiftBridgeCore.swift"; } \
    > packages/swift/Sources/RustBridge/SwiftBridgeCore.swift
{ echo "import RustBridgeC"; cat "$OUT/html-to-markdown-rs-swift/html-to-markdown-rs-swift.swift"; } \
    > packages/swift/Sources/RustBridge/html-to-markdown-rs-swift.swift

swift build --package-path packages/swift --configuration release
```

## Notes

- Files in `Sources/RustBridgeC/` and the generated Swift files in
  `Sources/RustBridge/` are **generated artifacts** — overwritten by the copy step.
- `Sources/RustBridge/RustBridge.swift` is a placeholder and is overwritten.
- `target/` is in `.gitignore`; regenerate after every `cargo clean`.
