# world.set_default_config

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\world.rs` (lines 104-106)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn set_default_config(&self, default_config: Arc<Config>) {
        self.default_config.store(default_config);
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn set_default_config(&self, default_config: Arc<Config>) {
        self.default_config.store(default_config);
    }
```

## Git History

- Commits: 13
- Authors: 3
- Bug fixes: 4
- Features: 5

## Callers

None

## Callees

None
