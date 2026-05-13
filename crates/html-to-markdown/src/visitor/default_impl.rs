//! Default visitor implementations and utilities.
//!
//! This module provides standard visitor patterns and helpers for common use cases.

use std::sync::{Arc, Mutex};

/// Type alias for a visitor handle (`Arc`-wrapped `Mutex` for thread-safe shared mutation).
///
/// `Send + Sync` so that types embedding a `VisitorHandle` (e.g. `ConversionOptions`)
/// can be shared across threads — required by callers that stash configs inside
/// axum/rmcp/tokio Send-bound contexts.
pub type VisitorHandle = Arc<Mutex<dyn super::traits::HtmlVisitor + Send>>;

#[cfg(test)]
mod tests {
    use super::super::traits::HtmlVisitor;
    use super::super::types::{NodeContext, NodeType, VisitResult};
    use std::collections::BTreeMap;

    #[derive(Debug)]
    struct TrackingVisitor {
        element_count: usize,
        text_count: usize,
    }

    impl HtmlVisitor for TrackingVisitor {
        fn visit_element_start(&mut self, _ctx: &NodeContext) -> VisitResult {
            self.element_count += 1;
            VisitResult::Continue
        }

        fn visit_text(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
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
            let ctx = NodeContext {
                node_type: NodeType::Text,
                tag_name: "p".to_string(),
                attributes: BTreeMap::new(),
                depth: 1,
                index_in_parent: 0,
                parent_tag: Some("div".to_string()),
                is_inline: false,
            };
            v.visit_text(&ctx, "test");
        }

        let text_count = handle.lock().expect("visitor mutex poisoned").text_count;
        assert_eq!(text_count, 1);
    }
}
