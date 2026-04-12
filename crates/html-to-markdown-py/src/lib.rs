#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

// Module declarations for modular organization
pub mod helpers;
pub mod options;
pub mod types;

#[cfg(feature = "visitor")]
pub mod visitor;

// Re-exports from modules for public API
pub use options::{ConversionOptions, PreprocessingOptions};

use helpers::{run_with_guard, to_py_err};
#[cfg(feature = "visitor")]
use html_to_markdown_rs::visitor::HtmlVisitor;
use pyo3::prelude::*;
#[cfg(feature = "visitor")]
use std::cell::RefCell;
#[cfg(feature = "visitor")]
use std::rc::Rc;

/// Convert HTML to Markdown, returning a dict with content, metadata, tables, images, and warnings.
///
/// This is the primary API. It calls the Rust `convert()` function and returns all
/// extracted data as a Python dictionary.
///
/// Args:
///     html: HTML string to convert
///     options: Optional conversion configuration
///     visitor: Optional visitor object with callback methods
///
/// Returns:
///     dict with keys:
///         - content (str | None): Converted markdown output, or None in extraction-only mode
///         - document (None): Document structure (not yet exposed in bindings)
///         - metadata (dict | None): Extracted HTML metadata (requires metadata feature)
///         - tables (list[dict]): Extracted tables with grid and markdown fields
///         - images (list): Extracted inline images (requires inline-images feature)
///         - warnings (list[dict]): Non-fatal processing warnings with message and kind fields
///
/// Raises:
///     ValueError: Invalid HTML or configuration
///
/// Example:
///     ```ignore
///     from html_to_markdown import convert
///
///     result = convert("<h1>Hello</h1><p>World</p>")
///     print(result["content"])    # "# Hello\n\nWorld\n"
///     print(result["warnings"])   # []
///     ```
#[pyfunction]
#[cfg(feature = "visitor")]
#[pyo3(signature = (html, options=None, visitor=None))]
fn convert<'py>(
    py: Python<'py>,
    html: &str,
    options: Option<ConversionOptions>,
    visitor: Option<Py<PyAny>>,
) -> PyResult<Py<pyo3::types::PyDict>> {
    use pyo3::types::{PyDict, PyList};

    let html = html.to_owned();
    let rust_options = options.map(|opts| opts.to_rust());

    let result = if let Some(visitor_py) = visitor {
        let visitor_handle = std::sync::Arc::new(std::sync::Mutex::new(visitor_py));
        py.detach(move || {
            run_with_guard(|| {
                let rc_visitor: Rc<RefCell<dyn HtmlVisitor>> = {
                    Python::attach(|py| {
                        let guard = visitor_handle.lock().unwrap();
                        let bridge = visitor::PyVisitorBridge::new(guard.clone_ref(py));
                        Rc::new(RefCell::new(bridge)) as Rc<RefCell<dyn HtmlVisitor>>
                    })
                };
                let res =
                    html_to_markdown_rs::convert_with_visitor_result(&html, rust_options.clone(), Some(rc_visitor))?;
                Ok(res)
            })
        })
        .map_err(to_py_err)?
    } else {
        py.detach(move || run_with_guard(|| html_to_markdown_rs::convert(&html, rust_options.clone())))
            .map_err(to_py_err)?
    };

    let dict = PyDict::new(py);

    // content: Option<String>
    match result.content {
        Some(ref s) => dict.set_item("content", s)?,
        None => dict.set_item("content", py.None())?,
    }

    // document: not yet wired in bindings
    dict.set_item("document", py.None())?;

    // metadata: cfg(feature = "metadata")
    #[cfg(feature = "metadata")]
    {
        use crate::types::extended_metadata_to_py;
        let metadata_py = extended_metadata_to_py(py, result.metadata)?;
        dict.set_item("metadata", metadata_py)?;
    }
    #[cfg(not(feature = "metadata"))]
    dict.set_item("metadata", py.None())?;

    // tables: Vec<TableData> with grid (TableGrid) and markdown fields
    {
        let tables_list = PyList::empty(py);
        for table in &result.tables {
            let table_dict = PyDict::new(py);
            // grid: TableGrid { rows, cols, cells: Vec<GridCell> }
            let grid_dict = PyDict::new(py);
            grid_dict.set_item("rows", table.grid.rows)?;
            grid_dict.set_item("cols", table.grid.cols)?;
            let cells_list = PyList::empty(py);
            for cell in &table.grid.cells {
                let cell_dict = PyDict::new(py);
                cell_dict.set_item("content", &cell.content)?;
                cell_dict.set_item("row", cell.row)?;
                cell_dict.set_item("col", cell.col)?;
                cell_dict.set_item("row_span", cell.row_span)?;
                cell_dict.set_item("col_span", cell.col_span)?;
                cell_dict.set_item("is_header", cell.is_header)?;
                cells_list.append(cell_dict)?;
            }
            grid_dict.set_item("cells", cells_list)?;
            table_dict.set_item("grid", grid_dict)?;
            table_dict.set_item("markdown", &table.markdown)?;
            tables_list.append(table_dict)?;
        }
        dict.set_item("tables", tables_list)?;
    }

    // images: cfg(feature = "inline-images")
    #[cfg(feature = "inline-images")]
    {
        use crate::types::inline_image_to_py;
        let images_list = PyList::empty(py);
        for image in result.images {
            let image_py = inline_image_to_py(py, image)?;
            images_list.append(image_py)?;
        }
        dict.set_item("images", images_list)?;
    }
    #[cfg(not(feature = "inline-images"))]
    dict.set_item("images", PyList::empty(py))?;

    // warnings: Vec<ProcessingWarning>
    {
        let warnings_list = PyList::empty(py);
        for warning in &result.warnings {
            let w_dict = PyDict::new(py);
            w_dict.set_item("message", &warning.message)?;
            let kind_str = match warning.kind {
                html_to_markdown_rs::WarningKind::ImageExtractionFailed => "image_extraction_failed",
                html_to_markdown_rs::WarningKind::EncodingFallback => "encoding_fallback",
                html_to_markdown_rs::WarningKind::TruncatedInput => "truncated_input",
                html_to_markdown_rs::WarningKind::MalformedHtml => "malformed_html",
                html_to_markdown_rs::WarningKind::SanitizationApplied => "sanitization_applied",
            };
            w_dict.set_item("kind", kind_str)?;
            warnings_list.append(w_dict)?;
        }
        dict.set_item("warnings", warnings_list)?;
    }

    Ok(dict.into())
}

