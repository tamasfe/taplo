# shared.deref

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\shared.rs` (lines 73-75)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
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
