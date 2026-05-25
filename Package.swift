// swift-tools-version: 6.0
// Root-level Package.swift shim — alef-generated for SwiftPM URL consumers.
// The canonical manifest lives at packages/swift/Package.swift; see that file
// (and packages/swift/README.md) for the in-tree development workflow.
import PackageDescription

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
    .target(
      name: "RustBridgeC",
      path: "packages/swift/Sources/RustBridgeC",
      publicHeadersPath: "."
    ),
    .target(
      name: "RustBridge",
      dependencies: ["RustBridgeC"],
      path: "packages/swift/Sources/RustBridge",
      linkerSettings: [
        .unsafeFlags([
          "-Ltarget/release",
          "-Ltarget/debug",
        ]),
        .linkedLibrary("html_to_markdown_rs_swift"),
        .linkedFramework("Security", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("CoreFoundation", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("SystemConfiguration", .when(platforms: [.macOS])),
      ]
    ),
    .target(name: "HtmlToMarkdown", dependencies: ["RustBridge"], path: "packages/swift/Sources/HtmlToMarkdown"),
  ]
)
