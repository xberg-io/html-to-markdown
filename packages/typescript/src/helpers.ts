import { readFile } from "node:fs/promises";
import type { ConversionOptions, ConversionResult } from "@kreuzberg/html-to-markdown-node";
import { convert } from "@kreuzberg/html-to-markdown-node";

/**
 * Convert an HTML file to Markdown.
 */
export async function convertFile(
  path: string,
  options?: Partial<ConversionOptions>,
): Promise<ConversionResult> {
  const html = await readFile(path, "utf-8");
  return convert(html, options as ConversionOptions | undefined);
}

/**
 * Convert a readable stream of HTML to Markdown.
 */
export async function convertStream(
  stream: NodeJS.ReadableStream,
  options?: Partial<ConversionOptions>,
): Promise<ConversionResult> {
  const chunks: Buffer[] = [];
  for await (const chunk of stream) {
    chunks.push(Buffer.from(chunk));
  }
  const html = Buffer.concat(chunks).toString("utf-8");
  return convert(html, options as ConversionOptions | undefined);
}
