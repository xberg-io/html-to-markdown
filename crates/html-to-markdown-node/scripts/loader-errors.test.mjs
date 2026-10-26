import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import test from "node:test";
import { runInNewContext } from "node:vm";

const source = readFileSync(new URL("../index.js", import.meta.url), "utf8");
const require = createRequire(new URL("../index.js", import.meta.url));

test("committed loader preserves structured errors through named exports", () => {
  const native = {
    convert() {
      throw new Error("invalid configuration", {
        cause: new Error('alef:error-info:{"kind":"ConfigError"}'),
      });
    },
  };
  const module = { exports: {} };
  runInNewContext(source, {
    module,
    process: { platform: "darwin", arch: "arm64" },
    require: (name) => (name.endsWith(".node") ? native : require(name)),
  });
  assert.throws(() => module.exports.convert(), { message: "invalid configuration", kind: "ConfigError" });
});
