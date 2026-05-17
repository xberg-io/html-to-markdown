```dart
import 'package:h2m/h2m.dart';
import 'package:h2m/src/html_to_markdown_rs_bridge_generated/frb_generated.dart'
    show RustLib;

Future<void> main() async {
  await RustLib.init();

  const html = '''
<table>
  <tr><th>Name</th><th>Age</th></tr>
  <tr><td>Alice</td><td>30</td></tr>
  <tr><td>Bob</td><td>25</td></tr>
</table>
''';

  final result = await H2mBridge.convert(html);

  for (final table in result.tables) {
    for (final cell in table.grid.cells) {
      final kind = cell.isHeader ? 'Header' : 'Cell';
      print('  $kind (r${cell.row},c${cell.col}): ${cell.content}');
    }
  }
}
```
