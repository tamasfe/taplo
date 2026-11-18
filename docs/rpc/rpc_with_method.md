# rpc.with_method

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 70-75)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn with_method(self, method: &str) -> Self {
        Self {
            method: method.into(),
            ..self
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn with_method(self, method: &str) -> Self {
        Self {
            method: method.into(),
            ..self
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
