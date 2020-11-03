// Wrapper over the WASM module.
//
// Proxies all messages between the IPC
// channel and the WASM module.
//
// And provides some utilities.

import * as taplo from "taplo";
import * as fs from "fs";
import * as path from "path";

// For reqwest
const fetch = require("node-fetch");
(global as any).Headers = fetch.Headers;
(global as any).Request = fetch.Request;
(global as any).Response = fetch.Response;
(global as any).Window = Object;
(global as any).fetch = fetch;

console.log(global);

(global as any).sendMessage = (msg: any) => {
  if (process.send) {
    process.send(msg);
  }
};

(global as any).readFile = (path: string): Uint8Array => {
  return fs.readFileSync(path);
};

(global as any).isAbsolutePath = (p: string): boolean => {
  return (
    path.resolve(p) === path.normalize(p).replace(RegExp(path.sep + "$"), "")
  );
};

let initialized = false;

process.on("message", async d => {
  console.log(global);
  if (!initialized) {
    await taplo.init();
    initialized = true;
  }
  taplo.message(d);
});

// These are panics from rust
process.on("unhandledRejection", up => {
  throw up;
});
