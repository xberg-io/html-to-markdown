package dev.kreuzberg.htmltomarkdown;

import java.util.List;
import java.util.Map;

/**
 * Bridge interface for the HtmlVisitor plugin system.
 *
 * Implementations are wrapped by HtmlVisitorBridge and exposed to the native runtime through Panama FFM upcall stubs.
 */
public interface IHtmlVisitor {

    /** visit_element_start. */
    VisitResult visit_element_start(NodeContext _ctx) throws Exception;

    /** visit_element_end. */
    VisitResult visit_element_end(NodeContext _ctx, String _output) throws Exception;

    /** visit_text. */
    VisitResult visit_text(NodeContext _ctx, String _text) throws Exception;

    /** visit_link. */
    VisitResult visit_link(NodeContext _ctx, String _href, String _text, String _title) throws Exception;

    /** visit_image. */
    VisitResult visit_image(NodeContext _ctx, String _src, String _alt, String _title) throws Exception;

    /** visit_heading. */
    VisitResult visit_heading(NodeContext _ctx, int _level, String _text, String _id) throws Exception;

    /** visit_code_block. */
    VisitResult visit_code_block(NodeContext _ctx, String _lang, String _code) throws Exception;

    /** visit_code_inline. */
    VisitResult visit_code_inline(NodeContext _ctx, String _code) throws Exception;

    /** visit_list_item. */
    VisitResult visit_list_item(NodeContext _ctx, boolean _ordered, String _marker, String _text) throws Exception;

    /** visit_list_start. */
    VisitResult visit_list_start(NodeContext _ctx, boolean _ordered) throws Exception;

    /** visit_list_end. */
    VisitResult visit_list_end(NodeContext _ctx, boolean _ordered, String _output) throws Exception;

    /** visit_table_start. */
    VisitResult visit_table_start(NodeContext _ctx) throws Exception;

    /** visit_table_row. */
    VisitResult visit_table_row(NodeContext _ctx, List<String> _cells, boolean _is_header) throws Exception;

    /** visit_table_end. */
    VisitResult visit_table_end(NodeContext _ctx, String _output) throws Exception;

    /** visit_blockquote. */
    VisitResult visit_blockquote(NodeContext _ctx, String _content, long _depth) throws Exception;

    /** visit_strong. */
    VisitResult visit_strong(NodeContext _ctx, String _text) throws Exception;

    /** visit_emphasis. */
    VisitResult visit_emphasis(NodeContext _ctx, String _text) throws Exception;

    /** visit_strikethrough. */
    VisitResult visit_strikethrough(NodeContext _ctx, String _text) throws Exception;

    /** visit_underline. */
    VisitResult visit_underline(NodeContext _ctx, String _text) throws Exception;

    /** visit_subscript. */
    VisitResult visit_subscript(NodeContext _ctx, String _text) throws Exception;

    /** visit_superscript. */
    VisitResult visit_superscript(NodeContext _ctx, String _text) throws Exception;

    /** visit_mark. */
    VisitResult visit_mark(NodeContext _ctx, String _text) throws Exception;

    /** visit_line_break. */
    VisitResult visit_line_break(NodeContext _ctx) throws Exception;

    /** visit_horizontal_rule. */
    VisitResult visit_horizontal_rule(NodeContext _ctx) throws Exception;

    /** visit_custom_element. */
    VisitResult visit_custom_element(NodeContext _ctx, String _tag_name, String _html) throws Exception;

    /** visit_definition_list_start. */
    VisitResult visit_definition_list_start(NodeContext _ctx) throws Exception;

    /** visit_definition_term. */
    VisitResult visit_definition_term(NodeContext _ctx, String _text) throws Exception;

    /** visit_definition_description. */
    VisitResult visit_definition_description(NodeContext _ctx, String _text) throws Exception;

    /** visit_definition_list_end. */
    VisitResult visit_definition_list_end(NodeContext _ctx, String _output) throws Exception;

    /** visit_form. */
    VisitResult visit_form(NodeContext _ctx, String _action, String _method) throws Exception;

    /** visit_input. */
    VisitResult visit_input(NodeContext _ctx, String _input_type, String _name, String _value) throws Exception;

    /** visit_button. */
    VisitResult visit_button(NodeContext _ctx, String _text) throws Exception;

    /** visit_audio. */
    VisitResult visit_audio(NodeContext _ctx, String _src) throws Exception;

    /** visit_video. */
    VisitResult visit_video(NodeContext _ctx, String _src) throws Exception;

    /** visit_iframe. */
    VisitResult visit_iframe(NodeContext _ctx, String _src) throws Exception;

    /** visit_details. */
    VisitResult visit_details(NodeContext _ctx, boolean _open) throws Exception;

    /** visit_summary. */
    VisitResult visit_summary(NodeContext _ctx, String _text) throws Exception;

    /** visit_figure_start. */
    VisitResult visit_figure_start(NodeContext _ctx) throws Exception;

    /** visit_figcaption. */
    VisitResult visit_figcaption(NodeContext _ctx, String _text) throws Exception;

    /** visit_figure_end. */
    VisitResult visit_figure_end(NodeContext _ctx, String _output) throws Exception;

}
