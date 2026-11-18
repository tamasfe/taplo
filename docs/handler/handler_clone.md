# handler.clone

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\handler.rs` (lines 147-152)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn clone(&self) -> Self {
        Self {
            f: self.f,
            t: Default::default(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn clone(&self) -> Self {
        Self {
            f: self.f,
            t: Default::default(),
        }
    }
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
