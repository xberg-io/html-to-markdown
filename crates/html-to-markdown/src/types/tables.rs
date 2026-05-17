//! Structured table types aligned with kreuzberg's `TableGrid`.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A structured table grid with cell-level data including spans.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TableGrid {
    /// Number of rows.
    pub rows: u32,
    /// Number of columns.
    pub cols: u32,
    /// All cells in the table as a flat, sparse list.
    ///
    /// The list is ordered by `(row, col)` but is **not** a dense `rows × cols` matrix: cells
    /// that are covered by a spanning cell (via `row_span > 1` or `col_span > 1`) do not appear
    /// in the list. Only the top-left "origin" cell of a span is present, with its `row_span`
    /// and `col_span` fields set accordingly.
    ///
    /// To reconstruct the full visual grid, iterate over all cells and mark the rectangular
    /// region `[row .. row+row_span, col .. col+col_span]` as occupied by that cell. Any
    /// `(row, col)` position that is not the origin of any cell is covered by a span from an
    /// earlier cell.
    ///
    /// The length of this vec is `≤ rows * cols`. An empty table (`rows == 0 || cols == 0`)
    /// produces an empty vec.
    pub cells: Vec<GridCell>,
}

/// A single cell in a table grid.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridCell {
    /// The text content of the cell.
    pub content: String,
    /// 0-indexed row position.
    pub row: u32,
    /// 0-indexed column position.
    pub col: u32,
    /// Number of rows this cell spans (default 1).
    #[cfg_attr(feature = "serde", serde(default = "default_span"))]
    pub row_span: u32,
    /// Number of columns this cell spans (default 1).
    #[cfg_attr(feature = "serde", serde(default = "default_span"))]
    pub col_span: u32,
    /// Whether this is a header cell (`<th>`).
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_header: bool,
}

#[cfg(feature = "serde")]
fn default_span() -> u32 {
    1
}

/// A top-level extracted table with both structured data and markdown representation.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TableData {
    /// The structured table grid.
    pub grid: TableGrid,
    /// The markdown rendering of this table.
    pub markdown: String,
}
