```php
use HtmlToMarkdown\HtmlToMarkdownApi;
use HtmlToMarkdown\ConversionOptions;

$options = ConversionOptions::builder()
    ->headingStyle('atx')
    ->listIndentWidth(2)
    ->build();

$result = HtmlToMarkdownApi::convert('<h1>Hello</h1>', $options);
echo $result->content;
```
