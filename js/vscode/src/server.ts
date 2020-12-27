// Wrapper over the WASM module.
//
// Proxies all messages between the IPC
// channel and the WASM module.
//
// And provides some utilities.

// @ts-ignore
import * as fs from "fs";
import * as path from "path";
import { exit } from "process";
import { TaploLsp } from "@taplo/lsp";
import fetch, { Headers, Request, Response } from "node-fetch";

// For reqwest
(global as any).Headers = Headers;
(global as any).Request = Request;
(global as any).Response = Response;
(global as any).Window = Object;
(global as any).fetch = fetch;

let taplo: TaploLsp;

process.on("message", async d => {
  if (d.method === "exit") {
    exit(0);
  }

  if (typeof taplo === "undefined") {
    taplo = await TaploLsp.initialize({
      isWindows: () => {
        return process.platform == "win32";
      },
      sendMessage: (msg: any) => {
        if (process.send) {
          process.send(msg);
        }
      },
      readFile: (path: string): Promise<Uint8Array> => {
        return fs.promises.readFile(path);
      },
      writeFile: (path: string, data: Uint8Array): Promise<void> => {
        return fs.promises.writeFile(path, data);
      },
      isAbsolutePath: (p: string): boolean => {
        return (
          path.resolve(p) ===
          path.normalize(p).replace(RegExp(path.sep + "$"), "")
        );
      },
      fileExists: (p: string): boolean => {
        return fs.existsSync(p);
      },
      mkdir: (p: string) => {
        fs.mkdirSync(p, { recursive: true });
      },
      needsUpdate: (path: string, newDate: number): boolean =>
        fs.statSync(path).mtimeMs < newDate,
    });
  }

  taplo.message(d);
});

// These are panics from Rust.
process.on("unhandledRejection", up => {
  throw up;
});
