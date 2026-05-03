fn main() {
    use html_to_markdown_rs::{convert, options::{ConversionOptions, PreprocessingOptions, PreprocessingPreset}};

    let html = "<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>";

    // Test with Aggressive preset
    let mut options = ConversionOptions::default();
    options.preprocessing = PreprocessingOptions {
        enabled: true,
        preset: PreprocessingPreset::Aggressive,
        remove_navigation: true,
        remove_forms: false,
    };

    match convert(html, Some(options)) {
        Ok(result) => {
            let content = result.content.as_deref().unwrap_or("");
            println!("Output: {}", content);
            println!("Contains 'Title': {}", content.contains("Title"));
            println!("Contains 'Content': {}", content.contains("Content"));
            println!("Contains 'Menu': {}", content.contains("Menu"));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
