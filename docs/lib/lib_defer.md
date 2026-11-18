# lib.defer

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 196-198)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn defer<F: Future<Output = ()> + 'static>(&self, fut: F) {
        self.deferred.lock().await.push(Box::pin(fut));
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn defer<F: Future<Output = ()> + 'static>(&self, fut: F) {
        self.deferred.lock().await.push(Box::pin(fut));
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
