import * as vscode from "vscode";
import { getOutput } from "../util";
import { BaseLanguageClient } from "vscode-languageclient";

// FIXME: this could be a bit more DRY.
export function register(ctx: vscode.ExtensionContext, c: BaseLanguageClient) {
  ctx.subscriptions.push(
    vscode.commands.registerTextEditorCommand(
      "evenBetterToml.copyAsJson",
      async editor => {
        const document = editor?.document;
        // Avoid accidental copying of nothing
        if (!document || editor.selection.isEmpty) {
          return;
        }

        const selectedText = document.getText(editor.selection);
        // Avoid accidental copying of nothing
        if (selectedText.trim().length === 0) {
          return;
        }

        const res = await c.sendRequest<{ text?: string; error?: string }>(
          "taplo/convertToJson",
          {
            text: selectedText,
          }
        );

        const out = getOutput();

        if (res.error?.length ?? 0 !== 0) {
          out.appendLine(`Failed to convert TOML to JSON: ${res.error}`);

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

        await vscode.window.showInformationMessage("JSON copied!");
      }
    ),
    vscode.commands.registerTextEditorCommand(
      "evenBetterToml.copyAsToml",
      async editor => {
        const document = editor?.document;
        // Avoid accidental copying of nothing
        if (!document || editor.selection.isEmpty) {
          return;
        }

        const selectedText = document.getText(editor.selection);
        // Avoid accidental copying of nothing
        if (selectedText.trim().length === 0) {
          return;
        }

        const res = await c.sendRequest<{ text?: string; error?: string }>(
          "taplo/convertToToml",
          {
            text: selectedText,
          }
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

        await vscode.window.showInformationMessage("TOML copied!");
      }
    ),
    vscode.commands.registerTextEditorCommand(
      "evenBetterToml.pasteAsJson",
      async editor => {
        const out = getOutput();
        let input: string;
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

        const res = await c.sendRequest<{ text?: string; error?: string }>(
          "taplo/convertToJson",
          {
            text: input,
          }
        );

        if (res.error?.length ?? 0 !== 0) {
          out.appendLine(`Failed to convert to JSON: ${res.error}`);

          const show = await vscode.window.showErrorMessage(
            "Pasting JSON has failed!",
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
      "evenBetterToml.pasteAsToml",
      async editor => {
        const out = getOutput();
        let input: string;
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

        const res = await c.sendRequest<{ text?: string; error?: string }>(
          "taplo/convertToToml",
          {
            text: input,
          }
        );

        if (res.error?.length ?? 0 !== 0) {
          out.appendLine(`Failed to convert to TOML: ${res.error}`);

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
}
