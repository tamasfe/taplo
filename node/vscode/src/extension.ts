import * as vscode from "vscode";
import * as client from "vscode-languageclient";
import * as path from "path";
import { registerCommands } from "./commands";
import {
  CacheSchema,
  GetCachedSchema,
  MessageWithOutput,
  UpdateBuiltInSchemas,
} from "./requestExt";
import deepEqual from "deep-equal";
import fs from "fs";

let output: vscode.OutputChannel;
let extensionContext: vscode.ExtensionContext;

export function getOutput(): vscode.OutputChannel {
  return output;
}

export async function activate(context: vscode.ExtensionContext) {
  let p = context.asAbsolutePath(path.join("dist", "server.js"));

  extensionContext = context;

  let serverOpts: client.ServerOptions = {
    run: { module: p, transport: client.TransportKind.ipc },
    debug: { module: p, transport: client.TransportKind.ipc },
  };

  let clientOpts: client.LanguageClientOptions = {
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

  let c = new client.LanguageClient(
    "evenBetterToml",
    "Even Better TOML LSP",
    serverOpts,
    clientOpts
  );

  c.registerProposedFeatures();

  output = vscode.window.createOutputChannel("Even Better TOML");

  registerCommands(context, c);

  context.subscriptions.push(output, c.start());

  const showNotification = vscode.workspace
    .getConfiguration()
    .get("evenBetterToml.activationStatus");

  if (showNotification) {
    await vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Window,
        title: "TOML loading...",
      },
      async _ => {
        await c.onReady();
      }
    );
  } else {
    await c.onReady();
  }

  c.onNotification(MessageWithOutput.METHOD, async params =>
    showMessage(params, c)
  );
  c.onNotification(UpdateBuiltInSchemas.METHOD, updateAssociations);
  c.onRequest(CacheSchema.METHOD, cacheSchema);
  c.onRequest(GetCachedSchema.METHOD, getCachedSchema);
}

async function showMessage(
  params: MessageWithOutput.Params,
  c: client.LanguageClient
) {
  let show: string | undefined;
  switch (params.kind) {
    case MessageWithOutput.MessageKind.Info:
      show = await vscode.window.showInformationMessage(
        params.message,
        "Show Details"
      );
    case MessageWithOutput.MessageKind.Warn:
      show = await vscode.window.showWarningMessage(
        params.message,
        "Show Details"
      );
    case MessageWithOutput.MessageKind.Error:
      show = await vscode.window.showErrorMessage(
        params.message,
        "Show Details"
      );
  }

  if (show) {
    c.outputChannel.show();
  }
}

async function cacheSchema(params: CacheSchema.Params) {
  const storagePath = extensionContext.globalStoragePath;
  const schemaPath = path.join(storagePath, "schemas.json");

  output.appendLine(storagePath);

  await fs.promises.mkdir(storagePath, { recursive: true });

  let schemas: { [key: string]: string } = {};
  if (fs.existsSync(schemaPath)) {
    schemas = JSON.parse(await fs.promises.readFile(schemaPath, "utf-8"));
  }

  schemas[params.schemaUri] = params.schemaJson;

  await fs.promises.writeFile(schemaPath, JSON.stringify(schemas));
}

async function getCachedSchema(
  params: GetCachedSchema.Params
): Promise<GetCachedSchema.Response> {
  const storagePath = extensionContext.globalStoragePath;
  const schemaPath = path.join(storagePath, "schemas.json");

  await fs.promises.mkdir(storagePath, { recursive: true });

  if (!fs.existsSync(schemaPath)) {
    return {};
  }
  const schemas: { [key: string]: string } = JSON.parse(
    await fs.promises.readFile(schemaPath, "utf-8")
  );

  return { schemaJson: schemas[params.schemaUri] };
}

