# homebrew test_app

Exercises the configured Homebrew formulae from tap `kreuzberg-dev/tap` at version `3.6.0-rc.2`.

| Formula | Purpose |
|---------|--------|
| `html-to-markdown` | CLI binary: converts HTML from stdin to Markdown on stdout |
| `libhtml-to-markdown` | Shared library: C FFI for embedding in other languages |

## Running

```bash
bash run_tests.sh
```

## What it tests

1. `brew bundle install` succeeds (tap + both formulae install without error).
2. `html-to-markdown --version` output contains `3.6.0-rc.2`.
3. `echo '<h1>Hi</h1>' | html-to-markdown` produces output containing `# Hi`.
4. `ffi_smoke.c` compiles against `libhtml-to-markdown` (via `pkg-config`) and the
compiled binary converts `<h1>Hi</h1>` successfully.
