# native.env_vars

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 58-60)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn env_vars(&self) -> Vec<(String, String)> {
        std::env::vars().collect()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn env_vars(&self) -> Vec<(String, String)> {
        std::env::vars().collect()
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
