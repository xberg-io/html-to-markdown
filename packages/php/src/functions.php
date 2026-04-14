<?php

declare(strict_types=1);

namespace {

    use Html\To\Markdown\Rs\ConversionOptions;

    if (!\function_exists('html_to_markdown_convert')) {
        /**
         * Convert HTML to Markdown and return the content string.
         *
         * This is a pure-PHP fallback used when the native `html_to_markdown`
         * extension is not loaded. It delegates to the ext-php-rs binding's
         * `convert()` function and maps array-based options to a ConversionOptions
         * object via JSON.
         *
         * When the native extension IS loaded this function is already provided
         * natively; the guard above prevents a redeclaration error.
         *
         * Enum-valued options accept lowercase strings matching the Rust variant names:
         *   - heading_style:      "atx" (default), "atx_closed", "underlined"
         *   - code_block_style:   "backticks" (default), "tildes", "indented"
         *   - list_indent_type:   "spaces" (default), "tabs"
         *   - whitespace_mode:    "normalized" (default), "strict"
         *   - newline_style:      "spaces" (default), "backslash"
         *   - highlight_style:    "double_equal" (default), "html", "bold", "none"
         *   - link_style:         "inline" (default), "reference"
         *   - output_format:      "markdown" (default), "djot", "plain"
         *
         * @param string               $html    The HTML string to convert.
         * @param array<string, mixed> $options Optional conversion options.
         *
         * @throws \Html\To\Markdown\Rs\HtmlToMarkdownRsException on conversion error.
         */
        function html_to_markdown_convert(string $html, array $options = []): string
        {
            if ($options === []) {
                $result = convert($html, null);
            } else {
                // Encode the default ConversionOptions object to get all field defaults,
                // merge the caller's overrides on top, then reconstruct via from_json().
                $defaults = \json_decode(\json_encode(ConversionOptions::default()), true);
                if (!\is_array($defaults)) {
                    $defaults = [];
                }
                $merged = \array_merge($defaults, $options);
                $opts = ConversionOptions::from_json(\json_encode($merged));
                $result = convert($html, $opts);
            }

            return $result->getContent() ?? '';
        }
    }

} // end namespace
