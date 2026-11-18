# mod.validate_root

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 91-102)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn validate_root(
        &self,
        schema_url: &Url,
        root: &dom::Node,
    ) -> Result<Vec<NodeValidationError>, anyhow::Error> {
        let value = serde_json::to_value(root)?;
        self.validate(schema_url, &value)
            .await?
            .into_iter()
            .map(|error| NodeValidationError::new(root, error))
            .collect::<Result<Vec<_>, _>>()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn validate_root(
        &self,
        schema_url: &Url,
        root: &dom::Node,
    ) -> Result<Vec<NodeValidationError>, anyhow::Error> {
        let value = serde_json::to_value(root)?;
        self.validate(schema_url, &value)
            .await?
            .into_iter()
            .map(|error| NodeValidationError::new(root, error))
            .collect::<Result<Vec<_>, _>>()
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
