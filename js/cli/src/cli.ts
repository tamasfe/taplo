// @ts-ignore
import taploCli from "../../../taplo-cli/Cargo.toml";
import fs from "fs";
import fastGlob from "fast-glob";
import fetch, { Headers, Request, Response } from "node-fetch";
import path from "path";

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

(global as any).fileExists = (p: string): boolean => {
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

(global as any).mkdir = (p: string) => {
  fs.mkdirSync(p, { recursive: true });
};

// For cached schemas.
(global as any).needsUpdate = (path: string, newDate: number): boolean =>
  fs.statSync(path).mtimeMs < newDate;

async function runTaplo() {
  const taplo = await taploCli();
  taplo.run_node(process.argv.slice(2));
}

runTaplo().catch(e => {
  console.error(e);
  process.exit(1);
});
