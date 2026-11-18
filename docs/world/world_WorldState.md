# world.WorldState

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\world.rs` (lines 77-82)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct WorldState<E: Environment> {
    pub(crate) init_config: ArcSwap<InitConfig>,
    pub(crate) env: E,
    pub(crate) workspaces: AsyncRwLock<Workspaces<E>>,
    pub(crate) default_config: ArcSwap<Config>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct WorldState<E: Environment> {
    pub(crate) init_config: ArcSwap<InitConfig>,
    pub(crate) env: E,
    pub(crate) workspaces: AsyncRwLock<Workspaces<E>>,
    pub(crate) default_config: ArcSwap<Config>,
}
```

## Git History

- Commits: 13
- Authors: 3
- Bug fixes: 4
- Features: 5

## Callers

None

## Callees

None
