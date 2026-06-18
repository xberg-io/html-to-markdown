```php
use HtmlToMarkdown\HtmlToMarkdownApi;

$html = '<html><head><title>Example</title></head><body><h1>Welcome</h1><a href="https://example.com">Link</a></body></html>';

// extract_metadata defaults to true.
$result = HtmlToMarkdownApi::convert($html);

echo $result->content;
echo $result->metadata->document->title;
foreach ($result->metadata->links as $link) {
    echo $link->href . ': ' . $link->text;
}
```
