# handler.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\handler.rs` (lines 133-138)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new(f: fn(Context<W>, Params<N::Params>) -> F) -> Self {
        Self {
            f,
            t: Default::default(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new(f: fn(Context<W>, Params<N::Params>) -> F) -> Self {
        Self {
            f,
            t: Default::default(),
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
