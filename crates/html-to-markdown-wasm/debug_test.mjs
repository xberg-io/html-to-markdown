import { convert } from "./dist/html_to_markdown_wasm.js";

const html =
  '<html><head><title>Fallback Title</title><meta property="og:title" content="OG Title"></head><body><p>Content</p></body></html>';
const result = convert(html, undefined);
const doc = result.metadata?.document;
console.log("doc.openGraph:", doc?.openGraph);
console.log("typeof doc.openGraph:", typeof doc?.openGraph);
console.log('doc.openGraph["title"]:', doc?.openGraph?.["title"]);
console.log("Object.keys:", doc?.openGraph ? Object.keys(doc.openGraph) : "null");
