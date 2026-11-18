# lib.Server

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 293-295)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Server<W: Clone> {
    inner: Arc<AsyncMutex<Inner<W>>>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Server<W: Clone> {
    inner: Arc<AsyncMutex<Inner<W>>>,
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
