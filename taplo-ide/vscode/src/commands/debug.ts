// Various debug commands

import * as vscode from "vscode";
import * as client from "vscode-languageclient";
import * as requestExt from "../requestExt";

export function register(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  c.onReady().then(() => {
    registerShowSyntaxTree(ctx, c);
    registerShowLineMappings(ctx, c);
  });
}

function registerShowSyntaxTree(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  ctx.subscriptions.push(
    vscode.commands.registerTextEditorCommand(
      "evenBetterToml.debug.showSyntaxTree",
      async (editor) => {
        const params: requestExt.SyntaxTree.Params = {
          uri: editor.document.uri.toString(),
        };

        const res = await c.sendRequest<requestExt.SyntaxTree.Response>(
          requestExt.SyntaxTree.METHOD,
          params
        );

        let doc = await vscode.workspace.openTextDocument({
          content: res.text,
          language: "ra_syntax_tree",
        });

        await vscode.window.showTextDocument(doc, {
          preview: true,
          viewColumn: vscode.ViewColumn.Beside,
        });
      }
    )
  );
}

function registerShowLineMappings(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  ctx.subscriptions.push(
    vscode.commands.registerTextEditorCommand(
      "evenBetterToml.debug.showLineMappings",
      async (editor) => {
        const params: requestExt.SyntaxTree.Params = {
          uri: editor.document.uri.toString(),
        };

        const res = await c.sendRequest<requestExt.LineMappings.Response>(
          requestExt.LineMappings.METHOD,
          params
        );

        let s = "";
        for (const line of res.lines) {
          s += line + "\n";
        }

        let doc = await vscode.workspace.openTextDocument({
          content: s,
        });

        await vscode.window.showTextDocument(doc, {
          preview: true,
          viewColumn: vscode.ViewColumn.Beside,
        });
      }
    )
  );
}
