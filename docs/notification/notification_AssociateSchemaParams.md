# notification.AssociateSchemaParams

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\lsp_ext\notification.rs` (lines 39-45)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct AssociateSchemaParams {
    pub document_uri: Option<Url>,
    pub schema_uri: Url,
    pub rule: AssociationRule,
    pub priority: Option<usize>,
    pub meta: Option<Value>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct AssociateSchemaParams {
    pub document_uri: Option<Url>,
    pub schema_uri: Url,
    pub rule: AssociationRule,
    pub priority: Option<usize>,
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
