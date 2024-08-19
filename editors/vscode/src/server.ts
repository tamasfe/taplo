import fs from "fs";
import fsPromise from "fs/promises";
import path from "path";
import { exit } from "process";
import { RpcMessage, TaploLsp } from "@taplo/lsp";
import fetch, { Headers, Request, Response } from "node-fetch";
import glob from "fast-glob";

let taplo: TaploLsp;

process.on("message", async (d: RpcMessage) => {
  if (d.method === "exit") {
    exit(0);
  }

  if (typeof taplo === "undefined") {
    taplo = await TaploLsp.initialize(
      {
        cwd: () => process.cwd(),
        envVar: name => process.env[name],
        envVars: () => Object.entries(process.env),
        findConfigFile: from => {
          const fileNames = [".taplo.toml", "taplo.toml"];

          for (const name of fileNames) {
            try {
              const fullPath = path.join(from, name);
              fs.accessSync(fullPath);
              return fullPath;
            } catch {}
          }
        },
        glob: p => glob.sync(p),
        isAbsolute: p => path.isAbsolute(p),
        now: () => new Date(),
        readFile: path => fsPromise.readFile(path),
        writeFile: (path, content) => fsPromise.writeFile(path, content),
        stderr: process.stderr,
        stdErrAtty: () => process.stderr.isTTY,
        stdin: process.stdin,
        stdout: process.stdout,
        urlToFilePath: (url: string) => {
          const c = decodeURIComponent(url).slice("file://".length);

          if (process.platform === "win32" && c.startsWith("/")) {
            return c.slice(1);
          }

          return c;
        },
        fetch: {
          fetch,
          Headers,
          Request,
          Response,
        },
      },
      {
        onMessage(message) {
          process.send(message);
        },
      }
    );
  }

  taplo.send(d);
});

// These are panics from Rust.
process.on("unhandledRejection", up => {
  throw up;
});
