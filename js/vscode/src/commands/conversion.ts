import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import { Methods } from "@taplo/lsp";
import { getOutput } from "../extension";

export function register(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  c.onReady().then(() => {
    ctx.subscriptions.push(
      vscode.commands.registerTextEditorCommand(
        "evenBetterToml.copyTomlAsJson",
        async editor => {
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

          let params: Methods.TomlToJson.Params = {
            text: selectedText,
          };

          const res = await c.sendRequest<Methods.TomlToJson.Response>(
            Methods.TomlToJson.METHOD,
            params
          );

          const out = getOutput();

          if (res.errors?.length ?? 0 !== 0) {
            let errLines = `Selection Parse Errors (${editor.document.fileName}):`;
            for (const err of res.errors!) {
              errLines += `\n\t${err}`;
            }
            out.appendLine(errLines);

            const show = await vscode.window.showErrorMessage(
              "Copying has failed!",
              "Show Details"
            );

            if (show) {
              out.show();
            }
            return;
          }

          try {
            if (!res.text) {
              out.appendLine(`The response shouldn't be empty, but it is.`);
              const show = await vscode.window.showErrorMessage(
                "Copying has failed!",
                "Show Details"
              );

              if (show) {
                out.show();
              }
              return;
            }
            await vscode.env.clipboard.writeText(res.text);
          } catch (e) {
            out.appendLine(`Couldn't write to clipboard: ${e}`);
            const show = await vscode.window.showErrorMessage(
              "Copying has failed!",
              "Show Details"
            );

            if (show) {
              out.show();
            }
            return;
          }

          await vscode.window.showInformationMessage("JSON has been copied!");
        }
      ),
      vscode.commands.registerTextEditorCommand(
        "evenBetterToml.copyJsonAsToml",
        async editor => {
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

          let params: Methods.JsonToToml.Params = {
            text: selectedText,
          };

          const res = await c.sendRequest<Methods.JsonToToml.Response>(
            Methods.JsonToToml.METHOD,
            params
          );

          const out = getOutput();

          if (res.error?.length ?? 0 !== 0) {
            out.appendLine(`Failed to convert JSON to TOML: ${res.error}`);

            const show = await vscode.window.showErrorMessage(
              "Copying has failed!",
              "Show Details"
            );

            if (show) {
              out.show();
            }
            return;
          }

          try {
            if (!res.text) {
              out.appendLine(`The response shouldn't be empty, but it is.`);
              const show = await vscode.window.showErrorMessage(
                "Copying has failed!",
                "Show Details"
              );

              if (show) {
                out.show();
              }
              return;
            }
            await vscode.env.clipboard.writeText(res.text);
          } catch (e) {
            out.appendLine(`Couldn't write to clipboard: ${e}`);
            const show = await vscode.window.showErrorMessage(
              "Copying has failed!",
              "Show Details"
            );

            if (show) {
              out.show();
            }
            return;
          }

          await vscode.window.showInformationMessage("TOML has been copied!");
        }
      ),
      vscode.commands.registerTextEditorCommand(
        "evenBetterToml.pasteTomlAsJson",
        async editor => {
          const out = getOutput();
          let input;
          try {
            input = await vscode.env.clipboard.readText();
          } catch (e) {
            out.appendLine(`Failed to read from clipboard:${e}`);
            const show = await vscode.window.showErrorMessage(
              "Paste from clipboard has failed!",
              "Show Details"
            );

            if (show) {
              out.show();
            }
            return;
          }

          let params: Methods.TomlToJson.Params = {
            text: input,
          };

          const res = await c.sendRequest<Methods.TomlToJson.Response>(
            Methods.TomlToJson.METHOD,
            params
          );

          if (res.errors?.length ?? 0 !== 0) {
            let errLines = `Clipboard Parse Errors:`;
            for (const err of res.errors!) {
              errLines += `\n\t${err}`;
            }
            out.appendLine(errLines);

            const show = await vscode.window.showErrorMessage(
              "Paste from clipboard has failed!",
              "Show Details"
            );

            if (show) {
              out.show();
            }
            return;
          }

          editor.edit(e => {
            e.replace(editor.selection, res.text!);
          });
        }
      ),
      vscode.commands.registerTextEditorCommand(
        "evenBetterToml.pasteJsonAsToml",
        async editor => {
          const out = getOutput();
          let input;
          try {
            input = await vscode.env.clipboard.readText();
          } catch (e) {
            out.appendLine(`Failed to read from clipboard:${e}`);
            const show = await vscode.window.showErrorMessage(
              "Paste from clipboard has failed!",
              "Show Details"
            );

            if (show) {
              out.show();
            }
            return;
          }

          let params: Methods.JsonToToml.Params = {
            text: input,
          };

          const res = await c.sendRequest<Methods.JsonToToml.Response>(
            Methods.JsonToToml.METHOD,
            params
          );

          if (res.error?.length ?? 0 !== 0) {
            out.appendLine(`Failed to convert JSON to TOML: ${res.error}`);

            const show = await vscode.window.showErrorMessage(
              "Paste from clipboard has failed!",
              "Show Details"
            );

            if (show) {
              out.show();
            }
            return;
          }

          editor.edit(e => {
            e.replace(editor.selection, res.text!);
          });
        }
      )
    );
  });
}
