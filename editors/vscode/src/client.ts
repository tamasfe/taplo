import * as vscode from "vscode";
import * as node from "vscode-languageclient/node";
import * as browser from "vscode-languageclient/browser";
import which from "which";
import { getOutput } from "./util";
import { BaseLanguageClient } from "vscode-languageclient";

export async function createClient(
  context: vscode.ExtensionContext
): Promise<BaseLanguageClient> {
  console.log(import.meta.env.BROWSER);

  if (import.meta.env.BROWSER) {
    return await createBrowserClient(context);
  } else {
    return await createNodeClient(context);
  }
}

async function createBrowserClient(context: vscode.ExtensionContext) {
  const serverMain = vscode.Uri.joinPath(
    context.extensionUri,
    "dist/server-worker.js"
  );
  const worker = new Worker(serverMain.toString(true));
  return new browser.LanguageClient(
    "taplo-lsp",
    "Taplo LSP",
    await clientOpts(context),
    worker
  );
}

async function createNodeClient(context: vscode.ExtensionContext) {
  const out = getOutput();

  const bundled = !!vscode.workspace
    .getConfiguration()
    .get("evenBetterToml.taplo.bundled");

  let serverOpts: node.ServerOptions;
  if (bundled) {
    const taploPath = vscode.Uri.joinPath(
      context.extensionUri,
      "dist/server.js"
    ).fsPath;

    const run: node.NodeModule = {
      module: taploPath,
      transport: node.TransportKind.ipc,
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

    const run: node.Executable = {
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

  return new node.LanguageClient(
    "evenBetterToml",
    "Even Better TOML LSP",
    serverOpts,
    await clientOpts(context)
  );
}

async function clientOpts(context: vscode.ExtensionContext): Promise<any> {
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
