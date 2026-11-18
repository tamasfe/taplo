# configuration.configuration_change

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\configuration.rs` (lines 12-29)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn configuration_change<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidChangeConfigurationParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;

    for (_, ws) in workspaces.iter_mut() {
        if let Err(error) = ws.config.update_from_json(&p.settings) {
            tracing::error!(?error, "invalid configuration");
        }

        if let Err(error) = ws.initialize(context.clone(), &context.env).await {
            tracing::error!(%error, "failed to update workspace");
        }
    }
}
```

## Docstring

_No docstring provided._

## Source

```
pub async fn configuration_change<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidChangeConfigurationParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;

    for (_, ws) in workspaces.iter_mut() {
        if let Err(error) = ws.config.update_from_json(&p.settings) {
            tracing::error!(?error, "invalid configuration");
        }

        if let Err(error) = ws.initialize(context.clone(), &context.env).await {
            tracing::error!(%error, "failed to update workspace");
        }
    }
}
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
