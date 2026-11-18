# convert.json_to_toml

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\convert.rs` (lines 3-6)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn json_to_toml(json: &str, inline: bool) -> Result<String, anyhow::Error> {
    let root: Node = serde_json::from_str(json)?;
    Ok(root.to_toml(inline, false))
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn json_to_toml(json: &str, inline: bool) -> Result<String, anyhow::Error> {
    let root: Node = serde_json::from_str(json)?;
    Ok(root.to_toml(inline, false))
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
