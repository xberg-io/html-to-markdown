package dev.kreuzberg.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class VisitorTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("html_to_markdown_rs_jni")
        }
    }

    @Test
    fun test_visitor_audio_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_audio_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_audio_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_audio_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_button_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_button_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_button_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_button_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_continue_default() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_continue_default */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_custom_blockquote() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_custom_blockquote */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_custom_emphasis() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_custom_emphasis */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_custom_heading() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_custom_heading */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_custom_image() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_custom_image */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_custom_link_format() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_custom_link_format */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_custom_link_static() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_custom_link_static */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_custom_output() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_custom_output */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_definition_list_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_definition_list_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_definition_list_custom_format() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_definition_list_custom_format */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_definition_list_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_definition_list_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_details_summary_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_details_summary_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_details_summary_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_details_summary_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_figure_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_figure_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_figure_custom_wrap() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_figure_custom_wrap */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_figure_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_figure_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_form_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_form_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_form_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_form_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_heading_bare_string_preserves_case() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_heading_bare_string_preserves_case */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_horizontal_rule_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_horizontal_rule_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_horizontal_rule_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_horizontal_rule_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_iframe_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_iframe_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_iframe_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_iframe_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_image_bare_string_preserves_case() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_image_bare_string_preserves_case */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_input_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_input_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_input_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_input_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_line_break_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_line_break_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_line_break_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_line_break_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_link_bare_string_preserves_case() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_link_bare_string_preserves_case */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_mark_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_mark_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_mark_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_mark_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_preserve_html() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_preserve_html */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_skip_code_blocks() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_skip_code_blocks */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_skip_heading() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_skip_heading */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_skip_images() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_skip_images */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_skip_links() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_skip_links */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_skip_strong() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_skip_strong */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_subscript_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_subscript_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_subscript_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_subscript_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_superscript_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_superscript_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_superscript_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_superscript_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_underline_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_underline_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_underline_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_underline_skip */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_video_custom() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_video_custom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_visitor_video_skip() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: visitor_video_skip */)
        // TODO: assert result is not an error
    }

}
