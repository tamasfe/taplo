# notification.DidChangeSchemaAssociationParams

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\lsp_ext\notification.rs` (lines 56-60)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct DidChangeSchemaAssociationParams {
    pub document_uri: Url,
    pub schema_uri: Option<Url>,
    pub meta: Option<Value>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct DidChangeSchemaAssociationParams {
    pub document_uri: Url,
    pub schema_uri: Option<Url>,
    pub meta: Option<Value>,
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
