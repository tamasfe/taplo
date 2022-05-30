import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import * as conversionCommands from "./conversion";
import * as schemaCommands from "./schema";

export function registerCommands(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  conversionCommands.register(ctx, c);
  schemaCommands.register(ctx, c);
}
