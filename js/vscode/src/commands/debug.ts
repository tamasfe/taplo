// Various debug commands

import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";

// export function register(
//   ctx: vscode.ExtensionContext,
//   c: client.LanguageClient
// ) {
//   c.onReady().then(() => {
//     registerShowSyntaxTree(ctx, c);
//   });
// }

// function registerShowSyntaxTree(
//   ctx: vscode.ExtensionContext,
//   c: client.LanguageClient
// ) {
//   ctx.subscriptions.push(
//     vscode.commands.registerTextEditorCommand(
//       "evenBetterToml.debug.showSyntaxTree",
//       async (editor) => {
//         const params: Methods.SyntaxTree.Params = {
//           uri: editor.document.uri.toString(),
//         };

//         const res = await c.sendRequest<Methods.SyntaxTree.Response>(
//           Methods.SyntaxTree.METHOD,
//           params
//         );

//         let doc = await vscode.workspace.openTextDocument({
//           content: res.text,
//           language: "ra_syntax_tree",
//         });

//         await vscode.window.showTextDocument(doc, {
//           preview: true,
//           viewColumn: vscode.ViewColumn.Beside,
//         });
//       }
//     )
//   );
// }
