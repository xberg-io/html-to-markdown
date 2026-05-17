package dev.kreuzberg.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class RealWorldTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("html_to_markdown_rs_jni")
        }
    }

    @Test
    fun test_real_world_blog_post() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: real_world_blog_post */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_real_world_documentation_page() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: real_world_documentation_page */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_real_world_product_page() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: real_world_product_page */)
        // TODO: assert result is not an error
    }

}
