```dart
import 'package:h2m/h2m.dart';
import 'package:h2m/src/html_to_markdown_rs_bridge_generated/frb_generated.dart'
    show RustLib;

Future<void> main() async {
  await RustLib.init();

  final options = await createConversionOptionsFromJson(
    json: '{"extract_metadata":true}',
  );
  final result = await H2mBridge.convert(
    '<html><head><title>Example</title>'
    '<meta name="description" content="A sample page">'
    '</head><body><h1>Hello</h1><a href="https://example.com">link</a></body></html>',
    options: options,
  );

  print(result.content);
  print(result.metadata.document.title); // "Example"
  print(result.metadata.document.description); // "A sample page"
  print(result.metadata.headers); // List<HeaderMetadata>
  print(result.metadata.links); // List<LinkMetadata>
  print(result.metadata.images); // List<ImageMetadata>
}
```
