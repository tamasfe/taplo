# lib.handle_response

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 335-344)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn handle_response(
        inner: Arc<AsyncMutex<Inner<W>>>,
        response: rpc::Response<serde_json::Value>,
    ) {
        if let Some(sender) = inner.lock().await.requests.remove(&response.id) {
            sender.send(response).ok();
        } else {
            tracing::error!(?response, "unexpected response")
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn handle_response(
        inner: Arc<AsyncMutex<Inner<W>>>,
        response: rpc::Response<serde_json::Value>,
    ) {
        if let Some(sender) = inner.lock().await.requests.remove(&response.id) {
            sender.send(response).ok();
        } else {
            tracing::error!(?response, "unexpected response")
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
