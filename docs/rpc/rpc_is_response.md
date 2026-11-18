# rpc.is_response

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 27-29)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_response(&self) -> bool {
        self.method.is_none()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_response(&self) -> bool {
        self.method.is_none()
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
