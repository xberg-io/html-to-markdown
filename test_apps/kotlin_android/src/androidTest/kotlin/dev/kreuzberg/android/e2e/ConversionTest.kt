package dev.kreuzberg.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class ConversionTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("dev.kreuzberg:html_to_markdown_android_jni")
        }
    }

    @Test
    fun test_blockquote_multiple_paragraphs() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: blockquote_multiple_paragraphs */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_blockquote_nested() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: blockquote_nested */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_blockquote_simple() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: blockquote_simple */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_blockquote_with_list() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: blockquote_with_list */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_bold_and_italic() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: bold_and_italic */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_bold_strong() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: bold_strong */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_code_block() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: code_block */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_code_block_no_language() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: code_block_no_language */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_code_inline_in_paragraph() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: code_inline_in_paragraph */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_code_with_backticks_in_content() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: code_with_backticks_in_content */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_emphasis_mark_highlight() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: emphasis_mark_highlight */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_emphasis_strikethrough_del() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: emphasis_strikethrough_del */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_emphasis_strikethrough_s() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: emphasis_strikethrough_s */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_emphasis_subscript() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: emphasis_subscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_emphasis_superscript() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: emphasis_superscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_emphasis_underline_u() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: emphasis_underline_u */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_form_input_elements() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: form_input_elements */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_form_select_options() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: form_select_options */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_form_textarea() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: form_textarea */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_heading_h1() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: heading_h1 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_heading_h2() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: heading_h2 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_heading_h3() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: heading_h3 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_heading_h4() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: heading_h4 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_heading_h5() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: heading_h5 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_heading_h6() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: heading_h6 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_image_figure_figcaption() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: image_figure_figcaption */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_image_linked() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: image_linked */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_image_no_alt() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: image_no_alt */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_image_simple() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: image_simple */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_image_with_title() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: image_with_title */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_inline_code() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: inline_code */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_italic_em() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: italic_em */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_line_break_br_tag() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: line_break_br_tag */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_line_break_hr_tag() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: line_break_hr_tag */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_line_break_multiple_br() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: line_break_multiple_br */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_link_anchor_fragment() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: link_anchor_fragment */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_link_empty_href() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: link_empty_href */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_link_image_inside() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: link_image_inside */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_link_mailto() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: link_mailto */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_link_simple() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: link_simple */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_link_with_bold_text() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: link_with_bold_text */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_link_with_title() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: link_with_title */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_definition_dl() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: list_definition_dl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_item_multiple_paragraphs() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: list_item_multiple_paragraphs */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_mixed_nested() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: list_mixed_nested */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_nested_ordered() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: list_nested_ordered */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_nested_unordered() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: list_nested_unordered */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_list_task_checkboxes() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: list_task_checkboxes */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_ordered_list() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: ordered_list */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_paragraph_multiple() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: paragraph_multiple */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_paragraph_nested_divs() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: paragraph_nested_divs */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_paragraph_simple() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: paragraph_simple */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_paragraph_with_inline_formatting() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: paragraph_with_inline_formatting */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_paragraph_with_line_breaks() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: paragraph_with_line_breaks */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_semantic_abbr() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: semantic_abbr */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_semantic_article() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: semantic_article */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_semantic_definition_list() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: semantic_definition_list */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_semantic_details_summary() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: semantic_details_summary */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_semantic_hr() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: semantic_hr */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_semantic_mark_highlight() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: semantic_mark_highlight */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_semantic_section_with_heading() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: semantic_section_with_heading */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_semantic_sub_superscript() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: semantic_sub_superscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_simple_table() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: simple_table */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_table_empty() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: table_empty */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_table_no_thead() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: table_no_thead */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_table_pipe_chars_in_content() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: table_pipe_chars_in_content */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_table_with_alignment() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: table_with_alignment */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_table_with_colspan() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: table_with_colspan */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_unordered_list() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: unordered_list */)
        // TODO: assert result is not an error
    }

}
