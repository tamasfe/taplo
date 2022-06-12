import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import { registerCommands } from "./commands";
import { createClient } from "./client";
import { syncExtensionSchemas } from "./tomlValidation";
import { getOutput } from "./util";

export async function activate(context: vscode.ExtensionContext) {
  const schemaIndicator = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    0
  );

  schemaIndicator.text = "no schema selected";
  schemaIndicator.tooltip = "TOML Schema";
  schemaIndicator.command = "evenBetterToml.selectSchema";

  const c = await createClient(context);
  await c.start();

  if (vscode.window.activeTextEditor?.document.languageId === "toml") {
    schemaIndicator.show();
  }

  registerCommands(context, c);
  syncExtensionSchemas(context, c);

  context.subscriptions.push(
    getOutput(),
    schemaIndicator,
    c.onNotification("taplo/messageWithOutput", async params =>
      showMessage(params, c)
    ),
    vscode.window.onDidChangeActiveTextEditor(editor => {
      if (editor.document.languageId === "toml") {
        schemaIndicator.show();
      } else {
        schemaIndicator.hide();
      }
    }),
    c.onNotification(
      "taplo/didChangeSchemaAssociation",
      async (params: {
        documentUri: string;
        schemaUri?: string;
        meta?: Record<string, any>;
      }) => {
        const currentDocumentUrl =
          vscode.window.activeTextEditor?.document.uri.toString();

        if (!currentDocumentUrl) {
          return;
        }

        if (params.documentUri === currentDocumentUrl) {
          schemaIndicator.text =
            params.meta?.name ?? params.schemaUri ?? "no schema selected";
        }
      }
    )
  );
}

export async function showMessage(
  params: { kind: "info" | "warn" | "error"; message: string },
  c: client.LanguageClient
) {
  let show: string | undefined;
  switch (params.kind) {
    case "info":
      show = await vscode.window.showInformationMessage(
        params.message,
        "Show Details"
      );
    case "warn":
      show = await vscode.window.showWarningMessage(
        params.message,
        "Show Details"
      );
    case "error":
      show = await vscode.window.showErrorMessage(
        params.message,
        "Show Details"
      );
  }

  if (show) {
    c.outputChannel.show();
  }
}
