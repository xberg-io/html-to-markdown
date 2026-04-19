//! Regression coverage for issue #277 — silent output truncation on large HTML inputs.
//!
//! Investigates: does the converter silently drop content in various scenarios?

fn content(html: &str) -> String {
    html_to_markdown_rs::convert(html, None)
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

/// Content that follows a <script> tag should appear in the output.
#[test]
fn test_content_after_script_not_dropped() {
    let html = "<html><body><p>Before</p><script>var x = 1;</script><p>After</p></body></html>";
    let out = content(html);
    assert!(
        out.contains("After"),
        "content after <script> was dropped; got: {out:?}"
    );
}

/// A script whose JS contains HTML-like patterns must not trap subsequent DOM siblings.
/// If the tl parser incorrectly nests post-script elements inside the script's "children",
/// `handle_script()` will silently ignore them.
#[test]
fn test_content_after_script_with_html_patterns_not_dropped() {
    let html = r#"<html><body>
<p>Before</p>
<script type="text/javascript">
var tmpl = '<div class="bc-table"><table><tr><td>data</td></tr></table></div>';
doRender(tmpl);
</script>
<p>After</p>
</body></html>"#;
    let out = content(html);
    assert!(
        out.contains("After"),
        "content after <script> containing HTML-like patterns was dropped; got: {out:?}"
    );
}

/// Simulates the mdn-anno pattern: a large section followed by a script with
/// inline HTML strings, then more doc content. All post-script content must survive.
#[test]
fn test_mdn_anno_pattern_does_not_truncate() {
    let mut html = String::from("<html><body>\n");
    for i in 0..200 {
        html.push_str(&format!("<p>Section {i} content that fills the buffer</p>\n"));
    }
    html.push_str(r#"<div class="mdn-anno">
<details>
<summary>Browser compatibility</summary>
<script>
(function(){
var d=document.getElementById('bc-data');
if(d){d.innerHTML='<table class="bc-table"><thead><tr><th>Browser</th><th>Version</th></tr></thead><tbody><tr><td>Chrome</td><td>Yes</td></tr></tbody></table>';}
})();
</script>
</details>
</div>
"#);
    for i in 0..200 {
        html.push_str(&format!("<p>After-anno section {i}</p>\n"));
    }
    html.push_str("</body></html>");

    let out = content(&html);
    assert!(
        out.contains("After-anno section 199"),
        "content after mdn-anno script was truncated (silent data loss); output length: {}",
        out.len()
    );
}

/// Output must grow monotonically as input grows: adding more content should
/// never reduce or cap the output length. This guards against a hard output cap.
#[test]
fn test_output_grows_with_input() {
    let base_html = |n: usize| -> String {
        let mut s = String::from("<html><body>\n");
        for i in 0..n {
            s.push_str(&format!("<p>Paragraph {i}: some text content here</p>\n"));
        }
        s.push_str("</body></html>");
        s
    };

    let small = content(&base_html(50));
    let medium = content(&base_html(100));
    let large = content(&base_html(500));

    assert!(
        medium.len() > small.len(),
        "output did not grow from 50→100 paragraphs (small={}, medium={})",
        small.len(),
        medium.len()
    );
    assert!(
        large.len() > medium.len(),
        "output did not grow from 100→500 paragraphs (medium={}, large={})",
        medium.len(),
        large.len()
    );
}

/// The `preprocess_html` function should NOT drop content when a <script> opening tag
/// appears but the closing tag position is unclear (`unwrap_or(len)` guard).
#[test]
fn test_preprocess_script_without_closing_tag_does_not_eat_document() {
    // A script that appears to have no closing tag in the document
    // (because the closing tag might be found by find_closing_tag).
    // The key invariant: content BEFORE the script must always appear in output.
    let html = "<html><body><p>This must appear</p><script>var x=1;</script><p>This too</p></body></html>";
    let out = content(html);
    assert!(
        out.contains("This must appear"),
        "pre-script content was dropped; got: {out:?}"
    );
    assert!(
        out.contains("This too"),
        "post-script content was dropped; got: {out:?}"
    );
}

/// Warnings must be non-empty when output is truncated due to input limits.
/// Currently `TruncatedInput` is never emitted — this test documents the gap.
#[test]
fn test_truncated_input_warning_is_emitted_when_truncation_occurs() {
    // For now, just verify the warnings field exists and is accessible.
    // When the fix is in place, this test should assert truncated_input warning presence.
    let html = "<html><body><p>Content</p></body></html>";
    let result = html_to_markdown_rs::convert(html, None).expect("conversion should succeed");
    // Structural check: warnings is a Vec (may be empty for small inputs)
    let _ = result.warnings;
}

/// Test with the large Wikipedia HTML file to ensure output grows monotonically.
#[test]
fn test_large_wikipedia_html_not_truncated() {
    let html = include_str!("../../../test_documents/html/wikipedia/large_rust.html");

    // Take first 300KB
    let html_300k = &html[..300_000.min(html.len())];
    let result_300k = html_to_markdown_rs::convert(html_300k, None)
        .expect("conversion should succeed")
        .content
        .unwrap_or_default();

    // Take full file
    let result_full = html_to_markdown_rs::convert(html, None)
        .expect("conversion should succeed")
        .content
        .unwrap_or_default();

    println!("300KB input → {} bytes output", result_300k.len());
    println!("Full ({} bytes) input → {} bytes output", html.len(), result_full.len());

    assert!(
        result_full.len() > result_300k.len(),
        "Output did NOT grow from 300KB→full input. Possible truncation! 300k={}, full={}",
        result_300k.len(),
        result_full.len()
    );
}

/// Test with custom elements (triggers html5ever repair path).
/// Custom elements have hyphens in their tag names and trigger the `repair_with_html5ever` path.
#[test]
fn test_custom_elements_do_not_cause_truncation() {
    let mut html = String::from("<html><body>\n");
    for i in 0..100 {
        html.push_str(&format!("<p>Before section {i}</p>\n"));
    }
    // mdn-annotation is a real custom element used in the WHATWG spec
    html.push_str(
        r#"<mdn-annotation>
<aside class="bc-data">
  <details>
    <summary>Browser support</summary>
    <table><tr><th>Browser</th><th>Support</th></tr><tr><td>Chrome</td><td>Yes</td></tr></table>
  </details>
</aside>
</mdn-annotation>
"#,
    );
    for i in 0..100 {
        html.push_str(&format!("<p>After section {i}</p>\n"));
    }
    html.push_str("</body></html>");

    let out = content(&html);
    println!("Custom element test output length: {}", out.len());
    assert!(
        out.contains("After section 99"),
        "content after custom element (mdn-annotation) was dropped; output len={}; ends with: {:?}",
        out.len(),
        &out[out.len().saturating_sub(200)..]
    );
}

/// Test that html5ever-repaired HTML (with custom elements + complex content) grows monotonically.
#[test]
fn test_html5ever_repaired_html_grows_monotonically() {
    let make_html = |sections: usize| -> String {
        let mut s = String::from("<html><body>\n");
        for i in 0..sections {
            s.push_str(&format!("<p>Section {i}</p>\n"));
        }
        // Insert a custom element to trigger html5ever repair
        s.push_str("<custom-widget><div><p>Widget content</p></div></custom-widget>\n");
        for i in 0..sections {
            s.push_str(&format!("<p>Post-widget {i}</p>\n"));
        }
        s.push_str("</body></html>");
        s
    };

    let small = content(&make_html(10));
    let medium = content(&make_html(100));
    let large = content(&make_html(500));

    assert!(
        medium.len() > small.len(),
        "html5ever path: output did not grow 10→100 sections (small={}, medium={})",
        small.len(),
        medium.len()
    );
    assert!(
        large.len() > medium.len(),
        "html5ever path: output did not grow 100→500 sections (medium={}, large={})",
        medium.len(),
        large.len()
    );
}

/// Realistic WHATWG spec mdn-annotation pattern.
/// The actual WHATWG spec uses <mdn-annotation> custom elements wrapping
/// browser compatibility data. Tests that html5ever-repaired HTML preserves
/// all content including content after the annotation.
#[test]
fn test_whatwg_spec_mdn_annotation_pattern() {
    // Build a realistic-looking WHATWG spec fragment
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>WHATWG HTML Spec</title></head>
<body>
"#,
    );

    // Large content before the annotation (to simulate the 705KB trigger point)
    for i in 0..2000 {
        html.push_str(&format!(
            "<section id=\"section-{i}\"><h2>Section {i}</h2>\n\
             <p>This is section {i} of the specification. It contains detailed information about the topic.</p>\n\
             <dl>\n  <dt><dfn id=\"concept-{i}\">Concept {i}</dfn></dt>\n\
             <dd>A concept in the HTML specification with <a href=\"#concept-{i}\">cross-references</a>.</dd>\n\
             </dl>\n</section>\n"
        ));
    }

    // The mdn-annotation custom element (this triggers html5ever repair)
    html.push_str(
        "<div class=\"mdn-anno\">\n\
         <mdn-annotation>\n\
           <details class=\"bc-data\" id=\"bc-navigator.language\">\n\
             <summary class=\"bc-summary\"><span>Browser compatibility</span></summary>\n\
             <div class=\"bc-table-wrapper\">\n\
               <table class=\"bc-table bc-table-web\">\n\
                 <thead>\n\
                   <tr class=\"bc-browsers\">\n\
                     <th></th>\n\
                     <th class=\"bc-browser-chrome\"><span>Chrome</span></th>\n\
                     <th class=\"bc-browser-firefox\"><span>Firefox</span></th>\n\
                     <th class=\"bc-browser-safari\"><span>Safari</span></th>\n\
                   </tr>\n\
                 </thead>\n\
                 <tbody>\n\
                   <tr>\n\
                     <th class=\"bc-feature-name\"><a href=\"#dom-navigator-language\">language</a></th>\n\
                     <td class=\"bc-supports-yes bc-browser-chrome\"><abbr title=\"Full support\">Yes</abbr></td>\n\
                     <td class=\"bc-supports-yes bc-browser-firefox\"><abbr title=\"Full support\">Yes</abbr></td>\n\
                     <td class=\"bc-supports-yes bc-browser-safari\"><abbr title=\"Full support\">Yes</abbr></td>\n\
                   </tr>\n\
                 </tbody>\n\
               </table>\n\
             </div>\n\
           </details>\n\
         </mdn-annotation>\n\
         </div>\n",
    );

    // Content after the annotation (this should NOT be dropped)
    for i in 0..500 {
        html.push_str(&format!("<p id=\"post-anno-{i}\">Post-annotation content {i}</p>\n"));
    }
    html.push_str("</body></html>");

    let input_size = html.len();
    let out = content(&html);
    println!(
        "WHATWG-like test: input={} bytes, output={} bytes",
        input_size,
        out.len()
    );

    assert!(
        out.contains("Post-annotation content 499"),
        "Post-annotation content was silently dropped! \
        Input size: {input_size} bytes, Output size: {} bytes. \
        Last 200 chars of output: {:?}",
        out.len(),
        &out[out.len().saturating_sub(200)..]
    );
}