async function updateAssociations(params: UpdateBuiltInSchemas.Params) {
  type Choice = "ask" | "always" | "never";

  const updateNew: Choice =
    vscode.workspace
      .getConfiguration()
      .get("evenBetterToml.extension.actions.schema.addNewBuiltins") ?? "ask";

  const removeOld: Choice =
    vscode.workspace
      .getConfiguration()
      .get("evenBetterToml.extension.actions.schema.removeOldBuiltins") ??
    "ask";

  if (updateNew === "never" && removeOld === "never") {
    return;
  }

  const defaultAssociations: any =
    vscode.workspace
      .getConfiguration()
      .inspect("evenBetterToml.schema.associations")?.defaultValue ?? {};

  let currentAssociations: any =
    vscode.workspace
      .getConfiguration()
      .get("evenBetterToml.schema.associations") ?? {};

  if (deepEqual(defaultAssociations, currentAssociations, { strict: true })) {
    // default values, nothing to do
    return;
  }

  if (updateNew !== "never") {
    const toAdd: any = {};
    let needUpdate = false;

    for (const key of Object.keys(params.associations)) {
      const newAssoc = params.associations[key];

      let found = false;
      for (const currentKey of Object.keys(currentAssociations)) {
        const currentAssoc = currentAssociations[currentKey];

        if (newAssoc === currentAssoc) {
          found = true;
          break;
        }
      }

      if (!found) {
        toAdd[key] = newAssoc;
        needUpdate = true;
      }
    }

    if (needUpdate) {
      if (updateNew === "ask") {
        let action = await vscode.window.showInformationMessage(
          "There are new built-in schemas available. Update the associations?",
          "Update",
          "Never Update",
          "Always Update"
        );

        switch (action) {
          case "Update":
            await vscode.workspace.getConfiguration().update(
              "evenBetterToml.schema.associations",
              {
                ...currentAssociations,
                ...toAdd,
              },
              vscode.ConfigurationTarget.Global
            );
            break;
          case "Never Update":
            await vscode.workspace
              .getConfiguration()
              .update(
                "evenBetterToml.extension.actions.schema.addNewBuiltins",
                "never",
                vscode.ConfigurationTarget.Global
              );
            break;
          case "Always Update":
            await vscode.workspace
              .getConfiguration()
              .update(
                "evenBetterToml.extension.actions.schema.addNewBuiltins",
                "always",
                vscode.ConfigurationTarget.Global
              );

            await vscode.workspace.getConfiguration().update(
              "evenBetterToml.schema.associations",
              {
                ...currentAssociations,
                ...toAdd,
              },
              vscode.ConfigurationTarget.Global
            );
            break;
        }
      } else {
        // always update
        await vscode.workspace.getConfiguration().update(
          "evenBetterToml.schema.associations",
          {
            ...currentAssociations,
            ...toAdd,
          },
          vscode.ConfigurationTarget.Global
        );
      }
    }
  }

  currentAssociations =
    vscode.workspace
      .getConfiguration()
      .get("evenBetterToml.schema.associations") ?? {};

  if (deepEqual(defaultAssociations, currentAssociations, { strict: true })) {
    // default values, nothing to do
    return;
  }

  if (removeOld !== "never") {
    const finalAssociations: any = {};
    let needRemove = false;

    const deprecated = [
      (val: string): boolean => val.startsWith("toml_builtin://"),
    ];

    for (const key of Object.keys(currentAssociations)) {
      const currentAssoc: string = currentAssociations[key];

      let toRemove = false;
      for (const isDeprecated of deprecated) {
        if (isDeprecated(currentAssoc)) {
          toRemove = true;
          needRemove = true;
          break;
        }
      }

      if (!toRemove) {
        finalAssociations[key] = currentAssoc;
      }
    }

    if (needRemove) {
      if (removeOld === "ask") {
        let action = await vscode.window.showWarningMessage(
          "There are deprecated built-in schemas in associations. Remove them?",
          "Remove",
          "Never Remove",
          "Always Remove"
        );

        switch (action) {
          case "Remove":
            await vscode.workspace
              .getConfiguration()
              .update(
                "evenBetterToml.schema.associations",
                finalAssociations,
                vscode.ConfigurationTarget.Global
              );
            break;
          case "Never Remove":
            await vscode.workspace
              .getConfiguration()
              .update(
                "evenBetterToml.extension.actions.schema.removeOldBuiltins",
                "never",
                vscode.ConfigurationTarget.Global
              );
            break;
          case "Always Remove":
            await vscode.workspace
              .getConfiguration()
              .update(
                "evenBetterToml.extension.actions.schema.removeOldBuiltins",
                "always",
                vscode.ConfigurationTarget.Global
              );

            await vscode.workspace
              .getConfiguration()
              .update(
                "evenBetterToml.schema.associations",
                finalAssociations,
                vscode.ConfigurationTarget.Global
              );
            break;
        }
      } else {
        await vscode.workspace
          .getConfiguration()
          .update(
            "evenBetterToml.schema.associations",
            finalAssociations,
            vscode.ConfigurationTarget.Global
          );
      }
    }
  }
}
