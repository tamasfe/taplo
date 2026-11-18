# convert.toml_to_json

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\convert.rs` (lines 8-11)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn toml_to_json(toml: &str) -> Result<String, anyhow::Error> {
    let root = parse(toml).into_dom();
    Ok(serde_json::to_string_pretty(&root)?)
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn toml_to_json(toml: &str) -> Result<String, anyhow::Error> {
    let root = parse(toml).into_dom();
    Ok(serde_json::to_string_pretty(&root)?)
}
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
