<?php

declare(strict_types=1);

/**
 * Stub declarations for the html_to_markdown PHP extension functions.
 * These stubs allow PHPStan to analyze code that calls extension functions
 * that may not be available in the current PHP environment during analysis.
 */

/**
 * Convert HTML to Markdown using the native Rust extension.
 *
 * @param array{
 *   heading_style?: 'underlined'|'atx'|'atx_closed',
 *   list_indent_type?: 'spaces'|'tabs',
 *   list_indent_width?: int,
 *   bullets?: string,
 *   strong_em_symbol?: string,
 *   escape_asterisks?: bool,
 *   escape_underscores?: bool,
 *   escape_misc?: bool,
 *   escape_ascii?: bool,
 *   code_language?: string,
 *   autolinks?: bool,
 *   default_title?: bool,
 *   br_in_tables?: bool,
 *   highlight_style?: 'double_equal'|'html'|'bold'|'none',
 *   extract_metadata?: bool,
 *   whitespace_mode?: 'normalized'|'strict',
 *   strip_newlines?: bool,
 *   wrap?: bool,
 *   wrap_width?: int,
 *   convert_as_inline?: bool,
 *   sub_symbol?: string,
 *   sup_symbol?: string,
 *   newline_style?: 'spaces'|'backslash',
 *   code_block_style?: 'indented'|'backticks'|'tildes',
 *   keep_inline_images_in?: list<string>,
 *   preprocessing?: array<string, bool|string>|null,
 *   encoding?: string,
 *   debug?: bool,
 *   strip_tags?: list<string>,
 *   preserve_tags?: list<string>,
 *   skip_images?: bool,
 *   output_format?: 'markdown'|'djot'|'plain',
 *   include_document_structure?: bool,
 *   extract_images?: bool,
 *   max_image_size?: int,
 *   capture_svg?: bool,
 *   infer_dimensions?: bool,
 *   max_depth?: int,
 * }|null $options
 * @return array{
 *   content: string|null,
 *   document: string|null,
 *   metadata: string|null,
 *   tables: list<array{grid: array{rows: int, cols: int, cells: list<array{content: string, row: int, col: int, row_span: int, col_span: int, is_header: bool}>}, markdown: string}>,
 *   images: list<array{data: string, format: string, filename: string|null, description: string|null, width: int|null, height: int|null, source: string, attributes: array<string, string>}>,
 *   warnings: list<array{message: string, kind: string}>,
 * }
 */
function html_to_markdown_convert(string $html, ?array $options = null): array
{
    throw new \RuntimeException('html_to_markdown extension not loaded');
}
