import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import * as path from "path";
import { registerCommands } from "./commands";
import { Methods } from "@taplo/lsp";

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

    initializationOptions: {
      configuration: vscode.workspace.getConfiguration().get("evenBetterToml"),
    },

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

  checkAssociations();

  if (showNotification) {
    await vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Window,
        title: "TOML loading...",
      },
      async _ => {
        await c.onReady();
      }
    );
  } else {
    await c.onReady();
  }
  c.sendNotification(Methods.CachePath.METHOD, {
    path: context.globalStorageUri.fsPath,
  });
  c.onNotification(Methods.MessageWithOutput.METHOD, async params =>
    showMessage(params, c)
  );
}

async function checkAssociations() {
  const oldBuiltins = [
    "taplo://taplo@taplo.toml",
    "taplo://cargo@Cargo.toml",
    "taplo://python@pyproject.toml",
    "taplo://rust@rustfmt.toml",
  ];

  if (
    vscode.workspace
      .getConfiguration()
      .get("evenBetterToml.actions.ignoreDeprecatedAssociations") === true
  ) {
    return;
  }

  const assoc = vscode.workspace
    .getConfiguration()
    .get("evenBetterToml.schema.associations");

  if (!assoc) {
    return;
  }

  for (const k of Object.keys(assoc)) {
    const val = assoc[k];

    if (oldBuiltins.indexOf(val) !== -1) {
      const c = await vscode.window.showWarningMessage(
        "Your schema associations reference schemas that are not bundled anymore and will not work.",
        "More Information",
        "Ignore"
      );

      if (c === "More Information") {
        vscode.env.openExternal(
          vscode.Uri.parse(
            "https://taplo.tamasfe.dev/configuration/#official-schemas"
          )
        );
      } else if (c === "Ignore") {
        await vscode.workspace
          .getConfiguration()
          .update(
            "evenBetterToml.actions.ignoreDeprecatedAssociations",
            true,
            vscode.ConfigurationTarget.Global
          );
      }
      break;
    }
  }
}

async function showMessage(
  params: Methods.MessageWithOutput.Params,
  c: client.LanguageClient
) {
  let show: string | undefined;
  switch (params.kind) {
    case Methods.MessageWithOutput.MessageKind.Info:
      show = await vscode.window.showInformationMessage(
        params.message,
        "Show Details"
      );
    case Methods.MessageWithOutput.MessageKind.Warn:
      show = await vscode.window.showWarningMessage(
        params.message,
        "Show Details"
      );
    case Methods.MessageWithOutput.MessageKind.Error:
      show = await vscode.window.showErrorMessage(
        params.message,
        "Show Details"
      );
  }

  if (show) {
    c.outputChannel.show();
  }
}
