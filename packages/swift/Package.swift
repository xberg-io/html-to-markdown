// swift-tools-version: 6.0
import PackageDescription
import Foundation

// NOTE: Run `cargo build -p html-to-markdown-rs-swift` and then rerun `alef generate`
// before `swift build`. Alef materializes the swift-bridge Swift/C outputs into
// Sources/RustBridge and Sources/RustBridgeC when the Cargo build output exists.
// See README.md for the full workflow.

// Absolute path to the Cargo target dir, resolved from this manifest's own location so
// library resolution is independent of the process working directory (`swift test` may
// chdir into fixture dirs). `#filePath` is a compile-time literal, so computing this string
// performs no filesystem access.
let rustTargetDir = (#filePath as NSString).deletingLastPathComponent.appending("/../../target")

// Resolve the static archive for a Rust crate explicitly, preferring `release` over `debug`.
// `crates/html-to-markdown-rs-swift` and the FFI crate both build `crate-type = ["cdylib", "staticlib"]`,
// so `target/{release,debug}` holds both `lib<name>.a` and `lib<name>.dylib`. A bare
// `.linkedLibrary("<name>")` / `-l<name>` lets the linker pick between them, and ld64 prefers
// the `.dylib` when both sit in the same search directory — but that dylib was built with
// `-undefined dynamic_lookup` and does not itself define the swift-bridge glue symbols (e.g.
// `__swift_bridge__$<Type>$_free`), so the link silently succeeds while the resulting binary
// fails to resolve those symbols at dlopen/runtime. Passing the archive's resolved absolute
// path forces static linking unambiguously.
func resolvedStaticLib(_ name: String) -> String {
  let release = "\(rustTargetDir)/release/lib\(name).a"
  let debug = "\(rustTargetDir)/debug/lib\(name).a"
  return FileManager.default.fileExists(atPath: release) ? release : debug
}

let package = Package(
  name: "HtmlToMarkdown",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  products: [
    .library(name: "HtmlToMarkdown", targets: ["HtmlToMarkdown"])
  ],
  targets: [
    // RustBridgeC: pure C/headers target. Swift files in RustBridge import this
    // to access C types (RustStr, etc.) produced by swift-bridge.
    // publicHeadersPath: "." exposes RustBridgeC.h to dependents.
    .target(
      name: "RustBridgeC",
      path: "Sources/RustBridgeC",
      publicHeadersPath: "."
    ),
    // RustBridge: Swift wrapper around the Rust static library.
    // Depends on RustBridgeC so the generated Swift files can use the C types.
    // linkerSettings wire the Rust staticlibs (libhtml_to_markdown_rs_swift.a and libhtml_to_markdown_ffi.a)
    // produced by `cargo build -p html-to-markdown-rs-swift` and the FFI crate so
    // `swift build` / `swift test` can resolve the `__swift_bridge__$*` and FFI C symbols.
    // Explicit absolute paths (see `resolvedStaticLib` above) are used instead of
    // `.linkedLibrary(...)` so the linker cannot substitute the sibling `.dylib` artifacts.
    // The FFI library is needed because the generated Swift service API code (App.swift)
    // calls FFI functions directly via @_silgen_name declarations.
    .target(
      name: "RustBridge",
      dependencies: ["RustBridgeC"],
      path: "Sources/RustBridge",
      // swift-bridge's generated async glue (`withCheckedThrowingContinuation` fed by a
      // non-`@Sendable` callback) does not pass Swift 6 region-isolation checking: every
      // `async fn` bridge function fails with "sending 'rustFnRetVal' risks causing data
      // races". The generated code is not ours to edit, so this target stays in Swift 5
      // language mode; the facade module keeps the manifest's default. ~keep
      swiftSettings: [.swiftLanguageMode(.v5)],
      linkerSettings: [
        .unsafeFlags([
          resolvedStaticLib("html_to_markdown_rs_swift"),
          resolvedStaticLib("html_to_markdown_ffi"),
        ]),
        // The Rust staticlib records native-library dependencies (e.g. `lzma-sys`
        // via the archive/`xz2` path emits `cargo:rustc-link-lib`) that cargo would
        // resolve when it drives the final link, but a `staticlib` `.a` does not
        // embed them and SwiftPM does not read cargo's link metadata, so undefined
        // symbols like `_lzma_stream_decoder` surface at the swift link step. Link
        // the system library here. `liblzma` ships in the macOS SDK and on Linux.
        .linkedLibrary("lzma"),
        // Same staticlib-doesn't-embed-native-deps reasoning as lzma above: the
        // bzip2 crates (archive/zip/unhwp paths) emit `-lbz2`, surfacing undefined
        // `_BZ2_bzDecompress*` at the swift link step. `libbz2` ships in the macOS
        // SDK and on Linux.
        .linkedLibrary("bz2"),
        // The Rust staticlib pulls in C++ dependencies (onnxruntime, tesseract,
        // ClipperLib) that reference the C++ runtime/ABI (`__cxa_throw`,
        // `__gxx_personality_v0`, `__cxa_guard_acquire`, ...). A `staticlib` `.a`
        // does not carry the transitive `-lc++`/`-lstdc++` system-lib dependency,
        // so SwiftPM must link the C++ standard library explicitly or the final
        // link fails with undefined symbols from those crates.
        .linkedLibrary("c++", .when(platforms: [.macOS, .iOS])),
        .linkedLibrary("stdc++", .when(platforms: [.linux])),
        .linkedFramework("Security", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("CoreFoundation", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("SystemConfiguration", .when(platforms: [.macOS])),
      ]
    ),
    .target(
      name: "HtmlToMarkdown", dependencies: ["RustBridge"],
      path: "Sources/HtmlToMarkdown",
      exclude: ["LICENSE"]),
    .testTarget(
      name: "HtmlToMarkdownTests", dependencies: ["HtmlToMarkdown"],
      path: "Tests/HtmlToMarkdownTests"),
  ]
)
