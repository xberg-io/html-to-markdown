//! Example: Testing HTML list conversion (ordered and unordered lists)

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

fn main() {
    let html = "<ol><li>First item</li><li>Second item</li><li>Third item</li></ol>";
    match convert(html, None) {
        Ok(markdown) => {
            println!("Test - Ordered list:");
            println!("HTML: {html}");
            println!("Markdown:\n{markdown}");
            println!("Expected:");
            println!("1. First item");
            println!("2. Second item");
            println!("3. Third item");
            println!();
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    let html2 = "<ul><li>First item</li><li>Second item</li></ul>";
    match convert(html2, None) {
        Ok(markdown) => {
            println!("Test - Unordered list:");
            println!("HTML: {html2}");
            println!("Markdown:\n{markdown}");
            println!("Expected:");
            println!("* First item");
            println!("* Second item");
            println!();
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
