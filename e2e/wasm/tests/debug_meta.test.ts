import { expect, it } from "vitest";
import { convert } from "@kreuzberg/html-to-markdown-wasm";

it("debug_og", () => {
  const result = convert(
    '<html><head><meta property="og:title" content="OG Title"></head><body><p>Content</p></body></html>',
    undefined,
  );
  const og = result.metadata?.document?.openGraph;
  console.log("openGraph:", og);
  console.log("openGraph type:", typeof og);
  if (og && typeof og === "object") {
    console.log("keys:", Object.keys(og));
    console.log("title:", (og as any)["title"]);
  }
});
