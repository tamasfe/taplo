# rpc.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 227-233)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new(message: &str) -> Self {
        Error {
            code: 0,
            message: message.into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new(message: &str) -> Self {
        Error {
            code: 0,
            message: message.into(),
            data: None,
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
