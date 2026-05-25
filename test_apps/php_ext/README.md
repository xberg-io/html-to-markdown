# php-ext test_app

Exercises the html-to-markdown PHP native extension (`kreuzberg-dev/html-to-markdown-ext` v`3.5.1`)
installed via [PIE](https://github.com/php/pie).

## Running

```bash
bash run_tests.sh
```

## What it tests

- PIE installs the extension successfully.
- The extension loads (`extension_loaded('html_to_markdown')`).
- `html_to_markdown_convert('<h1>Hi</h1>')` returns a string containing `Hi`.
