# native.env_var

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 54-56)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn env_var(&self, name: &str) -> Option<String> {
        std::env::var(name).ok()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn env_var(&self, name: &str) -> Option<String> {
        std::env::var(name).ok()
    }
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
