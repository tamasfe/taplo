# rpc.Error

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 214-218)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Error {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Error {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
