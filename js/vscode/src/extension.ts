import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import { registerCommands } from "./commands";
import { createClient } from "./client";

let output: vscode.OutputChannel;

export function getOutput(): vscode.OutputChannel {
  if (!output) {
    output = vscode.window.createOutputChannel("Even Better TOML");
  }

  return output;
}

export async function activate(context: vscode.ExtensionContext) {
  const c = createClient(context);

  c.registerProposedFeatures();

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
  // c.sendNotification(Methods.CachePath.METHOD, {
  //   path: context.globalStorageUri.fsPath,
  // });
  // c.onNotification(Methods.MessageWithOutput.METHOD, async params =>
  //   showMessage(params, c)
  // );
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

// export async function showMessage(
//   params: Methods.MessageWithOutput.Params,
//   c: client.LanguageClient
// ) {
//   let show: string | undefined;
//   switch (params.kind) {
//     case Methods.MessageWithOutput.MessageKind.Info:
//       show = await vscode.window.showInformationMessage(
//         params.message,
//         "Show Details"
//       );
//     case Methods.MessageWithOutput.MessageKind.Warn:
//       show = await vscode.window.showWarningMessage(
//         params.message,
//         "Show Details"
//       );
//     case Methods.MessageWithOutput.MessageKind.Error:
//       show = await vscode.window.showErrorMessage(
//         params.message,
//         "Show Details"
//       );
//   }

//   if (show) {
//     c.outputChannel.show();
//   }
// }
