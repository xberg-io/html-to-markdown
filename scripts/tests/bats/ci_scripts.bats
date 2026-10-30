#!/usr/bin/env bats

setup() {
  bats_load_library xberg-bats
  xberg_setup
  REPO_ROOT="$(cd "$BATS_TEST_DIRNAME/../../.." && pwd -P)"
  mkdir -p "$XBERG_WORK/packages/ruby" "$XBERG_WORK/packages/r"
  cd "$XBERG_WORK" || return
}

script() {
  local relative="$1"
  shift
  mkdir -p "$(dirname "$relative")"
  cp "$REPO_ROOT/$relative" "$relative"
  run bash "$relative" "$@"
}

@test "Ruby validators invoke their exact bundle commands" {
  xberg_stub_trace bundle
  script scripts/ci/ruby/run-rbs-validate.sh
  xberg_assert_status 0
  script scripts/ci/ruby/run-rubocop.sh
  xberg_assert_status 0
  script scripts/ci/ruby/run-steep.sh
  xberg_assert_status 0
  script scripts/ci/ruby/run-rspec-unix.sh
  xberg_assert_status 0
  xberg_assert_trace $'bundle exec rbs validate\nbundle exec rubocop --config .rubocop.yml\nbundle exec steep check\nbundle exec rspec --format progress'
}

@test "Ruby extension build clears Bundler environment and preserves arguments" {
  # shellcheck disable=SC2016
  xberg_stub rake 'printf "%s:%s:%s\n" "${RUBYOPT-unset}" "${BUNDLE_GEMFILE-unset}" "$*"'
  export RUBYOPT=bad BUNDLE_GEMFILE=bad
  script packages/ruby/build-extension.sh 'argument with spaces'
  xberg_assert_status 0
  xberg_assert_output 'unset:unset:compile argument with spaces'
}

@test "preferred Ruby honors explicit executable and arguments" {
  xberg_stub_trace ruby
  export HTML_TO_MARKDOWN_RUBY="$XBERG_STUB_BIN/ruby"
  script scripts/preferred-ruby.sh --version
  xberg_assert_status 0
  xberg_assert_trace 'ruby --version'
}

@test "Elixir scripts invoke dependencies and required test environments" {
  # shellcheck disable=SC2016
  xberg_stub mix 'printf "%s|%s|%s\n" "${MIX_ENV-unset}" "${RUSTLER_PRECOMPILED_FORCE_BUILD_ALL-unset}" "$*" >>"$XBERG_TRACE"'
  script scripts/ci/elixir/install-deps.sh
  xberg_assert_status 0
  script scripts/ci/elixir/install-hex-rebar.sh
  xberg_assert_status 0
  script scripts/ci/elixir/run-credo.sh
  xberg_assert_status 0
  script scripts/ci/elixir/run-tests.sh
  xberg_assert_status 0
  xberg_assert_trace $'unset|unset|deps.get\nunset|unset|local.hex --force\nunset|unset|local.rebar --force\ndev|unset|credo --strict\ntest|1|test'
}

@test "CI wrappers propagate tool failures" {
  xberg_stub_exit bundle 42
  script scripts/ci/ruby/run-steep.sh
  xberg_assert_status 42
  xberg_stub_exit mix 43
  script scripts/ci/elixir/run-tests.sh
  xberg_assert_status 43
}

@test "Go lint uses repository configuration" {
  xberg_stub_trace golangci-lint
  script scripts/ci/go/run-golangci-lint.sh
  xberg_assert_status 0
  xberg_assert_trace "golangci-lint run --config $XBERG_WORK/.golangci.yml ./..."
}

@test "Go lint installation uses requested version and exports binary directory" {
  xberg_stub_trace go
  export GOLANGCI_LINT_VERSION=v2.4.0
  script scripts/ci/go/install-golangci-lint.sh
  xberg_assert_status 0
  xberg_assert_trace 'go install github.com/golangci/golangci-lint/v2/cmd/golangci-lint@v2.4.0'
  xberg_assert_github_path "$HOME/go/bin"
}

@test "Go module discovery excludes test-only modules" {
  # shellcheck disable=SC2016
  xberg_stub git 'case "$*" in
    *go.mod*) printf "%s\n" app/go.mod tests/go.mod ;;
    *app*) printf "%s\n" app/main.go ;;
    *tests*) printf "%s\n" tests/main_test.go ;;
  esac'
  script scripts/ci/go/detect-go-modules.sh
  xberg_assert_status 0
  xberg_assert_github_output 'modules=["app"]'
}

