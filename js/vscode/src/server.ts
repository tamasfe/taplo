// Wrapper over the WASM module.
//
// Proxies all messages between the IPC
// channel and the WASM module.
//
// And provides some utilities.

// @ts-ignore
import loadTaplo from "../../../taplo-lsp/Cargo.toml";
import * as fs from "fs";
import * as path from "path";
import fetch, { Headers, Request, Response } from "node-fetch";
import { exit } from "process";

// For reqwest
(global as any).Headers = Headers;
(global as any).Request = Request;
(global as any).Response = Response;
(global as any).Window = Object;
(global as any).fetch = fetch;

// Needed for taplo-cli's glob matching
(global as any).isWindows = () => {
  return process.platform == "win32";
};

(global as any).sendMessage = (msg: any) => {
  if (process.send) {
    process.send(msg);
  }
};

(global as any).readFile = (path: string): Promise<Uint8Array> => {
  return fs.promises.readFile(path);
};

(global as any).writeFile = (path: string, data: Uint8Array): Promise<void> => {
  return fs.promises.writeFile(path, data);
};

(global as any).isAbsolutePath = (p: string): boolean => {
  return (
    path.resolve(p) === path.normalize(p).replace(RegExp(path.sep + "$"), "")
  );
};

(global as any).fileExists = (p: string): boolean => {
  return fs.existsSync(p);
};

(global as any).mkdir = (p: string) => {
  fs.mkdirSync(p, { recursive: true });
};

// For cached schemas.
(global as any).needsUpdate = (path: string, newDate: number): boolean =>
  fs.statSync(path).mtimeMs < newDate;

let taplo: any;

process.on("message", async d => {
  if (d.method === "exit") {
    exit(0);
  }

  if (typeof taplo === "undefined") {
    taplo = await loadTaplo();
    await taplo.initialize();
  }

  taplo.message(d);
});

// These are panics from rust
process.on("unhandledRejection", up => {
  throw up;
});
