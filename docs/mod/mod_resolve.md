# mod.resolve

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 656-665)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn resolve(
        &self,
        _root_schema: &serde_json::Value,
        url: &Url,
        _original_ref: &str,
    ) -> Result<Arc<serde_json::Value>, jsonschema::SchemaResolverError> {
        self.cache
            .get_schema(url)
            .ok_or_else(|| WouldBlockError.into())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn resolve(
        &self,
        _root_schema: &serde_json::Value,
        url: &Url,
        _original_ref: &str,
    ) -> Result<Arc<serde_json::Value>, jsonschema::SchemaResolverError> {
        self.cache
            .get_schema(url)
            .ok_or_else(|| WouldBlockError.into())
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