@test "Go module discovery emits an empty matrix when no modules exist" {
  xberg_stub_exit git 0
  script scripts/ci/go/detect-go-modules.sh
  xberg_assert_status 0
  xberg_assert_github_output 'modules=[]'
}

@test "PHP tests configure requested extension and use root phpunit" {
  # shellcheck disable=SC2016
  xberg_stub php 'cat "$PHPRC"; printf "%s\n" "$*"'
  export EXTENSION_PATH=/tmp/test-extension.so
  script scripts/ci/php/run-php-tests.sh
  xberg_assert_status 0
  xberg_assert_output $'extension=/tmp/test-extension.so\nvendor/bin/phpunit --configuration crates/html-to-markdown-php/src/phpunit.xml'
}

@test "PHP configuration exports the resolved php-config path" {
  xberg_stub_exit php-config
  script scripts/ci/php/set-php-config.sh
  xberg_assert_status 0
  xberg_assert_github_env "PHP_CONFIG=$XBERG_STUB_BIN/php-config"
}

@test "FFI staging copies Go and CSharp Linux artifacts" {
  # shellcheck disable=SC2016
  xberg_stub uname 'case "$1" in -s) echo Linux;; -m) echo x86_64;; esac'
  mkdir -p target/release
  printf 'ffi fixture\n' >target/release/libhtml_to_markdown_ffi.so
  script scripts/ci/go/copy-ffi.sh
  xberg_assert_status 0
  xberg_assert_file packages/go/.lib/linux-x86_64/libhtml_to_markdown_ffi.so 'ffi fixture'
  script scripts/ci/csharp/copy-ffi.sh
  xberg_assert_status 0
  xberg_assert_file packages/csharp/HtmlToMarkdown/runtimes/linux-x64/native/libhtml_to_markdown_ffi.so 'ffi fixture'
}

@test "FFI staging rejects unsupported hosts" {
  xberg_stub uname 'echo unsupported'
  script scripts/ci/go/copy-ffi.sh
  xberg_assert_status 1
  script scripts/ci/csharp/copy-ffi.sh
  xberg_assert_status 1
}

@test "CSharp runtime packing follows concrete runtime graph entries" {
  mkdir -p packages/csharp/HtmlToMarkdown
  printf '<Project><PropertyGroup><PackageId>Example</PackageId></PropertyGroup></Project>' >packages/csharp/HtmlToMarkdown/HtmlToMarkdown.csproj
  printf '{"runtimes":{"linux-x64":{"Example":{}},"alias":{"#import":["linux-x64"]}}}' >packages/csharp/HtmlToMarkdown/runtime.json.template
  xberg_stub_trace dotnet
  script scripts/ci/csharp/pack-runtimes.sh 1.2.3 output
  xberg_assert_status 0
  xberg_assert_trace 'dotnet pack packages/csharp/HtmlToMarkdown.Runtime/HtmlToMarkdown.Runtime.csproj -c Release -p:Version=1.2.3 -p:PublishedRID=linux-x64 -o output'
}

@test "Wasm Rust tests delegate to cargo and propagate failures" {
  xberg_stub_trace cargo
  script scripts/ci/wasm/test-wasm-rust.sh
  xberg_assert_status 0
  xberg_assert_trace 'cargo test'
  xberg_stub_exit cargo 44
  script scripts/ci/wasm/test-wasm-rust.sh
  xberg_assert_status 44
}

@test "Wasm bundle skips absent tests and runs existing tests with timeout" {
  xberg_stub_trace pnpm
  script scripts/ci/wasm/test-wasm-bundle.sh
  xberg_assert_status 0
  xberg_assert_output_contains 'No test files found'
  xberg_assert_trace_empty
  touch example.spec.ts
  export VITEST_TIMEOUT_MS=1234
  script scripts/ci/wasm/test-wasm-bundle.sh
  xberg_assert_status 0
  xberg_assert_trace 'pnpm vitest run --reporter=verbose --pool=threads --maxWorkers=1 --exclude=.venv/** --test-timeout=1234'
}

@test "Wasm target setup requests and verifies the intended target" {
  xberg_stub_trace rustup
  xberg_stub_trace rustc
  script scripts/common/ensure-wasm-target.sh
  xberg_assert_status 0
  xberg_assert_trace $'rustup target add wasm32-unknown-unknown\nrustc --print target-libdir --target wasm32-unknown-unknown'
}

