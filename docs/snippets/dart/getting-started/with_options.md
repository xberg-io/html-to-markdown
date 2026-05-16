```dart
import 'package:h2m/h2m.dart';
import 'package:h2m/src/html_to_markdown_rs_bridge_generated/frb_generated.dart'
    show RustLib;

Future<void> main() async {
  await RustLib.init();

  final options = await createConversionOptionsFromJson(
    json: '{"heading_style":"atx","list_indent_width":2,"wrap":true}',
  );

  const html = '<h1>Hello</h1><p>This is <strong>formatted</strong> content.</p>';
  final result = await H2mBridge.convert(html, options: options);
  print(result.content);
}
```
