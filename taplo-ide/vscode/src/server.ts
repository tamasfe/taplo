// Wrapper over the WASM module.
//
// Proxies all messages between the IPC
// channel and the WASM module.
//
// And provides some utilities.

import * as taplo from "taplo";

import * as fs from "fs";

(global as any).sendMessage = (msg: any) => {
  if (process.send) {
    process.send(msg);
  }
};

(global as any).readFile = (path: string): Uint8Array => {
  return fs.readFileSync(path);
};

taplo.init();

process.on("message", (d) => {
  taplo.message(d);
});

// These are panics from rust
process.on("unhandledRejection", (up) => {
  throw up;
});
