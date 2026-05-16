package dev.kreuzberg.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class MetadataTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("html_to_markdown_rs_jni")
        }
    }

    @Test
    fun test_metadata_author_meta() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_author_meta */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_canonical_url() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_canonical_url */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_description_meta() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_description_meta */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_dublin_core() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_dublin_core */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_extract_all_images() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_extract_all_images */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_extract_all_links() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_extract_all_links */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_headers_hierarchy() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_headers_hierarchy */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_keywords_meta() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_keywords_meta */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_lang_attribute() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_lang_attribute */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_microdata_schema_article() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_microdata_schema_article */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_microdata_schema_breadcrumb() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_microdata_schema_breadcrumb */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_microdata_schema_organization() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_microdata_schema_organization */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_microdata_schema_person() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_microdata_schema_person */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_microdata_schema_product() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_microdata_schema_product */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_text_direction_ltr() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_text_direction_ltr */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_text_direction_rtl() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_text_direction_rtl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_metadata_title_tag() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: metadata_title_tag */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_og_basic_tags() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: og_basic_tags */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_og_multiple_tags() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: og_multiple_tags */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_structured_data_json_ld() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structured_data_json_ld */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_structured_data_multiple_json_ld() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: structured_data_multiple_json_ld */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_twitter_card_tags() {
        val client = dev.kreuzberg.android.HtmlToMarkdownRs()
        val result = client.convert(/* fixture: twitter_card_tags */)
        // TODO: assert result is not an error
    }

}
