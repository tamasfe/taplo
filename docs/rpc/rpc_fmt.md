# rpc.fmt

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 221-223)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RPC error ({}): {}", self.code, self.message)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RPC error ({}): {}", self.code, self.message)
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
