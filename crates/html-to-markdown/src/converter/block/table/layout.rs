//! Table layout and indentation utilities.
//!
//! Handles table indentation for list context and table content
//! formatting when tables are nested within lists.

use crate::options::ListIndentType;

/// Indent table lines for list context.
///
/// When a table appears inside a list item, this function indents the table
/// content so it maintains proper list nesting.
///
/// # Arguments
/// * `table_content` - The Markdown table content to indent
/// * `list_depth` - The nesting depth in the list hierarchy
/// * `list_indent_columns` - Cumulative width of every ancestor `<li>`'s own marker
///   (see `Context::list_indent_columns`); used for the `Spaces` indent type.
/// * `options` - Conversion options (for indent type)
///
/// # Returns
/// Indented table content
pub fn indent_table_for_list(
    table_content: &str,
    list_depth: usize,
    list_indent_columns: usize,
    options: &crate::options::ConversionOptions,
) -> String {
    if list_depth == 0 {
        return table_content.to_string();
    }

    let Some(mut indent) = crate::converter::list::utils::continuation_indent_string(list_indent_columns, options)
    else {
        return table_content.to_string();
    };

    if matches!(options.list_indent_type, ListIndentType::Spaces) {
        let space_count = indent.chars().filter(|c| *c == ' ').count();
        if space_count < 4 {
            for _ in 0..(4 - space_count) {
                indent.push(' ');
            }
        }
    }

    let mut result = String::with_capacity(table_content.len() + indent.len() * 4);
    for segment in table_content.split_inclusive('\n') {
        if segment.starts_with('|') {
            result.push_str(&indent);
            result.push_str(segment);
        } else {
            result.push_str(segment);
        }
    }
    result
}
