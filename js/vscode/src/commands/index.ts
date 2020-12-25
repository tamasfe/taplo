import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import * as conversion from "./conversion";
import * as debugCommands from "./debug";

export function registerCommands(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  conversion.register(ctx, c);
  debugCommands.register(ctx, c);
}
