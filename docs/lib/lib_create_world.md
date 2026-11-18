# lib.create_world

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\lib.rs` (lines 57-59)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn create_world<E: Environment>(env: E) -> World<E> {
    Arc::new(WorldState::new(env))
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn create_world<E: Environment>(env: E) -> World<E> {
    Arc::new(WorldState::new(env))
}
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
