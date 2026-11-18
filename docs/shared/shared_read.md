# shared.read

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\shared.rs` (lines 47-49)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn read(&self) -> SharedGuard<T> {
        SharedGuard(self.0.load())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn read(&self) -> SharedGuard<T> {
        SharedGuard(self.0.load())
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
