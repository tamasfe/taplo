import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import * as conversionCommands from "./conversion";
import * as debugCommands from "./debug";
import * as cacheCommands from "./cache";

export function registerCommands(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  conversionCommands.register(ctx, c);
  debugCommands.register(ctx, c);
  cacheCommands.register(ctx, c);
}
