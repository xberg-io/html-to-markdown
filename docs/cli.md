# CLI

The `html-to-markdown` CLI converts HTML files or URLs to Markdown from the command line.

## Installation

```bash
cargo install html-to-markdown-cli
```

Or via Homebrew:

```bash
brew install kreuzberg-dev/tap/html-to-markdown
```

## Basic Usage

```bash
# Convert stdin
echo '<h1>Title</h1><p>Content</p>' | html-to-markdown

# Convert a file
html-to-markdown input.html

# Convert a file and save output
html-to-markdown input.html -o output.md

# Fetch and convert a remote URL
html-to-markdown --url https://example.com > output.md
```

## Input

| Flag | Description |
|------|-------------|
| `FILE` | Input HTML file. Use `-` or omit for stdin. |
| `--url URL` | Fetch HTML from a URL (alternative to file/stdin). |
| `--user-agent UA` | Custom User-Agent header when using `--url`. |
| `-e`, `--encoding ENCODING` | Input character encoding (default: `utf-8`). |

## Output

| Flag | Description |
|------|-------------|
| `-o`, `--output FILE` | Write output to file (default: stdout). |
| `-f`, `--output-format FORMAT` | Output format: `markdown` (default) or `djot`. |

## Heading Options

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--heading-style STYLE` | `atx`, `underlined`, `atx-closed` | `atx` | How headings are formatted. `atx` uses `#` prefixes; `underlined` uses `===`/`---` for h1/h2. |

## List Options

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--list-indent-type TYPE` | `spaces`, `tab` | `spaces` | Indentation character for nested lists. |
| `--list-indent-width N` | 1–8 | `2` | Spaces per nesting level. |
| `-b`, `--bullets CHARS` | e.g. `*+-` | `-` | Characters to cycle through for unordered list markers. |

## Text Formatting

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--strong-em-symbol CHAR` | `*`, `_` | `*` | Symbol for bold and italic. |
| `--newline-style STYLE` | `backslash`, `spaces` | `backslash` | How `<br>` tags are rendered. |
| `--sub-symbol SYMBOL` | any string | `""` | Wrapper symbol for `<sub>` text. |
| `--sup-symbol SYMBOL` | any string | `""` | Wrapper symbol for `<sup>` text. |
| `--highlight-style STYLE` | `double-equal`, `html`, `bold`, `none` | `double-equal` | Rendering of `<mark>` elements. |
| `--escape-asterisks` | — | off | Escape `*` characters. |
| `--escape-underscores` | — | off | Escape `_` characters. |
| `--escape-misc` | — | off | Escape `[`, `]`, `<`, `>`, `#`, etc. |
| `--escape-ascii` | — | off | Escape all ASCII punctuation (strict CommonMark). |

## Code Blocks

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--code-block-style STYLE` | `indented`, `backticks`, `tildes` | `indented` | Format for multi-line code blocks. |
| `-l`, `--code-language LANG` | any string | `""` | Default language tag for fenced code blocks. |

## Links

| Flag | Description |
|------|-------------|
| `-a`, `--autolinks` | When link text equals the href, use `<url>` instead of `[url](url)`. |
| `--default-title` | Use href as link title when no `title` attribute exists. |

## Images

| Flag | Description |
|------|-------------|
| `--keep-inline-images-in ELEMENTS` | Comma-separated element names where images stay as `![alt](src)` (e.g. `a,strong`). |

## Tables

| Flag | Description |
|------|-------------|
| `--br-in-tables` | Preserve line breaks in table cells as `<br>` rather than converting to spaces. |

## Whitespace

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--whitespace-mode MODE` | `normalized`, `strict` | `normalized` | Whitespace handling strategy. |
| `--strip-newlines` | — | off | Remove all newlines from input before processing. |

## Wrapping

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `-w`, `--wrap` | — | off | Enable output line wrapping. |
| `--wrap-width N` | 20–500 | `80` | Column width for wrapping. |

## Element Handling

| Flag | Description |
|------|-------------|
| `--convert-as-inline` | Treat block elements as inline (no paragraph breaks). |
| `--strip-tags TAGS` | Comma-separated tags to strip (text content preserved, no Markdown conversion). |

## Metadata

| Flag | Description |
|------|-------------|
| `--extract-metadata` | Prepend a metadata comment block to the Markdown output. |
| `--json` | Output a full `ConversionResult` as JSON (content, metadata, tables, images, warnings). |
| `--extract-document` | Extract document-level metadata (requires `--json`). |
| `--extract-headers` | Extract heading elements (requires `--json`). |
| `--extract-links` | Extract anchor tags (requires `--json`). |
| `--extract-images` | Extract image elements (requires `--json`). |
| `--extract-structured-data` | Extract JSON-LD, Microdata, and RDFa (requires `--json`). |

## Preprocessing

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `-p`, `--preprocess` | — | off | Clean up HTML before conversion. |
| `--preset LEVEL` | `minimal`, `standard`, `aggressive` | `standard` | Preprocessing aggressiveness (requires `--preprocess`). |
| `--keep-navigation` | — | off | Preserve `<nav>` elements during preprocessing (requires `--preprocess`). |
| `--keep-forms` | — | off | Preserve form elements during preprocessing (requires `--preprocess`). |

## Debugging

| Flag | Description |
|------|-------------|
| `--debug` | Output diagnostic warnings and information. |

## Shell Completions and Man Page

```bash
# Generate shell completions
html-to-markdown --generate-completion bash > html-to-markdown.bash
html-to-markdown --generate-completion zsh > _html-to-markdown
html-to-markdown --generate-completion fish > html-to-markdown.fish

# Generate man page
html-to-markdown --generate-man > html-to-markdown.1
```

## Examples

```bash
# Web scraping with aggressive preprocessing
html-to-markdown page.html --preprocess --preset aggressive

# Extract full structured result as JSON
html-to-markdown input.html --json \
    --extract-document --extract-headers --extract-links --extract-images \
    -o output.json

# Discord/Slack-friendly output (2-space list indents)
html-to-markdown input.html --list-indent-width 2

# Custom heading and list styles
html-to-markdown input.html \
    --heading-style atx \
    --bullets '*' \
    --list-indent-width 2

# Fetch and convert with Djot output
html-to-markdown --url https://example.com --output-format djot
```
