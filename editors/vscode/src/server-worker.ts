import {
  BrowserMessageReader,
  BrowserMessageWriter,
} from "vscode-languageserver-protocol/browser";

import { TaploLsp, RpcMessage } from "@taplo/lsp";
import { normalizeRpcMessage } from "./lspMessage";

const worker = globalThis as any;

const writer = new BrowserMessageWriter(worker);
const reader = new BrowserMessageReader(worker);

let taplo: TaploLsp;
let initPromise: Promise<TaploLsp> | undefined;

reader.listen(async message => {
  if (typeof taplo === "undefined") {
    if (!initPromise) {
      initPromise = TaploLsp.initialize(
        {
          cwd: () => "/",
          envVar: () => "",
          envVars: () => [],
          findConfigFile: () => undefined,
          glob: () => [],
          isAbsolute: () => true,
          now: () => new Date(),
          readFile: () => Promise.reject("not implemented"),
          writeFile: () => Promise.reject("not implemented"),
          stderr: async (bytes: Uint8Array) => {
            console.log(new TextDecoder().decode(bytes));
            return bytes.length;
          },
          stdErrAtty: () => false,
          stdin: () => Promise.reject("not implemented"),
          stdout: async (bytes: Uint8Array) => {
            console.log(new TextDecoder().decode(bytes));
            return bytes.length;
          },
          urlToFilePath: (url: string) => url.slice("file://".length),
        },
        {
          onMessage(message) {
            writer.write(normalizeRpcMessage(message));
          },
        }
      );
    }
    try {
      taplo = await initPromise;
    } catch (e) {
      initPromise = undefined;
      console.error(`taplo LSP failed to initialize: ${e}`);
      return;
    }
  }

  taplo.send(message as RpcMessage);
});
