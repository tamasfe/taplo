# lib.handle_message

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 312-328)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn handle_message(
        &self,
        world: W,
        message: rpc::Message,
        writer: impl MessageWriter + Clone + 'static,
    ) -> impl Future<Output = Result<(), io::Error>> {
        let inner = self.inner.clone();

        async move {
            if message.is_response() {
                Server::handle_response(inner, message.into_response()).await;
                Ok(())
            } else {
                Server::handle_request(inner, world, message.into_request(), writer).await
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn handle_message(
        &self,
        world: W,
        message: rpc::Message,
        writer: impl MessageWriter + Clone + 'static,
    ) -> impl Future<Output = Result<(), io::Error>> {
        let inner = self.inner.clone();

        async move {
            if message.is_response() {
                Server::handle_response(inner, message.into_response()).await;
                Ok(())
            } else {
                Server::handle_request(inner, world, message.into_request(), writer).await
            }
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
