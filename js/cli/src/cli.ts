import fsPromise from "fs/promises";
import fs from "fs";
import process from "process";
import path from "path";
import glob from "fast-glob";
import fetch, { Headers, Request, Response } from "node-fetch";
import loadTaplo from "../../../crates/taplo-wasm/Cargo.toml";
import { convertEnv, Environment, prepareEnv } from "@taplo/core";

(async function main() {
  const taplo = await loadTaplo();
  taplo.initialize();

  const env: Environment = {
    cwd: () => process.cwd(),
    envVar: name => process.env[name],
    envVars: () => Object.entries(process.env),
    findConfigFile: from => {
      try {
        fs.accessSync(path.join(from, ".taplo.toml"));
        return path.join(from, ".taplo.toml");
      } catch {}

      try {
        fs.accessSync(path.join(from, "taplo.toml"));
        return path.join(from, "taplo.toml");
      } catch {}
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
  };
  prepareEnv(env);

  try {
    await taplo.run_cli(convertEnv(env), process.argv.slice(1));
  } catch (err) {
    process.exitCode = 1;
  }
})();
