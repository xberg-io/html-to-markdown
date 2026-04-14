import { readFile } from "node:fs/promises";
import type { JsConversionOptions, JsConversionResult } from "@kreuzberg/html-to-markdown-node";
import { convert } from "@kreuzberg/html-to-markdown-node";

/**
 * Convert an HTML file to Markdown.
 */
export async function convertFile(path: string, options?: Partial<JsConversionOptions>): Promise<JsConversionResult> {
	const html = await readFile(path, "utf-8");
	return convert(html, options);
}

/**
 * Convert a readable stream of HTML to Markdown.
 */
export async function convertStream(
	stream: NodeJS.ReadableStream,
	options?: Partial<JsConversionOptions>,
): Promise<JsConversionResult> {
	const chunks: Buffer[] = [];
	for await (const chunk of stream) {
		chunks.push(Buffer.from(chunk));
	}
	const html = Buffer.concat(chunks).toString("utf-8");
	return convert(html, options);
}
