# lsp.TaploWasmLsp

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lsp.rs` (lines 11-15)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct TaploWasmLsp {
    pub(crate) server: Server<Arc<WorldState<WasmEnvironment>>>,
    pub(crate) world: Arc<WorldState<WasmEnvironment>>,
    pub(crate) lsp_interface: WasmLspInterface,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct TaploWasmLsp {
    pub(crate) server: Server<Arc<WorldState<WasmEnvironment>>>,
    pub(crate) world: Arc<WorldState<WasmEnvironment>>,
    pub(crate) lsp_interface: WasmLspInterface,
}
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
