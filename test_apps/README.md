# test_apps/ - published-package test apps

Each directory here is a small consumer project that installs html-to-markdown from its public registry (PyPI,
npm, RubyGems, Packagist, pkg.go.dev, Maven Central, NuGet, Hex.pm, CRAN-style R install, crates.io, pub.dev,
GitHub releases, Homebrew) and checks that the released package installs and converts HTML.

They test what users get, not the working tree: they need network access and a version that is already published.
No CI workflow runs them. Run them by hand after a release, from the app's directory.

## Layout

- `fixtures/` - JSON cases from an earlier hand-written layout; no app reads them
- one directory per consumer: `bun`, `c`, `csharp`, `dart`, `elixir`, `go`, `homebrew`, `java`, `kotlin_android`,
  `node`, `php`, `php_ext`, `python`, `r`, `ruby`, `rust`, `swift_e2e`, `wasm`, `zig`

Most apps have a fast smoke test (import plus one conversion) and a fuller suite. alef generates both from the
repository's top-level `fixtures/` (`alef test-apps generate`, also run by `mise run version:sync`); the generated
files carry an `alef:hash` header and are not edited by hand.

## Running

Smoke tests, from `test_apps/<app>`:

| App        | Commands                                                                                              |
| ---------- | ----------------------------------------------------------------------------------------------------- |
| `python`   | `uv sync --no-install-project --no-install-workspace && uv run --no-sync pytest tests/test_smoke.py -v` |
| `node`     | `pnpm install && pnpm exec vitest run tests/smoke.test.ts`                                            |
| `bun`      | `bun install && bun test smoke.test.ts`                                                               |
| `wasm`     | `pnpm install && pnpm exec vitest run tests/smoke.test.ts`                                            |
| `ruby`     | `bundle install && bundle exec rspec spec/smoke_spec.rb`                                              |
| `php`      | `bash install.sh && composer install && vendor/bin/phpunit tests/SmokeTest.php`                       |
| `php_ext`  | `./run_tests.sh`                                                                                      |
| `go`       | `bash ./download_ffi.sh && go test -v -run Smoke`                                                     |
| `java`     | `./mvnw clean install -DskipTests && ./mvnw test -Dtest=SmokeTest`                                    |
| `csharp`   | `dotnet restore && dotnet test --filter FullyQualifiedName~SmokeTest`                                 |
| `elixir`   | `mix deps.get && mix test test/smoke_test.exs`                                                        |
| `r`        | `Rscript install.R`, then `Rscript -e "library(testthat); library(htmltomarkdown); test_file('tests/test_smoke.R')"` |
| `rust`     | `cargo test --test smoke_test`                                                                        |
| `c`        | `make test`                                                                                           |
| `homebrew` | see `homebrew/README.md`                                                                              |

Comprehensive suites, from `test_apps/<app>`:

| App              | Commands                                                                                               |
| ---------------- | ------------------------------------------------------------------------------------------------------ |
| `python`         | `uv sync --no-install-project --no-install-workspace && uv run --no-sync pytest tests/test_conversion.py -v` |
| `node`           | `pnpm install && pnpm test`                                                                            |
| `ruby`           | `bundle install && bundle exec rspec spec/conversion_spec.rb`                                          |
| `php`            | `bash install.sh && composer install && vendor/bin/phpunit tests`                                      |
| `go`             | `go mod tidy && go test -v -run Test_`                                                                 |
| `java`           | `./mvnw test -Dtest=ConversionTest`                                                                    |
| `csharp`         | `dotnet test --filter FullyQualifiedName~ConversionTests`                                              |
| `elixir`         | `mix deps.get && mix test test/conversion_test.exs`                                                    |
| `r`              | `Rscript install.R && Rscript run_tests.R`                                                             |
| `swift_e2e`      | `swift test`                                                                                           |
| `dart`           | `dart pub get && dart test`                                                                            |
| `kotlin_android` | `./gradlew test`                                                                                       |
| `zig`            | `(cd ../c && bash download_ffi.sh)`, then `zig fetch --save=html_to_markdown <release zig tarball URL>` and `zig build test` |

The Zig tarball URL is
`https://github.com/xberg-io/html-to-markdown/releases/download/v<version>/html-to-markdown-rs-zig-v<version>.tar.gz`,
with `<version>` from `mise run version:show`.

## Versions

The apps pin the release version. `mise run version:sync` moves those pins with every version bump, through
`alef sync-versions` and `scripts/sync_consumer_versions.py`. Check `git diff test_apps/` after a sync.

## Adding a test case

Add or change a fixture under the repository's top-level `fixtures/`, then run `alef test-apps generate`. A pin
that `alef sync-versions` does not rewrite belongs in `scripts/sync_consumer_versions.py`.
