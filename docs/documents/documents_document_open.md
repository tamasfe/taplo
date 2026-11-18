# documents.document_open

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\documents.rs` (lines 18-84)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn document_open<E: Environment>(
    mut context: Context<World<E>>,
    params: Params<DidOpenTextDocumentParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document_mut(&p.text_document.uri);

    if let Some(pth) = context.env.to_file_path_normalized(&p.text_document.uri) {
        if !ws.taplo_config.is_included(&pth) {
            drop(workspaces);
            context
                .write_notification::<notification::PublishDiagnostics, _>(Some(
                    PublishDiagnosticsParams {
                        uri: p.text_document.uri.clone(),
                        diagnostics: vec![Diagnostic {
                            range: Default::default(),
                            severity: Some(DiagnosticSeverity::HINT),
                            code: None,
                            code_description: None,
                            source: Some("Even Better TOML".into()),
                            message: "this document has been excluded".into(),
                            related_information: None,
                            tags: None,
                            data: None,
                        }],
                        version: None,
                    },
                ))
                .await
                .unwrap_or_else(|err| tracing::error!("{err}"));
            return;
        }
    }

    let parse = taplo::parser::parse(&p.text_document.text);
    let mapper = Mapper::new_utf16(&p.text_document.text, false);

    let dom = parse.clone().into_dom();

    if ws.config.schema.enabled {
        ws.schemas
            .associations()
            .retain(|(rule, assoc)| match rule {
                AssociationRule::Url(u) => {
                    !(u == &p.text_document.uri
                        && (assoc.meta["source"] != source::DIRECTIVE
                            || assoc.meta["source"] != source::SCHEMA_FIELD))
                }
                _ => true,
            });
        ws.schemas
            .associations()
            .add_from_document(&p.text_document.uri, &dom);
        ws.emit_associations(context.clone()).await;
    }

    ws.documents.insert(
        p.text_document.uri.clone(),
        DocumentState { parse, dom, mapper },
    );

    let ws_root = ws.root.clone();
    drop(workspaces);
    diagnostics::publish_diagnostics(context.clone(), ws_root, p.text_document.uri).await;
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn document_open<E: Environment>(
    mut context: Context<World<E>>,
    params: Params<DidOpenTextDocumentParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document_mut(&p.text_document.uri);

    if let Some(pth) = context.env.to_file_path_normalized(&p.text_document.uri) {
        if !ws.taplo_config.is_included(&pth) {
            drop(workspaces);
            context
                .write_notification::<notification::PublishDiagnostics, _>(Some(
                    PublishDiagnosticsParams {
                        uri: p.text_document.uri.clone(),
                        diagnostics: vec![Diagnostic {
                            range: Default::default(),
                            severity: Some(DiagnosticSeverity::HINT),
                            code: None,
                            code_description: None,
                            source: Some("Even Better TOML".into()),
                            message: "this document has been excluded".into(),
                            related_information: None,
                            tags: None,
                            data: None,
                        }],
                        version: None,
                    },
                ))
                .await
                .unwrap_or_else(|err| tracing::error!("{err}"));
            return;
        }
    }

    let parse = taplo::parser::parse(&p.text_document.text);
    let mapper = Mapper::new_utf16(&p.text_document.text, false);

    let dom = parse.clone().into_dom();

    if ws.config.schema.enabled {
        ws.schemas
            .associations()
            .retain(|(rule, assoc)| match rule {
                AssociationRule::Url(u) => {
                    !(u == &p.text_document.uri
                        && (assoc.meta["source"] != source::DIRECTIVE
                            || assoc.meta["source"] != source::SCHEMA_FIELD))
                }
                _ => true,
            });
        ws.schemas
            .associations()
            .add_from_document(&p.text_document.uri, &dom);
        ws.emit_associations(context.clone()).await;
    }

    ws.documents.insert(
        p.text_document.uri.clone(),
        DocumentState { parse, dom, mapper },
    );

    let ws_root = ws.root.clone();
    drop(workspaces);
    diagnostics::publish_diagnostics(context.clone(), ws_root, p.text_document.uri).await;
}
```

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 2
- Features: 2

## Callers

None

## Callees

None
