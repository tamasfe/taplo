# config.is_schema_enabled

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 142-176)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_schema_enabled(&self, path: &Path) -> bool {
        let enabled = self
            .global_options
            .schema
            .as_ref()
            .and_then(|s| s.enabled)
            .unwrap_or(true);

        for rule in &self.rule {
            let rule_matched = match &self.file_rule {
                Some(r) => r.is_match(path),
                None => {
                    tracing::debug!("no file matches were set up");
                    false
                }
            };

            if !rule_matched {
                continue;
            }

            let rule_schema_enabled = rule
                .options
                .schema
                .as_ref()
                .and_then(|s| s.enabled)
                .unwrap_or(true);

            if !rule_schema_enabled {
                return false;
            }
        }

        enabled
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_schema_enabled(&self, path: &Path) -> bool {
        let enabled = self
            .global_options
            .schema
            .as_ref()
            .and_then(|s| s.enabled)
            .unwrap_or(true);

        for rule in &self.rule {
            let rule_matched = match &self.file_rule {
                Some(r) => r.is_match(path),
                None => {
                    tracing::debug!("no file matches were set up");
                    false
                }
            };

            if !rule_matched {
                continue;
            }

            let rule_schema_enabled = rule
                .options
                .schema
                .as_ref()
                .and_then(|s| s.enabled)
                .unwrap_or(true);

            if !rule_schema_enabled {
                return false;
            }
        }

        enabled
    }
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 4
- Features: 1

## Callers

None

## Callees

None
