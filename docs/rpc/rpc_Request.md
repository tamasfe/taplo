# rpc.Request

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 52-58)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Request<T = ()> {
    pub jsonrpc: String,
    pub method: String,

    pub id: Option<RequestId>,
    pub params: Option<T>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Request<T = ()> {
    pub jsonrpc: String,
    pub method: String,

    pub id: Option<RequestId>,
    pub params: Option<T>,
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
