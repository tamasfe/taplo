import * as vscode from "vscode";
import * as client from "vscode-languageclient";
import * as requestExt from "../requestExt";
import * as clipboardy from "clipboardy";
import { getOutput } from "../extension";

export function register(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  c.onReady().then(() => {
    ctx.subscriptions.push(
      vscode.commands.registerTextEditorCommand(
        "evenBetterToml.copyTomlAsJson",
        async (editor) => {
          const document = editor.document;
          // Avoid accidental copying of nothing
          if (editor.selection.isEmpty) {
            return;
          }

          const selectedText = document.getText(editor.selection);
          // Avoid accidental copying of nothing
          if (selectedText.trim().length === 0) {
            return;
          }

          let params: requestExt.TomlToJsonParams = {
            text: selectedText,
          };

          const res = await c.sendRequest<requestExt.TomlToJsonResponse>(
            requestExt.TOML_TO_JSON,
            params
          );

          if (res.errors?.length ?? 0 !== 0) {
            for (const err of res.errors!) {
              const out = getOutput();
              out.appendLine(
                `Selection Parse Error: (${editor.document.fileName}) ${err}`
              );
            }

            await vscode.window.showErrorMessage(
              "Copy failed! (Details under the output tab)"
            );

            return;
          }

          try {
            await clipboardy.write(res.text!);
          } catch (e) {
            getOutput().appendLine(e);
            await vscode.window.showErrorMessage(
              "Copy failed! (Details under the output tab)"
            );
            return;
          }

          await vscode.window.showInformationMessage("JSON has been copied!");
        }
      )
    );
  });
}
