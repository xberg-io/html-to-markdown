#!/bin/bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SWIFT_PKG="$REPO_ROOT/packages/swift"
SWIFT_E2E="$(cd "$(dirname "$0")" && pwd)"

echo "Building Rust binding..."
cargo build -p html-to-markdown-rs-swift --release || exit 1

echo "Copying swift-bridge output to packages/swift..."
OUT=$(ls -dt target/release/build/html-to-markdown-rs-swift-*/out 2>/dev/null | head -1)
if [ -z "$OUT" ]; then
    echo "Error: Could not find swift-bridge output directory"
    exit 1
fi

# Copy headers
mkdir -p "$SWIFT_PKG/Sources/RustBridgeC"
cat "$OUT/SwiftBridgeCore.h" "$OUT/html-to-markdown-rs-swift/html-to-markdown-rs-swift.h" \
    > "$SWIFT_PKG/Sources/RustBridgeC/RustBridgeC.h"

# Copy Swift files
mkdir -p "$SWIFT_PKG/Sources/RustBridge"
{ echo "import RustBridgeC"; cat "$OUT/SwiftBridgeCore.swift"; } \
    > "$SWIFT_PKG/Sources/RustBridge/SwiftBridgeCore.swift"
{ echo "import RustBridgeC"; cat "$OUT/html-to-markdown-rs-swift/html-to-markdown-rs-swift.swift"; } \
    > "$SWIFT_PKG/Sources/RustBridge/html-to-markdown-rs-swift.swift"

echo "Running swift test in $SWIFT_E2E..."
cd "$SWIFT_E2E"
export LD_LIBRARY_PATH="$REPO_ROOT/target/release:$LD_LIBRARY_PATH"
swift test
