```dart
import 'package:h2m/h2m.dart';
import 'package:h2m/src/html_to_markdown_rs_bridge_generated/frb_generated.dart'
    show RustLib;

Future<void> main() async {
  await RustLib.init();

  try {
    final result = await H2mBridge.convert('<h1>Hello</h1>');
    print(result.content);
  } catch (error) {
    // ConversionError variants surface via flutter_rust_bridge as thrown errors.
    print('Conversion failed: $error');
  }
}
```
