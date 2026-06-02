# php-ext test_app

Exercises the configured PHP native extension (`kreuzberg-dev/html-to-markdown-ext` v`3.6.0-rc.6`)
installed via [PIE](https://github.com/php/pie).

## Running

```bash
bash run_tests.sh
```

## What it tests

- PIE installs the extension successfully.
- The extension loads successfully.
- The configured convert function returns a string containing `Hi`.
