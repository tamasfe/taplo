# rpc.parse

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 245-251)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn parse() -> Error {
        Error {
            code: -32700,
            message: "Parse error".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn parse() -> Error {
        Error {
            code: -32700,
            message: "Parse error".into(),
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
