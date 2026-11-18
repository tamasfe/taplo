# lib.required

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 624-629)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn required(self) -> Result<P, rpc::Error> {
        match self.0 {
            None => Err(rpc::Error::invalid_params().with_data("params are required")),
            Some(p) => Ok(p),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn required(self) -> Result<P, rpc::Error> {
        match self.0 {
            None => Err(rpc::Error::invalid_params().with_data("params are required")),
            Some(p) => Ok(p),
        }
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
