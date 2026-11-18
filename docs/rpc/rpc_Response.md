# rpc.Response

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 120-130)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Response<R = ()> {
    pub jsonrpc: String,

    pub id: RequestId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<R>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Response<R = ()> {
    pub jsonrpc: String,

    pub id: RequestId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<R>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
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
