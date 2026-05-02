//! Example: Testing whitespace handling and normalization

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

fn main() {
    let html = "<p>text    with    multiple    spaces</p>";
    match convert(html, None) {
        Ok(markdown) => {
            println!("Test - Multiple spaces:");
            println!("HTML: {html}");
            println!("Markdown: {markdown}");
            println!("Expected: text with multiple spaces");
            println!();
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    let html2 = "<p>text\nwith\nnewlines</p>";
    match convert(html2, None) {
        Ok(markdown) => {
            println!("Test - Newlines:");
            println!("HTML: {html2}");
            println!("Markdown: {markdown}");
            println!("Expected: text with newlines");
            println!();
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
