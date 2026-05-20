<?php

declare(strict_types=1);

namespace HtmlToMarkdown;

/**
 * Callback interface for custom HTML traversal and transformation.
 *
 * Implement this interface and pass your implementation to the `visitor` field of
 * `ConversionOptions` to customize how the HTML document is traversed and converted to Markdown.
 *
 * All methods have default no-op implementations that return `VisitResult::Continue`.
 * Override only the methods you need.
 */
interface HtmlVisitorInterface
{
    /**
     * Visit text nodes (most frequent callback - ~100+ per document).
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_text(NodeContext $context, string $_text): VisitResult;

    /**
     * Called before entering any element.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_element_start(NodeContext $context): VisitResult;

    /**
     * Called after exiting any element.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_output
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_element_end(NodeContext $context, string $_output): VisitResult;

    /**
     * Visit anchor links `<a href="...">`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_href
     * @param string $_text
     * @param ?string $_title
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_link(NodeContext $context, string $_href, string $_text, ?string $_title): VisitResult;

    /**
     * Visit images `<img src="...">`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_src
     * @param string $_alt
     * @param ?string $_title
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_image(NodeContext $context, string $_src, string $_alt, ?string $_title): VisitResult;

    /**
     * Visit heading elements `<h1>` through `<h6>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param int $_level
     * @param string $_text
     * @param ?string $_id
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_heading(NodeContext $context, int $_level, string $_text, ?string $_id): VisitResult;

    /**
     * Visit code blocks `<pre><code>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param ?string $_lang
     * @param string $_code
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_code_block(NodeContext $context, ?string $_lang, string $_code): VisitResult;

    /**
     * Visit inline code `<code>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_code
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_code_inline(NodeContext $context, string $_code): VisitResult;

    /**
     * Visit list items `<li>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param bool $_ordered
     * @param string $_marker
     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_list_item(NodeContext $context, bool $_ordered, string $_marker, string $_text): VisitResult;

    /**
     * Called before processing a list `<ul>` or `<ol>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param bool $_ordered
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_list_start(NodeContext $context, bool $_ordered): VisitResult;

    /**
     * Called after processing a list `</ul>` or `</ol>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param bool $_ordered
     * @param string $_output
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_list_end(NodeContext $context, bool $_ordered, string $_output): VisitResult;

    /**
     * Called before processing a table `<table>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_table_start(NodeContext $context): VisitResult;

    /**
     * Visit table rows `<tr>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param mixed $_cells
     * @param bool $_is_header
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_table_row(NodeContext $context, mixed $_cells, bool $_is_header): VisitResult;

    /**
     * Called after processing a table `</table>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_output
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_table_end(NodeContext $context, string $_output): VisitResult;

    /**
     * Visit blockquote elements `<blockquote>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_content
     * @param int $_depth
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_blockquote(NodeContext $context, string $_content, int $_depth): VisitResult;

    /**
     * Visit strong/bold elements `<strong>`, `<b>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_strong(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit emphasis/italic elements `<em>`, `<i>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_emphasis(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit strikethrough elements `<s>`, `<del>`, `<strike>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_strikethrough(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit underline elements `<u>`, `<ins>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_underline(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit subscript elements `<sub>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_subscript(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit superscript elements `<sup>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_superscript(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit mark/highlight elements `<mark>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_mark(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit line break elements `<br>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_line_break(NodeContext $context): VisitResult;

    /**
     * Visit horizontal rule elements `<hr>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_horizontal_rule(NodeContext $context): VisitResult;

    /**
     * Visit custom elements (web components) or unknown tags.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_tag_name
     * @param string $_html
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_custom_element(NodeContext $context, string $_tag_name, string $_html): VisitResult;

    /**
     * Visit definition list `<dl>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_definition_list_start(NodeContext $context): VisitResult;

    /**
     * Visit definition term `<dt>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_definition_term(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit definition description `<dd>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_definition_description(NodeContext $context, string $_text): VisitResult;

    /**
     * Called after processing a definition list `</dl>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_output
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_definition_list_end(NodeContext $context, string $_output): VisitResult;

    /**
     * Visit form elements `<form>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param ?string $_action
     * @param ?string $_method
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_form(NodeContext $context, ?string $_action, ?string $_method): VisitResult;

    /**
     * Visit input elements `<input>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_input_type
     * @param ?string $_name
     * @param ?string $_value
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_input(NodeContext $context, string $_input_type, ?string $_name, ?string $_value): VisitResult;

    /**
     * Visit button elements `<button>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_button(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit audio elements `<audio>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param ?string $_src
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_audio(NodeContext $context, ?string $_src): VisitResult;

    /**
     * Visit video elements `<video>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param ?string $_src
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_video(NodeContext $context, ?string $_src): VisitResult;

    /**
     * Visit iframe elements `<iframe>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param ?string $_src
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_iframe(NodeContext $context, ?string $_src): VisitResult;

    /**
     * Visit details elements `<details>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param bool $_open
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_details(NodeContext $context, bool $_open): VisitResult;

    /**
     * Visit summary elements `<summary>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_summary(NodeContext $context, string $_text): VisitResult;

    /**
     * Visit figure elements `<figure>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_figure_start(NodeContext $context): VisitResult;

    /**
     * Visit figcaption elements `<figcaption>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_text
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_figcaption(NodeContext $context, string $_text): VisitResult;

    /**
     * Called after processing a figure `</figure>`.
     *
     * @param NodeContext $context Node context information (type, depth, path, etc.)

     * @param string $_output
     * @return VisitResult How to proceed with traversal (Continue, Skip, or Custom output)
     */
    public function visit_figure_end(NodeContext $context, string $_output): VisitResult;

}
