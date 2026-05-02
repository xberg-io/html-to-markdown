//! Regression coverage for issues #200 and #214.

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

#[test]
fn test_definition_list_spacing_consistency() {
    let html1 = r#"
<div>
 <dl>
  <dt>Tags:</dt>
  <dd>
   <ul>
    <li>
     <a href="https://site.com">php</a>
    </li>
    <li>
     <a href="https://site.com/search/">closure</a>
    </li>
   </ul>
   <button type="button">Add tags</button>
  </dd>
 </dl>
</div>
"#;

    let html2 = r#"<div><dl><dt>Tags:</dt><dd><ul><li><a href="https://site.com">php</a></li><li><a href="https://site.com/search/">closure</a></li></ul><button type="button">Add tags</button></dd></dl></div>"#;

    let markdown1 = convert(html1, None).expect("conversion should succeed");
    let markdown2 = convert(html2, None).expect("conversion should succeed");

    assert_eq!(markdown1, markdown2);
    assert!(markdown1.contains("Tags:"));
    assert!(markdown1.contains("[php]"));
    assert!(markdown1.contains("[closure]"));
    assert!(markdown1.contains("Add tags"));
    // Ensure no Pandoc definition list colon prefix is introduced
    assert!(!markdown1.contains(":   "));
}

/// Regression test for issue #214: colon introduced into text from dd elements.
#[test]
fn test_definition_list_no_colon_prefix() {
    let html = r#"<dl>
        <dt id="canRequestFocus" class="property inherited">
  <span class="name"><a href="material/InkResponse/canRequestFocus.html">canRequestFocus</a></span>
  <span class="signature">&#8594; <a href="dart-core/bool-class.html">bool</a></span>
</dt>
<dd class="inherited">
  If true, this widget may request the primary focus.
  <div class="features"><span class="feature">final</span><span class="feature">inherited</span></div>
</dd>
</dl>"#;
    let markdown = convert(html, None).expect("conversion should succeed");
    assert!(markdown.contains("If true, this widget may request the primary focus."));
    // The dd content must not be prefixed with ": " (Pandoc definition list syntax)
    assert!(!markdown.contains(":   If true"));
}
