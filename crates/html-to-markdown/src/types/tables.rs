//! Structured table types aligned with kreuzberg's `TableGrid`.

use serde::{Deserialize, Serialize};

/// A structured table grid with cell-level data including spans.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableGrid {
    /// Number of rows.
    pub rows: u32,
    /// Number of columns.
    pub cols: u32,
    /// All cells in the table (may be fewer than rows*cols due to spans).
    pub cells: Vec<GridCell>,
}

/// A single cell in a table grid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridCell {
    /// The text content of the cell.
    pub content: String,
    /// 0-indexed row position.
    pub row: u32,
    /// 0-indexed column position.
    pub col: u32,
    /// Number of rows this cell spans (default 1).
    #[serde(default = "default_span")]
    pub row_span: u32,
    /// Number of columns this cell spans (default 1).
    #[serde(default = "default_span")]
    pub col_span: u32,
    /// Whether this is a header cell (`<th>`).
    #[serde(default)]
    pub is_header: bool,
}

fn default_span() -> u32 {
    1
}

/// A top-level extracted table with both structured data and markdown representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    /// The structured table grid.
    pub grid: TableGrid,
    /// The markdown rendering of this table.
    pub markdown: String,
}
