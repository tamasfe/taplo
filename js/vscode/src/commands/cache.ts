import * as vscode from "vscode";
import * as client from "vscode-languageclient/node";
import fs from "fs";
import path from "path";

// export function register(
//   ctx: vscode.ExtensionContext,
//   _c: client.LanguageClient
// ) {
//   ctx.subscriptions.push(
//     vscode.commands.registerTextEditorCommand(
//       "evenBetterToml.clearCache",
//       async () => {
//         try {
//           await fs.promises.rmdir(ctx.globalStorageUri.fsPath, {
//             recursive: true,
//           });
//         } catch (e) {
//           // It might not exist.
//           console.warn(e);
//         }
//         await fs.promises.mkdir(ctx.globalStorageUri.fsPath, {
//           recursive: true,
//         });

//         vscode.window.showInformationMessage(
//           "The cache directory has been cleared."
//         );
//       }
//     ),
//     vscode.commands.registerTextEditorCommand(
//       "evenBetterToml.downloadSchemas",
//       async () => {
//         await vscode.window.withProgress(
//           {
//             location: vscode.ProgressLocation.Notification,
//             title: "Downloading Schemas.",
//           },
//           async progress => {
//             const config: any = vscode.workspace
//               .getConfiguration()
//               .get("evenBetterToml");
//             if (!config?.schema?.repositoryEnabled) {
//               vscode.window.showInformationMessage(
//                 "Schema repository is disabled in the settings."
//               );
//               return;
//             }

//             vscode.workspace
//               .getConfiguration()
//               .update("", "", vscode.ConfigurationTarget.Global);

//             const indexUrl = config?.schema?.repositoryUrl;

//             if (!indexUrl) {
//               vscode.window.showInformationMessage(
//                 "Schema repository is not available in the settings."
//               );
//               return;
//             }

//             progress.report({
//               message: "Fetching schema index",
//             });

//             try {
//               const index: any = await fetch(indexUrl).then(res => res.json());

//               if (!index?.schemas) {
//                 throw new Error("invalid index JSON");
//               }

//               await fs.promises.writeFile(
//                 path.join(ctx.globalStorageUri.fsPath, "schema_index.json"),
//                 JSON.stringify(index)
//               );

//               const schemaCount: number = index.schemas?.length ?? 0;
//               let schemaDone: number = 0;

//               const schemaStep = schemaCount === 0 ? 0 : 100 / schemaCount;

//               const schemasPath = path.join(
//                 ctx.globalStorageUri.fsPath,
//                 "schemas"
//               );

//               await fs.promises.mkdir(schemasPath, { recursive: true });

//               // FIXME: maybe do this concurrently?
//               for (let i = 0; i < schemaCount; i++) {
//                 const schemaMeta = index.schemas[i];
//                 try {
//                   const schema = await fetch(schemaMeta.url).then(res =>
//                     res.json()
//                   );

//                   await fs.promises.writeFile(
//                     path.join(schemasPath, `${schemaMeta.urlHash}.json`),
//                     JSON.stringify({
//                       url: schemaMeta.url,
//                       schema: schema,
//                     })
//                   );

//                   schemaDone += 1;
//                 } catch (e) {
//                   // TODO: handle this better.
//                   console.warn(e);
//                 }

//                 progress.report({
//                   message: `Downloaded schema (${i}/${schemaCount}).`,
//                   increment: schemaStep,
//                 });
//               }

//               vscode.window.showInformationMessage(
//                 `Updated ${schemaDone}/${schemaCount} schemas from the repository.`
//               );
//             } catch (e) {
//               console.error(e);
//               vscode.window.showErrorMessage("Failed to download schemas.");
//             }
//           }
//         );
//       }
//     )
//   );
// }
