import * as os from "os";
import * as path from "path";
import * as vscode from "vscode";
import { BaseLanguageClient } from "vscode-languageclient";

let output: vscode.OutputChannel;

export function getOutput(): vscode.OutputChannel {
  if (!output) {
    output = vscode.window.createOutputChannel("Even Better TOML");
  }

  return output;
}

export function allRange(doc: vscode.TextDocument): vscode.Range {
  let firstLine = doc.lineAt(0);
  let lastLine = doc.lineAt(doc.lineCount - 1);
  let textRange = new vscode.Range(firstLine.range.start, lastLine.range.end);
  return textRange;
}

export function resolveVariables(value: string): string {
  value = value.replace(/\$\{userHome\}/g, os.homedir());
  value = value.replace(/\$\{pathSeparator\}/g, path.sep);
  value = value.replace(/\$\{\/\}/g, path.sep);
  value = value.replace(
    /\$\{workspaceFolder\}/g,
    vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? ""
  );
  value = value.replace(
    /\$\{workspaceFolder:(.+?)\}/g,
    (_, name: string) =>
      vscode.workspace.workspaceFolders?.find(f => f.name === name)?.uri
        .fsPath ?? ""
  );
  value = value.replace(
    /\$\{workspaceFolderBasename\}/g,
    vscode.workspace.workspaceFolders?.[0]?.name ?? ""
  );
  value = value.replace(/\$\{cwd\}/g, process.cwd());
  value = value.replace(/\$\{execPath\}/g, process.execPath);
  value = value.replace(
    /\$\{env:(.+?)\}/g,
    (_, varName: string) => process.env[varName] ?? ""
  );
  value = value.replace(
    /\$\{config:(.+?)\}/g,
    (_, section: string) =>
      String(vscode.workspace.getConfiguration().get(section) ?? "")
  );
  return value;
}


export async function showMessage(
  params: { kind: "info" | "warn" | "error"; message: string },
  c: BaseLanguageClient
) {
  let show: string | undefined;
  switch (params.kind) {
    case "info":
      show = await vscode.window.showInformationMessage(
        params.message,
        "Show Details"
      );
    case "warn":
      show = await vscode.window.showWarningMessage(
        params.message,
        "Show Details"
      );
    case "error":
      show = await vscode.window.showErrorMessage(
        params.message,
        "Show Details"
      );
  }

  if (show) {
    c.outputChannel.show();
  }
}
