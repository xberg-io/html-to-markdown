#!/usr/bin/env bash
# alef-generated installer for registry-mode PHP test_app.
# Installs the kreuzberg-dev/html-to-markdown extension via PIE before `composer install` runs.
# Requires `php` on PATH; downloads and runs PIE if needed.
# Version is alef-injected at generate time so the script is self-contained.
set -euo pipefail

# Version override: pass as $1 to test an arbitrary tag; defaults to the
# alef-pinned version from `[crates.e2e.registry.packages.php].version`.
VERSION="${1:-3.6.0-rc.23}"

# PIE >= 1.3.7 supports the array-form `php-ext.download-url-method`
# our composer.json emits; 1.4.0+ is preferred. Download PIE if we don't
# already have a recent enough version.
need_pie_install=true
if command -v pie >/dev/null 2>&1; then
  current="$(pie --version 2>&1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1 || echo '0.0.0')"
  if printf '%s\n%s\n' "1.3.7" "$current" | sort -V -C; then
    need_pie_install=false
  fi
fi
if [[ "$need_pie_install" == "true" ]]; then
  # Download PIE PHAR from latest GitHub release if not already installed.
  pie_dir="${HOME}/.local/bin"
  mkdir -p "$pie_dir"
  curl -fL --output "$pie_dir/pie" "https://github.com/php/pie/releases/latest/download/pie.phar" 2>/dev/null || {
    echo "::error::Failed to download PIE from GitHub; ensure network access or pre-install PIE." >&2
    exit 1
  }
  chmod +x "$pie_dir/pie"
  PIE="$pie_dir/pie"
  # Ensure newly downloaded PIE is on PATH for this script.
  export PATH="$pie_dir:$PATH"
else
  PIE="pie"
fi

# Install the extension binary into the running PHP's extension dir.
# Always run PIE — an existence-only skip leaves a stale .so from a prior rc
# (different ABI / missing symbols) in $EXT_DIR, which then fails the verification
# step below. PIE itself is idempotent: re-installing overwrites the existing
# binary cleanly. The php.ini-append guard below prevents duplicate `extension=`
# lines so the verification step doesn't trip on "Module already loaded".
EXT_DIR="$(php -r 'echo ini_get("extension_dir");')"
"$PIE" install "kreuzberg-dev/html-to-markdown:$VERSION" --skip-enable-extension

# Verify the .so/.dylib/.dll exists after install (or was already present).
test -f "$EXT_DIR/html_to_markdown.so" || test -f "$EXT_DIR/html_to_markdown.dylib" || test -f "$EXT_DIR/html_to_markdown.dll"

# Enable the extension in php.ini (PIE with --skip-enable-extension doesn't do this automatically).
# Find the loaded php.ini, check if already enabled, and append if missing.
PHP_INI="$(php --ini 2>&1 | grep -m1 'Loaded Configuration File:' | awk '{print $NF}')"
if [[ -z "$PHP_INI" ]]; then
  echo "::warning::Could not locate php.ini; extension may not be auto-loaded by default" >&2
else
  if [[ ! -f "$PHP_INI" ]]; then
    echo "::warning::php.ini at $PHP_INI not found; extension may not be auto-loaded by default" >&2
  else
    # Guard against duplicate: check if extension line already exists (uncommented).
    if ! grep -q "^extension=html_to_markdown" "$PHP_INI"; then
      echo "extension=html_to_markdown" >>"$PHP_INI"
    fi
  fi
fi

# Export the installed extension path for downstream test runners (composer test).
# The test app's run_tests.php checks for PIE_INSTALLED_EXTENSION_PATH and loads the extension via `-d`.
export PIE_INSTALLED_EXTENSION_PATH="$EXT_DIR/html_to_markdown.so"
if [[ "$OSTYPE" == "darwin"* ]]; then
  export PIE_INSTALLED_EXTENSION_PATH="$EXT_DIR/html_to_markdown.dylib"
fi

# Verify the extension loads. Use `extension_loaded()` via `php -r` instead of
# parsing `php -m` output: `php -m` is fragile when an extension is enabled via
# both php.ini *and* a conf.d drop-in (e.g. when a prior PIE install left a
# conf.d entry behind), because PHP prints "Module ... is already loaded" to
# stderr and the test harness 2>&1 capture treats it as fatal. `extension_loaded`
# checks runtime state directly and is unaffected by load source or stderr noise.
if php -r 'exit(extension_loaded("html_to_markdown") ? 0 : 1);' 2>/dev/null; then
  echo "html_to_markdown extension loaded via php.ini"
elif php -d extension=html_to_markdown -r 'exit(extension_loaded("html_to_markdown") ? 0 : 1);' 2>/dev/null; then
  echo "html_to_markdown extension loaded via -d flag"
else
  echo "::error::html_to_markdown extension failed to load after PIE install" >&2
  exit 1
fi
echo "html_to_markdown extension installed and loaded"
