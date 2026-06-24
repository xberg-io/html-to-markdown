#![allow(missing_docs)]

//! Regression: deeply nested or malformed markup must not overflow the native
//! stack. Real pages with tens of thousands of unclosed `<td>` nest into a
//! multi-thousand-deep DOM (`tl` does not apply HTML5 implied-end-tags). The
//! auxiliary tree walks (`record_node_hierarchy`, `extract_head_metadata`,
//! `scan_table_node`, descendant text extraction, and plain/document-structure
//! walkers) used native recursion and aborted the process. The whole-subtree
//! helpers are now iterative, and the remaining recursive conversion walkers are
//! bounded by a native-stack safety limit.
//!
//! `<td>` is used rather than `<div>` so the deep chain exercises the auxiliary
//! walks without also driving `walk_node` deep: `td` dispatches to a non-
//! recursing handler, and `walk_node` is independently bounded by `max_depth`.
//! That keeps the bound on native recursion tight enough to catch a regression
//! on a small stack — an unbounded walk over a 20k-deep chain overflows it,
//! while the fixed iterative walks use the heap and pass comfortably.

use html_to_markdown_rs::convert;
use html_to_markdown_rs::options::{ConversionOptions, OutputFormat};
use std::sync::{Mutex, MutexGuard};
use std::thread;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

fn test_lock() -> MutexGuard<'static, ()> {
    TEST_MUTEX.lock().expect("deep nesting test mutex poisoned")
}

fn converts_without_overflow(html: String, options: ConversionOptions) -> bool {
    converts_without_overflow_on_stack(html, options, 256 * 1024)
}

fn converts_without_overflow_on_stack(html: String, options: ConversionOptions, stack_size: usize) -> bool {
    thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || convert(&html, Some(options)).is_ok())
        .expect("spawn conversion thread")
        .join()
        .expect("conversion thread overflowed the stack")
}

/// Exercises `record_node_hierarchy` (pre-pass) and `scan_table_node` (table
/// scan); the `<head>` is found without descending the deep chain.
#[test]
fn deep_unclosed_table_cells_do_not_overflow_stack() {
    let _guard = test_lock();
    let mut html = String::from("<html><head><title>t</title></head><body><table><tr>");
    for _ in 0..20_000 {
        html.push_str("<td>x");
    }
    html.push_str("</tr></table></body></html>");
    let options = ConversionOptions::builder().max_depth(Some(200)).build();
    assert!(converts_without_overflow(html, options));
}

/// No `<head>`, so `extract_head_metadata` must search the entire deep chain —
/// the path that overflowed on the 2stable.com page.
#[test]
fn deep_markup_without_head_does_not_overflow_stack() {
    let _guard = test_lock();
    let mut html = String::from("<html><body><table><tr>");
    for _ in 0..20_000 {
        html.push_str("<td>x");
    }
    html.push_str("</tr></table></body></html>");
    let options = ConversionOptions::builder().max_depth(Some(200)).build();
    assert!(converts_without_overflow(html, options));
}

#[test]
fn deep_link_descendant_text_does_not_overflow_stack() {
    let _guard = test_lock();
    let mut html = String::from("<html><head><title>t</title></head><body><a href=\"/deep\">");
    for _ in 0..1_000 {
        html.push_str("<span>");
    }
    html.push_str("deep");
    html.push_str("</a></body></html>");

    let options = ConversionOptions::builder().max_depth(Some(200)).build();
    assert!(converts_without_overflow_on_stack(html, options, 8 * 1024 * 1024));
}

#[test]
fn default_depth_uses_stack_safe_limit() {
    let _guard = test_lock();
    let mut html = String::from("<html><body>");
    for _ in 0..1_000 {
        html.push_str("<div>");
    }
    html.push_str("deep");
    for _ in 0..1_000 {
        html.push_str("</div>");
    }
    html.push_str("</body></html>");

    assert!(converts_without_overflow(html, ConversionOptions::default()));
}

#[test]
fn plain_text_output_does_not_overflow_stack() {
    let _guard = test_lock();
    let mut html = String::from("<html><body>");
    for _ in 0..1_000 {
        html.push_str("<div>");
    }
    html.push_str("deep");
    for _ in 0..1_000 {
        html.push_str("</div>");
    }
    html.push_str("</body></html>");

    let options = ConversionOptions {
        output_format: OutputFormat::Plain,
        max_depth: Some(200),
        ..Default::default()
    };
    assert!(converts_without_overflow(html, options));
}

#[test]
fn document_structure_builder_does_not_overflow_stack() {
    let _guard = test_lock();
    let mut html = String::from("<html><body>");
    for _ in 0..1_000 {
        html.push_str("<section>");
    }
    html.push_str("<p>deep</p>");
    for _ in 0..1_000 {
        html.push_str("</section>");
    }
    html.push_str("</body></html>");

    assert!(
        thread::Builder::new()
            .stack_size(8 * 1024 * 1024)
            .spawn(move || {
                let dom = tl::parse(&html, tl::ParserOptions::default()).expect("parse deep html");
                let document = html_to_markdown_rs::types::build_document_structure(&dom);
                !document.nodes.is_empty()
            })
            .expect("spawn structure thread")
            .join()
            .expect("structure thread overflowed the stack")
    );
}
