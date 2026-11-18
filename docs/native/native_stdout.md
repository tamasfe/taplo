# native.stdout

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 71-73)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn stdout(&self) -> Self::Stdout {
        tokio::io::stdout()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn stdout(&self) -> Self::Stdout {
        tokio::io::stdout()
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