#[pyfunction]
#[cfg(not(feature = "visitor"))]
#[pyo3(signature = (html, options=None))]
fn convert<'py>(py: Python<'py>, html: &str, options: Option<ConversionOptions>) -> PyResult<Py<pyo3::types::PyDict>> {
    use pyo3::types::{PyDict, PyList};

    let html = html.to_owned();
    let rust_options = options.map(|opts| opts.to_rust());

    let result = py
        .detach(move || run_with_guard(|| html_to_markdown_rs::convert(&html, rust_options.clone())))
        .map_err(to_py_err)?;

    let dict = PyDict::new(py);

    match result.content {
        Some(ref s) => dict.set_item("content", s)?,
        None => dict.set_item("content", py.None())?,
    }

    dict.set_item("document", py.None())?;

    #[cfg(feature = "metadata")]
    {
        use crate::types::extended_metadata_to_py;
        let metadata_py = extended_metadata_to_py(py, result.metadata)?;
        dict.set_item("metadata", metadata_py)?;
    }
    #[cfg(not(feature = "metadata"))]
    dict.set_item("metadata", py.None())?;

    {
        let tables_list = PyList::empty(py);
        for table in &result.tables {
            let table_dict = PyDict::new(py);
            let grid_dict = PyDict::new(py);
            grid_dict.set_item("rows", table.grid.rows)?;
            grid_dict.set_item("cols", table.grid.cols)?;
            let cells_list = PyList::empty(py);
            for cell in &table.grid.cells {
                let cell_dict = PyDict::new(py);
                cell_dict.set_item("content", &cell.content)?;
                cell_dict.set_item("row", cell.row)?;
                cell_dict.set_item("col", cell.col)?;
                cell_dict.set_item("row_span", cell.row_span)?;
                cell_dict.set_item("col_span", cell.col_span)?;
                cell_dict.set_item("is_header", cell.is_header)?;
                cells_list.append(cell_dict)?;
            }
            grid_dict.set_item("cells", cells_list)?;
            table_dict.set_item("grid", grid_dict)?;
            table_dict.set_item("markdown", &table.markdown)?;
            tables_list.append(table_dict)?;
        }
        dict.set_item("tables", tables_list)?;
    }

    #[cfg(feature = "inline-images")]
    {
        use crate::types::inline_image_to_py;
        let images_list = PyList::empty(py);
        for image in result.images {
            let image_py = inline_image_to_py(py, image)?;
            images_list.append(image_py)?;
        }
        dict.set_item("images", images_list)?;
    }
    #[cfg(not(feature = "inline-images"))]
    dict.set_item("images", PyList::empty(py))?;

    {
        let warnings_list = PyList::empty(py);
        for warning in &result.warnings {
            let w_dict = PyDict::new(py);
            w_dict.set_item("message", &warning.message)?;
            let kind_str = match warning.kind {
                html_to_markdown_rs::WarningKind::ImageExtractionFailed => "image_extraction_failed",
                html_to_markdown_rs::WarningKind::EncodingFallback => "encoding_fallback",
                html_to_markdown_rs::WarningKind::TruncatedInput => "truncated_input",
                html_to_markdown_rs::WarningKind::MalformedHtml => "malformed_html",
                html_to_markdown_rs::WarningKind::SanitizationApplied => "sanitization_applied",
            };
            w_dict.set_item("kind", kind_str)?;
            warnings_list.append(w_dict)?;
        }
        dict.set_item("warnings", warnings_list)?;
    }

    Ok(dict.into())
}

