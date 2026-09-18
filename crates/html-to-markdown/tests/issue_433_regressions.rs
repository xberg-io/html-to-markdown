// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "inline-images")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn content(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options)).unwrap().content.unwrap_or_default()
}

/// A link-heavy, headerless, two-row table (issue #500's layout trigger) whose one cell
/// holds an image, wrapped so the reporter's `<p><span>` nesting from #433 survives.
const LAYOUT_TABLE_WITH_IMAGE: &str = r#"
<table role="presentation">
  <tr><td><p><span><img src="image.png" alt="Image"></span></p></td><td><a href="/1">x</a></td></tr>
  <tr><td><a href="/2">Name</a></td><td><a href="/3">Title</a></td></tr>
</table>
"#;

#[test]
fn keep_inline_images_in_td_survives_tier2_layout_cell_issue_433() {
    // ~keep A short table dense with links classifies this as a layout table, so cells are
    // ~keep converted as inline — the exact path issue #433 hits, where the image would
    // ~keep otherwise drop to alt text.
    let options = ConversionOptions {
        keep_inline_images_in: vec!["td".to_string(), "th".to_string()],
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    };

    let result = content(LAYOUT_TABLE_WITH_IMAGE, options);
    assert!(
        result.contains("![Image](image.png)"),
        "Image in a td listed in keep_inline_images_in must stay markdown: {result:?}"
    );
}

#[test]
fn keep_inline_images_in_excluding_cell_still_keeps_image_issue_500() {
    // ~keep issue #500: a layout row is a list item, and list items keep inline images by
    // ~keep default (only headings degrade an image to its alt text), so the image now
    // ~keep survives even when `keep_inline_images_in` excludes the cell tag entirely --
    // ~keep the option no longer gates images in a layout cell (it still governs headings
    // ~keep and links). This supersedes the pre-#500 behavior this test used to pin.
    let options = ConversionOptions {
        keep_inline_images_in: vec!["h1".to_string()], // ~keep excludes td/th
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    };

    let result = content(LAYOUT_TABLE_WITH_IMAGE, options);
    assert!(
        result.contains("![Image](image.png)"),
        "A layout-cell image must stay markdown regardless of keep_inline_images_in: {result:?}"
    );
}
