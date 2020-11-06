// @ts-ignore
import taploCli from "../../../taplo-cli/Cargo.toml";
import fs from "fs";
import fastGlob from "fast-glob";
import fetch, { Headers, Request, Response } from "node-fetch";

// In order to support reqwest
(global as any).Headers = Headers;
(global as any).Request = Request;
(global as any).Response = Response;
(global as any).Window = Object;
(global as any).fetch = fetch;

(global as any).isWindows = () => {
  return process.platform == "win32";
};

(global as any).readStdin = () => {
  return fs.readFileSync(0, "utf-8");
};

(global as any).readFile = (p: string) => {
  return fs.readFileSync(p, "utf-8");
};

(global as any).writeFile = (p: string, data: Uint8Array) => {
  return fs.writeFileSync(p, data);
};

(global as any).fileExists = (p: string) => {
  return fs.existsSync(p);
};

(global as any).globPaths = (p: string): string[] => {
  return fastGlob.sync(p, {
    dot: true,
    caseSensitiveMatch: process.platform !== "win32",
  });
};

(global as any).isATty = (): boolean => {
  return process.stdout.isTTY;
};

async function runTaplo() {
  const taplo = await taploCli();

  taplo.run_node(process.argv.slice(2));
}

runTaplo().catch(e => {
  console.error(e);
  process.exit(1);
});