/// Python bindings for html-to-markdown
#[pymodule]
fn _html_to_markdown(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_class::<ConversionOptions>()?;
    m.add_class::<PreprocessingOptions>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_returns_markdown() {
        Python::initialize();
        Python::attach(|py| -> PyResult<()> {
            let html = "<h1>Hello</h1>";
            let result = convert(py, html, None, None)?;
            let content: String = result
                .bind(py)
                .get_item("content")?
                .expect("content key must exist")
                .extract()?;
            assert!(content.contains("Hello"));
            Ok(())
        })
        .expect("conversion succeeds");
    }

    #[test]
    fn test_conversion_options_defaults() {
        let opts = ConversionOptions::new(
            "underlined".to_string(),
            "spaces".to_string(),
            4,
            "*+-".to_string(),
            '*',
            false,
            false,
            false,
            false,
            "".to_string(),
            true,
            false,
            false,
            "double-equal".to_string(),
            false,
            "normalized".to_string(),
            true,
            false,
            80,
            false,
            "".to_string(),
            "".to_string(),
            "spaces".to_string(),
            "indented".to_string(),
            Vec::new(),
            None,
            false,
            Vec::new(),
            Vec::new(),
            "utf-8".to_string(),
            false,
            "inline".to_string(),
            "markdown".to_string(),
            false,
            false,
            5242880u64,
            false,
            true,
        );
        let rust_opts = opts.to_rust();
        assert_eq!(rust_opts.list_indent_width, 4);
        assert_eq!(rust_opts.wrap_width, 80);
    }

    #[test]
    fn test_preprocessing_options_conversion() {
        let preprocessing = PreprocessingOptions::new(true, "aggressive".to_string(), true, false);
        let rust_preprocessing = preprocessing.to_rust();
        assert!(rust_preprocessing.enabled);
        assert!(matches!(
            rust_preprocessing.preset,
            html_to_markdown_rs::PreprocessingPreset::Aggressive
        ));
        assert!(rust_preprocessing.remove_navigation);
        assert!(!rust_preprocessing.remove_forms);
    }
}
