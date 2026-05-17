```dart
import 'package:h2m/h2m.dart';
import 'package:h2m/src/html_to_markdown_rs_bridge_generated/frb_generated.dart'
    show RustLib;

Future<void> main() async {
  await RustLib.init();

  // flutter_rust_bridge requires every visit callback — default to continue_()
  // and override only the hooks you care about (here: links and headings).
  final visitor = await createHtmlVisitor(
    visitText: (ctx, text) async => VisitResult.continue_(),
    visitElementStart: (ctx) async => VisitResult.continue_(),
    visitElementEnd: (ctx, output) async => VisitResult.continue_(),
    visitLink: (ctx, href, text, title) async =>
        VisitResult.custom(field0: '[$text]($href)'),
    visitImage: (ctx, src, alt, title) async => VisitResult.continue_(),
    visitHeading: (ctx, level, text, id) async =>
        VisitResult.custom(field0: '${'#' * level.toInt()} $text\n'),
    visitCodeBlock: (ctx, lang, code) async => VisitResult.continue_(),
    visitCodeInline: (ctx, code) async => VisitResult.continue_(),
    visitListItem: (ctx, ordered, marker, text) async => VisitResult.continue_(),
    visitListStart: (ctx, ordered) async => VisitResult.continue_(),
    visitListEnd: (ctx, ordered, output) async => VisitResult.continue_(),
    visitTableStart: (ctx) async => VisitResult.continue_(),
    visitTableRow: (ctx, cells, isHeader) async => VisitResult.continue_(),
    visitTableEnd: (ctx, output) async => VisitResult.continue_(),
    visitBlockquote: (ctx, content, depth) async => VisitResult.continue_(),
    visitStrong: (ctx, text) async => VisitResult.continue_(),
    visitEmphasis: (ctx, text) async => VisitResult.continue_(),
    visitStrikethrough: (ctx, text) async => VisitResult.continue_(),
    visitUnderline: (ctx, text) async => VisitResult.continue_(),
    visitSubscript: (ctx, text) async => VisitResult.continue_(),
    visitSuperscript: (ctx, text) async => VisitResult.continue_(),
    visitMark: (ctx, text) async => VisitResult.continue_(),
    visitLineBreak: (ctx) async => VisitResult.continue_(),
    visitHorizontalRule: (ctx) async => VisitResult.continue_(),
    visitCustomElement: (ctx, tagName, html) async => VisitResult.continue_(),
    visitDefinitionListStart: (ctx) async => VisitResult.continue_(),
    visitDefinitionTerm: (ctx, text) async => VisitResult.continue_(),
    visitDefinitionDescription: (ctx, text) async => VisitResult.continue_(),
    visitDefinitionListEnd: (ctx, output) async => VisitResult.continue_(),
    visitForm: (ctx, action, method) async => VisitResult.continue_(),
    visitInput: (ctx, inputType, name, value) async => VisitResult.continue_(),
    visitButton: (ctx, text) async => VisitResult.continue_(),
    visitAudio: (ctx, src) async => VisitResult.continue_(),
    visitVideo: (ctx, src) async => VisitResult.continue_(),
    visitIframe: (ctx, src) async => VisitResult.continue_(),
    visitDetails: (ctx, open) async => VisitResult.continue_(),
    visitSummary: (ctx, text) async => VisitResult.continue_(),
    visitFigureStart: (ctx) async => VisitResult.continue_(),
    visitFigcaption: (ctx, text) async => VisitResult.continue_(),
    visitFigureEnd: (ctx, output) async => VisitResult.continue_(),
  );

  final options = await createConversionOptionsFromJsonWithVisitor(
    json: '{}',
    visitor: visitor,
  );
  final result = await H2mBridge.convert(
    '<h1>Title</h1><a href="https://example.com">Link</a>',
    options: options,
  );
  print(result.content);
}
```
