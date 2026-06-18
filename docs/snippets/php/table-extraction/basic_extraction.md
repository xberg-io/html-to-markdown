```php
use HtmlToMarkdown\HtmlToMarkdownApi;

$html = <<<HTML
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
HTML;

$result = HtmlToMarkdownApi::convert($html);

foreach ($result->tables as $table) {
    foreach ($table->grid->cells as $cell) {
        $kind = $cell->isHeader ? 'Header' : 'Cell';
        echo "  {$kind} (r{$cell->row},c{$cell->col}): {$cell->content}\n";
    }
}
```
