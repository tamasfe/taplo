# environment.cwd_normalized

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment.rs` (lines 69-71)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn cwd_normalized(&self) -> Option<PathBuf> {
        self.cwd().map(Normalize::normalize)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn cwd_normalized(&self) -> Option<PathBuf> {
        self.cwd().map(Normalize::normalize)
    }
```

## Git History

- Commits: 4
- Authors: 1
- Bug fixes: 1
- Features: 3

## Callers

None

## Callees

None
