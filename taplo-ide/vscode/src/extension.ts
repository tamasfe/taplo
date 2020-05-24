import * as vscode from "vscode";
import * as client from "vscode-languageclient";
import * as path from "path";
import { registerCommands } from "./commands";

let output: vscode.OutputChannel;

export function getOutput(): vscode.OutputChannel {
  return output;
}

export function activate(context: vscode.ExtensionContext) {
  let p = context.asAbsolutePath(path.join("out", "server.js"));

  let serverOpts: client.ServerOptions = {
    run: { module: p, transport: client.TransportKind.ipc },
    debug: { module: p, transport: client.TransportKind.ipc },
  };

  let clientOpts: client.LanguageClientOptions = {
    documentSelector: [
      { scheme: "file", language: "toml" },
      { scheme: "file", language: "cargoLock" },
    ],

    synchronize: {
      configurationSection: "evenBetterToml",
      fileEvents: [
        vscode.workspace.createFileSystemWatcher("**/.toml"),
        vscode.workspace.createFileSystemWatcher("**/Cargo.lock"),
      ],
    },
  };

  let c = new client.LanguageClient(
    "evenBetterToml",
    "Even Better Toml LSP",
    serverOpts,
    clientOpts
  );

  c.registerProposedFeatures();

  output = vscode.window.createOutputChannel("Even Better TOML");

  registerCommands(context, c);

  context.subscriptions.push(
    output,
    c.start()
  );
}
