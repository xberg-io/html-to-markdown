# Changelog

All notable changes to html-to-markdown will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.2.6] - 2026-04-20

### Fixed

- **Python type mismatch** — `convert()` return type annotation now uses the public `ConversionResult` instead of `_rust.ConversionResult`, fixing Pylance type errors when annotating with the re-exported type (#310).
- **Maturin build failure** — removed `readme = "README.md"` from binding crate Cargo.toml files (ffi, node, php, py, wasm) since no README exists for these internal crates (#309).

### Changed

- **PHPUnit** — bumped from `^12.5` to `^13.1` across root, e2e, and test_apps.
- **Pre-commit shfmt hook** — fixed rev from non-existent `v3.14.2-1` to `v3.9.0-1`.
- **Pre-commit ai-rulez hook** — updated from `v3.14.0` to `v3.14.2`.

## [3.2.5] - 2026-04-18

### Fixed

- **Silent truncation on large HTML inputs** (#277) — fixed preprocessing pipeline gap where `strip_script_and_style_tags` was not called after `repair_with_html5ever`, causing documents with custom elements and scripts containing literal `<script>` strings to be silently truncated. Also fixed `preprocess_html` fallback to skip only the problematic tag instead of consuming the rest of the document.
- **WASM plain-object options** (#303) — WASM package now exports a wrapper `convert()` function that accepts plain JavaScript objects for options, eliminating the need to construct `WasmConversionOptions` class instances. The raw WASM classes remain available for advanced use.

### Added

- **`max_depth` option** — new `max_depth` field on `ConversionOptions` to limit DOM traversal depth, preventing stack overflow on deeply nested or malicious HTML. Default is `None` (unlimited). When set, subtrees beyond the limit are silently truncated.

### Removed

- Phantom type references (`MetadataResult`, `HeadingInfo`, `LinkInfo`, `ImageInfo`) from alef configuration that did not correspond to any Rust types.

## [3.2.4] - 2026-04-17

### Fixed

- **Elixir Hex package** — fixed precompiled NIF pipeline: correct build output paths, removed Rust source from Hex files list, removed Windows target (no build job), simplified build script for `rustler_precompiled`.
- **C# NuGet package** — republished with `htm_conversion_options_from_json` and related FFI functions.

## [3.2.3] - 2026-04-17

### Fixed

- **Java/C#/Go FFI functions** — `htm_conversion_options_from_json`, `htm_preprocessing_options_from_json`, and related `to_json` functions now generated. Fixed alef IR extraction to detect serde derives inside `#[cfg_attr(...)]` attributes.
- **Node.js native binding loader** — regenerated `index.js` with correct NAPI platform-aware loader (was referencing old `html-to-markdown-rs.node` binary name).
- **Go module path** — fixed from non-existent `github.com/kreuzberg-dev/html-to-markdown-go` to monorepo path `github.com/kreuzberg-dev/html-to-markdown/packages/go/v3`.
- **Elixir precompiled NIFs** — switched from `Rustler` (compile-from-source) to `RustlerPrecompiled` with CI jobs for building and uploading platform-specific NIF binaries to GitHub releases.

### Removed

- Stale hand-written test files superseded by alef-generated e2e tests (comprehensive_test, feature_test, smoke_test duplicates across Go, Python, Ruby, PHP, Node, WASM, Elixir, R).
- Empty placeholder crate directories (`html-to-markdown-rs-ffi`, `html-to-markdown-rs-wasm`).
- Duplicate Ruby extension directory (`html-to-markdown_rb` with wrong naming).

## [3.2.2] - 2026-04-16

### Fixed

- **Ruby binding compilation** — fixed serde derive errors and Default trait conflicts by conditionally deriving serde traits only when all field types support it, and generating Default derives for kwargs constructors.
- **Ruby deprecated Magnus API** — replaced `magnus::exception::type_error()` / `runtime_error()` with `Ruby::exception_type_error()` / `Ruby::exception_runtime_error()` (Magnus 0.7+ API).
- **PHP e2e tests** — fixed missing class import causing "Class not found" fatal error.
- **Cargo.toml metadata** — added missing `readme`, `keywords`, `categories`, `description` fields to binding crate Cargo.toml files.
- **C# dotnet format** — auto-formatted generated C# bindings.

## [3.2.1] - 2026-04-16

### Fixed

- **Node.js Docker/cross-platform installs** (#273) — platform-specific native packages (`@kreuzberg/html-to-markdown-node-linux-x64-gnu`, etc.) are now correctly published with `optionalDependencies` via NAPI prepublish, resolving cross-platform lockfile issues.
- **Homebrew formula** (#304) — formula updated with correct source tarball SHA and bottle configuration.
- **Ruby gem build failure** — fixed `NodeContent::MetadataBlock` type mismatch (`Vec<(String, String)>` → `String`) in binding-to-core conversion by deserializing sanitized fields from JSON.
- **Maven Central publish** — aligned `pom.xml` with kreuzberg: GPG plugin in main build section, developer email, correct `groupId` (`dev.kreuzberg`), `pluginManagement` with pinned plugin versions.
- **All binding compilation failures** — fixed private module path (`convert_api`) in generated bindings, missing `From` impls for function return types, glob import conflicts in PyO3/FFI backends, and cbindgen compatibility (removed `const extern fn`, updated to cbindgen 0.29).
- **Elixir NIF compilation** — added `compilers: [:rustler] ++ Mix.compilers()` to `mix.exs` so Rustler compiles the NIF during `mix compile`.
- **FFI `from_json`/`to_json` functions** — `htm_conversion_options_from_json`, `htm_conversion_result_to_json`, etc. now generated for all serde-compatible types, fixing Java (Panama FFM) and Go (cgo) bindings.
- **PHP e2e tests** — fixed function call generation to use correct `HtmlToMarkdownRs::convert()` pattern.
- **Python `pyproject.toml`** — corrected `module-name` and `python-packages` to match `html-to-markdown` pip package name.
- **`docs/llms.txt`** metadata defaults corrected from `false` to `true` for `extract_metadata`, `extract_document`, `extract_headers`, `extract_links`, `extract_images`, `extract_structured_data` (#276).

- **WASM type prefix restored** (#303) — `WasmConversionOptions` (not `JsConversionOptions`) via configurable `type_prefix` in alef. No breaking change for WASM users.

### Known Issues

- **Python silent output cap** (#277) — `convert()` silently truncates output at ~439 KB on certain large HTML inputs. Under investigation.

## [3.2.0] - 2026-04-14

### Breaking Changes

#### Core Defaults Changed

- **`code_block_style`** default changed from `Indented` to `Backticks` — code blocks now use triple-backtick fences by default instead of 4-space indentation.
- **`bullets`** default changed from `"-"` to `"-*+"` — nested unordered lists now cycle through `-`, `*`, `+` at successive nesting levels.
- **`preprocessing.enabled`** default changed from `false` to `true` — HTML preprocessing (navigation removal, form stripping) is now on by default.
- **Rust serde field names** changed from `camelCase` to `snake_case` — affects JSON serialization/deserialization of the Rust core `ConversionOptions` struct (`heading_style` instead of `headingStyle`). Language bindings are not affected — each binding uses its language-native naming convention (camelCase for JS/TS/Java/C#, snake_case for Python/Ruby/Elixir/R).

#### Package Renames

- **TypeScript/Node.js**: npm package renamed from `@kreuzberg/html-to-markdown` to `@kreuzberg/html-to-markdown-node`.
- **PHP**: Namespace changed from `HtmlToMarkdown\` to `Html\To\Markdown\Rs\`. Main class renamed from `HtmlToMarkdown` to `HtmlToMarkdownRs`.
- **Java**: Maven coordinates remain `dev.kreuzberg:html-to-markdown`. Internal package namespace changed to `dev.kreuzberg.htmltomarkdown`.
- **C FFI**: Function prefix changed from `html_to_markdown_` to `htm_` (e.g., `htm_convert`, `htm_last_error_code`). Header moved to `include/html_to_markdown.h`.
- **C#**: Main class renamed from `HtmlToMarkdownConverter` to `HtmlToMarkdownRs`.

#### Python Exception Hierarchy

- Old exceptions (`HtmlToMarkdownError`, `EmptyHtmlError`, `InvalidParserError`, etc.) replaced with new hierarchy: `ConversionError`, `ParseError`, `SanitizationError`, `ConfigError`, `IoError`, `InvalidInputError`, `PanicError`, `OtherError`.

### Added

- **`ConversionOptionsBuilder`** is now public in the Rust API — use `ConversionOptions::builder()` for ergonomic option construction.
- **`TableData`** exported from crate root — no longer requires importing from submodules.
- **Per-language API reference documentation** — generated `docs/reference/api-{lang}.md` pages for Python, TypeScript, Go, Java, C#, Ruby, PHP, Elixir, WASM, and C with full type mappings, signatures, and docstrings.
- **Alef codegen** — all 12 language bindings (Rust, Python, TypeScript, Go, Java, C#, Ruby, PHP, Elixir, WASM, C, R) are now auto-generated from a single IR via [alef](https://github.com/kreuzberg-dev/alef), configured in `alef.toml`.
- **E2E test suite** — 130 fixture-driven tests across all 12 languages, generated from shared JSON fixtures in `fixtures/`.
- **`AnnotationKind`** and **`NodeContent`** now implement `Default`.
- **`ConversionResult`** now derives `Serialize`/`Deserialize`.

### Changed

- **Docs platform**: Switched from MkDocs to [Zensical](https://zensical.dev) (`zensical.toml` replaces `mkdocs.yaml`).
- **READMEs**: Now generated by `alef readme` from minijinja templates with inline configuration in `alef.toml`.
- **Version sync**: `task version:sync` now uses `alef` binary for all operations (replaces `sync_versions.py`).
- **CI workflows**: Simplified to e2e-only testing per language — removed per-binding unit test jobs.

### Removed

- **`crates/html-to-markdown-bindings-common`** — shared bindings helper crate replaced by alef codegen.
- **`tools/e2e-generator`** — replaced by `alef e2e generate`.
- **`tools/snippet-runner`** — snippet validation removed.
- **`scripts/generate_readme.py`**, **`scripts/sync_versions.py`**, **`scripts/readme_filters.py`** — replaced by alef.
- **Hand-written binding code** — all per-language binding implementations replaced by alef-generated code.

### Fixed

- **Preserve metadata when using AST visitors** (#279).
- **`deny_unknown_fields`** added to serde option structs — invalid JSON fields now produce errors instead of being silently ignored.
- **Ruby e2e generator** — fixed camelCase→snake_case field name conversion.
- **WASM test options** — added missing `link_style` field.

## [3.1.0] - 2026-04-01

### Added

- **Reference-style links**: New `link_style` option (`"inline"` default, `"reference"`) renders links as `[text][1]` with numbered `[1]: url "title"` definitions appended at the end of the output. Supports URL+title deduplication, images (`![alt][1]`), and media elements (audio, video, iframe). Available across all bindings (Python, Node.js, WASM, PHP, CLI `--link-style`, FFI via JSON).

## [3.0.2] - 2026-04-01

### Fixed

- **Structure collector in tables**: Suppressed `StructureCollector` calls for headings and lists inside table cells, preventing spurious document-structure nodes from table content.
- **Char boundary safety**: Fixed potential panics from slicing at non-UTF-8-char boundaries in `generate_id` hash truncation and list item text extraction.
- **Dead feature gates removed**: Cleaned up unused `document-structure` feature gates that were no longer wired to any Cargo feature.
- **Structure collector coverage**: Added missing `StructureCollector` calls for lists, images, and code blocks so document structure captures all block-level elements.

## [3.0.1] - 2026-03-31

### Fixed

- **WASM TypeScript types**: `convert()` now returns typed `WasmConversionResult` instead of `any`. All `WasmConversionTable`, `WasmGridCell`, `WasmTableGrid`, `WasmConversionWarning`, and `WasmInlineImage` interfaces are now emitted in generated `.d.ts` files. Added missing options fields (`skipImages`, `outputFormat`, `includeDocumentStructure`, `extractImages`, `maxImageSize`, `captureSvg`, `inferDimensions`). Fixes #265.
- **Python type stubs**: Synced crate `.pyi` stub with package stub — added keyword-only (`*`) parameter markers and `visitor` parameter to `convert()`.
- **PHP type stubs**: Expanded PHPStan stubs with full `ConversionResult`, `ConversionOptions`, and all nested type shapes. Wired stubs into `composer.json` PHPStan config.

## [3.0.0] - 2026-03-30

### Added

- **Single `convert()` API**: One entry point across all 12 language bindings returning `ConversionResult` with content, document, metadata, tables, images, and warnings.
- **`ConversionResult` type**: Structured result with `content` (markdown/djot/plain), `document` (optional `DocumentStructure`), `metadata` (`HtmlMetadata`), `tables` (grid-based), `images` (inline image data), and `warnings`.
- **`DocumentStructure`**: Structured document tree with flat node array, index-based parent/child references, and `TextAnnotation` for inline formatting.
- **Options support in all bindings**: Go, Java, C# now accept options. All generators wire fixture options into e2e tests.
- **GFM defaults**: Code blocks default to backtick fences (was indented). ATX headings remain default.
- **E2E contract validation**: Generators produce tests validating ConversionResult structure (metadata, tables, warnings) across all 12 languages.
- **New options**: `includeDocumentStructure`, `extractImages`, `maxImageSize`, `captureSvg`, `inferDimensions`, `outputFormat` (markdown/djot/plain).
- **`<q>` element**: Wraps content in quotation marks.
- **`<figure>`/`<figcaption>` elements**: Routed to semantic handler with caption separation.
- **`hidden` attribute**: Elements with `hidden` stripped before parsing.

### Changed

- **`convert()` returns `ConversionResult`** instead of `String` in all bindings (Go, Java, C#, Node, Python, PHP, Ruby, Elixir, R, WASM, C FFI).
- **`ExtendedMetadata` renamed to `HtmlMetadata`** across all crates and bindings.
- **Go `Convert()`** returns `*ConversionResult` with `Content`, `Metadata`, `Tables`, `Images`, `Warnings` fields. Accepts optional JSON options via variadic parameter.
- **Table data** uses grid-based schema (`TableGrid` with `GridCell`) instead of flat `cells [][]string`.
- **`serde(deny_unknown_fields)`** on `MetadataConfig`, `MetadataConfigUpdate`, `InlineImageConfigUpdate`.
- **Go 1.26**, golangci-lint@latest.

### Removed

- **All `convert_with_*` functions**: `convert_with_metadata`, `convert_with_inline_images`, `convert_with_visitor` (standalone), `convert_with_tables`, `convert_with_async_visitor` removed from public API. Single `convert()` replaces all.
- **Async visitor**: Feature removed entirely (`async-visitor` Cargo feature, `AsyncHtmlVisitor` trait, async bridge/dispatch code).
- **Profiling**: All profiling infrastructure removed (8 binding crates, CI workflow, C tests, `start_profiling`/`stop_profiling` APIs).
- **Benchmarks**: All benchmark scripts and harness removed.
- **hOCR support**: Entire `hocr` module deleted. The `hocr_spatial_tables` option removed.
- **Python v1 compatibility**: `convert_to_markdown()` and `markdownify()` removed.
- **Redundant binding tests**: Tests covered by e2e generators removed from Python, Ruby, Elixir, R.

## [2.30.0] - 2026-03-27

### Deprecated

- **hOCR support**: The `hocr_spatial_tables` option and all hOCR-related APIs are deprecated and will be removed in v3. All hOCR functionality continues to work but emits deprecation warnings. This is the final v2 release.

### Fixed

- **PHP PHPStan CI errors**: Removed redundant `@var` annotations and `is_array()` runtime checks in `ExtensionBridge.php` that PHPStan flagged as always-true due to stub-defined return types. Removed redundant `array_values()` calls in `ConversionOptions.php` on properties already typed as `list<string>`. Updated PHPStan baseline count for callable invocations.

## [2.29.0] - 2026-03-22

### Added

- **`full` feature group**: Added a `full` feature to core crate and all binding crates (PHP, Python, Node, WASM, FFI, Elixir, bindings-common) that enables all available features. All bindings now default to `full`.
- **Dublin Core metadata extraction**: `DC.*` and `DCTERMS.*` meta tags now map to dedicated `DocumentMetadata` fields (title, description, author, keywords). Other DC/DCTERMS fields stored in `meta_tags` with `dc_`/`dcterms_` prefix.
- **Extended keyword variants**: Keywords now extracted from `news_keywords`, `citation_keywords`, `DC.subject`, `DC.keywords`, `DCTERMS.subject`, `subject`, `topic`, `category`, and `classification` meta tags.
- **`cargo-sort` pre-commit hook**: Added for consistent Cargo.toml key ordering.
- **`checkmake` pre-commit hook**: Added for Makefile linting.
- **`typescript-typecheck` pre-commit hook**: Added TypeScript type checking via `tsc --noEmit`.
- **`typecheck` npm script**: Added to `packages/typescript/package.json`.

### Fixed

- **Case-insensitive meta tag matching** ([#251](https://github.com/kreuzberg-dev/html-to-markdown/issues/251)): All meta tag name matching is now case-insensitive per the HTML spec. `<meta name="Keywords">` and `<meta name="DC.keywords">` are now correctly captured.
- **PHP `convertWithTables()` not found** ([#250](https://github.com/kreuzberg-dev/html-to-markdown/issues/250)): The `visitor` feature was not enabled by default in the PHP binding crate, causing `html_to_markdown_convert_with_tables` to be missing from the extension.
- **PHP binding defaults**: PHP crate now defaults to `["full"]` (was `["metadata"]`), enabling visitor support.
- **Python binding defaults**: Python crate now defaults to `["full"]` (was `[]`), enabling metadata, visitor, async-visitor, and inline-images.
- **PHPStan 2.x compatibility**: Fixed 40+ PHPStan errors from the 1.x→2.x upgrade (type narrowing, property access on mixed, redundant assertions). Added `--memory-limit=512M` to prevent OOM.
- **Makefile `test` target**: Added missing `.PHONY: test` target to FFI test Makefile.

### Changed

- **Pre-commit config aligned with kreuzberg**: Added `cargo-sort`, `checkmake`, `typescript-typecheck`. Updated `taplo-format` to exclude `Cargo.toml`. Excluded `Makefile.frag` from checkmake.
- **Cargo.toml formatting**: All workspace Cargo.toml files sorted via `cargo-sort`.
- **pyproject.toml formatting**: All pyproject.toml files formatted via `pyproject-fmt`.

## [2.28.6] - 2026-03-20

### Changed

- **Ruby gem vendoring**: Replaced bash+embedded-Python vendoring script with a standalone Python vendoring script adapted from kreuzberg, using `vendor/` directory instead of `rust-vendor/` for core crate vendoring.
- **Ruby gem build**: Added `build-native-gem.rb` for platform-specific pre-compiled gem builds, following kreuzberg patterns.
- **Pre-commit hooks**: Switched Ruby hooks (rubocop, rbs-validate, steep-check) from inline bash commands to task-based delegation matching kreuzberg.
- **Dependabot config**: Expanded from GitHub Actions only to full multi-ecosystem coverage (Cargo, pip, npm, bundler, composer, gomod, maven, nuget, mix).
- **Task update commands**: Aligned all language update tasks with kreuzberg's comprehensive approach (outdated checks, aggressive updates).
- **C# update**: Switched from slow `dotnet list --outdated` Python script to `dotnet-outdated-tool` for faster dependency updates.

### Fixed

- **CI Validate shfmt failure**: Fixed `packages/r/configure.win` tab indentation to match shfmt 2-space requirement.
- **Java linting**: Added PMD plugin (3.28.0), JaCoCo coverage (0.8.14), and pinned checkstyle runtime (13.3.0). Bumped maven-compiler-plugin to 3.15.0, maven-surefire-plugin to 3.5.5, spotless to 3.4.0, central-publishing to 0.10.0.

### Updated

- **GitHub Actions**: Bumped `go-task/setup-task` from v1 to v2, `nick-fields/retry` from v3 to v4.
- **Dependencies**: Updated all language dependencies via `task update`.

## [2.28.5] - 2026-03-19

### Fixed

- **Table colspan parsing** ([#233](https://github.com/kreuzberg-dev/html-to-markdown/issues/233)): Fixed column count calculation to accurately use colspan values instead of incrementing by 1, and refined layout table heuristics to exempt simple data tables with colspans while correctly catching layout tables.
- **Ruby version.rb formatting**: Fixed missing space around `=` operator in `version.rb` that caused Rubocop lint failures in CI.
- **CI tooling alignment**: Aligned tooling and documentation with kreuzberg standards, fixing CI failures.

## [2.28.4] - 2026-03-13

### Fixed

- **Panic with cid image followed by italic paragraph** ([#222](https://github.com/kreuzberg-dev/html-to-markdown/issues/222)): Confirmed fix for panic ("byte index 53 is out of bounds of ``") when converting HTML containing `cid:` image paragraphs followed by italicized text. This was resolved in v2.28.0 via the block_content_start bounds fix (#216, #217) and multi-byte UTF-8 character boundary fix (#218).
- **Ruby gem installation on macOS** ([#219](https://github.com/kreuzberg-dev/html-to-markdown/issues/219)): Confirmed fix for `Cargo.lock` missing from published gem causing `magnus` dependency load failure. Resolved in v2.28.3 with native platform gem builds.

## [2.28.3] - 2026-03-10

### Fixed

- **Java visitor FFI struct return type**: Fixed `IllegalArgumentException: Wrong method handle type` when using visitors in Java. The Panama FFI callback descriptors incorrectly used `JAVA_LONG` (8 bytes) as the return type instead of the actual `HtmlToMarkdownVisitResult` C struct (24 bytes: enum + 2 pointers). All 14 callback descriptors now use a proper `StructLayout` matching the C ABI.
- **Homebrew bottle tarball structure**: Fixed bottle tarballs missing the required `html-to-markdown/{version}/` prefix directory. Homebrew expects this prefix for proper cellar installation.
- **Ruby gem publishing**: Added native platform gem builds (`rake native gem`) alongside source gems so precompiled extensions are available for Linux, macOS, and Windows.

## [2.28.2] - 2026-03-08

### Fixed

- **Publish workflow republish flag**: Fixed republish mode skipping all publish jobs because `INPUT_REF` resolved to a branch name instead of the tag ref.
- **Definition list fixture**: Aligned real-world test fixture for `<dl>`/`<dt>`/`<dd>` with actual converter output (plain text, no Pandoc-style `:` prefix).

## [2.28.1] - 2026-03-06

### Fixed

- **Panic with multi-byte UTF-8 and visitor** ([#218](https://github.com/kreuzberg-dev/html-to-markdown/issues/218)): Fixed a panic ("byte index N is not a char boundary") when converting HTML containing multi-byte UTF-8 characters (Cyrillic, CJK, emoji, etc.) with tabs between block elements and any visitor. The stale byte position captured before whitespace trimming could land inside a multi-byte character when new content was appended.
- **Java formatting**: Fixed spotless formatting violations in `HtmlToMarkdown.java`, `TableData.java`, and `TableExtractionResult.java`.

## [2.28.0] - 2026-03-05

### Added

- **Table extraction API**: New `convert_with_tables` function that extracts structured table data during HTML-to-Markdown conversion. Returns `TableData` structs containing cell contents as `Vec<Vec<String>>`, rendered markdown output, and per-row header flags. Uses the visitor pattern internally with a built-in `TableCollector` to capture table structure in a single pass. Available across all language bindings:
  - **Rust**: `convert_with_tables(html, options, metadata_config)` returning `ConversionWithTables`
  - **Python**: `convert_with_tables(html, options, preprocessing, metadata_config)` returning `TableExtractionResult`
  - **TypeScript/Node.js**: `convertWithTables(html, options?, metadataConfig?)` returning `TableExtraction`
  - **Ruby**: `HtmlToMarkdown.convert_with_tables(html, options, metadata_config)` returning a Hash
  - **PHP**: `HtmlToMarkdown::convertWithTables($html, $options, $metadataConfig)` returning `TableExtractionResult`
  - **Go**: `ConvertWithTables(html)` returning `TableExtractionResult`
  - **Java**: `HtmlToMarkdown.convertWithTables(html)` returning `TableExtractionResult`
  - **C#**: `HtmlToMarkdownConverter.ConvertWithTables(html)` returning `TableExtractionResult`
  - **Elixir**: `HtmlToMarkdown.convert_with_tables(html, options, metadata_config)` returning `{:ok, content, tables, metadata}`
  - **R**: `convert_with_tables(html, options, metadata_config)` returning a list
  - **C (FFI)**: `html_to_markdown_convert_with_tables(html, options_json, metadata_json)` returning JSON
  - **WASM**: `convertWithTables(html, options?, metadataConfig?)` returning a JS object

### Fixed

- **Plain text fast path skipping visitor callbacks**: When `OutputFormat::Plain` was used with `convert_with_tables`, the plain text fast path returned before the visitor could extract table data, resulting in empty tables. The conversion pipeline now runs the full visitor walk before returning plain text content.

## [2.27.3] - 2026-03-05

### Fixed

- **Panic on block_content_start out of bounds**: Fixed a crash (`byte index N is out of bounds`) in text node processing when inline handlers (e.g. `<strong>`, `<em>`) collected children into a fresh buffer while inheriting a parent paragraph context. The `block_content_start` index pointed into the wrong buffer, causing a panic on certain HTML structures — notably `<details>` containing `<p>` with inline formatting. (Issues #216, #217)

## [2.27.2] - 2026-03-02

### Fixed

- **Plain text list items missing markers**: `<ul>` and `<ol>` list items in `OutputFormat::Plain` were output without any bullet or number prefix. Now emits `-` for unordered lists and sequential `N.` for ordered lists, respecting the `start` attribute on `<ol>`.

## [2.27.1] - 2026-03-01

### Fixed

- **Colon introduced into definition list text**: `<dd>` elements inside `<dl>` were incorrectly prefixed with `:` (Pandoc definition list syntax), introducing spurious colons into converted text. Standard Markdown and GFM do not support definition list syntax, so `<dd>` content is now output as plain blocks. (Issue #214, thanks @smoyerx)
- **Go test app go.sum out of sync**: Updated `tests/test_apps/go/go.sum` to match the v2.27.0 module version, fixing the CI Go lint job.

## [2.27.0] - 2026-03-01

### Added

- **Plain text output format**: New `OutputFormat::Plain` option that strips all markup and returns only visible text content. Set `output_format` to `"plain"` (also accepts `"plaintext"` or `"text"`). This fast-path bypasses the full Markdown/Djot conversion pipeline — after DOM parsing, a lightweight text extractor walks the tree collecting only visible text with structural whitespace. Useful for search indexing, text extraction, and feeding content to LLMs.

## [2.26.3] - 2026-02-28

### Fixed

- **Subscript/superscript content silently dropped**: When `sub_symbol` or `sup_symbol` was empty (the default), text inside `<sub>` and `<sup>` tags was discarded entirely — e.g. `H<sub>2</sub>O` produced `HO` instead of `H2O`.
- **Missing whitespace between newline-separated inline elements**: Whitespace-only text nodes containing newlines between adjacent inline elements (e.g. `<a>…</a>\n<a>…</a>`) were dropped, causing links and other inline markup to merge without a word boundary. Now collapses to a single space per HTML white-space normalization rules.

## [2.26.2] - 2026-02-28

### Fixed

- **Inconsistent whitespace before inline elements across paragraphs**: Fixed a stateful bug where `\n` before `<a>`, `<strong>`, `<em>`, and other inline elements inside `<p>` tags was handled differently depending on the paragraph's position in the document. The second and subsequent paragraphs would drop the space before inline elements, producing `text[link](url)` instead of `text [link](url)`. (Issue #212, thanks @haroldparis)

## [2.26.1] - 2026-02-27

### Fixed

- **YAML frontmatter in `convert_with_metadata` output**: `convert_with_metadata` no longer prepends YAML frontmatter to the markdown string. Since metadata is returned as a structured `ExtendedMetadata` object, embedding it in the content string was redundant and polluted the output.

## [2.26.0] - 2026-02-26

### Added

- **C FFI distribution infrastructure**: Distribution-grade C FFI library with CMake/pkg-config integration, installation scripts, and packaging for system-level consumption.
- **C FFI test coverage**: Comprehensive C test suite covering conversion, metadata extraction, error handling, visitor pattern, profiling, and version queries.
- **C documentation and examples**: C API reference, getting-started snippets, and example programs for basic conversion, metadata extraction, and visitor pattern usage.

### Fixed

- **R package r-universe build**: Configure scripts now download the source archive from GitHub when the monorepo is unavailable, enabling r-universe and standalone source installs to vendor crates automatically.

## [2.25.2] - 2026-02-25

### Fixed

- **Visitor panic with metadata extraction**: Fixed an out-of-bounds slice panic when using visitors (e.g. image visitors returning `Custom`) combined with metadata extraction on minified HTML. The issue occurred because parent element output offsets became stale after child visitor truncations. (PR #204, thanks @gmalette)

### Added

- **R language bindings**: Full-parity R bindings via extendr framework with support for `convert()`, `convert_with_options()`, `convert_with_options_handle()`, `convert_with_metadata()`, `convert_with_inline_images()`, `convert_with_visitor()`, and profiling. Includes `conversion_options()` helper, testthat test suite, CI workflow, lintr/styler pre-commit hooks, and task automation.
- **R CRAN publishing infrastructure**: Added `publish-cran` job to publish workflow, `cran-comments.md`, and `NEWS.md` for CRAN submission compliance.

### Changed

- **Workspace restructuring**: Moved Ruby native crate out of the root Cargo workspace into a standalone workspace (matching Elixir/R pattern), resolving `clang-sys` link conflict with `ext-php-rs` 0.15.6.
- **Rust update task**: Now updates dependencies in all separate workspaces (Ruby, Elixir, R) via `--manifest-path` entries.
- **Upgraded `wasmtime`** from 41 to 42.
- **Upgraded `ext-php-rs`** from 0.15.4 to 0.15.6.
- **Upgraded `pyo3`** from 0.28.1 to 0.28.2.
- **Upgraded `wasm-bindgen`** from 0.2.112 to 0.2.113.
- **Upgraded `rustls`** from 0.23.36 to 0.23.37.

## [2.25.1] - 2026-02-17

### Fixed

- **hOCR heading detection**: Improved hierarchy logic to use font size (`x_fsize`) and bbox height as a proxy when detecting headings. Large-font paragraphs now support longer text (up to 80 chars) and single-word headings. Added comprehensive test coverage for heading detection edge cases.

## [2.25.0] - 2026-02-15

### Added

- **Bun runtime support**: Official support for Bun 1.2+ via Node-API compatibility. The existing NAPI-RS bindings work in Bun without changes. Added Bun to CI test matrix and updated documentation to reflect runtime compatibility.

### Changed

- **Vendored `markup5ever_rcdom`**: Brought the `markup5ever_rcdom` code (MIT/Apache-2.0) into the core crate as an internal `rcdom` module. This removes the external dependency on the "+unofficial" crate, eliminates the unused `xml5ever` transitive dependency, and removes the pinned `html5ever`/`markup5ever_rcdom` version constraints. See `ATTRIBUTIONS.md` for license details.
- **Upgraded `html5ever`** from 0.36.1 to 0.38.0 (now unpinned).
- **Upgraded `pyo3`** from 0.28.0 to 0.28.1.

## [2.24.6] - 2026-02-14

### Fixed

- **Dependency update stability**: Pinned compatible `html5ever`/`markup5ever_rcdom` versions to prevent trait-mismatch breakages during workspace dependency updates.
- **Python bindings build**: Added explicit `#[pyclass(from_py_object)]` on Python config wrapper classes to avoid PyO3 deprecation failures under `-D warnings`.
- **Rust lint consistency**: Aligned crate-level clippy configuration so `multiple_crate_versions` does not fail Node/WASM/FFI crate lint runs.
- **WASM dependency behavior**: Updated hashing dependency configuration to avoid wasm randomness backend breakage after dependency updates.
- **PHP PIE publish verification (macOS)**: Hardened PIE verification/build scripts for Darwin linker behavior and shell-safe package spec handling.
- **CI reliability**: Updated validation and Python CI tasks to reduce flakiness (PHP 8.4 setup in validate; avoid redundant Rust CLI release builds in Python test runs).

## [2.24.5] - 2026-02-01

### Fixed

- **Subscript/superscript whitespace handling**: Subscript and superscript tags now trim inner whitespace and place it outside delimiters, matching the behavior of bold, italic, and strikethrough (issue #202).

## [2.24.4] - 2026-01-31

### Performance

- **Reduced allocations in hot conversion paths**: Return `Cow<str>` from escape to avoid allocating on no-op paths, replace `.repeat()` with direct push loops in heading/list/table/div/paragraph formatters, eliminate `collect::<Vec>::join()` in text dedentation, and use `AHashMap` for hOCR property maps.

### Fixed

- **WASM builds**: Updated `getrandom` backend configuration from `"js"` to `"wasm_js"` for compatibility with getrandom 0.3.x.
- **Elixir/Ruby vendor scripts**: Added missing `ahash` workspace dependency replacement for standalone builds.

## [2.24.3] - 2026-01-31

### Fixed

- **Definition lists**: Ensure `<dl>/<dt>/<dd>` output is consistent regardless of HTML whitespace/minification, and properly indent multiline definition content (issue #200).
- **Link labels**: Removed hard truncation of long link labels to avoid broken Markdown for large image links (issue #199).

## [2.24.2] - 2026-01-29

### Fixed

- **Java packaging**: Bundle native FFI libraries in published Maven JAR for all platforms (linux-x86_64, linux-aarch64, osx-aarch64, windows-x86_64). The Java package now works out-of-the-box when installed from Maven Central without requiring local FFI builds or manual java.library.path configuration. Native libraries are automatically extracted to a temp directory on first use with platform detection and fallback support.

## [2.24.1] - 2026-01-27

### Fixed

- **UTF-16 recovery**: Automatically recovers UTF-16 HTML (including data without BOM) that was read via lossy UTF-8 decoding, instead of rejecting it as binary data.
- **URL sanitization**: Hardened markdown-like URL sanitization to extract the real URL from `...[text](url)` patterns in `href`/`src` attributes, preventing caller-side URL join/parsing errors.
- **Issue #190 coverage**: Added regression fixtures and tests covering the reported real-world HTML inputs.

## [2.24.0] - 2026-01-24

### Changed

- **Bindings API**: Removed `_json` conversion entrypoints across bindings; convert functions now pass full option payloads directly.

### Fixed

- **Visitor docs**: Corrected Python visitor documentation and examples (argument order + ctx access).
- **Python visitor options**: `convert_with_visitor` now respects full conversion options payloads.
- **skip_images**: Skip flag now suppresses SVG/graphic outputs in addition to `<img>`.
- **Code block dedent**: Handles Unicode whitespace without panicking on UTF-8 boundaries.
- **Input validation**: Tolerates small NUL byte artifacts and strips them before conversion.

## [2.23.6] - 2026-01-21

### Fixed

- **pnpm lockfile synchronization**: Fixed pnpm lockfile to include Node.js platform-specific optional dependency version updates (2.19.0-rc.1 → 2.23.6) that were applied during v2.23.5 version sync. This resolves the `ERR_PNPM_OUTDATED_LOCKFILE` errors that caused the v2.23.5 publish workflow to fail.
- **CI Java version**: Updated CI Java workflow from Java 24 to Java 25 to match maven.compiler.release=25 configuration, ensuring CI and local builds use the same compiler version.

## [2.23.5] - 2026-01-21

### Fixed

- **Maven Central publishing**: Corrected group ID in Maven Central check script from legacy `io.github.goldziher` to `dev.kreuzberg`, enabling successful Java package publishing. This resolves the issue where Java v2.23.4 failed to publish to Maven Central.
- **Go module publishing**: Added automated Go module tag creation (`packages/go/v{version}`) to publish workflow, ensuring Go packages are immediately available on Go proxy after release.
- **Go FFI version synchronization**: Updated Go FFI default version constants from outdated versions (2.19.1/2.23.0) to 2.23.5 in both `ffi_loader.go` and `cmd/install/main.go`, ensuring automatic downloads use the correct library version.
- **Node.js platform dependencies**: Synchronized all platform-specific optional dependencies in `@kreuzberg/html-to-markdown-node` package.json from 2.19.0-rc.1 to match main package version, preventing dependency resolution issues.
- **Java benchmark packaging**: Updated benchmark-pom.xml to use correct group ID (`dev.kreuzberg`), version (2.23.5), and main class namespace (`dev.kreuzberg.benchmark.Benchmark`). Removed outdated generated `dependency-reduced-pom.xml`.
- **PHP package references**: Updated all PHP package references from `goldziher/html-to-markdown` to `kreuzberg-dev/html-to-markdown` across composer.json, PIE verification scripts, and smoke test actions to reflect current package organization.
- **Java smoke tests**: Updated smoke-java GitHub action to use correct group ID (`dev.kreuzberg`) and package namespace (`dev.kreuzberg.htmltomarkdown.SmokeTest`) for JAR installation and test execution.
- **Build tooling**: Fixed Python script execution in task runner to use `uv run python3` instead of system python3, ensuring consistent dependency resolution. Added PyYAML and Jinja2 to workspace dev dependencies.
- **Version sync automation**: Enhanced version sync script to automatically update Node.js platform-specific optional dependencies alongside main package version, preventing manual version drift.

## [2.23.4] - 2026-01-20

### Fixed

- **TypeScript wrapper publishing**: Fixed TypeScript wrapper dependency resolution by installing dependencies directly from npm registry instead of from workspace after Node packages are published. This ensures `@kreuzberg/html-to-markdown-node` is available from npm when building the TypeScript wrapper, eliminating the workspace resolution issues that caused previous build failures.

## [2.23.3] - 2026-01-20

### Fixed

- **Go FFI packaging**: Fixed missing `html_to_markdown.h` header file in Go FFI archive tarballs, which caused `go:generate` installation to fail with "fatal error: 'html_to_markdown.h' file not found". The header is now included in all platform archives (tar.gz and zip).
- **TypeScript wrapper publishing**: Fixed pnpm lockfile frozen mode error during TypeScript wrapper dependency reinstallation by adding `--no-frozen-lockfile` flag. The reinstall step after publishing Node packages now correctly updates workspace dependencies despite lockfile version mismatches.

## [2.23.2] - 2026-01-20

### Fixed

- **TypeScript wrapper publishing**: Fixed TypeScript wrapper build failures by moving the build and publish steps into the same `publish-node` job. This eliminates npm CDN propagation delays that caused `@kreuzberg/html-to-markdown` to fail building because `@kreuzberg/html-to-markdown-node` wasn't available yet. Added workspace dependency reinstallation step to ensure pnpm correctly resolves the local package after publishing.
- **Go FFI library installation**: Fixed critical bugs in the `go:generate` install script that prevented automatic FFI library downloads:
  - Corrected artifact naming from `go-ffi-{platform}.tar.gz` to `html-to-markdown-ffi-{version}-{platform}.tar.gz`
  - Fixed platform mapping to match GitHub release artifacts (darwin-arm64, linux-x64, etc.)
  - Added support for all library formats (.dylib for macOS, .so for Linux, .dll for Windows)
- **Ruby native Cargo.toml**: Fixed workspace dependency configuration to use `workspace = true` instead of vendored path reference, preventing Cargo workspace resolution failures during builds.
- **CI workflows**: Upgraded all CI workflows from Java 24 to Java 25 to match maven.compiler.release=25 configuration in pom.xml.
- **Go linting**: Resolved golangci-lint warnings by adding constants for OS names and library names, and converting if-else chains to switch statements.

### Changed

- **Go README**: Updated installation documentation to explain the `go:generate` workflow for automatic FFI library installation, including details about caching in `~/.html-to-markdown/` and alternative manual configuration.

## [2.23.1] - 2026-01-19

### Fixed

- **Go module versioning**: Created 14 missing Go module tags (packages/go/v2.16.1, v2.19.1-v2.19.8, v2.20.1, v2.21.1, v2.22.1-v2.22.5) to ensure all versions since v2.15.0 are available via Go proxy. Users can now `go get` any version from v2.15.0 onwards.
- **TypeScript wrapper publishing**: Added missing `publish-typescript` job to publish workflow to properly publish `@kreuzberg/html-to-markdown` TypeScript wrapper package to npm alongside the native Node.js bindings (`@kreuzberg/html-to-markdown-node`).
- **Ruby gem vendoring**: Fixed Ruby gem installation failures due to missing `.cargo-checksum.json` files. Updated gemspec to include hidden files with `File::FNM_DOTMATCH` flag, and improved vendoring script to generate checksums correctly with `--locked` flag and proper cleanup.
- **Elixir package size**: Reduced Hex package size from 134 MB to under 128 MB limit by aggressively removing unnecessary files from vendored dependencies (tests, docs, examples, static libraries, Windows-only crates on Unix builds).

### Added

- **Go automatic FFI library installation**: Implemented `go:generate` pattern following Kreuzberg approach. Added `cmd/install` package that automatically downloads platform-specific FFI libraries from GitHub releases and generates CGO flags. Users can now run `go generate` after installation instead of manually setting `CGO_CFLAGS` and `CGO_LDFLAGS` environment variables. FFI loader updated to check `~/.html-to-markdown/` for installed libraries.

## [2.23.0] - 2026-01-18

### Added

- **Djot output format support**: New `output_format` option in `ConversionOptions` enables conversion to [Djot](https://djot.net/) lightweight markup language as an alternative to Markdown. Djot uses different syntax for emphasis (`_text_`), strong (`*text*`), strikethrough (`{-text-}`), inserted (`{+text+}`), highlighted (`{=text=}`), subscript (`~text~`), and superscript (`^text^`).
- **CLI**: Added `--output-format` / `-f` flag to specify output format (`markdown` or `djot`)
- **All language bindings**: OutputFormat enum/option added to Python, TypeScript/Node.js, Ruby, PHP, Elixir, Go, Java, and C# bindings
- **Documentation**: Added Djot output format section to all package READMEs with syntax comparison table

### Fixed

- **Python**: Fixed async visitor bridge to properly await coroutines. `PyAsyncVisitorBridge::call_visitor_method_sync()` now detects async methods via `__await__` attribute and uses `PYTHON_TASK_LOCALS` event loop for proper async execution (issue #187)
- **Ruby**: Fixed visitor parameter being ignored in `convert()` wrapper method. Now correctly passes visitor to native `convert_with_visitor` function when provided (issue #187)

### Changed

- **Rust**: Updated `async-visitor` feature to include required `tokio` "sync" feature for `Mutex` support
- **Documentation**: Added comprehensive visitor pattern support matrix showing which bindings support visitors
- **Documentation**: Documented WASM visitor pattern architectural limitation with four alternative approaches

## [2.22.6] - 2026-01-16

### Fixed

- **Ruby gem dependency resolution**: Ruby native extension now uses workspace version inheritance with vendoring approach. During gem build, the entire `html-to-markdown` crate is vendored with exact dependency versions into `packages/ruby/vendor/`, making gems completely self-contained and eliminating crates.io dependency resolution during installation. Local development uses symlink to workspace crate for seamless workflow.
- **URL parsing robustness**: Fixed IPv6 URL parsing error when processing malformed markdown-like URLs in HTML attributes (e.g., `//[domain.com/path](http://domain.com/path)`). New `sanitize_markdown_url()` function detects and extracts actual URLs from markdown syntax that wasn't properly converted in source HTML. Applied to both link `href` and image `src` attributes (fixes issue #186).

### Changed

- **Ruby gem build process**: Added `vendor-html-to-markdown.sh` script that creates standalone vendor workspace before gem packaging. Ruby native `Cargo.toml` now references vendored path for maximum reproducibility and build reliability.

## [2.22.5] - 2026-01-16

### Fixed

- **Core**: Added `#[serde(default)]` attribute to `ConversionOptions` struct to enable partial JSON deserialization. This allows deserializing JSON with only a subset of fields specified, using default values for missing fields. Fixes compatibility with language bindings (C#, Go, Java) that serialize partial configuration objects.

## [2.22.4] - 2026-01-15

### Fixed

- **Core**: Fixed `br_in_tables` option not being respected correctly. HTML `<br>` tags in table cells now properly convert to markdown line breaks (spaces or backslash style based on `newline_style` option), while block elements (divs, paragraphs) continue to generate literal `<br>` tags when needed for rowspan scenarios (issue #184)
- **WASM**: Updated GitHub Pages demo to v2.22.4 with latest BR tag handling fixes

## [2.22.3] - 2026-01-14

### Fixed

- **Python**: Exposed `skip_images` option in `ConversionOptions` API, including type stub files (.pyi) for proper type checking support (issue #183)
- **Elixir**: Added `skip_images` option to `HtmlToMarkdown.Options` module (was completely missing from Elixir binding)
- **Core**: Fixed `<br>` tags being output literally in table cells instead of converting to proper Markdown line breaks. Table cell paragraph and div separators now respect `newline_style` option (issue #184)

## [2.22.2] - 2026-01-13

### Fixed

- **Ruby gem standalone build** - Fixed Ruby gem failing to build when installed from RubyGems. Removed `lints.workspace = true` (which requires workspace context) and added inline lint configuration. This resolves issue #181.
- **Ruby gem version pinning** - Changed `html-to-markdown-rs` dependency from loose semver (`"2.x.x"`) to exact pin (`"=2.22.2"`) to prevent older gems from pulling incompatible newer crate versions.
- **Version sync script** - Updated `sync_versions.py` to preserve exact version pin prefix (`=`) when syncing Ruby gem dependencies.

## [2.22.1] - 2026-01-13

### Fixed

- **Java Maven Central publishing** - Fixed Maven Central deployment by adding proper `publish` profile with `central-publishing-maven-plugin` configuration. The plugin is now correctly activated with `-Ppublish` flag and uses `ossrh` server credentials.
- **Java Spotless formatting** - Updated google-java-format to 1.28.0 for Java 25 compatibility.

## [2.22.0] - 2026-01-13

### Fixed

- **C FFI visitor implementation** - Fixed `html_to_markdown_convert_with_visitor` to properly use the visitor handle during conversion instead of discarding it. Previously the visitor was created but the plain `convert()` function was called instead of `convert_with_visitor()`.
- **C# visitor callbacks** - P/Invoke bindings now correctly invoke visitor callbacks during HTML-to-Markdown conversion (42/42 tests passing).
- **Go visitor callbacks** - Removed regex-based post-processing workaround; Go bindings now use real FFI visitor callbacks with proper struct field ordering.
- **PHP visitor callbacks** - Wired up `PhpVisitorBridge` to pass visitor to Rust core instead of ignoring the visitor parameter.
- **Java visitor callbacks** - Added Panama FFI upcall stubs for all 38 visitor callbacks, enabling full visitor pattern support (95/95 tests passing).

### Added

- **Java `VisitorCallbackFactory`** - New class that creates Panama FFI upcall stubs for visitor callbacks, enabling Java code to receive callbacks from the Rust core during conversion.
- **Java `HtmlToMarkdown.convertWithVisitor()`** - Public API method for converting HTML with a custom visitor implementation.

## [2.21.1] - 2026-01-13

### Added

- **Serde serialization support for ConversionOptions** - Added `Serialize` and `Deserialize` traits to `ConversionOptions`, `PreprocessingOptions`, and all related structs. Enables JSON serialization/deserialization with camelCase field naming and lowercase string enum representations.

### Changed

- **Major refactor: Complete Phase 1 modular architecture** - Restructured core converter into modular handler components:
  - Extracted block element handlers (block-level HTML elements)
  - Extracted inline element handlers (2,363 lines of focused code)
  - Extracted table, list, and media handlers (2,528 lines)
  - Extracted semantic and form handlers (1,532 lines)
  - Improved code organization and maintainability across all language bindings
- **Unified FFI bindings architecture** - Consolidated common binding logic into shared crate, reducing duplication across Python, TypeScript, Ruby, PHP, Go, and Java bindings
- **Added visitor callback code generation system** - FFI now supports dynamic visitor callbacks for all language bindings (Python, Ruby, PHP, Elixir, etc.)
- **Enhanced preprocessing system** - Footer and nav element removal now integrated into preprocessing pipeline
- **Improved custom element detection** - Enhanced `has_custom_element_tags` to accurately detect only tag names with hyphens

### Internal

- Updated dependencies across all language bindings (Python, Ruby, PHP, JavaScript, Go, etc.)
- Refactored benchmark harness to modularize script adapters and reduce code duplication
- Refactored performance examples to extract and reuse shared utilities
- Improved sync_versions.py to handle all internal workspace dependency version pins
- Refactored README generation script to modularize template handling
- Improved clippy lint handling and CI coverage workflows
- Added documentation to Node.js binding example files

## [2.21.0] - 2026-01-10

### Added

- **`skip_images` configuration option** - New option to skip all `<img>` elements during conversion, enabling greater control over image handling in the output.
- **Optional visitor parameter across all convert functions** - Unified API for applying visitor patterns to all conversion modes:
  - `convert(html, options, visitor)` - Basic conversion with optional visitor
  - `convert_with_inline_images(html, options, image_cfg, visitor)` - Inline image extraction with optional visitor
  - `convert_with_metadata(html, options, metadata_cfg, visitor)` - Metadata extraction with optional visitor
- **Visitor pattern integration with advanced features** - Support for using visitor pattern simultaneously with inline images and metadata extraction, providing complete control over the conversion process.
- **Comprehensive test coverage** - Added tests validating `skip_images` functionality and visitor pattern integration across all conversion functions and language bindings.

### Changed

- **Visitor parameter unified across all APIs** - The visitor parameter is now optional on all conversion functions, enabling consistent API design across basic, inline-images, and metadata extraction paths.
- **Improved feature-gated architecture** - Refined the feature gate handling for better flexibility when combining visitor patterns with other optional features.

### Deprecated

- **`convert_with_visitor()` function** - Deprecated in favor of passing visitor as an optional parameter to `convert()`. The dedicated function will be removed in a future major release. Use `convert(html, options, visitor)` instead.

### Fixed

- **Unused dependency warnings in npm packages** - Resolved unused dependency warnings reported during builds of JavaScript/TypeScript packages.
- **Feature gate handling for visitor combinations** - Fixed issues with feature gate combinations when using visitor patterns alongside inline images and metadata extraction.

## [2.20.1] - 2026-01-09

### Code Quality

- **Resolved all clippy warnings comprehensively**: Fixed 207+ clippy pedantic/nursery warnings across entire workspace
  - Removed blanket `#![allow(clippy::pedantic)]` directives from all crate roots
  - Fixed trivial copy pass-by-ref issues in converter functions
  - Added missing documentation sections (# Errors) to public APIs
  - Fixed doc markdown formatting (added backticks to technical terms)
  - Applied selective allows only for architecturally justified cases
  - FFI/binding layers use targeted allows due to interop constraints
  - Core library maintains strict clippy compliance
- **Updated workspace lint configuration**: Changed pedantic lints from deny to warn to allow module-level selective overrides
- **Dependency modernization**: Migrated from `once_cell::sync::Lazy` to stdlib `std::sync::LazyLock` (stabilized in Rust 1.80+)

## [2.20.0] - 2026-01-05

### Dependencies

- **Updated reqwest to 0.13.1**: Migrated to new rustls defaults
  - rustls is now the default TLS backend (previously native-tls)
  - aws-lc is the default crypto provider (previously ring)
  - rustls-platform-verifier is used by default for root certificates
  - All reqwest features updated to new naming conventions
- **Updated development dependencies**: Updated pnpm packages, Ruby gems, and pre-commit hooks
  - oxlint pre-commit hook updated from v1.36.0 to v1.37.0
  - All language bindings dependencies refreshed

### Infrastructure

- **Fixed C# package update task**: Updated dotnet list command to specify project files explicitly
  - Prevents "project or solution file could not be found" errors
  - Now checks both HtmlToMarkdown.csproj and HtmlToMarkdown.Tests.csproj individually

## [2.19.8] - 2026-01-05

### Bug Fixes

- **Blockquote newline preservation**: Fixed Issue #176 - Newlines were not preserved when block elements like `<strong>` were directly adjacent to `<blockquote>` elements
  - Blockquotes now add proper spacing before and after themselves
  - Fixed blockquote+paragraph spacing to match CommonMark spec
  - Fixed blockquote+HR spacing to avoid extra newlines
  - Added comprehensive regression tests to prevent future regressions
  - Maintains CommonMark compliance (132/132 tests passing)

### Improvements

- **Debug logging cleanup**: Removed extensive debug logging from hOCR processing and core converter
  - Removed ~30 debug eprintln! statements that were spamming output
  - Removed unused debug parameters from hOCR functions (parse_properties, reconstruct_table, extract_hocr_document, etc.)
  - Cleaner output and reduced noise during HTML to Markdown conversion

## [2.19.7] - 2026-01-03

### Improvements

- **Homebrew bottle CI debugging**: Added verification steps to diagnose artifact upload/download issues
  - Added verification after bottle creation to confirm file exists in workspace
  - Added `if-no-files-found: error` to fail fast if bottle file not found during upload
  - Added verification after artifact download to show what was actually retrieved
  - These steps will help identify why Homebrew bottle artifacts aren't being found in release workflow

## [2.19.6] - 2026-01-03

### Bug Fixes

- **WASM npm package publishing**: Fixed Issue #172 - WASM package was published with only 3 files (LICENSE, package.json, README.md) instead of 25 files
  - Root cause: publish workflow downloaded WASM artifact tarballs but never extracted them before running `npm publish`
  - Added extraction step in `.github/workflows/publish.yaml` to unpack dist/, dist-node/, and dist-web/ directories
  - Added safeguard to remove .gitignore files from dist directories that could exclude content
  - Complete package now includes all WASM binaries and JavaScript wrappers (7.8 MB unpacked)

## [2.19.5] - 2025-01-02

### Bug Fixes

- **Homebrew bottle naming**: Fixed bottle filename format to match Homebrew convention
  - Changed from double-dash (`html-to-markdown--2.19.x`) to single-dash (`html-to-markdown-2.19.x`)
  - Homebrew constructs bottle URLs based on formula name and version, expecting single dash separator
  - Fixes bottle download failures when installing via `brew install`

## [2.19.4] - 2025-01-02

### Bug Fixes

- **Homebrew formula publishing**: Fixed publish workflow script that updates the Homebrew tap formula
  - Corrected bottle block deletion regex (was looking for `# bottle do` instead of `bottle do`), preventing duplicate bottle blocks from accumulating on each release
  - Added automatic source tarball SHA256 computation and formula update to ensure correct checksums
  - Formula now properly replaces old bottle blocks with new ones rather than appending

## [2.19.3] - 2025-01-02

### Bug Fixes

- **Table image processing**: Fixed Issue #175 - images inside Blogger-style HTML tables (e.g., `<table class="tr-caption-container">`) were being stripped during conversion. Enhanced table scanner to recognize images as content and properly process non-table elements like `<a>` and `<img>` that are direct children of table elements.
- **WASM npm package**: Fixed Issue #172 completely - package was published but missing all WASM binaries and JavaScript wrappers (only 23 KB with 3 files). Created `.npmignore` to include `dist/`, `dist-node/`, and `dist-web/` directories that were excluded by `.gitignore` during npm publish.
- **PHP Packagist publishing**: Fixed version mismatch that caused Packagist to reject v2.19.2 tag. Updated `sync_versions.py` to synchronize both root `composer.json` and `packages/php/composer.json`.
- **Test apps**: Fixed relative fixture paths in C#, Java, and Elixir test apps. Updated Elixir tests to handle tuple-returning API. Added Java native library path configuration.

### Infrastructure

- Enhanced `sync_versions.py` script to update root `composer.json` for Packagist validation
- Recreated v2.19.2 git tag with correct composer.json version

## [2.19.2] - 2025-12-30

### Bug Fixes

- **WASM npm package**: Fixed missing `.d.ts` files in published package by updating `files` field with glob patterns (fixes #172)
- **Test apps**: Fixed API mismatches across all language test apps (Python, Node.js, WASM, Go, Java, C#)
  - Python: Changed `convert_html_to_markdown()` to `convert()`
  - Node.js: Updated to scoped package `@kreuzberg/html-to-markdown`
  - WASM: Changed `convertHtmlToMarkdown()` to `convert()`
  - Go: Updated FFI version from 2.16.0 to 2.19.1 with enhanced error handling
  - Java: Added Maven wrapper files for portability
  - C#: Updated to `KreuzbergDev.HtmlToMarkdown` package name
- **Packagist publishing**: Added automated workflow job and moved `composer.json` to repository root
- **Maven Central publishing**: Fixed GitHub secrets configuration (corrected `GPG_PASSPHRASE` typo)
- **Go bindings**: Enhanced FFI download error messages with actionable troubleshooting guidance
- **Pre-commit hooks**: Fixed Go linting errors (errcheck, staticcheck) and formatting violations

### Infrastructure

- Created new WASM test app with comprehensive smoke and integration tests
- Updated all test apps to version 2.19.0 for consistent validation
- Enhanced Java package formatting to comply with 120-character line limit

## [2.19.1] - 2025-12-29

### Bug Fixes

- **Go formatting**: Applied `gofmt` to `packages/go/v2/htmltomarkdown/visitor.go` to align constant declarations
- **Java tooling**: Upgraded google-java-format from 1.21.0 to 1.25.2 for Java 25 compatibility
- **Homebrew distribution**: Added html-to-markdown formula to kreuzberg-dev homebrew tap for CLI installation

## [2.19.0] - 2025-12-29

### Breaking Changes

- **npm package namespace**: All npm packages now use the `@kreuzberg` scope for better organization and discoverability
  - `html-to-markdown-node` → `@kreuzberg/html-to-markdown-node`
  - `html-to-markdown-wasm` → `@kreuzberg/html-to-markdown-wasm`
- **Java package namespace**: Java binding now uses `dev.kreuzberg` package prefix instead of `com.goldziher`
  - Updated all Maven artifact IDs and Java package names for semantic clarity
  - Affects all public classes and imports in Java projects
- **C# namespace**: C# bindings now use `KreuzbergDev` namespace instead of `Goldziher`
  - Updated NuGet package ID to `KreuzbergDev.HtmlToMarkdown`
  - All public types now under `KreuzbergDev.HtmlToMarkdown` namespace

### Features

- **XML table support (TEI/JATS formats)**: Added support for TEI (Text Encoding Initiative) and JATS (Journal Article Tag Suite) table elements
  - `<row>` elements for table rows with proper cell grouping and nesting
  - `<cell>` elements with full attribute support including `role="head"` for header cells
  - `<graphic>` elements for figure/image references within cells and content blocks
  - Proper table structure preservation when converting scientific markup formats
  - Aligns with CommonMark table output while respecting source document semantics

### Bug Fixes

- Fixed Clippy warnings across Rust core and all binding crates for cleaner compilation
- Improved test suite with enhanced error messages and edge case coverage
- Refined table element handling for robustness with malformed markup

### Infrastructure

- **CI/CD improvements**: Enhanced C# workflow for improved reliability and platform coverage
- **Release distribution**: Added Homebrew bottle support for macOS CLI binary distribution
- **Version synchronization**: All language bindings now synchronized to v2.19.0

## [2.18.0] - 2025-12-28

### Added

- **Visitor Pattern**: Complete implementation of visitor pattern for custom HTML element processing across all 8 language bindings (Python, TypeScript, Ruby, PHP, Go, Java, C#, Elixir)
  - Synchronous and asynchronous visitor support (where applicable per language)
  - 40+ visitor methods with hooks for every HTML element type (text, links, images, headings, lists, tables, code blocks, and more)
  - `NodeContext` provides element metadata: tag name, attributes, depth, parent tag, inline status, and sibling index
  - Control flow options: Continue, Custom (provide custom markdown), Skip, PreserveHtml, or Error
  - Element lifecycle callbacks: `visit_element_start` and `visit_element_end` for complete control
  - **Python**: Full async visitor support with `convert_with_async_visitor()` function
  - **TypeScript**: Async visitor with full type definitions
  - **Ruby**: Sync visitor implementation with complete RBS type definitions
  - **PHP**: Full visitor support with PHPStan level 9 compliance
  - **Go**: Thread-safe visitor registry with markdown post-processing
  - **Java**: Panama FFI visitor (JDK 21+)
  - **C#**: P/Invoke visitor with cross-platform compatibility
  - **Elixir**: Rustler NIF visitor implementation

### Fixed

- **HTML parsing for modern websites**: Fixed issue where JavaScript-heavy websites (like Reuters) would lose article body content during conversion (GitHub issue #167)
  - The parser was incorrectly interpreting HTML-like strings inside `<script>` tags as actual HTML elements
  - Script and style tags are now properly stripped during preprocessing while preserving JSON-LD metadata
  - No performance impact on conversion speed
- **Python API**: Fixed missing `ConversionOptionsHandle` export in public API (GitHub issue #166)
  - Users can now import `ConversionOptionsHandle` directly from the `html_to_markdown` package
  - Maintains backward compatibility with existing `OptionsHandle` import

## [2.17.0] - 2025-12-22

### Added

- Go binding now auto-downloads the native FFI library from GitHub Releases with cache/override controls.
- Release pipeline now publishes per-platform Go FFI artifacts for Go installs.

## [2.16.1] - 2025-12-22

### Fixed

- Fast-path plain-text conversions now honor escape flags (asterisks/underscores/misc/ASCII).
- Fast-path plain-text conversions now normalize whitespace and trim trailing spaces.
- Fast-path plain-text conversions now respect `strip_newlines`.
- Python CLI proxy now only applies v1 translation defaults when v1-only flags are present.

## [2.16.0] - 2025-12-22

### Added

- Profiling harness and workflow for Rust core and bindings with consolidated flamegraph output.
- Benchmark scenarios for inline images, metadata extraction, and raw metadata output across fixtures.
- WASM profiling support with warmups and stable flamegraph parsing.
- FFI byte-based conversion path plus metadata-raw benchmark coverage.

### Changed

- Bench harness now supports expanded fixture coverage and results consolidation.
- Java benchmarks align on JDK 25 for consistent profiling runs.

### Fixed

- Node benchmark harness now runs from the package directory and uses native bindings.
- Profiling stability fixes across Go, Elixir, Java, and WASM adapters.
- Binary input detection now flags compressed/magic signatures and UTF-16 data with clearer errors.

### Performance

- Rust core conversion: metadata extraction, inline image handling, tag/whitespace caches, and text assembly hot paths.
- Bindings interop: tighter metadata serialization/deserialization paths.
- Rust bench harness (local, Apple M4): median ops/sec improved 18.8× on Wikipedia fixtures (53.7 → 1009.1).

## [2.15.0] - 2025-12-19

### Fixed

- Rust core: clamp table `colspan`/`rowspan` to prevent pathological allocations on malformed HTML.
- Rust core: reject binary-like inputs early to avoid OOMs when non-HTML data is passed to `convert`.

## [2.14.11] - 2025-12-16

### Fixed

- C# (NuGet): fix `ConvertWithMetadata()` deserialization for metadata enums (`link_type`, `image_type`, `data_type`, `text_direction`) by honoring the JSON wire values.

## [2.14.10] - 2025-12-16

### Fixed

- Python: release the GIL during native conversion so `ThreadPoolExecutor` parallelism doesn't regress performance, and always build the extension with metadata support (so `convert_with_metadata` is always available).

## [2.14.9] - 2025-12-16

### Fixed

- Structured data: JSON-LD is now extracted from `<script type="application/ld+json">` tags (including when placed in `<head>`), preserving the script contents for parsing.

## [2.14.8] - 2025-12-15

### Fixed

- Rust crate (`html-to-markdown-rs`): enable the `metadata` feature by default so `convert_with_metadata` is available without extra Cargo features.

## [2.14.7] - 2025-12-15

### Fixed

- Elixir (macOS): package now ships a `.cargo/config.toml` so Rustler can compile without requiring user-specific linker flags.

## [2.14.6] - 2025-12-15

### Fixed

- RubyGems publish: skip duplicate `ruby`-platform gems when multiple CI jobs produce identical artifacts for the same version.
- Hex publish: ensure the Rust core crate is staged into the Elixir package before publishing.

## [2.14.5] - 2025-12-15

### Fixed

- RubyGems publish: prevent corrupted gem pushes by downloading `rubygems-*` artifacts into separate directories (no merge), and publishing gems recursively with an integrity check.

## [2.14.4] - 2025-12-15

### Fixed

- Release pipeline: build the C# `osx-x64` native FFI library on `macos-15-intel` (macOS-13 runners are retired), unblocking NuGet publication.
- Elixir (Hex): package now vendors the Rust core crate so `mix deps.get && mix test` works outside this monorepo.

## [2.14.3] - 2025-12-15

### Fixed

- **Issue #150 / Discord report**: Python now always exports `convert_with_metadata` (no more `ImportError` on import).
- **Issue #149**: Blockquote text now word-wraps when `wrap=true`.
- **FFI JSON parity**: Metadata enums now serialize as snake_case (e.g. `external`, `relative`) to match cross-language expectations.
- PHP test runner now always builds the extension with the `metadata` feature enabled (avoids missing `html_to_markdown_convert_with_metadata` when the workspace was built with `--no-default-features`).

### Added

- Elixir: `convert_with_metadata/3` + `MetadataConfig` backed by the Rust metadata extractor.

### Changed

- WASM: metadata bindings are enabled by default so the published npm package exports `convertWithMetadata`.
- C# publish pipeline: stage native `html_to_markdown_ffi` libraries into the NuGet package under `runtimes/*/native`.
- Go: module path now uses semantic import versioning (`.../packages/go/v2`), and docs/examples were updated accordingly.
- Java: add `.sdkmanrc` for Java 25 + Maven 4; keep `maven-source-plugin` on `3.3.1` because `4.0.0-beta-1` is not compatible with Maven `4.0.0-rc-4`.

## [2.14.2] - 2025-12-13

### Changed

- CI/release automation: extracted Maven installer logic into `scripts/common/install-maven-latest.sh` and applied repo-wide lint/format cleanups.

## [2.14.1] - 2025-12-12

### Fixed

- **Issue #147**: Word wrap now works correctly in list items when using the `-w`/`--wrap` flag. List items with long text are properly wrapped while preserving list structure and indentation for both ordered and unordered lists.
- **Issue #146**: `strip_tags` and `preserve_tags` options now correctly prevent `<meta>` and `<title>` tags from being extracted into YAML frontmatter when `extract_metadata` is enabled.
- **Issue #145**: `strip_newlines=true` no longer causes excessive whitespace around block elements. Structural whitespace is now properly normalized while still removing newlines within paragraph content.

## [2.14.0] - 2025-12-11

### Added

- **CLI Metadata Extraction**: New `--with-metadata` flag with JSON output support for extracting document metadata, headers, links, images, and structured data from HTML documents.
  - Six extraction flags: `--extract-document`, `--extract-headers`, `--extract-links`, `--extract-images`, `--extract-structured-data`
  - JSON output format with markdown and metadata fields: `{"markdown": "...", "metadata": {...}}`
  - Feature enabled by default in CLI builds
- **Go FFI Binding**: Complete `ConvertWithMetadata()` function with typed structs for metadata extraction.
  - 12 Go struct types with JSON tags for type-safe metadata access
  - JSON unmarshaling from FFI layer
  - 18 comprehensive tests covering all metadata types
- **Java FFI Binding**: Complete `convertWithMetadata()` method with Java records for metadata extraction.
  - 11 Java record types using Panama FFM for FFI integration
  - Proper enum types for link/image/text direction (no string-based parsing)
  - Jackson JSON deserialization with error handling
  - 33 comprehensive tests including negative test cases
- **C# FFI Binding**: Complete `ConvertWithMetadata()` method with C# records for metadata extraction.
  - 11 C# record types using P/Invoke for FFI integration
  - System.Text.Json deserialization with proper error handling
  - 23 comprehensive tests covering all metadata types
- **FFI Core API**: New `html_to_markdown_convert_with_metadata()` C function for language-agnostic metadata extraction.
  - JSON serialization for cross-language compatibility
  - Proper memory management and error handling
  - 17 comprehensive tests including memory safety tests

### Changed

- **Documentation Consolidation**: Migrated all standalone METADATA.md files into binding READMEs for improved maintainability.
  - Deleted `packages/typescript/METADATA.md` (480 lines) and `packages/ruby/METADATA.md` (228 lines)
  - Enhanced Python, PHP, TypeScript, Ruby, Go, Java, and C# READMEs with comprehensive metadata sections
  - Root README now includes CLI metadata examples and links to all binding documentation
  - Each binding README is now self-contained with full metadata documentation
- **Type Definitions**: Enhanced metadata type definitions across all language bindings.
  - Go: Complete struct types with JSON tags and godoc comments
  - Java: Proper enum types (LinkType, ImageType, TextDirection) instead of strings
  - C#: Complete record types with XML documentation
  - Python: Fixed `max_structured_data_size` default (100KB → 1MB)
  - TypeScript: Verified dimensions field type (Array<number> for compatibility)
- **Docstrings**: Enhanced documentation strings across all language bindings.
  - Rust core: Improved function and module documentation
  - Python: Enhanced PyO3 docstrings with examples and type hints
  - Ruby: Added YARD tags for better documentation generation
  - PHP: Enhanced docblocks with detailed parameter descriptions

### Fixed

- **FFI Memory Safety**: Fixed critical memory safety bug where error paths could leave dangling metadata pointers.
  - Both markdown and metadata pointers now set to null on any error
  - Added comprehensive memory safety tests
- **CLI Flag Implementation**: Fixed `--extract-document` flag not being mapped to MetadataConfig.
  - Flag now correctly controls document metadata extraction
  - Added 9 new CLI tests for metadata flags
- **Java Type Safety**: Fixed metadata loss and silent failures from missing fields and string-based enums.
  - Added dimensions field to ImageMetadata (was missing, causing 50% metadata loss)
  - Changed linkType, imageType, textDirection from String to proper enum types
  - Fixed exception swallowing in getLastError() - now logs errors and returns descriptive messages
- **Python Default Values**: Fixed incorrect `max_structured_data_size` default (was 100KB, should be 1MB).
  - Now uses `DEFAULT_MAX_STRUCTURED_DATA_SIZE` constant from Rust core
- **Constants Extraction**: Eliminated DRY violations by extracting hardcoded magic numbers.
  - Added `DEFAULT_MAX_STRUCTURED_DATA_SIZE: usize = 1_000_000` constant in Rust core
  - Reused across FFI, CLI, and Python bindings

### Technical Details

- **Test Coverage**: Added 55 new tests across all bindings (71 → 126 total tests, 77% increase)
  - FFI: 13 new tests (4 → 17 total)
  - CLI: 9 new tests (67 → 76 total)
  - Java: 33 new tests (0 → 33 total)
  - Go: 18 tests total
  - C#: 23 tests total
- **Language Compliance**: Achieved 100% compliance across all bindings (up from 50%-100% range)
  - All bindings now correctly implement metadata extraction with proper types
  - Standardized error handling and JSON parsing patterns
- **Documentation**: Added 3,500+ lines of comprehensive metadata documentation across all binding READMEs
  - Migrated 708 lines from TypeScript and Ruby METADATA.md files
  - Enhanced Python and PHP READMEs with extensive examples
  - Added metadata sections to Go, Java, and C# READMEs

## [2.13.0] - 2025-12-10

### Added

- Comprehensive metadata extraction API across all language bindings (Python, TypeScript, Ruby, PHP, WASM).
- New `convert_with_metadata()` function returning both markdown and extracted metadata in a single pass.
- Metadata extraction includes: document metadata (title, description, keywords, author, language, Open Graph, Twitter Card), header hierarchy (h1-h6 with IDs and nesting), link classification (internal/external/anchor/email/phone), image metadata with type detection (data URIs, inline SVGs, external, relative), and structured data (JSON-LD, Microdata, RDFa).
- Python: 51 comprehensive integration tests with full TypedDict type stubs and mypy validation.
- TypeScript: 14 vitest tests with auto-generated NAPI types, runtime feature detection via `hasMetadataSupport()`, and 600+ lines of documentation.
- Ruby: 40+ RSpec tests with complete RBS type signatures and comprehensive API documentation.
- PHP: 21 PHPUnit tests with PHPStan level max compliance and readonly Value Objects.
- WASM: Complete metadata extraction with serde_wasm_bindgen serialization and getter/setter configuration structs.

### Changed

- Enabled metadata feature by default in TypeScript and Ruby bindings for production npm packages and gems.
- Updated all language binding versions to 2.13.0 with synchronized version management.

### Fixed

- Ruby: Added missing wrapper method for `convert_with_metadata` and fixed redundant `?` symbols in RBS type annotations.
- TypeScript: Enabled metadata feature in default Cargo features to ensure npm packages include metadata functionality.
- WASM: Fixed 3 clippy style violations (Default trait implementation, unwrap_or_default usage, struct initialization pattern).

## [2.12.1] - 2025-12-09

### Fixed

- Escape literal `|` characters inside table cells while leaving pipes inside `<code>` and `<pre>` untouched to avoid rendering backslashes in code spans/blocks (fixes #140).
- Handle nested tables without double-escaping pipes and add regression coverage for table cells containing code spans/blocks and nested tables.
- Preserve link-only list items when word wrapping is enabled so nested link lists are not merged or reflowed (fixes #143); added regression fixtures for the reported table-of-contents sample.

### Changed

- Updated dependency locks/manifests to align with the 2.12.1 release.
- Downgraded Java Maven compiler/source plugins back to 3.x to keep CI builds compatible with Maven 3 runners.

## [2.12.0] - 2025-12-08

### Added

- WebAssembly bundler target now supports Cloudflare Workers, Wrangler, and modern bundlers that provide `WebAssembly.Module` instead of `WebAssembly.Instance`.
- Three new WASM usage examples demonstrating different deployment targets:
  - `examples/wasm-node`: Node.js example using dist-node target
  - `examples/wasm-rollup`: Browser example using dist-web target with Rollup
  - `examples/wasm-cloudflare`: Cloudflare Workers example using bundler target with Wrangler

### Changed

- WASM bundler entry point now detects and handles `WebAssembly.Module` instances, building the proper import namespace for wasm-bindgen glue functions.

## [2.11.4] - 2025-12-08

### Fixed

- Node/WASM bundles now post-process their generated JS files to import the shared `WasmConversionOptions` typedef and emit typed doc comments (including typed inline-image `attributes`), so no `any` annotations leak into the published `dist`, `dist-node`, `dist-web`, or docs bundles.

## [2.11.3] - 2025-12-08

### Fixed

- Prevent link-label truncation from splitting multi-byte characters, which previously triggered a `PanicException` in the Python bindings when processing long anchors (resolves #139) and add a regression test to keep the truncation logic safe.

## [2.11.2] - 2025-12-07

### Added

- Explicitly ship typing artefacts in every binding: npm packages export `.d.ts` files by default, Ruby gems now include `sig/**/*.rbs` even when building outside git, and the Python wheel bundles `_html_to_markdown.pyi` plus a `py.typed` marker for static type checkers.

### Fixed

- Cleaned up the Python API’s inline-image helper to avoid redundant casts flagged by `mypy --strict`.
- Tightened PHP docblocks and psalm/phpstan annotations so option arrays use strongly typed shapes instead of `array<string, mixed>`.
- Hardened the WASM, Node, and Python bindings so their `options` argument is fully typed end-to-end (no `any` escapes in `.d.ts` files or placeholder `Any` annotations).

## [2.11.1] - 2025-12-05

### Fixed

- Preserve indentation in `<pre><code>` blocks while safely dedenting whitespace across multibyte characters to avoid panics when leading spaces are non-ASCII; regression fixture added for issue #134. Thanks @bbeardsley for the contribution.

## [2.11.0] - 2025-12-04

### Added

- CLI `--url` flag with optional `--user-agent` override to fetch remote HTML directly, plus charset-aware decoding.
- New GitHub Pages deploy workflow to publish the `docs/` demo from `main`.
- Additional CLI integration tests covering URL fetching (including custom UA, legacy markup, frameset/noframes, cp1252 decoding).

### Changed

- Demo layout now keeps input/output panes equal height and responsive.
- Rust core handles body-like content accidentally nested in `<head>` more gracefully.

## [2.10.1] - 2025-12-02

### Fixed

- Normalize whitespace inside link labels (collapse newlines and extra spaces) so anchors with messy HTML do not emit multi-line `[]` text.
- Flatten block children inside `<a>` (e.g., headings/paragraphs nested in anchors) into a single Markdown link instead of duplicating content; regression tests added for the reported Arabic product card case.

### Changed

- Synced all workspace/package versions to 2.10.1 via `task sync-versions`.

## [2.10.0] - 2025-12-02

### Added

- Centralized panic guarding for all bindings (Python, Node, PHP, WASM, C FFI) using a shared Rust helper so panics surface as language-native errors instead of unwinding across FFI boundaries.
- C FFI now stores the last error per thread and exposes it via `html_to_markdown_last_error`, with panic and UTF-8/null input diagnostics.
- Ruby binding now uses the shared panic guard and emits consistent panic messages; specs cover panic interception across conversion entrypoints.

### Changed

- Wasmtime test harness initializes conversion options via struct literals to reduce clippy noise in CI.

### Fixed

- Rust coverage CI now forces `cargo-llvm-cov` reinstall to avoid cached binary conflicts on GitHub runners.
- PHP smoke tests use the Packagist package name `goldziher/html-to-markdown`, matching README install instructions.

## [2.9.3] - 2025-12-01

### Changed

- **Version sync** – Bumped the entire workspace (Rust, Python, npm, Ruby, Elixir, Java, C#, Go) to 2.9.3 via `task sync-versions` to prep the next patch release.
- **Docs & install commands** – Pointed all Composer references to the published `goldziher/html-to-markdown` package and clarified npm usage to the shipped packages (`html-to-markdown-node` / `html-to-markdown-wasm`).

### Fixed

- **Go lint CI** – Replaced the invalid `go fmt -l` invocations with `gofmt -l` in the Taskfile so `task check`/CI lint runs complete successfully on Go 1.25.

## [2.9.2] - 2025-11-28

### Fixed

- **UTF-8 safety (Fix #127)** – Guarded whitespace trimming against mid-codepoint truncation, eliminating byte-boundary panics on multilingual documents; added fixture and regression test for the reported Ruby-path crash.
- **Image conversion (Fix #128)** – `<img>` elements with `width`/`height` now render as Markdown images instead of raw HTML; regression test covers inline-data URIs with dimensions.

## [2.9.1] - 2025-11-22

### Changed

- **HTML repair fallback** – Minified or malformed pages now reparse via html5ever when inline/block nesting is broken, keeping content that previously vanished (e.g., SPA shells and Hacker News markup).
- **Link label recovery** – Anchor text fallback prefers child formatting or hrefs only when appropriate, preventing empty labels while keeping CommonMark empty-link semantics intact.

### Fixed

- **Layout tables to lists** – Headless tables with mixed column counts/spans or nested tables render as list rows instead of broken Markdown tables, restoring Hacker News output.
- **Issue 121 regressions** – Added fixtures/tests for the empty SPA and malformed Hacker News samples; both now produce full Markdown content without frontmatter noise.

## [2.9.0] - 2025-11-20

### Added

- **Elixir bindings** – New `html_to_markdown` Hex package built with Rustler, exposing the Rust core converter to Elixir with configurable options plus `convert/2` and `convert!/2`.
- **WASM runtime verification** – Added a Wasmtime-backed e2e suite (`e2e/wasm-wasmtime`) plus `task wasm:test:wasmtime` to compile the `html-to-markdown-wasm` artefact for `wasm32-unknown-unknown` and execute it inside Wasmtime. CI now runs these tests to ensure the WASM package works outside the browser runtime.

### Changed

- **Astral `tl` parser** – The HTML parser dependency now points to the actively maintained `astral-tl` fork (still imported as `tl`) so comment parsing stays up to date with upstream fixes.
- **NuGet Package ID** – C# bindings now publish under `Goldziher.HtmlToMarkdown` to avoid clashing with an existing community package.
- **Wasmtime CI Coverage** – The Wasmtime e2e job now runs on Linux x64, Linux arm64, macOS, and Windows runners so every GitHub-hosted architecture executes the WASM tests.

### Fixed

- **PHP PIE source bundle** – Release packaging strips the Wasmtime e2e workspace from the staged `Cargo.toml`, fixing the “failed to load manifest” error in the publish workflow.
- **Horizontal rule rendering** – `<p>…</p><hr>` now emits a blank line before `---` while preserving blockquote spacing so the rule is never misinterpreted as a setext heading.
- **Empty HTML comments** – Zero-width `<!---->` comment nodes are normalized before parsing, so comment placeholders no longer cause the following content to disappear.

## [2.8.3] - 2025-11-15

### Changed

- **Deterministic uv installs** – Every `uv sync` invocation in CI and the Taskfile now runs with `--no-install-workspace`, ensuring Python dependencies are resolved without mutating editable installs before the subsequent build/test steps run.

### Fixed

- **NuGet Publishing** – Release automation now uses GitHub’s trusted publisher flow via `NuGet/login@v1` (OIDC → short-lived API key) before pushing artifacts, removing the dependency on long-lived secrets.
- **Hex Publishing** – The release workflow invokes `mix hex.publish --yes` from `packages/elixir`, with `ex_doc` bundled as a dev dependency so documentation generation works during release.

## [2.8.2] - 2025-11-15

### Changed

- **Unified Version Sync** – `scripts/sync_versions.py` now updates Elixir `@version` declarations, the C# `.csproj`, and the Java `pom.xml` (alongside every npm/pyproject/Gemfile manifest). `task sync-versions` bumps the entire multi-language stack to **2.8.2** in one shot.
- **CI / Release Toolchains** – GitHub Actions now installs Elixir dependencies ahead of Credo and runs on **Elixir 1.19 + OTP 28.1**, matching the README prerequisites and preventing per-job regex recompilation warnings.
- **Taskfile Coverage** – Added `elixir:update` plus full `java:{install,update,test,lint}` tasks so `task setup`, `task update`, `task test`, and `task lint` cover every published runtime (Go, C#, Elixir, Java) just like the CI workflows.

## [2.8.1] - 2025-11-15

### Fixed

- **Release Pipeline** – Bumped all package manifests to v2.8.1 so the publish workflow can push fresh artifacts after the v2.8.0 smoke-test fixes (PyPI, npm, and RubyGems refuse re-uploads of the same version).

## [2.8.0] - 2025-11-15

### Added

- **Java, C#, and Go Bindings (First Release)** – First public release of official Java (JNA), C# (.NET), and Go (CGO) language bindings. All three are integrated into the unified `task bench:bindings` harness and ship with comprehensive performance data in their READMEs. C# leads at ~1.4k ops/sec (≈171 MB/s), Go at ~1.3k ops/sec (≈165 MB/s), and Java at ~1.0k ops/sec (≈126 MB/s) on the 129 KB Wikipedia lists fixture.

### Changed

- **BREAKING: Preprocessing Disabled by Default** – HTML preprocessing is now disabled by default in the library API to prevent silent content loss. Previously, `<nav>`, `<form>`, and related elements (along with all their children) were dropped by default, causing important content inside these tags to be lost. Users who want preprocessing must now explicitly enable it via `PreprocessingOptions { enabled: true, ... }`. The CLI behavior is unchanged (preprocessing has always been opt-in with `--preprocess`).
- **Rust Toolchain Settings** – All crates (including the Ruby binding) now inherit `edition = "2024"` and `rust-version = "1.85"` from the workspace to keep toolchain configuration centralized.
- **GitHub Actions Workflow DRY** – Created 17 reusable composite actions (8 build actions + 9 smoke test actions) to eliminate ~267 lines of duplication between CI and publish workflows.
- **Toolchain Management** – Migrated to official GitHub Actions parameters for Ruby Bundler 2.7.2 and PHP Composer 2.9.1, removing manual installation scripts.

### Fixed

- **Windows PHP Extension Build** – Replaced php-windows-builder orchestration with direct `cargo build` matching ext-php-rs's proven approach, resolving LLVM 19 MMX header incompatibilities and Zend symbol linking errors.
- **Linux PHP Build** – Added php-config path capture and parameter passing to build-php-linux action, fixing "php-config executable not found" errors.
- **Ruby Linux Build** – Set LD_LIBRARY_PATH on Linux builds to match magnus best practices, preventing potential "strings.h not found" errors.
- **golangci-lint CI** – Split golangci-lint pre-commit hook into separate invocations for `packages/go` and `examples/go-smoke` modules, fixing "directory prefix does not contain main module" errors by running each check from within its Go module directory.
- **Windows Go CGO Smoke Test** – Documented MSVC/MinGW toolchain incompatibility and skip Windows Go smoke test with informative message, as Go CGO uses MinGW which cannot link against MSVC-compiled Rust FFI libraries.
- **Go Code Quality** – Removed redundant newline in `examples/go-smoke/main.go` fmt.Println call (detected by newly-working golangci-lint).

## [2.7.2] - 2025-11-12

### Fixed

- **Node/WASM Binding Regression** – HTML preprocessing no longer drops `<html>`, `<head>`, or `<body>` wrappers when their classes resemble navigation chrome, so large Wikipedia fixtures once again emit full markdown (restoring the Vitest length/table expectations for Node bindings and keeping WASM conversions consistent).
- **Cloudflare WASM Initialization** – Bundler builds of `html-to-markdown-wasm` now expose `initWasm()`/`wasmReady` so edge runtimes that instantiate WebAssembly modules asynchronously (Cloudflare Workers, Vite dev servers, etc.) can await initialization before calling `convert()`, eliminating the `__wbindgen_start` runtime error.
- **Footer Retention (Fix #120)** – The Rust preprocessor keeps plain `<footer>` content unless the element carries explicit navigation hints (role/class/id). Python and Rust conversions once again preserve footer copy while still stripping true navigation footers such as `.site-footer` menus.
- **Release Smoke Coverage** – The publish workflow now downloads the built artifacts (Node, WASM, Python wheels, Ruby gems, PHP zips) and reruns the README smoke installs across Linux/macOS/Windows before any packages are uploaded, ensuring we're testing the exact bits we ship.

## [2.7.1] - 2025-11-12

### Added

- **Language-Specific Benchmarks** – Every binding README (Node, WASM, Python, Ruby, PHP, TypeScript) now publishes the latest `task bench:bindings` throughput numbers so runtime documentation stays aligned with the shared fixtures.
- **Examples/Smoke Suite** – Added `examples/{node,wasm,python,ruby,php,rust}-smoke` plus an overview README to exercise both the published artifacts and local builds before a release.

### Changed

- **Docs Accuracy** – Node/WASM READMEs now clearly reference the real npm packages (`html-to-markdown-node`, `html-to-markdown-wasm`) and provide correct import samples.
- **TypeScript README** – Highlights that the CLI wrapper inherits the native Node benchmarks.
- **Repository Hygiene** – `.gitignore` now drops `.venv/`, vendor directories, and nested `node_modules/` so smoke tests and language-specific toolchains don’t dirty the tree.
- **Ruby Build Metadata** – `extconf.rb` uses a relative path for the embedded Cargo crate and the crate’s `Cargo.toml` now declares explicit `edition`, `rust-version`, and dependency pins, allowing `gem install` outside the workspace.
- **Version Sync Script** – `scripts/sync_versions.py` updates every `html-to-markdown-rs` dependency pin (workspace root plus downstream crates) to keep cross-language releases in lockstep.

### Fixed

- **Smoke Test Coverage** – Verified Node, WASM, Python, Ruby (local gem), PHP (Composer path repo), and Rust installs; documented gaps where external registries still need to publish `goldziher/html-to-markdown` or `html-to-markdown` 2.7.1 before release.

## [2.7.0] - 2025-11-12

### Added

- **Zero-Copy Inline Images** – Node/N-API and WASM bindings now expose `convertInlineImagesBuffer` / `convertBytesWithInlineImages`, letting benchmark harnesses feed `Buffer`/`Uint8Array` data directly without creating intermediate JS strings.

### Changed

- **Rust Core Preprocessing** – HTML normalization (self-closing fixes, malformed `<` escaping, script/style stripping) now happens in a single streaming pass that hands owned buffers straight to `tl::parse_owned`, cutting multiple allocations from every conversion.
- **Benchmark Harness + Docs** – Re-ran the cross-language runtime suite after the Rust core optimizations and refreshed the README tables, keeping the published throughput numbers (Node/Python/Rust/WASM/PHP) in sync with `tools/runtime-bench/results/latest.json`.
- **Version Alignment** – Bumped every package (Rust crates, npm packages, PyPI distribution, Ruby gem, PHP extension, WASM bundle) to `2.7.0` via `task sync-versions`.

### Fixed

- **Ruby Benchmark Output** – The Ruby benchmark driver now emits JSON without relying on `json` native extensions, preventing `libruby` incompatibility errors during `task bench:bindings`.
- **Nested `<strong>` Normalization (Fix #111)** – The Rust converter now tracks when bold markup is already active, so nested `<b>`/`<strong>` combinations (including `<mark>`, `<summary>`, `<legend>`) no longer generate `****` artifacts (`<b>bo<b>ld</b>er</b>` correctly becomes `**bolder**`). The CommonMark harness documents the four spec examples that expect stacked markers and skips them accordingly.
- **Heading Whitespace (Fix #118)** – ATX/Setext headings swallow layout-only newlines and indentation inside `<h1>…<h6>` so pretty-printed HTML like `<h2>Heading\n  Text</h2>` renders as a single Markdown heading line.
- **Inline Whitespace Preservation** – Reworked the inline text pipeline so removing zero-width inline elements (e.g., `<input>`, `<script>`, empty `<b>`) no longer collapses surrounding spaces; fixtures like `test_chomp`, `test_form_with_inputs_inline_mode`, and checkbox/task-list rendering now match their expected double-space gaps.
- **DOCTYPE Handling (Fix #119)** – `<!DOCTYPE …>` declarations are stripped during preprocessing so they never leak as stray `PUBLIC…` text in the output, even when metadata extraction is enabled.

## [2.6.6] - 2025-11-10

### Changed

- **Ruby Gem Packaging** – Moved the `html-to-markdown-rb` crate under `packages/ruby/ext/html-to-markdown-rb/native` and pointed `extconf.rb` at that path so every published gem now contains the Cargo sources it needs to compile on install.
- **Documentation Consistency** – Updated the root, crate, and package READMEs to drop references to the unrelated `html-to-markdown` npm package and to consistently list our supported targets (Node, WASM, Python, Ruby, PHP, CLI).
- **Dependency Refresh** – Ran `task update` to upgrade Rust crates, npm packages, Bundler gems, Python requirements, and Composer dependencies across the monorepo.

### Fixed

- **Rust Clippy Lints** – Addressed `clippy::unnecessary-map-or` in the converter and hOCR table builder by using `.is_none_or`, keeping inline-image filtering and column pruning logic clear while allowing `cargo clippy -D warnings` to pass.
- **PIE Source Packaging** – `scripts/package_php_pie_source.sh` now copies `packages/ruby/.../native` into the temporary workspace so the Ruby crate exists when PIE builds the PHP extension.

## [2.6.3] - 2025-11-07

### Fixed

- **Release Pipeline** - Fixed missing `is_tag` output in publish workflow that caused all publishing jobs to be skipped
- **Node.js Package Dependencies** - Added missing `optionalDependencies` to html-to-markdown-node package.json to properly link platform-specific binaries
- **Version Management** - Created centralized version sync script (`scripts/sync_versions.py`) to maintain consistency across all package manifests (Rust, Node.js, Python, Ruby, WASM)
- **Cargo Workspace** - Aligned html-to-markdown-rb crate version (was 2.5.7) with workspace version

### Changed

- Added `task sync-versions` command to Taskfile for easy version synchronization across the monorepo

## [2.6.2] - 2025-11-07

### Fixed

- **Table Rowspan Support** - Fixed tables with rowspan cells to correctly duplicate cell content across spanned rows instead of showing empty cells (fixes #116)
- **Node.js Platform Package Publishing** - Fixed workflow to correctly move packed .tgz files to npm directory for publishing
- **Deprecation Warnings** - Updated CLI tests to use `CARGO_BIN_EXE` env var instead of deprecated `cargo_bin` method
- **Deprecation Warnings** - Replaced deprecated `criterion::black_box` with `std::hint::black_box` in benchmarks
- **Clippy Warnings** - Fixed field assignment warnings by using struct initialization with defaults

## [2.6.1] - 2025-11-07

### Fixed

- **Node.js Platform Packages** - Fixed publishing of platform-specific npm packages. The workflow now correctly packs npm directories into .tgz files before publishing, ensuring all platform bindings (linux-x64-gnu, darwin-arm64, win32-x64-msvc, etc.) are published to npm.
- **WASM Package Publishing** - Added proper WASM package publishing workflow to ensure html-to-markdown-wasm is published to npm registry.

## [2.6.0] - 2025-11-07

### Added

- **PHP Extension Support** - Official PHP extension (`goldziher/html-to-markdown`) providing native HTML to Markdown conversion for PHP 8.2+
  - Built with ext-php-rs for high-performance Rust-backed conversion
  - Supports both Thread-Safe (TS) and Non-Thread-Safe (NTS) builds
  - Available for Windows (x86, x64), Linux, and macOS
  - Distributed via PIE (PHP Installer for Extensions) source bundles
  - Prebuilt Windows binaries for PHP 8.2, 8.3, and 8.4
  - Comprehensive test suite with PHPUnit

### Changed

- Refactored PHP build variable names from `HTM2MD_*` to `HTMLTOMARKDOWN_*` for improved clarity in Makefile.frag and config.m4
- Bumped all package versions to 2.6.0 across Rust crates, npm packages, PyPI wheels, Ruby gem, and PHP extension

## [2.5.7] - 2025-11-03

### Added

- Publish Windows PHP extension binaries alongside the PIE source bundle during the release pipeline, enabling one-click installs on every platform.
- Build and archive the CLI binary for Linux (gnu & musl), macOS arm64, and Windows x86_64, plus ship prebuilt WASM bundles (dist/dist-node/dist-web) so every runtime gets first-class artifacts.

### Changed

- Renamed the PHP extension package to `goldziher/html-to-markdown`, moved the Composer metadata to the repository root, and refreshed the documentation/badges for every language target.
- Bumped every package (Rust crates, npm packages, PyPI wheels, Ruby gem, PHP extension) to version 2.5.7.
- Restored the Node.js N-API build matrix so macOS, Windows, and Linux binaries ship automatically with each npm release.

### Fixed

- Preserve ordered list numbering and indentation when list items render headings or HTML tables, so mixed block content stays under the correct bullet (fixes #107).

## [2.5.6] - 2025-10-30

### Changed

- The Ruby gem now packages its own README at the gem root, so RubyGems renders the fully formatted documentation (benchmarks, configuration, CLI notes) without broken links.
- Documentation links: the Ruby README now surfaces GitHub resources (issues, changelog, live demo) alongside feature highlights.
- Bumped every package (Rust crates, npm, PyPI, Ruby gem) to version 2.5.6.

## [2.5.5] - 2025-10-30

### Changed

- Synced documentation: the root README now links to every language guide, and the Ruby README highlights GitHub resources alongside feature docs.
- Gem packaging now reads the README directly for the RubyGems long description while keeping Rubocop happy on all Ruby sources.
- Bumped every package (Rust crates, npm, PyPI, Ruby gem) to version 2.5.5.

## [2.5.4] - 2025-10-30

### Changed

- Polished the Ruby gem messaging and README with performance highlights, configuration examples, and CLI guidance to match other language docs.
- Bumped every package (Rust crates, npm, PyPI, Ruby gem) to version 2.5.4.

## [2.5.3] - 2025-10-30

### Changed

- Publish Ruby gems as precompiled artifacts for Linux (x86_64), macOS (arm64 & x86_64), and Windows (x64) via a matrix GitHub Action, ensuring the CLI executable matches the target platform.
- Split the release workflow into prepare/build/publish stages so dry runs build artifacts without pushing, and trusted publishing now uploads every generated `.gem`.
- Hardened the gem preparation script to clear stale CLI binaries before copying in the platform-specific build output.
- Re-enabled the cross-language release workflow so crates.io, PyPI wheels/sdist, and both npm packages ship alongside the Ruby release.

## [2.5.2] - 2025-10-29

- Fix Ruby gem packaging to embed standalone Cargo manifest (no workspace inheritance) so installs compile out of tree successfully.
- Bump versions across Rust, Node, Python, and Ruby bindings.

## [2.5.1] - 2025-10-28

### Added

- Magnus-based Ruby gem (`html-to-markdown-rb`) with CLI proxy and comprehensive specs.

### Changed

- CI now includes Ruby coverage across macOS, Linux, and Windows, installing the appropriate toolchains (MSYS2 on Windows) for Magnus builds.
- Release workflow prepares the Ruby gem via trusted publishing alongside existing crates/npm packages.

### Fixed

- Bundler version pinned to 2.5.12 to support Ruby 3.2 CI environments.

## [2.5.0] - 2025-10-24

### Added

- **New `preserve_tags` option** - Preserve specific HTML tags in their original HTML form instead of converting them to Markdown. This is useful for complex elements like tables that may not convert well to Markdown. Fixes issue #95.
  - Accepts a list of tag names (e.g., `["table", "form"]`)
  - Preserves all attributes and nested content as HTML
  - Works independently of `strip_tags` - can use both options together
  - Available in all bindings: Rust, Python, Node.js, and WASM
  - Comprehensive test coverage in Rust, Python (pytest), and TypeScript (vitest)

### Changed

- **HTML preprocessing is now enabled by default** - The `PreprocessingOptions.enabled` default changed from `False` to `True` to ensure robust handling of malformed HTML. Users who want minimal preprocessing can explicitly set `enabled=False`.

### Fixed

- **Task list checkbox support** - Fixed sanitizer removing `<input type="checkbox">` elements when `remove_forms` is enabled (default). Checkboxes are now preserved during preprocessing to enable proper task list conversion (`- [x]` / `- [ ]`).
  - Added `input` tag to allowed tags in all sanitization presets (minimal, standard, aggressive)
  - Preserved `type` and `checked` attributes on input elements
  - Fixed pre-existing bug where task list checkboxes were silently removed
- **Data URI support for inline images** - Fixed sanitizer stripping `data:` URLs from image src attributes. Base64-encoded inline images (data URIs) are now preserved during preprocessing.
  - Added `data` to allowed URL schemes in all sanitization presets
  - Fixes `convert_with_inline_images` functionality for base64-encoded images
- **CDATA section handling** - Fixed test expectation for CDATA sections. CDATA sections are now correctly preserved as-is during HTML parsing instead of being partially stripped.
- **hOCR word spacing** - Fixed missing whitespace between `<span class="ocrx_word">` elements in hOCR documents. Words now have proper spaces between them.
  - Modified `OcrxWord` converter to insert space before each word if output doesn't end with whitespace or markdown formatting characters
  - Ensures proper word separation in OCR-generated documents without breaking markdown formatting (e.g., `*text*`, `[alt](url)`, `` `code` ``)
- **hOCR detection with preprocessing** - Fixed hOCR documents not being detected when HTML preprocessing is enabled (new default). The sanitizer now preserves:
  - `class` attributes on all elements (required for detecting hOCR element types)
  - `<meta>` tags with `name` and `content` attributes (required for hOCR metadata detection)
  - `<head>` tags (container for meta tags)
- **hOCR metadata extraction after sanitization** - Fixed metadata extraction failing when preprocessing strips the `<head>` container element. The extractor now finds orphaned meta tags anywhere in the document, not just inside `<head>` elements.
- **`preserve_tags` functionality with preprocessing** - Fixed `preserve_tags` not working when HTML preprocessing is enabled (the new default). The sanitizer now:
  - Accepts the `preserve_tags` list and allows those tags through sanitization
  - Preserves common HTML attributes (`id`, `class`, `style`, `title`, etc.) on preserved tags
  - Prevents `remove_forms` from stripping form tags when they're in the preserve list
  - Ensures tags and attributes survive preprocessing so they can be output as HTML
- **SVG support for inline image extraction** - Fixed SVG elements being stripped by the sanitizer, breaking inline image capture. All sanitization presets now allow:
  - SVG elements: `svg`, `circle`, `rect`, `path`, `line`, `polyline`, `polygon`, `ellipse`, `g`
  - SVG attributes: `width`, `height`, `viewBox`, `cx`, `cy`, `r`, `x`, `y`, `d`, `fill`, `stroke`
  - Enables `convert_with_inline_images` to capture inline SVG elements
- **Robust handling of malformed angle brackets in HTML** - Fixed parser failures when bare `<` or `>` characters appear in HTML text content (e.g., `1<2`, mathematical comparisons). The converter now:
  - Automatically escapes malformed angle brackets that aren't part of valid HTML tags
  - Works correctly with preprocessing both enabled and disabled
  - Handles edge cases like `1<2`, `1 < 2 < 3`, and angle brackets at tag boundaries
  - Fixes issue #94 where content following malformed angle brackets was lost
- Added comprehensive test coverage for malformed angle bracket handling in both Rust and Python test suites
- Fixed WASM build configuration to use correct `getrandom` backend for wasm32-unknown-unknown targets

## [2.4.1] - 2025-10-22

### Fixed

- Ensure npm publishes include the generated Node bindings and platform binaries by running the N-API build during CI.
- Configure WebAssembly builds with the `wasm_js` backend and strip wasm-pack `.gitignore` files so published packages ship the compiled `.wasm` artifacts.

## [2.4.0] - 2025-10-22

### Changed

- Updated Rust workspace dependencies (including `pyo3`) to their latest compatible releases and refreshed lockfiles.
- Normalized hOCR conversion spacing by collapsing stray triple newlines, ensuring generated Markdown matches regression fixtures.

### Fixed

- Corrected the WASM crate to depend on `getrandom`'s `wasm_js` feature, restoring WebAssembly builds.
- Expanded the Node package `files` list so published tarballs now include compiled `.node` artifacts, CommonJS shims, and typings.

## [2.3.4] - 2025-10-12

### Changed

- Incremented all distribution metadata and CLI version checks to 2.3.4 following the previous release tag conflict.
- Regenerated package metadata artifacts for the new patch release.

## [2.3.3] - 2025-10-12

### Added

- Python API now exports inline image helpers (`InlineImage`, `InlineImageWarning`, and `InlineImageConfig`) alongside `convert_with_inline_images`, with dedicated regression tests.
- Node and WASM bindings include inline image extraction examples and TypeScript definitions, validated by Vitest coverage.

### Changed

- Bumped all package metadata (Python, Rust, Node, WASM, CLI) to version 2.3.3 for a synchronized release.

### Fixed

- CLI `--version` test updated to assert the new release number.

## [2.2.0] - 2025-10-11

### Added

- `hocr_spatial_tables` option on `ConversionOptions` (Rust, Python, CLI) with `--no-hocr-spatial-tables` flag to disable spatial table reconstruction when desired.
- New hOCR regression fixtures for complex tables and code blocks to guard against formatting regressions.

### Changed

- Improved hOCR conversion heuristics to distinguish between dense paragraph layouts and true tables, yielding cleaner Markdown for scientific data.
- hOCR code-block detection now preserves fenced formatting, restoring context headings when present.

### Fixed

- CLI `--version` output and package metadata now report version 2.2.0 consistently.

## [2.1.1] - 2025-10-11

### Fixed

- Improve hOCR table reconstruction when tables are represented as paragraphs, ensuring Markdown tables are emitted for Tesseract outputs without explicit `ocr_table` markers.

## [2.1.0] - 2025-10-11

### Added

- **Inline image extraction** - New `convert_with_inline_images()` function to extract embedded images during conversion
  - Supports data URI images (`data:image/*`)
  - Supports inline SVG elements
  - Configurable via `InlineImageConfig` with options for:
    - Maximum decoded size limits
    - Custom filename prefixes
    - SVG capture control
    - Optional dimension inference for raster images
  - Returns `HtmlExtraction` with markdown, extracted images, and warnings
  - Available through both Rust and Python APIs

### Changed

- **Simplified API** - Removed `ParsingOptions` class in favor of direct `encoding` parameter on `ConversionOptions`
- **Automatic hOCR table extraction** - hOCR tables are now extracted automatically without requiring configuration
  - Removed `hocr_extract_tables` option (always enabled for hOCR content)
  - Removed `hocr_table_column_threshold` option (uses built-in heuristics)
  - Removed `hocr_table_row_threshold_ratio` option (uses built-in heuristics)
- Updated pre-commit hook versions (commitlint v9.23.0, pyproject-fmt v2.10.0, ruff v0.14.0)

### Fixed

- **hOCR metadata now uses YAML frontmatter** instead of HTML comments for cleaner markdown output
- **hOCR code organization** - Restructured spatial table reconstruction into dedicated `hocr/spatial.rs` module
- **Conservative table detection** - hOCR spatial table reconstruction now only applies to explicit `ocr_table` elements, preventing false positives
- Windows CLI binary detection - now correctly searches for `.exe` extension on Windows
- CLI binary bundling in Python wheels - binary now included in package for all platforms
- hOCR extractor Rust doctest - added missing import statement
- 928 Python test expectations updated for CommonMark-compliant v2 defaults
- Python 3.14-dev → Python 3.14 stable in CI workflows
- Reorganized wheel preparation script to `scripts/` directory
- Removed duplicate markdown documentation files (BENCHMARKS.md, PERFORMANCE.md, BENCHMARK_RESULTS.md, COMMONMARK_COMPLIANCE.md, REFACTORING_SUMMARY.md)

## [2.0.0] - 2025-10-03

### 🚀 Major Rewrite: Rust Backend

Version 2.0.0 represents a complete rewrite of html-to-markdown with a high-performance Rust backend, delivering **10-30x performance improvements** while maintaining full backward compatibility through a v1 compatibility layer.

### ⚠️ Breaking Changes

#### CommonMark-Compliant Defaults

V2 adopts CommonMark-compliant defaults for better interoperability:

| Option                  | V1 Default   | V2 Default | Reason                           |
| ----------------------- | ------------ | ---------- | -------------------------------- |
| `list_indent_width`     | 4            | 2          | CommonMark standard              |
| `bullets`               | "-"          | "\*+-"     | Cycling bullets for nested lists |
| `escape_asterisks`      | true         | false      | Minimal escaping                 |
| `escape_underscores`    | true         | false      | Minimal escaping                 |
| `escape_misc`           | true         | false      | Minimal escaping                 |
| `newline_style`         | "backslash"  | "spaces"   | CommonMark two-space line breaks |
| `code_block_style`      | "backticks"  | "indented" | CommonMark 4-space indent        |
| `heading_style`         | "underlined" | "atx"      | CommonMark `#` headings          |
| `preprocessing.enabled` | false        | false      | No change (opt-in)               |

**Migration**: If you relied on v1 defaults, explicitly set options to match v1 behavior.

#### Removed CLI Flags

The following v1 CLI flags are **not supported** in v2. The Python CLI proxy will raise helpful error messages when these flags are used:

| Removed Flag | Reason                | Migration                                 |
| ------------ | --------------------- | ----------------------------------------- |
| `--strip`    | Feature removed in v2 | Remove flag (feature no longer available) |
| `--convert`  | Feature removed in v2 | Remove flag (feature no longer available) |

**Note on Redundant Flags**: The following v1 flags are redundant in v2 (they match the defaults) but are **silently accepted** for backward compatibility:

- `--no-escape-asterisks`, `--no-escape-underscores`, `--no-escape-misc` (v2 defaults to minimal escaping)
- `--no-wrap` (v2 defaults to no wrapping)
- `--no-autolinks` (Rust CLI defaults to no autolinks)
- `--no-extract-metadata` (Rust CLI defaults to no metadata extraction)

These flags can be safely removed from your commands, or you can leave them for compatibility.

**Note**: The Rust CLI only supports positive flags (e.g., `--escape-asterisks`, `--autolinks`, `--wrap`). Negative flags (`--no-*`) are only supported through the Python CLI proxy for v1 compatibility.

#### CommonMark-Compliant List Formatting

- **Tight lists no longer have blank lines before nested sublists** - This follows the [CommonMark specification](https://spec.commonmark.org/) for list formatting
- Previous behavior (v1): `* Item 1\n\n    + Nested\n`
- New behavior (v2): `* Item 1\n    + Nested\n`
- **Why**: CommonMark specifies that tight lists (lists without blank lines between items) should not have blank lines before nested sublists
- **Impact**: Generated markdown will render identically in CommonMark-compliant renderers but may look different in source form
- **Migration**: If you need the old behavior for specific platforms, you can post-process the output or use loose lists (with blank lines between items)

### Added

#### Core Rust Implementation

- **Complete Rust rewrite** of HTML-to-Markdown conversion engine using `scraper` and `html5ever`
- **Native Rust CLI** with improved argument parsing and validation
- **PyO3 Python bindings** for seamless Rust/Python integration
- **Automatic hOCR table extraction** with built-in heuristics for OCR documents

#### New V2 API

- Clean, modern API with dataclass-based configuration
- `convert(html, options, preprocessing)` - primary API entry point
- `ConversionOptions` - comprehensive conversion settings (now includes `encoding`)
- `PreprocessingOptions` - HTML cleaning configuration
- Legacy parsing options removed in favour of explicit encoding on `ConversionOptions`
- Improved type safety with full type stubs (`.pyi` files)

#### V1 Compatibility Layer

- **100% backward compatible** v1 API through compatibility layer
- `convert_to_markdown()` function with all v1 kwargs
- Smart translation of v1 options to v2 dataclasses
- CLI argument translation for v1 flags
- Clear error messages for unsupported v1 features

#### Testing & Quality

- **77 new tests** for v1 compatibility (32 bindings + 26 CLI + 19 integration)
- Comprehensive integration tests with actual CLI execution
- Wheel testing workflow for cross-platform validation
- Python 3.10, 3.12, 3.14-dev test matrix
- Dual coverage reporting (Python + Rust)

#### CI/CD Improvements

- Shared build-wheels action for consistent wheel building
- Test-wheels workflow with full test suite on built wheels
- Rust coverage with `cargo-llvm-cov`
- Python coverage in LCOV format
- Automated wheel building for Python 3.10-3.13

### Changed

#### Performance

- **60-80x faster** than v1 for most conversion operations (144-208 MB/s throughput)
- Memory-efficient processing with Rust's zero-cost abstractions
- Optimized table handling with rowspan/colspan tracking
- Faster list processing with unified helpers

#### Architecture

- Removed Python implementation (`converters.py`, `processing.py`, `preprocessor.py`)
- Migrated to Rust-based conversion engine
- Simplified Python layer to thin wrapper around Rust bindings
- CLI now proxies to native Rust binary with argument translation

#### API Design

- More explicit configuration with separate option classes
- Better separation of concerns (conversion/preprocessing/parsing)
- Clearer parameter naming and organization
- Improved error messages and exception handling

### Removed v1 Features

The following v1 features were **removed** in v2:

- `code_language_callback` - Removed (use `code_language` option for default language)
- `strip` option - Removed (use preprocessing options instead)
- `convert` option - Removed (all supported tags are converted by default)
- `convert_to_markdown_stream()` - Removed (html5ever does not support streaming parsing)

### Not Yet Implemented

- `custom_converters` - Planned for future release with Rust and Python callback support

### Migration Guide

#### For Most Users (No Changes Needed)

If you're using the v1 API, your code will continue to work:

```python
from html_to_markdown import convert_to_markdown

# This still works in v2!
markdown = convert_to_markdown(html, heading_style="atx")
```

#### To Use New V2 API (Recommended)

```python
from html_to_markdown import convert, ConversionOptions

options = ConversionOptions(heading_style="atx")
markdown = convert(html, options)
```

#### CLI Changes

V1 CLI flags are automatically translated to v2:

```bash
# V1 style (still works)
html-to-markdown --preprocess-html --escape-asterisks input.html

# V2 style (recommended)
html-to-markdown --preprocess input.html  # escaping is default
```

### Performance Benchmarks

Real-world performance improvements over v1 (Apple M4):

| Document Type       | Size  | V2 Latency | V2 Throughput | Speedup vs V1 (2.5 MB/s) |
| ------------------- | ----- | ---------- | ------------- | ------------------------ |
| Lists (Timeline)    | 129KB | 0.62ms     | 208 MB/s      | **83x**                  |
| Tables (Countries)  | 360KB | 2.02ms     | 178 MB/s      | **71x**                  |
| Mixed (Python wiki) | 656KB | 4.56ms     | 144 MB/s      | **58x**                  |

V2's Rust engine delivers **60-80x higher throughput** than V1's Python/BeautifulSoup implementation across real-world documents.

### Technical Details

#### Rust Crates Structure

```text
crates/
├── html-to-markdown/       # Core conversion library
├── html-to-markdown-py/    # Python bindings (PyO3)
└── html-to-markdown-cli/   # Native CLI binary
```

#### Python Package Structure

```text
html_to_markdown/
├── api.py                  # V2 API
├── options.py              # V2 configuration dataclasses
├── v1_compat.py           # V1 compatibility layer
├── cli_proxy.py           # CLI argument translation
├── _rust.pyi              # Rust binding type stubs
└── __init__.py            # Public API exports
```

### Breaking Changes Summary

None if using v1 compatibility layer. If migrating to v2 API:

1. **Import changes**: `convert_to_markdown` → `convert`
1. **Configuration**: Kwargs → Dataclasses (`ConversionOptions`)
1. **Defaults changed**: See CommonMark-compliant defaults table above
1. **Removed features**: See [Removed v1 Features](#removed-v1-features) section above

### Complete V1 vs V2 Comparison

#### API Differences

| Aspect                  | V1                              | V2                                               |
| ----------------------- | ------------------------------- | ------------------------------------------------ |
| **Primary API**         | `convert_to_markdown(**kwargs)` | `convert(html, options, preprocessing, parsing)` |
| **Configuration**       | Keyword arguments               | Dataclasses (`ConversionOptions`, etc.)          |
| **Type Safety**         | Basic type hints                | Full `.pyi` stubs + generics                     |
| **Compatibility Layer** | N/A                             | `convert_to_markdown()` with v1 kwargs           |

#### Performance Differences

| Document Type       | V1 Throughput | V2 Throughput | Speedup |
| ------------------- | ------------- | ------------- | ------- |
| Lists (Timeline)    | 2.5 MB/s      | 208 MB/s      | **83x** |
| Tables (Countries)  | 2.5 MB/s      | 178 MB/s      | **71x** |
| Mixed (Python wiki) | 2.5 MB/s      | 144 MB/s      | **58x** |
| Average             | 2.5 MB/s      | 177 MB/s      | **71x** |

#### Implementation Differences

| Component        | V1                         | V2                       |
| ---------------- | -------------------------- | ------------------------ |
| **HTML Parser**  | BeautifulSoup4 / lxml      | html5ever (Rust)         |
| **Sanitizer**    | Custom Python              | html5ever DOM filtering  |
| **Conversion**   | Pure Python (~3,850 lines) | Pure Rust (~4,800 lines) |
| **Bindings**     | N/A                        | PyO3                     |
| **CLI**          | Python wrapper             | Native Rust binary       |
| **Dependencies** | bs4, lxml, soupsieve       | None (statically linked) |

#### Output Differences (Default Settings)

| HTML                     | V1 Output                | V2 Output           |
| ------------------------ | ------------------------ | ------------------- |
| `<ul><li>Item</li></ul>` | `*   Item` (4 spaces)    | `- Item` (2 spaces) |
| `<h1>Title</h1>`         | `Title\n=====`           | `# Title`           |
| `Text*with*stars`        | `Text\*with\*stars`      | `Text*with*stars`   |
| `<br>`                   | Two trailing spaces      | Backslash `\`       |
| `<pre>code</pre>`        | ```` ```\ncode\n``` ```` | Indented 4 spaces   |

These differences reflect v2's alignment with CommonMark specification.

### Removed Python Implementation

- Python implementation of HTML conversion
- `html_to_markdown/converters.py` (1220 lines)
- `html_to_markdown/processing.py` (1195 lines)
- `html_to_markdown/preprocessor.py` (404 lines)
- `html_to_markdown/whitespace.py` (293 lines)
- `html_to_markdown/utils.py` (37 lines)
- Several test files migrated to Rust or marked as `.skip`

Total: **~3,850 lines** of Python code removed, replaced by **~4,800 lines** of Rust

### Notes

- **Platform Support**: Wheels built for Linux, macOS, Windows on x86_64
- **Python Version**: Requires Python 3.10+
- **ABI Compatibility**: Uses `abi3` for Python 3.10+ wheel reuse
- **Rust Version**: Built with stable Rust (tested on 1.75+)

______________________________________________________________________

## [1.x] - Previous Versions

For changes in v1.x releases, see git history before the v2 rewrite.

[2.0.0]: https://github.com/kreuzberg-dev/html-to-markdown/compare/v1.x...v2.0.0
