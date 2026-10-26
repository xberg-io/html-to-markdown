// Build-time regression guard for issue #450.
//
// Loads the freshly built package through a real Node ESM *named* import — the
// exact shape that failed when napi emitted only `module.exports = nativeBinding`.
// If the CJS named re-exports are ever missing again, this throws and fails the
// build instead of silently shipping a package whose `import { convert }` is broken.
//
// Host-only: it must actually load the native `.node`, so it belongs in the
// local/e2e `build` (which builds for the host via `--platform`), never in the
// cross-compiling publish pipeline.

import assert from "node:assert/strict";
import { convert, OutputFormat } from "../index.js";
import { convert as esmConvert } from "../index.mjs";

if (typeof convert !== "function") {
  throw new Error(`#450 regression: named ESM export 'convert' is ${typeof convert}, expected function`);
}
if (typeof OutputFormat !== "object" || OutputFormat === null) {
  throw new Error(`#450 regression: named ESM export 'OutputFormat' did not resolve to an enum object`);
}

const result = convert("<h1>Hi</h1>");
const markdown = result.markdown ?? result.content;
if (markdown !== "# Hi\n") {
  throw new Error(`#450 regression: convert('<h1>Hi</h1>') returned ${JSON.stringify(markdown)}, expected "# Hi\\n"`);
}

for (const convertEntry of [convert, esmConvert]) {
  assert.throws(() => convertEntry("too large", { maxInputSize: 1 }), {
    kind: "InputTooLarge",
    errorCode: 2,
  });
}

console.log("verify-esm-exports: named imports and structured native errors work through both entries");
