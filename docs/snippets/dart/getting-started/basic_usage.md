```dart
import 'package:h2m/h2m.dart';
import 'package:h2m/src/html_to_markdown_rs_bridge_generated/frb_generated.dart'
    show RustLib;

Future<void> main() async {
  await RustLib.init();

  const html = '<h1>Hello</h1><p>This is <strong>fast</strong>!</p>';
  final result = await H2mBridge.convert(html);
  print(result.content);
}
```
