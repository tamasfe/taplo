# lib.build

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 610-614)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn build(self) -> Server<W> {
        Server {
            inner: Arc::new(AsyncMutex::new(self.inner)),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn build(self) -> Server<W> {
        Server {
            inner: Arc::new(AsyncMutex::new(self.inner)),
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
