# config.update_from_json

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\config.rs` (lines 45-51)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn update_from_json(&mut self, json: &Value) -> Result<(), anyhow::Error> {
        *self = Figment::new()
            .merge(Serialized::defaults(&self))
            .merge(Serialized::defaults(json))
            .extract()?;
        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn update_from_json(&mut self, json: &Value) -> Result<(), anyhow::Error> {
        *self = Figment::new()
            .merge(Serialized::defaults(&self))
            .merge(Serialized::defaults(json))
            .extract()?;
        Ok(())
    }
```

## Git History

- Commits: 3
- Authors: 1
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
