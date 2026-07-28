import { RpcMessage } from "@taplo/lsp";

function normalizeValue(value: unknown): unknown {
  // serde_wasm_bindgen may serialize null/unit as undefined;
  // convert to null so values survive JSON serialization in Node.js IPC.
  if (value === undefined) {
    return null;
  }

  if (value instanceof Map) {
    return Object.fromEntries(
      Array.from(value.entries(), ([key, entryValue]) => [
        key,
        normalizeValue(entryValue),
      ])
    );
  }

  if (Array.isArray(value)) {
    return value.map(normalizeValue);
  }

  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, entryValue]) => [
        key,
        normalizeValue(entryValue),
      ])
    );
  }

  return value;
}

export function normalizeRpcMessage(message: RpcMessage): RpcMessage {
  return normalizeValue(message) as RpcMessage;
}
