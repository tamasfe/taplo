# mod.get_validator

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 220-226)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn get_validator(&self, schema_url: &Url) -> Option<Arc<JSONSchema>> {
        if self.cache().lru_expired() {
            self.validators.lock().clear();
        }

        self.validators.lock().get(schema_url).cloned()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn get_validator(&self, schema_url: &Url) -> Option<Arc<JSONSchema>> {
        if self.cache().lru_expired() {
            self.validators.lock().clear();
        }

        self.validators.lock().get(schema_url).cloned()
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
