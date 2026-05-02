//! Example: Testing HTML escape sequences and special characters

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

fn main() {
    let html = "<p>Use *wildcards* for search</p>";
    match convert(html, None) {
        Ok(markdown) => {
            println!("Test 1 - Asterisks:");
            println!("HTML: {html}");
            println!("Markdown: {markdown}");
            println!("Expected: Use \\*wildcards\\* for search");
            println!();
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    let html2 = "<p>variable_name in code</p>";
    match convert(html2, None) {
        Ok(markdown) => {
            println!("Test 2 - Underscores:");
            println!("HTML: {html2}");
            println!("Markdown: {markdown}");
            println!("Expected: variable\\_name in code");
            println!();
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    let html3 = "<code>use *wildcards* for search</code>";
    match convert(html3, None) {
        Ok(markdown) => {
            println!("Test 3 - Code (no escaping):");
            println!("HTML: {html3}");
            println!("Markdown: {markdown}");
            println!("Expected: `use *wildcards* for search`");
            println!();
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    let html4 = "<p>List: 1. First item</p>";
    match convert(html4, None) {
        Ok(markdown) => {
            println!("Test 4 - Numbered list escape:");
            println!("HTML: {html4}");
            println!("Markdown: {markdown}");
            println!("Expected: List: 1\\. First item");
            println!();
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
