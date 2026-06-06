# php-ext test_app

Exercises the configured PHP native extension (`kreuzberg-dev/html-to-markdown-ext` v`3.6.0-rc.16`)
installed via [PIE](https://github.com/php/pie).

## Running

```bash
bash run_tests.sh
```

## What it tests

- PIE installs the extension successfully.
- The extension loads successfully.
- The configured e2e call function, when present, returns a non-null value.
