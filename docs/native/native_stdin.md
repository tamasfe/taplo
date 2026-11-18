# native.stdin

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 67-69)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn stdin(&self) -> Self::Stdin {
        tokio::io::stdin()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn stdin(&self) -> Self::Stdin {
        tokio::io::stdin()
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
