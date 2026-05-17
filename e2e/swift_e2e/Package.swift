// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(path: "../../packages/swift"),
    ],
    targets: [
        .testTarget(
            name: "HtmlToMarkdownE2ETests",
            dependencies: [.product(name: "HtmlToMarkdown", package: "swift")]
        ),
    ]
)
