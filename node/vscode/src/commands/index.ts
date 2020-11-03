import * as vscode from "vscode";
import * as client from "vscode-languageclient";
import * as tomlToJsonCommand from "./tomlToJson";
import * as debugCommands from "./debug";

export function registerCommands(
  ctx: vscode.ExtensionContext,
  c: client.LanguageClient
) {
  tomlToJsonCommand.register(ctx, c);
  debugCommands.register(ctx, c);
}
