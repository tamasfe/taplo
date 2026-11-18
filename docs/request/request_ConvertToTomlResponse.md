# request.ConvertToTomlResponse

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\lsp_ext\request.rs` (lines 44-51)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct ConvertToTomlResponse {
    /// TOML text.
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
pub struct ConvertToTomlResponse {
    /// TOML text.
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
