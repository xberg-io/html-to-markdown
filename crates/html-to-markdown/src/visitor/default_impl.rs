//! Default visitor implementations and utilities.
//!
//! This module provides standard visitor patterns and helpers for common use cases.

use std::sync::{Arc, Mutex};

/// Shareable, thread-safe handle to a user-provided HTML visitor implementation.
///
/// Pass an instance wrapped in this handle to [`crate::ConversionOptions`] to
/// customise how the HTML document is traversed and converted to Markdown.
/// The handle may be cloned and shared across threads without additional
/// synchronisation on the caller's side.
pub type VisitorHandle = Arc<Mutex<dyn super::traits::HtmlVisitor + Send>>;

#[cfg(test)]
mod tests {
    use super::super::traits::HtmlVisitor;
    use super::super::types::{NodeContext, NodeType, VisitResult};
    use std::borrow::Cow;
    use std::collections::BTreeMap;

    #[derive(Debug)]
    struct TrackingVisitor {
        element_count: usize,
        text_count: usize,
    }

    impl HtmlVisitor for TrackingVisitor {
        fn visit_element_start(&mut self, _ctx: &NodeContext<'_>) -> VisitResult {
            self.element_count += 1;
            VisitResult::Continue
        }

        fn visit_text(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
            self.text_count += 1;
            VisitResult::Continue
        }
    }

    #[test]
    fn test_visitor_handle_creation() {
        let visitor = TrackingVisitor {
            element_count: 0,
            text_count: 0,
        };

        let handle = std::sync::Arc::new(std::sync::Mutex::new(visitor));

        {
            let mut v = handle.lock().expect("visitor mutex poisoned");
            let attrs = BTreeMap::new();
            let ctx = NodeContext {
                node_type: NodeType::Text,
                tag_name: Cow::Borrowed("p"),
                attributes: Cow::Borrowed(&attrs),
                depth: 1,
                index_in_parent: 0,
                parent_tag: Some(Cow::Borrowed("div")),
                is_inline: false,
            };
            v.visit_text(&ctx, "test");
        }

        let text_count = handle.lock().expect("visitor mutex poisoned").text_count;
        assert_eq!(text_count, 1);
    }
}
