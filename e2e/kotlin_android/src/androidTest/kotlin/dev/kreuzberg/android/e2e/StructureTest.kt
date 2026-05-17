package dev.kreuzberg.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class StructureTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("html_to_markdown_rs_jni")
        }
    }

    @Test
    fun test_structure_code_block() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structure_code_block */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_structure_deep_nesting_h1_h2_h3() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structure_deep_nesting_h1_h2_h3 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_structure_h1_h2_nested_group() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structure_h1_h2_nested_group */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_structure_heading_paragraph() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structure_heading_paragraph */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_structure_list() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structure_list */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_structure_multiple_headings() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structure_multiple_headings */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_structure_sibling_h1_groups() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structure_sibling_h1_groups */)
        // TODO: assert result is not an error
    }

}
