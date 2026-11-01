import { describe, expect, it } from "vitest";
import { convert, WasmConversionOptions } from "../pkg/nodejs/html_to_markdown_wasm.js";

const DEFAULT_LIMIT = 2 * 1024 * 1024;

function captureError(convertInput: () => unknown): unknown {
  try {
    convertInput();
  } catch (error: unknown) {
    return error;
  }
  throw new Error("expected conversion to reject oversized input");
}

describe("WASM input size limit", () => {
  it("rejects input above the default limit when options are omitted", () => {
    const html = "x".repeat(DEFAULT_LIMIT + 1);

    const error = captureError(() => convert(html));

    expect(error).toBeInstanceOf(Error);
    expect(error).toMatchObject({
      code: "input_too_large",
      message: `Input size ${DEFAULT_LIMIT + 1} bytes exceeds the configured maximum of ${DEFAULT_LIMIT} bytes`,
      observed_size: DEFAULT_LIMIT + 1,
      max_size: DEFAULT_LIMIT,
    });
  });

  it("retains the default limit in a default-constructed options object", () => {
    const html = "x".repeat(DEFAULT_LIMIT + 1);

    expect(captureError(() => convert(html, new WasmConversionOptions()))).toMatchObject({
      code: "input_too_large",
      observed_size: DEFAULT_LIMIT + 1,
      max_size: DEFAULT_LIMIT,
    });
  });

  it("honors an explicit limit override", () => {
    const options = new WasmConversionOptions();
    options.maxInputSize = 8n;

    expect(captureError(() => convert("123456789", options))).toMatchObject({
      code: "input_too_large",
      observed_size: 9,
      max_size: 8,
    });
  });
});
