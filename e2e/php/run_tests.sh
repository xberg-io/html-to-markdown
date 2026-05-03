#!/usr/bin/env bash
set -e

# Determine the extension path based on platform
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${SCRIPT_DIR}/../../"
TARGET_RELEASE="${REPO_ROOT}/target/release"

# macOS produces .dylib, Linux produces .so, Windows produces .dll
if [[ "$OSTYPE" == "darwin"* ]]; then
    EXTENSION_PATH="${TARGET_RELEASE}/libhtml_to_markdown_php.dylib"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    EXTENSION_PATH="${TARGET_RELEASE}/libhtml_to_markdown_php.dll"
else
    EXTENSION_PATH="${TARGET_RELEASE}/libhtml_to_markdown_php.so"
fi

# Run phpunit with the extension loaded
export PHP_EXTRA_CONFIGURE_ARGS=""
php -d "extension=${EXTENSION_PATH}" ./vendor/bin/phpunit "$@"
