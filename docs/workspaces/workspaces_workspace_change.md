# workspaces.workspace_change

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\workspaces.rs` (lines 7-36)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn workspace_change<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidChangeWorkspaceFoldersParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;
    let init_config = context.init_config.load();

    for removed in p.event.removed {
        workspaces.shift_remove(&removed.uri);
    }

    for added in p.event.added {
        let ws = workspaces
            .entry(added.uri.clone())
            .or_insert(WorkspaceState::new(context.env.clone(), added.uri));

        ws.schemas
            .cache()
            .set_cache_path(init_config.cache_path.clone());

        if let Err(error) = ws.initialize(context.clone(), &context.env).await {
            tracing::error!(?error, "failed to initialize workspace");
        }
    }

    drop(workspaces);
    update_configuration(context).await;
}
```

## Docstring

_No docstring provided._

## Source

```
pub async fn workspace_change<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidChangeWorkspaceFoldersParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;
    let init_config = context.init_config.load();

    for removed in p.event.removed {
        workspaces.shift_remove(&removed.uri);
    }

    for added in p.event.added {
        let ws = workspaces
            .entry(added.uri.clone())
            .or_insert(WorkspaceState::new(context.env.clone(), added.uri));

        ws.schemas
            .cache()
            .set_cache_path(init_config.cache_path.clone());

        if let Err(error) = ws.initialize(context.clone(), &context.env).await {
            tracing::error!(?error, "failed to initialize workspace");
        }
    }

    drop(workspaces);
    update_configuration(context).await;
}
```

## Git History

- Commits: 5
- Authors: 3
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
