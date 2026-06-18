```php
use HtmlToMarkdown\HtmlToMarkdownApi;
use HtmlToMarkdown\ConversionOptions;

// Visitors are duck-typed: define any subset of visit_* methods.
// Each method returns either 'skip', ['custom' => '...'], or null/'continue'.
$visitor = new class {
    public function visit_link($ctx, $href, $text, $title) {
        return ['custom' => "[{$text}]({$href})"];
    }

    public function visit_image($ctx, $src, $alt, $title) {
        return 'skip';
    }
};

$options = ConversionOptions::builder()->visitor($visitor)->build();

$result = HtmlToMarkdownApi::convert(
    '<a href="/page">Link</a><img src="pic.png" alt="pic">',
    $options
);
echo $result->content;
```
