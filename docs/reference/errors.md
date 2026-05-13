---
title: "Error Reference"
---

## Error Reference

All error types thrown by the library across all languages.

### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Message | Description |
|---------|---------|-------------|
| `ParseError` | HTML parsing error: {0} | HTML parsing error |
| `SanitizationError` | Sanitization error: {0} | HTML sanitization error |
| `ConfigError` | Invalid configuration: {0} | Invalid configuration |
| `IoError` | I/O error: {0} | I/O error |
| `Panic` | Internal panic: {0} | Internal error caught during conversion |
| `InvalidInput` | Invalid input: {0} | Invalid input data |
| `Other` | Conversion error: {0} | Generic conversion error |

---
