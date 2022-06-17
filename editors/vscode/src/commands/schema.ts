import * as vscode from "vscode";
import { BaseLanguageClient } from "vscode-languageclient";

export function register(ctx: vscode.ExtensionContext, c: BaseLanguageClient) {
  ctx.subscriptions.push(
    vscode.commands.registerTextEditorCommand(
      "evenBetterToml.selectSchema",
      async editor => {
        if (!editor) {
          return;
        }

        const schemasResp: { schemas: { url: string; meta?: any }[] } =
          await c.sendRequest("taplo/listSchemas", {
            documentUri: editor.document.uri.toString(),
          });

        interface SchemaItem extends vscode.QuickPickItem {
          url: string;
          meta?: Record<string, any>;
        }

        const selectedSchema: { schema?: { url: string } } =
          await c.sendRequest("taplo/associatedSchema", {
            documentUri: editor.document.uri.toString(),
          });

        const selection = await vscode.window.showQuickPick<SchemaItem>(
          schemasResp.schemas.map(s => ({
            label: s.meta?.name ?? s.url,
            description: schemaDescription(s.meta),
            detail: schemaDetails(s.url, s.meta),
            picked: selectedSchema.schema?.url === s.url,
            url: s.url,
            meta: s.meta,
          }))
        );

        if (!selection) {
          return;
        }

        c.sendNotification("taplo/associateSchema", {
          documentUri: editor.document.uri.toString(),
          schemaUri: selection.url,
          rule: {
            url: editor.document.uri.toString(),
          },
          meta: selection.meta,
        });
      }
    )
  );
}

function schemaDescription(meta: any | undefined): string | undefined {
  if (typeof meta?.description === "string") {
    return meta.description;
  } else {
    return undefined;
  }
}

function schemaDetails(url: string, _meta: any): string {
  let s = `${url}`;
  return s;
}
