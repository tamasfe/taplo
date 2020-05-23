import * as vscode from "vscode";
import * as client from "vscode-languageclient";
import * as path from "path";

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
  // res.registerFeature(new SemanticTokensFeature(res))
  let c = new client.LanguageClient(
    "evenBetterToml",
    "Even Better Toml",
    serverOpts,
    clientOpts
  );

  c.registerProposedFeatures();

  context.subscriptions.push(c.start());
}
