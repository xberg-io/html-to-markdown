package dev.kreuzberg.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class SmokeTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("dev.kreuzberg:html_to_markdown_android_jni")
        }
    }

    @Test
    fun test_smoke_empty_string() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: smoke_empty_string */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_simple_heading() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: smoke_simple_heading */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_simple_paragraph() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: smoke_simple_paragraph */)
        // TODO: assert result is not an error
    }

}
