// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/kreuzberg-dev/html-to-markdown", from: "3.6.3"),
    ],
    targets: [
        .testTarget(
            name: "HtmlToMarkdownE2ETests",
            dependencies: [.product(name: "HtmlToMarkdown", package: "html-to-markdown")]
        ),
    ]
)
