# initialize.initialized

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\initialize.rs` (lines 111-118)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn initialized<E: Environment>(
    context: Context<World<E>>,
    _params: Params<InitializedParams>,
) {
    context
        .env
        .spawn_local(update_configuration(context.clone()));
}
```

## Docstring

_No docstring provided._

## Source

```
pub async fn initialized<E: Environment>(
    context: Context<World<E>>,
    _params: Params<InitializedParams>,
) {
    context
        .env
        .spawn_local(update_configuration(context.clone()));
}
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
