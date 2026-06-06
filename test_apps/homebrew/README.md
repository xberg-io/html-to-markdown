# homebrew test_app

Exercises the configured Homebrew formulae from tap `kreuzberg-dev/tap` at version `3.6.0-rc.15`.

| Formula | Purpose |
|---------|--------|
| `html-to-markdown` | CLI binary |
| `libhtml-to-markdown` | Shared library: C FFI for embedding in other languages |

## Running

```bash
bash run_tests.sh
```

## What it tests

1. `brew bundle install` succeeds (tap + formulae install without error).
2. `$CLI_FORMULA --version` — output contains `$VERSION`.
3. `ffi_smoke.c` compiles against the FFI formula (via `pkg-config`) and the compiled binary calls `_version()` successfully.
