# rpc.into_result

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 148-154)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn into_result(self) -> Result<R, Error> {
        if let Some(r) = self.result {
            Ok(r)
        } else {
            Err(self.error.unwrap())
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn into_result(self) -> Result<R, Error> {
        if let Some(r) = self.result {
            Ok(r)
        } else {
            Err(self.error.unwrap())
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
