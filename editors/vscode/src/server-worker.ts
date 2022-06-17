import {
  BrowserMessageReader,
  BrowserMessageWriter,
} from "vscode-languageserver-protocol/browser";

import { TaploLsp, RpcMessage } from "@taplo/lsp";

const worker: Worker = self as any;

const writer = new BrowserMessageWriter(worker);
const reader = new BrowserMessageReader(worker);

let taplo: TaploLsp;

reader.listen(async message => {
  if (!taplo) {
    taplo = await TaploLsp.initialize(
      {
        cwd: () => "/",
        envVar: () => "",
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
          writer.write(message);
        },
      }
    );
  }

  taplo.send(message as RpcMessage);
});
