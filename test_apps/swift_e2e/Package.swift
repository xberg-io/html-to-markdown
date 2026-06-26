// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/xberg-io/html-to-markdown", branch: "release/swift/3.7.2"),
    ],
    targets: [
        .testTarget(
            name: "HtmlToMarkdownE2ETests",
            dependencies: [.product(name: "HtmlToMarkdown", package: "html-to-markdown")]
        ),
    ]
)
