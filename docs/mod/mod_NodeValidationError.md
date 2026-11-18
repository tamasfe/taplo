# mod.NodeValidationError

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 674-678)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct NodeValidationError {
    pub keys: Keys,
    pub node: dom::Node,
    pub error: ValidationError<'static>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct NodeValidationError {
    pub keys: Keys,
    pub node: dom::Node,
    pub error: ValidationError<'static>,
}
```

## Git History

- Commits: 13
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
