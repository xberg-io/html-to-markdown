# Error Handling

`convert()` returns `Result<ConversionResult, ConversionError>` in Rust. Every other binding maps the error to its native idiom: Python raises an exception, Go returns `(result, error)`, Java throws a checked exception, and so on.

Non-fatal issues never produce an error. They accumulate in `result.warnings` and the call still succeeds.

## ConversionError Variants

Eight variants. All carry a `String` message except `IoError`, which wraps `std::io::Error` via `#[from]`.

| Variant | Payload | Cause |
|---------|---------|-------|
| `ParseError` | `String` | Malformed HTML the parser could not recover from. |
| `SanitizationError` | `String` | The sanitizer rejected the input outright. |
| `ConfigError` | `String` | `ConversionOptions` contains an invalid combination (unknown format string, out-of-range width, etc.). |
| `IoError` | `std::io::Error` | Reading a file, reading stdin, or writing output failed. |
| `Panic` | `String` | A panic was caught inside the conversion core. The FFI boundaries catch unwinds so other bindings see a normal error instead of a crash. |
| `InvalidInput` | `String` | Empty input, input exceeding the configured size cap, or decoding failure for a wrong `encoding` setting. |
| `Visitor` | `String` | A visitor callback returned `VisitResult::Error(...)`. Only compiled with `features = ["visitor"]`. Rust users on default features never see this variant. |
| `Other` | `String` | Catch-all for anything that does not fit above. |

## Warnings

`result.warnings` is a `Vec<ProcessingWarning>`. Each warning has a kind and a message. The CLI prints them to stderr with `--show-warnings`; in library code, iterate and log them yourself.

Warnings are the right place to surface "this was weird but the conversion worked" signals: skipped oversized images, unknown class attributes on code blocks, malformed table rows that were repaired. None of these halt the call.

## Handling Patterns

=== "Rust"
    ```rust
    use html_to_markdown_rs::{convert, ConversionError};

    match convert(html, None) {
        Ok(result) => println!("{}", result.content.unwrap_or_default()),
        Err(ConversionError::InvalidInput(msg)) => eprintln!("bad input: {msg}"),
        Err(ConversionError::ParseError(msg)) => eprintln!("parse failed: {msg}"),
        Err(e) => eprintln!("conversion failed: {e}"),
    }
    ```

=== "Python"
    ```python
    from html_to_markdown import convert, ConversionError

    try:
        result = convert(html)
    except ConversionError as e:
        print(f"conversion failed: {e}")
    ```

=== "TypeScript"
    ```typescript
    import { convert, ConversionError } from '@kreuzberg/html-to-markdown';

    try {
      const result = convert(html);
    } catch (e) {
      if (e instanceof ConversionError) {
        console.error(`conversion failed: ${e.message}`);
      } else {
        throw e;
      }
    }
    ```

=== "Go"
    ```go
    result, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatalf("conversion failed: %v", err)
    }
    ```

=== "Ruby"
    ```ruby
    begin
      result = HtmlToMarkdown.convert(html)
    rescue HtmlToMarkdown::ConversionError => e
      warn "conversion failed: #{e.message}"
    end
    ```

=== "PHP"
    ```php
    try {
        $result = $converter->convert($html);
    } catch (ConversionException $e) {
        error_log("conversion failed: " . $e->getMessage());
    }
    ```

=== "Java"
    ```java
    try {
        ConversionResult result = HtmlToMarkdown.convert(html);
    } catch (ConversionException e) {
        System.err.println("conversion failed: " + e.getMessage());
    }
    ```

=== "C#"
    ```csharp
    try
    {
        var result = HtmlToMarkdownConverter.Convert(html);
    }
    catch (ConversionException e)
    {
        Console.Error.WriteLine($"conversion failed: {e.Message}");
    }
    ```

=== "Elixir"
    ```elixir
    case HtmlToMarkdown.convert(html) do
      {:ok, result} -> IO.puts(result.content)
      {:error, reason} -> IO.warn("conversion failed: #{reason}")
    end
    ```

=== "R"
    ```r
    result <- tryCatch(
      htmltomarkdown::convert(html),
      error = function(e) {
        message("conversion failed: ", conditionMessage(e))
        NULL
      }
    )
    ```

## CLI Warnings

The CLI hides warnings by default. Pass `--show-warnings` to print each one to stderr in the format `Warning [<kind>]: <message>`. The flag works with or without `--json`. See [CLI: JSON Output](cli.md#json-output).

--8<-- "snippets/feedback.md"
