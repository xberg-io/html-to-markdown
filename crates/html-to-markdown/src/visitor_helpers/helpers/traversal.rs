//! Visitor callback dispatch and result handling.
//!
//! This module provides the core dispatching logic for synchronous visitor callbacks,
//! safely handling optional visitors and translating `VisitResult` into concrete
//! control flow decisions.

use std::sync::{Arc, Mutex};

use crate::error::{ConversionError, Result};
use crate::visitor::HtmlVisitor;
use crate::visitor::VisitResult;

use super::content::VisitorDispatch;
use std::borrow::Cow;
use crate::visitor::EMPTY_ATTRS;

/// Dispatch a visitor callback and handle the result.
///
/// This is the core dispatcher for all visitor callbacks. It safely handles the
/// optional visitor, calls the callback function, and translates the `VisitResult`
/// into concrete control flow decisions.
///
/// # Type Parameters
///
/// - `F`: Visitor callback function type
///
/// # Parameters
///
/// - `visitor`: Optional visitor (wrapped in `Arc<Mutex<>>`)
/// - `callback`: Closure that invokes the appropriate visitor method
///
/// # Returns
///
/// - `Ok(Some(String))`: Custom markdown output from `VisitResult::Custom`
/// - `Ok(None)`: Continue with default behavior (`VisitResult::Continue`)
/// - `Err(Error)`: Stop conversion with error (`VisitResult::Error`)
///
/// The `VisitResult::Skip` and `VisitResult::PreserveHtml` variants are handled
/// by the caller based on context.
///
/// # Error Handling
///
/// - If the visitor panics during callback, the panic propagates normally
/// - If the visitor returns `VisitResult::Error`, this is converted to `Error::Visitor`
/// - `Mutex` lock failures panic on poison (should never happen with correct usage)
///
/// # Performance
///
/// - Zero-cost when visitor is None (common case)
/// - Single dynamic dispatch when visitor is present
/// - No allocations except for error messages
///
/// # Examples
///
/// ```text
/// let result = dispatch_visitor(
///     &visitor,
///     |v| v.visit_heading(&ctx, level, text, id),
/// )?;
///
/// match result {
///     Some(custom_output) => return Ok(custom_output),
///     None => { /* proceed with default conversion */ }
/// }
/// ```
#[allow(dead_code)]
#[inline]
pub fn dispatch_visitor<F>(visitor: &Option<Arc<Mutex<dyn HtmlVisitor + Send>>>, callback: F) -> Result<VisitorDispatch>
where
    F: FnOnce(&mut dyn HtmlVisitor) -> VisitResult,
{
    let Some(visitor_rc) = visitor else {
        return Ok(VisitorDispatch::Continue);
    };

    let mut visitor_ref = visitor_rc.lock().expect("visitor mutex poisoned");
    let result = callback(&mut *visitor_ref);

    match result {
        VisitResult::Continue => Ok(VisitorDispatch::Continue),
        VisitResult::Custom(output) => Ok(VisitorDispatch::Custom(output)),
        VisitResult::Skip => Ok(VisitorDispatch::Skip),
        VisitResult::PreserveHtml => Ok(VisitorDispatch::PreserveHtml),
        VisitResult::Error(msg) => Err(ConversionError::Visitor(msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::visitor::{NodeContext, NodeType};

    #[derive(Debug)]
    struct TestVisitor {
        mode: TestMode,
    }

    #[derive(Debug)]
    enum TestMode {
        Continue,
        Custom,
        Skip,
        PreserveHtml,
        Error,
    }

    impl HtmlVisitor for TestVisitor {
        fn visit_text(&mut self, _ctx: &NodeContext, text: &str) -> VisitResult {
            match self.mode {
                TestMode::Continue => VisitResult::Continue,
                TestMode::Custom => VisitResult::Custom(format!("CUSTOM: {}", text)),
                TestMode::Skip => VisitResult::Skip,
                TestMode::PreserveHtml => VisitResult::PreserveHtml,
                TestMode::Error => VisitResult::Error("test error".to_string()),
            }
        }
    }

    #[test]
    fn test_dispatch_visitor_none() {
        let visitor: Option<Arc<Mutex<dyn HtmlVisitor + Send>>> = None;

        let result = dispatch_visitor(&visitor, |v| {
            let ctx = NodeContext::with_borrowed_attributes(
                NodeType::Text,
                Cow::Borrowed(""),
                &EMPTY_ATTRS,
                0,
                0,
                None,
                true,
            );
            v.visit_text(&ctx, "test")
        })
        .unwrap();

        assert!(result.is_continue());
    }

    #[test]
    fn test_dispatch_visitor_continue() {
        let visitor: Arc<Mutex<dyn HtmlVisitor + Send>> = Arc::new(Mutex::new(TestVisitor {
            mode: TestMode::Continue,
        }));
        let visitor_opt = Some(visitor);

        let ctx = NodeContext::with_borrowed_attributes(
            NodeType::Text,
            Cow::Borrowed(""),
            &EMPTY_ATTRS,
            0,
            0,
            None,
            true,
        );

        let result = dispatch_visitor(&visitor_opt, |v| v.visit_text(&ctx, "hello")).unwrap();

        assert!(result.is_continue());
    }

    #[test]
    fn test_dispatch_visitor_custom() {
        let visitor: Arc<Mutex<dyn HtmlVisitor + Send>> = Arc::new(Mutex::new(TestVisitor { mode: TestMode::Custom }));
        let visitor_opt = Some(visitor);

        let ctx = NodeContext::with_borrowed_attributes(
            NodeType::Text,
            Cow::Borrowed(""),
            &EMPTY_ATTRS,
            0,
            0,
            None,
            true,
        );

        let result = dispatch_visitor(&visitor_opt, |v| v.visit_text(&ctx, "hello")).unwrap();

        assert!(result.is_custom());
        assert_eq!(result.as_custom(), Some("CUSTOM: hello"));
    }

    #[test]
    fn test_dispatch_visitor_skip() {
        let visitor: Arc<Mutex<dyn HtmlVisitor + Send>> = Arc::new(Mutex::new(TestVisitor { mode: TestMode::Skip }));
        let visitor_opt = Some(visitor);

        let ctx = NodeContext::with_borrowed_attributes(
            NodeType::Text,
            Cow::Borrowed(""),
            &EMPTY_ATTRS,
            0,
            0,
            None,
            true,
        );

        let result = dispatch_visitor(&visitor_opt, |v| v.visit_text(&ctx, "hello")).unwrap();

        assert!(result.is_skip());
    }

    #[test]
    fn test_dispatch_visitor_preserve_html() {
        let visitor: Arc<Mutex<dyn HtmlVisitor + Send>> = Arc::new(Mutex::new(TestVisitor {
            mode: TestMode::PreserveHtml,
        }));
        let visitor_opt = Some(visitor);

        let ctx = NodeContext::with_borrowed_attributes(
            NodeType::Text,
            Cow::Borrowed(""),
            &EMPTY_ATTRS,
            0,
            0,
            None,
            true,
        );

        let result = dispatch_visitor(&visitor_opt, |v| v.visit_text(&ctx, "hello")).unwrap();

        assert!(result.is_preserve_html());
    }

    #[test]
    fn test_dispatch_visitor_error() {
        let visitor: Arc<Mutex<dyn HtmlVisitor + Send>> = Arc::new(Mutex::new(TestVisitor { mode: TestMode::Error }));
        let visitor_opt = Some(visitor);

        let ctx = NodeContext::with_borrowed_attributes(
            NodeType::Text,
            Cow::Borrowed(""),
            &EMPTY_ATTRS,
            0,
            0,
            None,
            true,
        );

        let result = dispatch_visitor(&visitor_opt, |v| v.visit_text(&ctx, "hello"));

        assert!(result.is_err());
        if let Err(ConversionError::Visitor(msg)) = result {
            assert_eq!(msg, "test error");
        } else {
            panic!("Expected Visitor error");
        }
    }
}
