const assert = require("node:assert/strict");
const test = require("node:test");

import { normalizeRpcMessage } from "../src/lspMessage";

test("normalizeRpcMessage converts nested Map payloads into plain objects", () => {
  const rawMessage = {
    jsonrpc: "2.0",
    id: 7,
    result: new Map<string, any>([
      [
        "contents",
        new Map<string, any>([
          ["kind", "markdown"],
          ["value", "Human-readable phase identifier."],
        ]),
      ],
      [
        "range",
        new Map<string, any>([
          ["start", new Map<string, any>([["line", 8], ["character", 0]])],
          ["end", new Map<string, any>([["line", 8], ["character", 4]])],
        ]),
      ],
    ] as any),
  } as any;

  assert.deepEqual(normalizeRpcMessage(rawMessage), {
    jsonrpc: "2.0",
    id: 7,
    result: {
      contents: {
        kind: "markdown",
        value: "Human-readable phase identifier.",
      },
      range: {
        start: {
          line: 8,
          character: 0,
        },
        end: {
          line: 8,
          character: 4,
        },
      },
    },
  });
});

test("normalizeRpcMessage preserves arrays while normalizing Map entries inside them", () => {
  const rawMessage = {
    jsonrpc: "2.0",
    id: 11,
    result: new Map<string, any>([
      [
        "schemas",
        [
          new Map<string, any>([
            ["url", "file:///schema.json"],
            ["meta", new Map<string, any>([["source", "config"]])],
          ]),
        ],
      ],
    ] as any),
  } as any;

  assert.deepEqual(normalizeRpcMessage(rawMessage), {
    jsonrpc: "2.0",
    id: 11,
    result: {
      schemas: [
        {
          url: "file:///schema.json",
          meta: {
            source: "config",
          },
        },
      ],
    },
  });
});

test("normalizeRpcMessage converts undefined result to null (serde_wasm_bindgen compat)", () => {
  // serde_wasm_bindgen serializes serde_json::Value::Null as JS undefined.
  // Without conversion, process.send() JSON serialization drops undefined
  // properties, causing "neither result nor error" LSP client errors.
  const rawMessage = {
    jsonrpc: "2.0",
    id: 42,
    result: undefined,
  } as any;

  const normalized = normalizeRpcMessage(rawMessage);
  assert.deepEqual(normalized, {
    jsonrpc: "2.0",
    id: 42,
    result: null,
  });
});

test("normalizeRpcMessage converts nested undefined values to null", () => {
  const rawMessage = new Map<string, any>([
    ["jsonrpc", "2.0"],
    ["id", 5],
    ["result", new Map<string, any>([
      ["data", undefined],
      ["items", [undefined, "valid", undefined]],
    ])],
  ]) as any;

  assert.deepEqual(normalizeRpcMessage(rawMessage), {
    jsonrpc: "2.0",
    id: 5,
    result: {
      data: null,
      items: [null, "valid", null],
    },
  });
});
