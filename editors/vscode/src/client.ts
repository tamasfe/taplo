import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import * as path from "path";
import which from "which";
import { getOutput } from "./util";

export async function createClient(
  context: vscode.ExtensionContext
): Promise<client.LanguageClient> {
  const out = getOutput();

  const bundled = !!vscode.workspace
    .getConfiguration()
    .get("evenBetterToml.taplo.bundled");

  let serverOpts: client.ServerOptions;
  if (bundled) {
    const taploPath = context.asAbsolutePath(path.join("dist", "server.js"));

    const run: client.NodeModule = {
      module: taploPath,
      transport: client.TransportKind.ipc,
    };

    serverOpts = {
      run,
      debug: run,
    };
  } else {
    const taploPath =
      vscode.workspace.getConfiguration().get("evenBetterToml.taplo.path") ??
      which.sync("taplo", { nothrow: true });

    if (typeof taploPath !== "string") {
      out.appendLine("failed to locate Taplo LSP");
      throw new Error("failed to locate Taplo LSP");
    }

    let extraArgs = vscode.workspace
      .getConfiguration()
      .get("evenBetterToml.taplo.extraArgs");

    if (!Array.isArray(extraArgs)) {
      extraArgs = [];
    }

    const args: string[] = (extraArgs as any[]).filter(
      a => typeof a === "string"
    );

    const run: client.Executable = {
      command: taploPath,
      args: ["lsp", "stdio", ...args],
      options: {
        env:
          vscode.workspace
            .getConfiguration()
            .get("evenBetterToml.taplo.environment") ?? undefined,
      },
    };

    serverOpts = {
      run,
      debug: run,
    };
  }

  return new client.LanguageClient(
    "evenBetterToml",
    "Even Better TOML LSP",
    serverOpts,
    await clientOpts(context)
  );
}

async function clientOpts(
  context: vscode.ExtensionContext
): Promise<client.LanguageClientOptions> {
  await vscode.workspace.fs.createDirectory(context.globalStorageUri);

  return {
    documentSelector: [
      { scheme: "file", language: "toml" },
      { scheme: "file", language: "cargoLock" },
    ],

    initializationOptions: {
      configurationSection: "evenBetterToml",
      cachePath: context.globalStorageUri.fsPath,
    },
  };
}
