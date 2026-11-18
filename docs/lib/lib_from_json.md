# lib.from_json

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lib.rs` (lines 160-163)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn from_json(json: &str) -> Result<String, JsError> {
    let dom: Node = serde_json::from_str(json)?;
    Ok(dom.to_toml(false, false))
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn from_json(json: &str) -> Result<String, JsError> {
    let dom: Node = serde_json::from_str(json)?;
    Ok(dom.to_toml(false, false))
}
```

## Git History

- Commits: 8
- Authors: 2
- Bug fixes: 5
- Features: 2

## Callers

None

## Callees

None