@test "Wasm pack installation is offline when already available" {
  xberg_stub_exit wasm-pack
  xberg_stub_trace cargo
  xberg_stub_curl_offline
  script scripts/common/install-wasm-pack.sh
  xberg_assert_status 0
  xberg_assert_trace_empty
}

@test "R scripts invoke their package tools" {
  # shellcheck disable=SC2016
  xberg_stub Rscript 'printf "%s\n" "$*" >>"$XBERG_TRACE"'
  script scripts/ci/r/run-lintr.sh
  xberg_assert_status 0
  script scripts/ci/r/run-tests.sh
  xberg_assert_status 0
  xberg_assert_trace $'-e lints <- lintr::lint_package(); if (length(lints) > 0) { print(lints); quit(status = 1) }\n-e if (!requireNamespace("devtools", quietly = TRUE)) install.packages("devtools", repos = "https://cran.r-project.org")\n-e devtools::test()'
}

@test "R dependency install pins reference renderer and requests system libraries" {
  xberg_stub_trace sudo
  xberg_stub_exit apt-get
  xberg_stub_trace Rscript
  script scripts/ci/r/install-deps.sh
  xberg_assert_status 0
  [[ "$(cat "$XBERG_TRACE")" == *"libuv1-dev"* ]]
  [[ "$(cat "$XBERG_TRACE")" == *"version = '7.3.3'"* ]]
}

@test "R cleanup removes only generated Makevars" {
  mkdir -p src
  touch src/Makevars src/keep
  script packages/r/cleanup
  xberg_assert_status 0
  [ ! -e src/Makevars ]
  [ -f src/keep ]
}

@test "R configure uses populated vendor directory then configures R" {
  mkdir -p src/rust/vendor/html-to-markdown-rs
  touch src/rust/vendor/html-to-markdown-rs/Cargo.toml
  xberg_stub_curl_offline
  xberg_stub_trace Rscript
  export R_HOME="$XBERG_WORK/r-home"
  mkdir -p "$R_HOME/bin"
  cp "$XBERG_STUB_BIN/Rscript" "$R_HOME/bin/Rscript"
  script packages/r/configure
  xberg_assert_status 0
  xberg_assert_trace 'Rscript tools/config.R'
}

@test "R reference gate rejects an empty committed reference set" {
  mkdir -p packages/r/man
  script scripts/ci/r/verify-reference.sh
  xberg_assert_status 1
  xberg_assert_output_contains 'No committed R reference pages found'
}

@test "Swift module shim rejects an empty build output" {
  mkdir -p packages/swift/bin
  # shellcheck disable=SC2016
  xberg_stub swift 'printf "%s\n" "$XBERG_WORK/packages/swift/bin"'
  script scripts/ci/swift/snippet-module-shim.sh
  xberg_assert_status 1
  xberg_assert_output_contains 'no .swiftmodule'
}

@test "Swift module shim builds the expected import layout" {
  mkdir -p packages/swift/bin packages/swift/Sources/RustBridgeC
  touch packages/swift/bin/HtmlToMarkdown.swiftmodule
  printf 'header\n' >packages/swift/Sources/RustBridgeC/RustBridgeC.h
  # shellcheck disable=SC2016
  xberg_stub swift 'printf "%s\n" "$XBERG_WORK/packages/swift/bin"'
  script scripts/ci/swift/snippet-module-shim.sh
  xberg_assert_status 0
  [ -L packages/swift/bin/Modules/HtmlToMarkdown.swiftmodule ]
  xberg_assert_file packages/swift/bin/RustBridgeC/RustBridgeC.h header
  grep -q 'module RustBridgeC' packages/swift/bin/RustBridgeC/module.modulemap
}

@test "Bats library sync rejects mutable refs before downloading" {
  export BATS_LIB_REF=main
  xberg_stub_curl_offline
  script scripts/task/sync-bats-lib.sh --check
  xberg_assert_status 1
  xberg_assert_output_contains 'not'
}

@test "Changelog mirror gate rejects empty inputs" {
  mkdir -p docs-site/src/content/docs
  touch CHANGELOG.md docs-site/src/content/docs/changelog.md
  script scripts/ci/check-changelog-mirror.sh
  xberg_assert_status 1
  xberg_assert_output_contains 'has no'
}
