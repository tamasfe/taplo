import * as vscode from "vscode";
import * as client from "vscode-languageclient";
import * as path from "path";
import { registerCommands } from "./commands";
import { MessageWithOutput } from "./requestExt";

let output: vscode.OutputChannel;

export function getOutput(): vscode.OutputChannel {
  return output;
}

export async function activate(context: vscode.ExtensionContext) {
  let p = context.asAbsolutePath(path.join("dist", "server.js"));

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
    "Even Better TOML LSP",
    serverOpts,
    clientOpts
  );

  c.registerProposedFeatures();

  output = vscode.window.createOutputChannel("Even Better TOML");

  registerCommands(context, c);

  context.subscriptions.push(output, c.start());

  const showNotification = vscode.workspace
    .getConfiguration()
    .get("evenBetterToml.activationStatus");

  if (showNotification) {
    await vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Window,
        title: "TOML loading...",
      },
      async (_) => {
        await c.onReady();
      }
    );
  } else {
    await c.onReady();
  }

  c.onNotification(
    MessageWithOutput.METHOD,
    async (params: MessageWithOutput.Params) => {
      let show: string | undefined;
      switch (params.kind) {
        case MessageWithOutput.MessageKind.Info:
          show = await vscode.window.showInformationMessage(
            params.message,
            "Show Details"
          );
        case MessageWithOutput.MessageKind.Warn:
          show = await vscode.window.showWarningMessage(
            params.message,
            "Show Details"
          );
        case MessageWithOutput.MessageKind.Error:
          show = await vscode.window.showErrorMessage(
            params.message,
            "Show Details"
          );
      }

      if (show) {
        c.outputChannel.show();
      }
    }
  );
}
