# mod.create_validator

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 260-269)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn create_validator(&self, schema: &Value) -> Result<JSONSchema, anyhow::Error> {
        JSONSchema::options()
            .with_resolver(CacheSchemaResolver {
                cache: self.cache().clone(),
            })
            .with_format("semver", formats::semver)
            .with_format("semver-requirement", formats::semver_req)
            .compile(schema)
            .map_err(|err| anyhow!("invalid schema: {err}"))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn create_validator(&self, schema: &Value) -> Result<JSONSchema, anyhow::Error> {
        JSONSchema::options()
            .with_resolver(CacheSchemaResolver {
                cache: self.cache().clone(),
            })
            .with_format("semver", formats::semver)
            .with_format("semver-requirement", formats::semver_req)
            .compile(schema)
            .map_err(|err| anyhow!("invalid schema: {err}"))
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
