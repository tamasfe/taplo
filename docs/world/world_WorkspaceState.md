# world.WorkspaceState

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\world.rs` (lines 109-115)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct WorkspaceState<E: Environment> {
    pub(crate) root: Url,
    pub(crate) documents: HashMap<lsp_types::Url, DocumentState>,
    pub(crate) taplo_config: Config,
    pub(crate) schemas: Schemas<E>,
    pub(crate) config: LspConfig,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct WorkspaceState<E: Environment> {
    pub(crate) root: Url,
    pub(crate) documents: HashMap<lsp_types::Url, DocumentState>,
    pub(crate) taplo_config: Config,
    pub(crate) schemas: Schemas<E>,
    pub(crate) config: LspConfig,
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
