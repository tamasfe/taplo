# native.cwd

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 106-108)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn cwd(&self) -> Option<std::path::PathBuf> {
        std::env::current_dir().ok()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn cwd(&self) -> Option<std::path::PathBuf> {
        std::env::current_dir().ok()
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
