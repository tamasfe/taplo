# request.ConvertToJsonResponse

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\lsp_ext\request.rs` (lines 17-24)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct ConvertToJsonResponse {
    /// JSON text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct ConvertToJsonResponse {
    /// JSON text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
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
