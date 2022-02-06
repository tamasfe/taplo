import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import * as path from "path";
import which from "which";
import { getOutput } from "./extension";

export function createClient(
  context: vscode.ExtensionContext
): client.LanguageClient {
  const nativeTaplo = !vscode.workspace
    .getConfiguration()
    .get("evenBetterToml.taplo.bundled");

  let c: client.LanguageClient;
  if (nativeTaplo) {
    c = createNativeClient();
  } else {
    c = createBundledClient(context);
  }

  return c;
}

function createNativeClient(): client.LanguageClient {
  const taploPath =
    vscode.workspace
      .getConfiguration()
      .get("evenBetterToml.taplo.native.path") ??
    which.sync("taplo", { nothrow: true });

  if (typeof taploPath !== "string") {
    getOutput().appendLine("failed to locate Taplo LSP");
    throw new Error("failed to locate Taplo LSP");
  }

  let extraArgs = vscode.workspace
    .getConfiguration()
    .get("evenBetterToml.taplo.native.extraArgs");

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
          .get("evenBetterToml.taplo.native.env") ?? undefined,
    },
  };

  let serverOpts: client.ServerOptions = {
    run,
    debug: run,
  };

  return new client.LanguageClient(
    "evenBetterToml",
    "Even Better TOML LSP",
    serverOpts,
    clientOpts()
  );
}

function createBundledClient(
  context: vscode.ExtensionContext
): client.LanguageClient {
  let p = context.asAbsolutePath(path.join("dist", "server.js"));

  let serverOpts: client.ServerOptions = {
    run: { module: p, transport: client.TransportKind.ipc },
    debug: { module: p, transport: client.TransportKind.ipc },
  };

  return new client.LanguageClient(
    "evenBetterToml",
    "Even Better TOML LSP",
    serverOpts,
    clientOpts()
  );
}

function clientOpts(): client.LanguageClientOptions {
  return {
    documentSelector: [
      { scheme: "file", language: "toml" },
      { scheme: "file", language: "cargoLock" },
    ],

    initializationOptions: {
      configuration: vscode.workspace.getConfiguration().get("evenBetterToml"),
    },

    synchronize: {
      configurationSection: "evenBetterToml",
      fileEvents: [
        vscode.workspace.createFileSystemWatcher("**/.toml"),
        vscode.workspace.createFileSystemWatcher("**/Cargo.lock"),
      ],
    },
  };
}
