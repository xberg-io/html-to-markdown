# homebrew test_app

Exercises the configured Homebrew formulae from tap `xberg-io/tap` at the release version configured in `Cargo.toml`.

| Formula | Purpose |
|---------|--------|
| `html-to-markdown` | CLI binary |
| `libhtml-to-markdown` | Shared library: C FFI for embedding in other languages |

## Running

From `test_apps/homebrew`, with `<version>` from `mise run version:show`:

```bash
brew bundle install --file=Brewfile
"$(brew --prefix html-to-markdown)/bin/html-to-markdown" --version   # expect: html-to-markdown <version>
```

Then compile a C probe against `$(brew --prefix libhtml-to-markdown)` that prints `htm_version()` and check it
reports the same version.

## What it tests

1. `brew bundle install` succeeds (tap + formulae install without error).
2. The installed CLI reports the exact configured release version.
3. A C probe compiles against the installed FFI formula and verifies that `htm_version()` reports the same version.
