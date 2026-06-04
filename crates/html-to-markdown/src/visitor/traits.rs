//! Visitor traits for HTML to Markdown conversion.
//!
//! This module contains the synchronous visitor trait.

use super::types::{NodeContext, VisitResult};

/// Visitor for HTML→Markdown conversion.
///
/// Provide a visitor object whose methods customize the conversion behavior for any
/// HTML element type. Override only the methods you care about; unimplemented methods
/// default to `Continue` (emit the standard rendering).
///
/// Each callback returns one of:
///
/// - `Continue` (the default) — keep the standard rendering.
/// - `Skip` — drop the element from the output entirely.
/// - `PreserveHtml` — pass the original HTML through verbatim.
/// - `Custom(text)` — replace the rendering with `text`.
/// - `Error(message)` — abort conversion with `message`.
///
/// **Language idioms.** In Rust, return one of the [`VisitResult`] variants directly.
/// In Python, Ruby, JavaScript/TypeScript, and other duck-typed bindings, define a
/// plain class (no base class required) and return either a string (`"continue"`,
/// `"skip"`, `"preserve_html"`) or a tagged map (`{"custom": "..."}`,
/// `{"error": "..."}`) — the binding converts the return value to the corresponding
/// [`VisitResult`] variant automatically.
///
/// # Method Naming Convention
///
/// - `visit_*_start`: Called before entering an element (pre-order traversal)
/// - `visit_*_end`: Called after exiting an element (post-order traversal)
/// - `visit_*`: Called for specific element types (e.g., `visit_link`, `visit_image`)
///
/// # Execution Order
///
/// For a typical element like `<div><p>text</p></div>`:
/// 1. `visit_element_start` for `<div>`
/// 2. `visit_element_start` for `<p>`
/// 3. `visit_text` for "text"
/// 4. `visit_element_end` for `<p>`
/// 5. `visit_element_end` for `</div>`
///
/// # Performance Notes
///
/// - `visit_text` is the most frequently called method (~100+ times per document)
/// - Return `Continue` quickly for elements you don't need to customize
/// - Avoid heavy computation in visitor methods; consider caching if needed
pub trait HtmlVisitor: std::fmt::Debug + Send {
    /// Visit text nodes (most frequent callback - ~100+ per document).
    ///
    /// # Arguments
    /// - `ctx`: Node context (will have `node_type: NodeType::Text`)
    /// - `text`: The raw text content (HTML entities already decoded)
    fn visit_text(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Called before entering any element.
    ///
    /// This is the first callback invoked for every HTML element, allowing
    /// visitors to implement generic element handling before tag-specific logic.
    fn visit_element_start(&mut self, _ctx: &NodeContext) -> VisitResult {
        VisitResult::Continue
    }

    /// Called after exiting any element.
    ///
    /// Receives the default markdown output that would be generated.
    /// Visitors can inspect or replace this output.
    fn visit_element_end(&mut self, _ctx: &NodeContext<'_>, _output: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit anchor links `<a href="...">`.
    ///
    /// # Arguments
    /// - `ctx`: Node context with link element metadata
    /// - `href`: The link URL (from `href` attribute)
    /// - `text`: The link text content (already converted to markdown)
    /// - `title`: Optional title attribute
    fn visit_link(&mut self, _ctx: &NodeContext<'_>, _href: &str, _text: &str, _title: Option<&str>) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit images `<img src="...">`.
    ///
    /// # Arguments
    /// - `ctx`: Node context with image element metadata
    /// - `src`: The image source URL
    /// - `alt`: The alt text
    /// - `title`: Optional title attribute
    fn visit_image(&mut self, _ctx: &NodeContext<'_>, _src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit heading elements `<h1>` through `<h6>`.
    ///
    /// # Arguments
    /// - `ctx`: Node context with heading metadata
    /// - `level`: Heading level (1-6)
    /// - `text`: The heading text content
    /// - `id`: Optional id attribute (for anchor links)
    fn visit_heading(&mut self, _ctx: &NodeContext<'_>, _level: u32, _text: &str, _id: Option<&str>) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit code blocks `<pre><code>`.
    ///
    /// # Arguments
    /// - `ctx`: Node context
    /// - `lang`: Optional language specifier (from class attribute)
    /// - `code`: The code content
    fn visit_code_block(&mut self, _ctx: &NodeContext<'_>, _lang: Option<&str>, _code: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit inline code `<code>`.
    ///
    /// # Arguments
    /// - `ctx`: Node context
    /// - `code`: The code content
    fn visit_code_inline(&mut self, _ctx: &NodeContext<'_>, _code: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit list items `<li>`.
    ///
    /// # Arguments
    /// - `ctx`: Node context
    /// - `ordered`: Whether this is an ordered list item
    /// - `marker`: The list marker (e.g., "-", "1.", "a)")
    /// - `text`: The list item content (already converted)
    fn visit_list_item(&mut self, _ctx: &NodeContext<'_>, _ordered: bool, _marker: &str, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Called before processing a list `<ul>` or `<ol>`.
    fn visit_list_start(&mut self, _ctx: &NodeContext<'_>, _ordered: bool) -> VisitResult {
        VisitResult::Continue
    }

    /// Called after processing a list `</ul>` or `</ol>`.
    fn visit_list_end(&mut self, _ctx: &NodeContext<'_>, _ordered: bool, _output: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Called before processing a table `<table>`.
    fn visit_table_start(&mut self, _ctx: &NodeContext) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit table rows `<tr>`.
    ///
    /// # Arguments
    /// - `ctx`: Node context
    /// - `cells`: Cell contents (already converted to markdown)
    /// - `is_header`: Whether this row is in `<thead>`
    fn visit_table_row(&mut self, _ctx: &NodeContext<'_>, _cells: &[String], _is_header: bool) -> VisitResult {
        VisitResult::Continue
    }

    /// Called after processing a table `</table>`.
    fn visit_table_end(&mut self, _ctx: &NodeContext<'_>, _output: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit blockquote elements `<blockquote>`.
    ///
    /// # Arguments
    /// - `ctx`: Node context
    /// - `content`: The blockquote content (already converted)
    /// - `depth`: Nesting depth (for nested blockquotes)
    fn visit_blockquote(&mut self, _ctx: &NodeContext<'_>, _content: &str, _depth: usize) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit strong/bold elements `<strong>`, `<b>`.
    fn visit_strong(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit emphasis/italic elements `<em>`, `<i>`.
    fn visit_emphasis(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit strikethrough elements `<s>`, `<del>`, `<strike>`.
    fn visit_strikethrough(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit underline elements `<u>`, `<ins>`.
    fn visit_underline(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit subscript elements `<sub>`.
    fn visit_subscript(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit superscript elements `<sup>`.
    fn visit_superscript(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit mark/highlight elements `<mark>`.
    fn visit_mark(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit line break elements `<br>`.
    fn visit_line_break(&mut self, _ctx: &NodeContext) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit horizontal rule elements `<hr>`.
    fn visit_horizontal_rule(&mut self, _ctx: &NodeContext) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit custom elements (web components) or unknown tags.
    ///
    /// # Arguments
    /// - `ctx`: Node context
    /// - `tag_name`: The custom element's tag name
    /// - `html`: The raw HTML of this element
    fn visit_custom_element(&mut self, _ctx: &NodeContext<'_>, _tag_name: &str, _html: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit definition list `<dl>`.
    fn visit_definition_list_start(&mut self, _ctx: &NodeContext) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit definition term `<dt>`.
    fn visit_definition_term(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit definition description `<dd>`.
    fn visit_definition_description(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Called after processing a definition list `</dl>`.
    fn visit_definition_list_end(&mut self, _ctx: &NodeContext<'_>, _output: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit form elements `<form>`.
    fn visit_form(&mut self, _ctx: &NodeContext<'_>, _action: Option<&str>, _method: Option<&str>) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit input elements `<input>`.
    fn visit_input(
        &mut self,
        _ctx: &NodeContext<'_>,
        _input_type: &str,
        _name: Option<&str>,
        _value: Option<&str>,
    ) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit button elements `<button>`.
    fn visit_button(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit audio elements `<audio>`.
    fn visit_audio(&mut self, _ctx: &NodeContext<'_>, _src: Option<&str>) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit video elements `<video>`.
    fn visit_video(&mut self, _ctx: &NodeContext<'_>, _src: Option<&str>) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit iframe elements `<iframe>`.
    fn visit_iframe(&mut self, _ctx: &NodeContext<'_>, _src: Option<&str>) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit details elements `<details>`.
    fn visit_details(&mut self, _ctx: &NodeContext<'_>, _open: bool) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit summary elements `<summary>`.
    fn visit_summary(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit figure elements `<figure>`.
    fn visit_figure_start(&mut self, _ctx: &NodeContext) -> VisitResult {
        VisitResult::Continue
    }

    /// Visit figcaption elements `<figcaption>`.
    fn visit_figcaption(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        VisitResult::Continue
    }

    /// Called after processing a figure `</figure>`.
    fn visit_figure_end(&mut self, _ctx: &NodeContext<'_>, _output: &str) -> VisitResult {
        VisitResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::collections::BTreeMap;

    #[derive(Debug)]
    struct NoOpVisitor;

    impl HtmlVisitor for NoOpVisitor {}

    #[test]
    fn test_default_visitor_implementation() {
        let mut visitor = NoOpVisitor;
        let attrs = BTreeMap::new();

        let ctx = NodeContext::with_borrowed_attributes(
            super::super::types::NodeType::Text,
            Cow::Borrowed(""),
            &attrs,
            0,
            0,
            None,
            true,
        );

        matches!(visitor.visit_element_start(&ctx), VisitResult::Continue);
        matches!(visitor.visit_element_end(&ctx, "output"), VisitResult::Continue);
        matches!(visitor.visit_text(&ctx, "text"), VisitResult::Continue);
        matches!(visitor.visit_link(&ctx, "href", "text", None), VisitResult::Continue);
        matches!(visitor.visit_image(&ctx, "src", "alt", None), VisitResult::Continue);
        matches!(visitor.visit_heading(&ctx, 1, "text", None), VisitResult::Continue);
        matches!(visitor.visit_code_block(&ctx, None, "code"), VisitResult::Continue);
        matches!(visitor.visit_code_inline(&ctx, "code"), VisitResult::Continue);
    }

    #[derive(Debug)]
    struct CustomLinkVisitor;

    impl HtmlVisitor for CustomLinkVisitor {
        fn visit_link(&mut self, _ctx: &NodeContext<'_>, href: &str, text: &str, _title: Option<&str>) -> VisitResult {
            VisitResult::Custom(format!("{} ({})", text, href))
        }

        fn visit_image(&mut self, _ctx: &NodeContext<'_>, _src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
            VisitResult::Skip
        }
    }

    #[test]
    fn test_custom_visitor_implementation() {
        let mut visitor = CustomLinkVisitor;
        let attrs = BTreeMap::new();

        let ctx = NodeContext::with_borrowed_attributes(
            super::super::types::NodeType::Link,
            Cow::Borrowed("a"),
            &attrs,
            1,
            0,
            Some(Cow::Borrowed("p")),
            true,
        );

        let result = visitor.visit_link(&ctx, "https://example.com", "Example", None);
        if let VisitResult::Custom(output) = result {
            assert_eq!(output, "Example (https://example.com)");
        } else {
            panic!("Expected Custom result");
        }

        let img_result = visitor.visit_image(&ctx, "image.jpg", "Alt text", None);
        matches!(img_result, VisitResult::Skip);
    }
}
