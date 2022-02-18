import * as vscode from "vscode";

export function allRange(doc: vscode.TextDocument): vscode.Range {
  let firstLine = doc.lineAt(0);
  let lastLine = doc.lineAt(doc.lineCount - 1);
  let textRange = new vscode.Range(firstLine.range.start, lastLine.range.end);
  return textRange;
}
