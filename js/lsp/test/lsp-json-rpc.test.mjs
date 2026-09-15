import assert from "node:assert/strict";

const module = await import("../dist/index.js");
const { TaploLsp } = module.default ?? module["module.exports"];

function nextMessage(messages, waiters, label, predicate) {
  return new Promise((resolve, reject) => {
    const existing = messages.find(predicate);
    if (existing) {
      resolve(existing);
      return;
    }

    const waiter = message => {
      if (predicate(message)) {
        clearTimeout(timeout);
        resolve(message);
        return true;
      }

      return false;
    };
    const timeout = setTimeout(() => {
      waiters.delete(waiter);
      reject(new Error(`timed out waiting for ${label}`));
    }, 5_000);

    waiters.add(waiter);
  });
}

async function waitUntil(label, predicate) {
  const deadline = Date.now() + 5_000;

  while (!predicate()) {
    if (Date.now() >= deadline) {
      throw new Error(`timed out waiting for ${label}`);
    }
    await new Promise(resolve => setTimeout(resolve, 10));
  }
}

const messages = [];
const waiters = new Set();
const stderr = [];

const warnings = [];
const originalWarn = console.warn;
console.warn = (...args) => warnings.push(args);

let lsp;
try {
  lsp = await TaploLsp.initialize(
    {
      cwd: () => process.cwd(),
      envVar: name => (name === "RUST_LOG" ? "warn" : process.env[name]),
      envVars: () => Object.entries({ ...process.env, RUST_LOG: "warn" }),
      findConfigFile: () => undefined,
      glob: () => [],
      isAbsolute: path => path.startsWith("/"),
      now: () => new Date(),
      readFile: async () => new Uint8Array(),
      writeFile: async () => undefined,
      stderr: async bytes => {
        stderr.push(Buffer.from(bytes).toString());
        return bytes.length;
      },
      stdErrAtty: () => false,
      stdin: async () => new Uint8Array(),
      stdout: async bytes => bytes.length,
      urlToFilePath: url => decodeURIComponent(url).slice("file://".length),
    },
    {
      onMessage(message) {
        messages.push(message);
        for (const waiter of waiters) {
          if (waiter(message)) {
            waiters.delete(waiter);
          }
        }
      },
    },
  );
} finally {
  console.warn = originalWarn;
}

assert.deepEqual(warnings, []);

lsp.send({
  jsonrpc: "2.0",
  id: 1,
  method: "initialize",
  params: {
    processId: process.pid,
    rootUri: null,
    capabilities: { workspace: { configuration: true } },
    initializationOptions: { configurationSection: "evenBetterToml" },
    workspaceFolders: [],
  },
});

const initialize = await nextMessage(
  messages,
  waiters,
  "the initialize response",
  message => message.id === 1
);
assert.equal(initialize.result.serverInfo.name, "Taplo");
assert.equal(initialize.result.capabilities.textDocumentSync, 1);

lsp.send({ jsonrpc: "2.0", method: "initialized", params: {} });

const configuration = await nextMessage(
  messages,
  waiters,
  "the workspace/configuration request",
  message => message.method === "workspace/configuration"
);
assert.deepEqual(configuration.params.items, [
  { section: "evenBetterToml" },
]);

lsp.send({
  jsonrpc: "2.0",
  method: "taplo/test-unsupported-notification",
  params: {},
});
await waitUntil(
  "the unsupported-notification warning",
  () => /no notification handler/.test(stderr.join(""))
);
stderr.length = 0;

lsp.send({
  jsonrpc: "2.0",
  method: "$/setTrace",
  params: { value: "off" },
});
await new Promise(resolve => setTimeout(resolve, 10));
assert.doesNotMatch(stderr.join(""), /no notification handler/);

lsp.dispose();
