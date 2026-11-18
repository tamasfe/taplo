# lib.as_err

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 70-72)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_err(&mut self) -> CancelTokenErr<'_> {
        CancelTokenErr(self)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn as_err(&mut self) -> CancelTokenErr<'_> {
        CancelTokenErr(self)
    }
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
