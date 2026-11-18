# rpc.content_modified

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 301-307)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn content_modified() -> Error {
        Error {
            code: -32801,
            message: "Content modified".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn content_modified() -> Error {
        Error {
            code: -32801,
            message: "Content modified".into(),
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
