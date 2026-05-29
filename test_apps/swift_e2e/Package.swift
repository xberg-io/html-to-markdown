// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .binaryTarget(name: "HtmlToMarkdown", url: "https://github.com/kreuzberg-dev/html-to-markdown/releases/download/v3.5.7/HtmlToMarkdown-rs.artifactbundle.zip", checksum: "__ALEF_SWIFT_CHECKSUM__"),
    ],
    targets: [
        .testTarget(
            name: "HtmlToMarkdownE2ETests",
            dependencies: [.binaryTarget(name: "HtmlToMarkdown")]
        ),
    ]
)
