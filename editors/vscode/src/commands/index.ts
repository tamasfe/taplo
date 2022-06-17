import * as vscode from "vscode";
import * as conversionCommands from "./conversion";
import * as schemaCommands from "./schema";
import { BaseLanguageClient } from "vscode-languageclient";


export function registerCommands(
  ctx: vscode.ExtensionContext,
  c: BaseLanguageClient
) {
  conversionCommands.register(ctx, c);
  schemaCommands.register(ctx, c);
}
