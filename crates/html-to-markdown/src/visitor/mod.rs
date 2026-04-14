//! Visitor pattern for HTML to Markdown conversion.
//!
//! This module provides a comprehensive visitor trait that allows users to intervene
//! in the HTML→Markdown transformation logic at any point. Visitors can inspect,
//! modify, or replace the default conversion behavior for any HTML element type.
//!
//! # Design Philosophy
//!
//! - **Flexibility over performance**: Visitors prioritize giving users full control
//! - **Zero-cost when unused**: No performance impact if visitor feature is disabled
//! - **Comprehensive coverage**: All 60+ HTML element types have dedicated visitor methods
//! - **Pre/post hooks**: Both element entry and exit points are exposed
//!
//! # Example
//!
//! ```text
//! use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};
//!
//! struct CustomVisitor;
//!
//! impl HtmlVisitor for CustomVisitor {
//!     fn visit_link(&mut self, ctx: &NodeContext, href: &str, text: &str, title: Option<&str>) -> VisitResult {
//!         // Convert all links to plain text with URL in parentheses
//!         VisitResult::Custom(format!("{} ({})", text, href))
//!     }
//!
//!     fn visit_image(&mut self, ctx: &NodeContext, src: &str, alt: &str, title: Option<&str>) -> VisitResult {
//!         // Skip all images
//!         VisitResult::Skip
//!     }
//! }
//! ```

mod default_impl;
mod traits;
mod types;

// Re-export all public items from submodules
pub use default_impl::VisitorHandle;
pub use traits::HtmlVisitor;
pub use types::{NodeContext, NodeType, VisitResult};
