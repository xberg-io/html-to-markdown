<!--
🤖 AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Project: html-to-markdown
Generated: 2026-04-27 18:53:25
Source: .ai-rulez/config.toml

NEVER edit this file - modify .ai-rulez/ content instead
Use MCP server: npx -y ai-rulez@latest mcp
Regenerate: ai-rulez generate

Docs: https://github.com/Goldziher/ai-rulez
Content-Hash: blake3:a020ec9da5d90b96bad7393f9d2c413cb21612f9caabafceb0aef4673daf6a5f
-->

# html-to-markdown

High-performance HTML to Markdown converter with Rust core and polyglot bindings (Python, TypeScript, Ruby, PHP, Go, Java, C#, Elixir, R, WebAssembly, C FFI).

## Rules

### alef-generated-bindings

**Priority:** critical

- Files in `packages/*/` and binding crates are generated or managed by Alef — check `alef.toml` before editing
- `alef.toml` defines: output paths, module names, rename mappings, e2e call overrides, README templates
- Run `alef generate` after changing `alef.toml` — commit both source and generated files
- Never hand-edit generated files; modify `alef.toml` or the Rust source instead
- Fixtures under `fixtures/` feed `tools/e2e-generator/` — never add tests to `e2e/` directly

### bindings

**Priority:** critical

- Bindings are minimal glue: call Rust core, convert types, convert errors — no business logic
- Crate naming: {lib}-py (PyO3), {lib}-node (NAPI-RS), {lib}-rb (Magnus), {lib}-php (ext-php-rs), {lib}-wasm (wasm-bindgen), {lib}-ffi (C FFI)
- Distribution: packages/python/ (PyPI), typescript/ (npm), ruby/ (RubyGems), php/ (Composer), go/ (Go module), java/ (Maven), csharp/ (NuGet)
- Each binding has its own language-native test suite — 80%+ coverage
- Error conversion must preserve context (message + numeric code) at every FFI boundary
- Async: pyo3_asyncio (Python), native #[napi] async (TS), Fiber (Ruby), block on Tokio via FFI (Go/Java/C#)

### ffi-and-language-interop

**Priority:** critical

- Every pointer has one owner, documented with SAFETY comments
- Opaque handles only — never expose Rust types directly, use #[repr(transparent)] wrappers
- Null safety: check ALL pointers before use, return null + error code on failure
- Allocate/free pairs: every \_new() has a matching \_free(), caller owns \*mut
- Every unsafe block has a SAFETY comment: what invariant, why it holds, what breaks if violated
- Generate C headers with cbindgen — CI verifies generated headers match committed
- Semantic versioning for C headers, struct layouts frozen at MAJOR.MINOR boundaries
- All exported functions use #[no_mangle] extern "C"
- Rust Result\<T, E> → host exceptions via dedicated conversion functions at every boundary
- Preserve error context across boundaries: message, numeric code (1000+), source location, cause chain

### rust-polyglot-conventions

**Priority:** high

- Rust 2024 edition, clippy -D warnings, cargo fmt
- Result\<T, E> with thiserror — never .unwrap() or panic in library code
- 95% test coverage (cargo-llvm-cov), unit + integration + doc tests
- rustdoc on all public items, SAFETY comments on all unsafe blocks
- Tokio 1.x exclusively, 'static + Send + Sync bounds on public futures
- All public types FFI-friendly or have FFI equivalents (#[repr(C)])
- Avoid Rust-specific idioms that can't be expressed in target languages
- Cargo.toml version is single source of truth — synced to all binding manifests

### cicd-pipeline-standards

**Priority:** medium

- GitHub Actions split by domain: ci-rust, ci-python, ci-node, ci-ruby, ci-php, ci-go, ci-java, ci-elixir, ci-wasm, ci-validate
- Linting: ruff, clippy, rubocop, biome, phpstan, golangci-lint
- OS matrix: Linux (amd64, arm64), macOS, Windows
- Quality gates: zero warnings, tests pass, coverage thresholds (Rust 95%, bindings 80%)
- Workflows use task commands exclusively, BUILD_PROFILE=ci
- Stages: Validate → Build → Test → Deploy
- Tag-based releases trigger multi-platform builds

### gh-workflows

**Priority:** medium

- PRs: gh pr create, merge with --squash (preferred), link to issues with "Fixes #123"
- Issues: gh issue create with labels and assignees, use templates from .github/ISSUE_TEMPLATE/
- CI: gh run list/view for monitoring, gh run rerun --failed for retries
- Releases: gh release create v1.2.3 --generate-notes
- Never force-merge without CI passing, never skip PR description

### task-automation-build

**Priority:** medium

- Taskfile.yaml is the primary interface for all development tasks
- task setup / task build / task test / task lint / task format / task cov:all / task bench
- Lock files always committed: uv.lock, pnpm-lock.yaml, go.sum, Cargo.lock, composer.lock
- BUILD_PROFILE supports dev/release/ci variants
- Never use manual commands instead of task, never hardcode paths, never skip lock file commits

### containerization-docker

**Priority:** medium

- Multi-stage builds: full toolchain builder → minimal runtime (Alpine/Distroless/Scratch)
- Layer caching: copy Cargo.toml/Cargo.lock first, then source; use BuildKit mount caches
- Security: non-root user, Trivy/Grype scanning (fail on HIGH/CRITICAL), no secrets in image
- Signal handling: tini/dumb-init as PID 1, HEALTHCHECK for orchestrator detection
- Tagging: semantic version + commit SHA, never reuse tags

### gcloud-conventions

**Priority:** medium

- GCP project: kreuzberg-dev, region: europe-west1
- Services: Cloud Run (compute), Cloud Storage (artifacts), Cloud SQL/AlloyDB (Postgres), Pub/Sub (messaging), Artifact Registry (images), Secret Manager (credentials)
- Naming: kreuzberg-{service}-{environment}, buckets: kreuzberg-dev-{purpose}-{environment}
- Auth: workload identity federation — never service account keys
- IAM: least privilege, no owner-level for service accounts
- Use gcloud CLI for all operations — no manual console changes

### monitoring-observability

**Priority:** medium

- Logging: tracing crate (Rust) / structlog (Python), JSON output, key=value — never f-strings
- Spans: #[instrument] macro, context fields (user_id, request_id)
- Levels: ERROR (unrecoverable), WARN (degraded), INFO (state changes), DEBUG (flow), TRACE (off in prod)
- Metrics: Prometheus — counter (requests), gauge (connections), histogram (latency), no high-cardinality labels
- Health: /health endpoint with component status, wired to K8s liveness/readiness probes

### e2e-generator-conventions

**Priority:** high

- Alef CLI (`alef e2e generate`) reads JSON fixtures → generates runnable test suites per language
- E2E generation is configured in `alef.toml` `[e2e]` section with call configurations and language overrides
- Output: e2e/{language}/ directories, each a self-contained test project
- Fixtures: JSON under fixtures/ by category (smoke, chat, streaming, error-handling, etc.)
- Each fixture has unique snake_case id, loaded recursively, sorted by (category, id)
- Run `task e2e:generate:all` to regenerate, `task e2e:test:all` to verify
- Never hand-edit generated files — modify fixtures or `alef.toml` instead

### fixture-schema-design

**Priority:** high

- Fixture fields: id (unique snake_case), category, description, language, source_code, assertions, skip, tags
- Assertion types: equality (field_equals), contains (root_contains_node_type), boolean (tree_not_null), count (root_child_count_min)
- Skip conditions: requires_language, platform — generators emit language-native skip annotations
- IDs unique across ALL fixture files, used as test function names
- One category per file (smoke.json) or organized in subdirectories (fixtures/smoke/)

### generated-code-policy

**Priority:** critical

- All files in e2e/ are generated — DO NOT EDIT, they include a generated-code header
- To change: modify fixtures or generator source, run task generate:e2e, run task test:e2e, commit together
- CI validates freshness: task generate:e2e && git diff --exit-code e2e/

### batch-operations

**Priority:** medium

Group related file reads and writes into single operations. Combine independent tool calls in parallel rather than sequentially. When making multiple edits to the same file, batch them into one edit operation. Prefer multi-file search tools over individual file reads when exploring.

### context-preservation

**Priority:** medium

Record key findings (file paths, function signatures, patterns discovered) before they scroll out of context. Summarize investigation results before acting on them. When working on multi-step tasks, note intermediate decisions and their rationale to avoid re-deriving them later.

### incremental-approach

**Priority:** medium

Start with the smallest viable change, verify it works, then extend. Avoid generating large blocks of speculative code. Build iteratively: implement one piece, test, then move to the next. When uncertain about an approach, prototype the critical part first before committing to the full implementation.

### output-awareness

**Priority:** medium

Limit explanations to 1-3 sentences unless asked for detail. Use code blocks for code, not prose. Omit unchanged code when showing diffs — use comments like `// ... existing code ...` to indicate skipped sections. Never repeat information already visible in context. Prefer short, direct answers over comprehensive walkthroughs.

### task-runner

**Priority:** high

Prefer `task` commands over raw build/test/lint commands when a Taskfile.yaml exists. Task runners provide consistent, documented workflows. Use `task --list` to discover available tasks. Always check for a Taskfile before running manual commands. Standard task names: setup, build, test, lint, format, bench — prefer these conventions. Lock files always committed for reproducible builds.

### ruby-conventions

**Priority:** high

- Ruby 3.2+, `frozen_string_literal: true`, `.ruby-version` file.
- Linting: `rubocop` with auto-fix (120 char max). Plugins: `rubocop-rspec`, `rubocop-performance`.
- Type checking: RBS + `steep check`. Use `rbs prototype` for scaffolding type signatures.
- Testing: RSpec with `describe`/`context`/`it`, `factory_bot` over fixtures, `simplecov` (80%+).
- Security: `brakeman` for SAST (Rails), `bundler-audit` for dependency CVE scanning.
- Error handling: specific exceptions inheriting `StandardError`, no bare `rescue`.
- Composition over inheritance, `Comparable`/`Enumerable` mixins, `case/in` pattern matching.
- Dependencies: `bundler`, commit `Gemfile.lock`, pessimistic `~>` constraints.
- Gem packaging: use `gemspec` with `bundler` gem template, `rake release` for distribution.
- `&:method_name` block shorthand, `=>` pattern matching destructuring (3.2+).
- Anti-patterns: monkey patching, `method_missing` without `respond_to_missing?`, `eval` with user input.

### python-conventions

**Priority:** high

- Python 3.10+, type hints on all public APIs, no `Any` — use `Unknown`/generics.
- Formatting/linting: `ruff` (zero warnings), type checking: `mypy --strict`. Security: `bandit` for SAST.
- Testing: `pytest` with function-based tests, `pytest-cov` (80%+), `hypothesis` for property-based.
- Error handling: specific exceptions only, never bare `except:`, `contextlib.suppress` for intentional ignoring.
- Dataclasses or Pydantic for structured data — avoid raw dicts for known schemas.
- `pathlib.Path` over `os.path` for filesystem operations. Google-style docstrings on public APIs.
- Async: `async`/`await` for I/O, never mix blocking and async, `asyncio.gather()` for concurrency.
- Package management: `uv` with `uv.lock` committed, build with `maturin` or `hatchling`.
- Security: `pip-audit` for dependency CVE scanning. Zero tolerance for critical/high vulnerabilities.
- Logging: `structlog` with key=value pairs — never f-strings in log calls.
- Pattern matching (`match`/`case`) for multi-branch type dispatch (3.10+).
- Anti-patterns: mutable default args, `import *`, global state, `time.sleep` in async.

### typescript-conventions

**Priority:** high

- `strict: true` + `noUncheckedIndexedAccess` in tsconfig, never `any` — use `unknown` with type guards.
- ESM imports only, `const` over `let`, `as const` for literals, `interface` over `type` for objects.
- `import type` for type-only imports to avoid runtime overhead. Discriminated unions for type-safe state.
- Formatting/linting: `biome` + `oxlint`. Type checking: `tsc --noEmit` in CI.
- Testing: `vitest` (80%+ coverage). Runtime validation at system boundaries with `zod`.
- Error handling: discriminated unions for expected errors, throw only for unexpected.
- Package manager: `pnpm` with `pnpm-lock.yaml` committed, build: `tsup` or `esbuild`.
- Monorepo: workspace protocol (`workspace:*`), shared tsconfig base, `pnpm-workspace.yaml`.
- Node.js: `node:` prefix for core modules, `fetch` over `axios`.
- Security: `pnpm audit` for dependency CVE scanning. Zero tolerance for critical/high vulnerabilities.
- Anti-patterns: non-null assertions (`!`), type assertions (`as`), `enum` (use unions), `@ts-ignore`.

### meaningful-assertions

**Priority:** medium

Assert exact expected values, not just truthiness (`assert result == 42`, not `assert result`). Use snapshot testing for complex structured output. Consider property-based testing for functions with wide input ranges. Include descriptive failure messages. Always test error paths and edge cases, not just the happy path.

### tdd-workflow

**Priority:** high

Write tests before writing code, update tests when modifying behavior. When fixing bugs, write a failing test first — RED (failing test) → GREEN (minimal code to pass) → REFACTOR. Wrote production code before the test? Delete it, start over — no exceptions, don't keep as reference. Integration tests for API surfaces, unit tests for business logic, property tests for edge-case-heavy code. Run the full test suite before committing — never push untested code.

### test-alongside-code

**Priority:** high

Write tests when writing code, update tests when modifying behavior. When fixing bugs, write a failing test first (TDD). Use integration tests for the public API surface and unit tests for complex internal logic. Run the full test suite before committing.

### test-independence

**Priority:** high

Tests must be independent and idempotent — runnable in any order, in parallel. No shared mutable state between tests. Use factories or fixtures for setup. Clean up created resources (files, DB rows, env vars) after each test. Never rely on test execution order.

### test-naming

**Priority:** medium

Name tests to describe behavior: `should_return_error_when_input_is_empty`, `test_parse_handles_nested_objects`. Use `describe`/`it` blocks for grouping in languages that support them. Follow `given_when_then` or `should_when` patterns. Test names are specifications — a reader should understand the expected behavior without reading the test body.

### testing-anti-patterns

**Priority:** high

Do not test mock behavior instead of real behavior. Do not add test-only methods to production code. Do not mock what you don't own — wrap it and test the wrapper. Do not test implementation details — test observable behavior. Do not write tests that pass when the code is broken. If a test never fails, it's not testing anything.

### dependency-awareness

**Priority:** high

Audit dependencies before adding them. Prefer well-maintained, widely-used packages with active maintenance. Pin versions and commit lock files. Use language-specific audit tools in CI:
- Rust: `cargo audit`, `cargo deny` (license + advisory policies)
- Python: `pip-audit`, `bandit` (SAST)
- JavaScript/TypeScript: `npm audit`, `pnpm audit`
- Go: `govulncheck`
- Ruby: `bundler-audit`
- PHP: `composer audit`
- Java: OWASP `dependency-check` Maven/Gradle plugin
- C#: `dotnet list package --vulnerable`
- Elixir: `mix_audit`
Zero tolerance for critical/high CVEs. Automate dependency update PRs where possible.

### input-validation

**Priority:** high

Validate and sanitize all external input at system boundaries. Use allowlists over denylists. Validate types, ranges, and formats. Never trust user input.

### least-privilege

**Priority:** medium

Request only necessary permissions. Minimize file system access, network access, and API scopes. Run processes with minimal required privileges.

### secrets-handling

**Priority:** critical

Never hardcode secrets, API keys, tokens, or passwords. Use environment variables or secret management systems. Never log or expose sensitive values. Reject commits containing secrets.

### r-conventions

**Priority:** high

- R 4.1+, tidyverse style guide, `styler` formatting, `lintr` linting.
- Base R pipe `|>` over magrittr `%>%`, prefer base R when tidyverse not needed.
- Package dev: `devtools`/`usethis`/`roxygen2`, `testthat` (80%+ via `covr`).
- Documentation: roxygen2 for inline docs and NAMESPACE management. `@examples` on all exports.
- Vignettes with `knitr`/`rmarkdown` for long-form documentation and tutorials.
- Error handling: `tryCatch()`/`withCallingHandlers()`, `cli::cli_abort()` for user-facing errors.
- Input validation: `checkmate` or `rlang::arg_match()` for function parameter validation.
- Security: `oysteR` for vulnerability scanning of dependencies.
- C/C++: `Rcpp` or `.Call()`, PROTECT/UNPROTECT all R objects. Rust: `extendr`/`rextendr`.
- CRAN: `R CMD check --as-cran` zero warnings/notes, maintain DESCRIPTION with proper versioning.
- Anti-patterns: `eval(parse(text=))` with user input, mixing Rcpp and raw C API, `T`/`F` instead of `TRUE`/`FALSE`.

### anti-patterns

**Priority:** high

No magic numbers — use named constants. No global state — use dependency injection. No inheritance for code reuse — prefer composition. No bare exception handlers — catch specific types. No mocking internal services — use real objects for integration tests. No blocking I/O in async code paths — keep async paths fully async.

### avoid-duplication

**Priority:** medium

Extract shared logic after the third repetition, not before. Three similar lines of code are better than a premature abstraction. When extracting, ensure the shared code has a single reason to change — if two callers would evolve the logic differently, keep them separate. Premature abstraction creates worse coupling than duplication.

### complexity-limits

**Priority:** medium

Enforce concrete limits: max 20 cyclomatic complexity per function, max 4 levels of nesting depth, max 50 lines per function. Use early returns to flatten conditionals. Break complex functions into well-named helpers that each do one thing.

### dead-code

**Priority:** low

Remove dead code instead of commenting it out. Version control preserves history. Commented-out code creates confusion and maintenance burden.

### error-handling

**Priority:** high

Always wrap errors with context describing what operation failed. Never swallow errors silently — either handle, propagate, or log them. Use language-idiomatic patterns: `Result<T, E>` in Rust, `if err != nil` with `fmt.Errorf("doing X: %w", err)` in Go, typed exceptions in Python/Java. Fail fast on unrecoverable errors.

### readability-first

**Priority:** high

Max 120 character line width. Prefer explicit code over clever tricks — if it needs a comment to explain what it does, rewrite it. No abbreviations in public API names (`context` not `ctx` in public signatures, `repository` not `repo`). Keep functions short and focused on a single responsibility.

### php-conventions

**Priority:** high

- PHP 8.2+, `declare(strict_types=1)`, typed properties, union types, enums, readonly classes.
- Formatting: PSR-12 via `phpcs`/`phpcbf` or `php-cs-fixer`. Static analysis: `PHPStan` level 9 or `Psalm`.
- Testing: PHPUnit with `@dataProvider`, 80%+ coverage.
- Error handling: specific exceptions extending `RuntimeException`, constructor promotion for value objects.
- First-class callable syntax (`$fn = strlen(...)`) for callbacks. Arrow functions (`fn() =>`) for simple closures.
- Dependencies: Composer with `composer.lock` committed, `^` version constraints. `composer audit` in CI.
- Security: require `roave/security-advisories` as dev dependency to block vulnerable packages.
- PSR-4 autoloading exclusively — no `require`/`include` for classes.
- Intersection types for strict parameter contracts. Named arguments for readability.
- Anti-patterns: `@` suppression, `eval()`, dynamic property access, `extract()`.

### elixir-conventions

**Priority:** high

- Elixir 1.14+ OTP 25+, pattern matching extensively, `mix format` non-negotiable.
- Configure `.formatter.exs` with `:inputs` and `:import_deps` for consistent formatting across team.
- Linting: `Credo --strict`, type checking: `Dialyzer` via `dialyxir`.
- Security: `mix_audit` for dependency CVE scanning, `sobelow` for SAST. Run both in CI.
- Testing: ExUnit with `describe` blocks, `doctest` for examples, `excoveralls` (80%+).
- Documentation: ExDoc for generation, `@moduledoc` on all modules, `@doc` + `@spec` on public functions.
- `{:ok, value}` / `{:error, reason}` tuples — never raise for expected errors.
- `with` for multi-step ops, guard clauses for function overloading.
- OTP: GenServer for state, Supervisor for fault tolerance, `|>` pipe for transforms.
- Dependencies: commit `mix.lock`, `hex` with `~>` constraints.
- Anti-patterns: mutable state outside processes, long-running NIFs, `String.to_atom` with user input.

### go-conventions

**Priority:** high

- Follow Effective Go and Go Code Review Comments guidelines.
- Handle every error return. Wrap errors with context: `fmt.Errorf("operation failed: %w", err)`.
- Linting: `golangci-lint` with strict config (enable `govet`, `staticcheck`, `errcheck`, `gosec`, `gocritic`). Format with `gofmt`/`goimports`.
- Security: `govulncheck` for CVE scanning, `gosec` for SAST. Run both in CI.
- Testing: table-driven tests with `t.Run()`. Use `t.Parallel()` where safe. Use `testify` for assertions. Coverage with `go test -coverprofile`.
- Naming: use short, descriptive names. Receivers are 1-2 letters. Exported names are descriptive.
- Prefer composition over inheritance. Use interfaces for abstraction (accept interfaces, return structs).
- Keep packages small and focused. Avoid package-level state and `init()` functions.
- Use `context.Context` as first parameter for cancelable operations. Never store contexts in structs.
- Error types: use `errors.Is()`/`errors.As()` for comparison. Define sentinel errors with `errors.New()`.
- Modules: use Go modules. Commit `go.sum`. Use semantic version tags. Prefer stdlib over third-party when reasonable.
- Concurrency: prefer channels over mutexes. Use `sync.WaitGroup` for fan-out. Guard shared state.
- Benchmarking: use `testing.B` benchmarks. Profile with `go tool pprof`. Use `benchstat` for comparison.

### agent-workflow

**Priority:** high

Prefer subagents for non-trivial work — implementation, research, file exploration. Parallelize aggressively — launch independent subagents in a single message. Always critically review subagent output — check actual file changes, verify correctness, fix issues before reporting done. Never trust subagent summaries at face value; the summary describes intent, not necessarily what happened. Work in iterations: delegate → critically review → fix → verify. Run tests after every change — never assume code works without verification.

### communication-style

**Priority:** critical

Be concise and precise — no fluff, no emojis, no unnecessary checklists. PR descriptions: state what changed and why in 1-3 sentences, not bullet-point essays. Issue comments: answer the question directly. Code review: point out the problem and suggest the fix, skip praise and filler. Commit messages: imperative mood, under 72 chars, body explains why not what. Never pad output to appear thorough — brevity is clarity.

### explain-reasoning

**Priority:** medium

Briefly explain your reasoning for non-obvious decisions. State trade-offs when multiple approaches exist. Be transparent about uncertainty.

### minimal-changes

**Priority:** high

Make the smallest change that achieves the goal. Avoid unnecessary refactoring, reformatting, or scope creep. Don't fix what isn't broken.

### no-ai-signatures

**Priority:** critical

Never add AI attribution to commits (no Co-Authored-By AI lines, no "Generated by AI/Claude/GPT"). Never add AI attribution to PR titles or descriptions. Never add AI-generated comments or watermarks in code.

### read-before-write

**Priority:** critical

Read and understand existing files before editing them. Understand the codebase conventions, patterns, and architecture before making changes. Check imports, naming styles, and project structure to ensure new code fits the existing codebase.

### systematic-debugging

**Priority:** high

Never guess at bugs. Trace the root cause backward through the call stack to find the original trigger. Analyze patterns — is this a one-off or systemic? Form a hypothesis and verify it before implementing a fix. No shotgun debugging, no random changes hoping something works.

### verification-before-completion

**Priority:** critical

Never claim success without fresh verification. Run the test and see it pass. Check the file exists. Verify the build succeeds. Evidence before assertions — always. If you can't verify, say so explicitly rather than claiming success.

### verify-before-acting

**Priority:** critical

Verify assumptions before taking action. Check current state (branch, working directory, running processes) before making changes. Confirm file existence before editing. Test that build passes before committing. Never assume — confirm.

### csharp-conventions

**Priority:** high

- .NET 8+ C# 12, file-scoped namespaces, primary constructors, `<Nullable>enable</Nullable>`.
- Formatting: `dotnet format` + Roslyn analyzers, `<TreatWarningsAsErrors>true</TreatWarningsAsErrors>`.
- Style enforcement: `.editorconfig` for consistent rules across team. Use StyleCop.Analyzers or Roslynator.
- Testing: xUnit with `[Theory]`/`[InlineData]`, FluentAssertions, `coverlet` (80%+).
- Benchmarking: BenchmarkDotNet for performance testing — never `Stopwatch` loops.
- `record` types for immutable data, `required` properties, pattern matching in switch expressions.
- Async: `async`/`await` throughout — never `.Result` or `.Wait()`, `ValueTask` for hot paths, avoid deadlocks.
- Security: `dotnet list package --vulnerable` for CVE scanning. Zero tolerance for critical/high.
- Dependencies: NuGet `PackageReference`, commit `packages.lock.json`, `Directory.Build.props` for shared config.
- Collection expressions (`[1, 2, 3]`), `required` modifier, raw string literals for multi-line.
- Anti-patterns: `dynamic`, `object` params, unguarded `catch (Exception)`, `#pragma warning disable`.

### java-conventions

**Priority:** high

- Java 17+ LTS, records for immutable data, sealed classes for restricted hierarchies, pattern matching.
- Google Java Style (4-space, 100 char), `google-java-format`. Static analysis: `Error Prone` + `SpotBugs`.
- Build: Maven or Gradle with wrapper scripts (`mvnw`/`gradlew`). Commit wrapper files.
- Testing: JUnit 5 with `@ParameterizedTest`, AssertJ assertions, `JaCoCo` (80%+ coverage).
- Benchmarking: JMH for microbenchmarks — never use `System.nanoTime()` loops.
- Error handling: specific exceptions only, never `catch (Exception)`, use try-with-resources for `AutoCloseable`.
- `var` for obvious types, `Optional<T>` for returns (never params/fields), `final` fields by default.
- Prefer constructor injection over field injection for DI. Immutable collections: `List.of()`, `Map.of()`.
- Streams for collection transforms, `instanceof` pattern matching, switch expressions.
- Security: OWASP `dependency-check` Maven/Gradle plugin for CVE scanning.
- Dependencies: commit lock files, use BOM for version alignment, avoid `SNAPSHOT` in releases.
- Anti-patterns: public fields, mutable static state, raw types, checked exceptions in lambdas.

### rust-conventions

**Priority:** high

- Rust 2024 edition, `cargo fmt` + `clippy -D warnings`, zero warnings policy.
- `Result<T, E>` with `thiserror` for library errors, `anyhow` for applications. `?` for propagation — never `.unwrap()` in library code.
- Minimize `unsafe` — every block needs `// SAFETY:` comment explaining invariants.
- Prefer `&str` over `String` in params, `Cow<'_, str>` for conditional ownership, `Arc` for shared ownership.
- `impl Trait` in argument position for static dispatch, `dyn Trait` for dynamic dispatch when heterogeneous collections needed.
- Small, focused modules. Use `pub(crate)` for internal visibility. Workspace inheritance for multi-crate repos.
- `#[cfg(test)]` for unit tests, `tests/` for integration, `cargo-llvm-cov` for coverage.
- Benchmarking: `criterion` for microbenchmarks, profile with `cargo flamegraph`.
- Async: `tokio` runtime, `'static + Send + Sync` bounds, `tokio::spawn` for concurrency.
- Security: `cargo audit` for CVE scanning, `cargo deny` for license and advisory policies.
- Dependencies: pin versions, commit `Cargo.lock`, prefer well-maintained crates.
- Structured logging with `tracing` crate — use spans and events, not `println!`.
- API naming: follow `as_`/`to_`/`into_` conventions for conversions, `iter()`/`iter_mut()`/`into_iter()` for iterators. Getters are `field()` not `get_field()`. See [Rust API Guidelines](https://rust-lang.github.io/api-guidelines).
- Eagerly implement common traits: `Clone`, `Debug`, `Default`, `Eq`, `PartialEq`, `Hash`, `Send`, `Sync`. Use `From`/`AsRef`/`AsMut` for conversions, `FromIterator`/`Extend` for collections.
- Type safety: newtypes for static distinctions, builder pattern for complex construction, `bitflags` over enums for flag sets. Avoid `bool` params — use custom types or enums.
- Constructors: `new()` as static inherent methods. No out-parameters. Only smart pointers implement `Deref`/`DerefMut`.
- API flexibility: minimize parameter assumptions via generics, make traits object-safe when trait objects may be useful. Let callers decide where to copy and place data.
- Rustdoc: all public items have doc examples using `?` (not `unwrap`). Document errors, panics, and safety invariants. Hyperlink related items.
- Future-proofing: seal traits to prevent downstream implementations, keep struct fields private, don't duplicate derived trait bounds on structs. See [Rust Design Patterns](https://rust-unofficial.github.io/patterns).
- Anti-patterns: `unwrap()`, unguarded `unsafe`, panics in libraries, `Vec`/`HashMap` across FFI.

### atomic-commits

**Priority:** high

Each commit represents one logical change. Don't mix unrelated changes. Use conventional commits format (`feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `test:`). Keep commits small and focused for easier review and bisection.

### branch-hygiene

**Priority:** medium

Use descriptive branch names. Keep branches short-lived. Delete merged branches. Rebase or merge from main regularly to avoid drift.

### commit-messages

**Priority:** high

Use conventional commits: `feat: add user auth`, `fix: handle null input`, `chore: update deps`, `refactor: extract parser`, `docs: add API guide`, `test: cover edge case`. First line under 72 chars, imperative mood. Body explains *why*, not *what*. Add scope when useful: `feat(api): add pagination`.

### safe-git-operations

**Priority:** critical

Never force-push to shared branches. Always pull before pushing. Use `--force-with-lease` instead of `--force` when necessary. Confirm destructive operations with the user.

## Context

### crate-structure

## Workspace crates (`crates/`)

- `html-to-markdown` — core library, primary Rust API, `unsafe_code = "forbid"` at workspace level
- `html-to-markdown-cli` — CLI binary (clap)
- `html-to-markdown-ffi` — C FFI bridge, cbindgen headers, **only crate that overrides unsafe_code lint**
- `html-to-markdown-py` — PyO3 Python binding
- `html-to-markdown-node` — NAPI-RS Node/TypeScript binding
- `html-to-markdown-php` — ext-php-rs PHP binding
- `html-to-markdown-wasm` — wasm-bindgen WebAssembly binding

## Out-of-workspace packages (`packages/`)

- `csharp/`, `elixir/`, `go/`, `java/`, `r/`, `ruby/` — language-native packages wrapping the FFI crate
- `php/`, `python/`, `typescript/`, `wasm/` — distribution packages

## Primary API

- `convert(&str, Option<ConversionOptions>) -> Result<ConversionResult, ConversionError>`
- `ConversionResult`: `content`, `warnings`, optionally `metadata` and `inline_images` (feature-gated)
- Feature flags: `inline-images`, `metadata`, `visitor` (custom traversal), `serde`
- Dual parser: html5ever (spec-compliance) and astral-tl (performance), selectable via `ConversionOptions`

### polyrepo-structure

This repo is part of the `kreuzberg-dev` polyrepo. From any subrepo, `../` is the polyrepo root.

- Shared AI governance config: `../ai-rulez/` (git submodule with shared modules)
- Polyrepo-level orchestration: `../Taskfile.yml`, `../.pre-commit-config.yaml`
- Each subrepo is an independent git repo with its own `.ai-rulez/config.yaml`
- Navigate between repos via `../` relative paths

### pre-commit-tooling

Shared hooks run via `prek run --all-files`. Each repo's `.pre-commit-config.yaml` defines its hooks.

| Scope | Tools |
|-------|-------|
| **General** | trailing-whitespace, end-of-file-fixer, check-merge-conflict, detect-private-key, typos |
| **Rust** | cargo-fmt, cargo-clippy (-D warnings), cargo-machete, cargo-deny, cargo-sort |
| **Python** | ruff (lint+format), mypy |
| **JS/TS** | biome (format+lint), oxlint |
| **Go** | golangci-lint |
| **Java** | cpd, checkstyle, maven verify |
| **Ruby** | rubocop (format+lint), steep |
| **C#** | dotnet format |
| **PHP** | php-cs-fixer, phpstan |
| **Elixir** | mix credo, mix format |
| **R** | styler, lintr |
| **C/C++** | clang-format, cppcheck |
| **Shell** | shfmt, shellcheck |
| **TOML** | taplo, pyproject-fmt |
| **Markdown** | rumdl-fmt, textlint (docs/) |
| **Git** | gitfluff (commit msg linting), ai-rulez-generate |
| **GH Actions** | actionlint |
| **Helm/K8s** | helm-lint, kubeconform |

### prek

[prek](https://github.com/j178/prek) is a Rust rewrite of pre-commit. Single binary, no runtime dependencies, drop-in replacement.

- Validate all changes: `prek run --all-files`
- Install hooks: `prek install`
- Run on staged files only: `prek run`
- Manages language toolchains automatically (Python via uv, Node, Go, Rust, Ruby)
- Supports workspace mode for monorepo/polyrepo setups
- Config file: `.pre-commit-config.yaml` (same format as pre-commit)
- Docs: https://prek.j178.dev/

Always run `prek run --all-files` after making changes to verify compliance.

### taskfile-structure

All repos use [Taskfile.yaml](https://taskfile.dev) with the `task` CLI. Always prefer `task` commands over raw tool invocations.

- **Discovery:** `task --list` shows all available tasks
- **Common tasks:** `task setup`, `task build`, `task test`, `task lint`, `task format`
- **Language-scoped:** `task rust:test`, `task python:lint`, `task node:build`, etc.
- **Build profiles:** `BUILD_PROFILE=release task build` or `task build:release`

### owasp-quick-reference

1. **Broken Access Control** — enforce authorization checks on every request, deny by default.
2. **Cryptographic Failures** — use strong standard algorithms, never roll your own crypto.
3. **Injection** — parameterize all queries, sanitize and validate all inputs.
4. **Insecure Design** — threat model early, validate business logic at every layer.
5. **Security Misconfiguration** — harden defaults, disable unnecessary features and endpoints.
6. **Vulnerable Components** — keep dependencies updated, audit regularly with language-specific tools.
7. **Authentication Failures** — require MFA, enforce strong passwords, implement rate limiting.
8. **Data Integrity Failures** — verify software updates, use signed artifacts and checksums.
9. **Logging Failures** — log all security events with context, protect log data from tampering.
10. **SSRF** — validate and allowlist URLs, restrict outbound network requests.

