```python
from html_to_markdown import convert

html = """
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"""

result = convert(html)

for table in result.tables:
    for cell in table.grid.cells:
        prefix = "Header" if cell.is_header else "Cell"
        print(f"  {prefix}: {cell.content}")
```
