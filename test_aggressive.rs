#[test]
fn test_aggressive_preset() {
    use html_to_markdown_rs::{convert, options::{ConversionOptions, PreprocessingOptions, PreprocessingPreset}};
    
    let html = "<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>";
    
    let options = ConversionOptions {
        preprocessing: PreprocessingOptions {
            enabled: true,
            preset: PreprocessingPreset::Aggressive,
            remove_navigation: true,
            remove_forms: true,
        },
        ..Default::default()
    };
    
    let result = convert(html, Some(options)).unwrap();
    let content = result.content.as_deref().unwrap_or("");
    println!("Output: '{}'", content);
    println!("Contains 'Menu': {}", content.contains("Menu"));
    println!("Contains 'Title': {}", content.contains("Title"));
    println!("Contains 'Content': {}", content.contains("Content"));
    
    assert!(!content.contains("Menu"), "Menu should not be in output");
    assert!(content.contains("Title"), "Title should be in output");
    assert!(content.contains("Content"), "Content should be in output");
}
