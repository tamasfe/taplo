# handler.handle

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\handler.rs` (lines 167-179)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        _writer: Option<&mut dyn MessageWriter>,
    ) {
        let req = match message.into_params::<N::Params>() {
            Ok(r) => r,
            Err(_) => return,
        };

        (self.f)(context, req.params.into()).await;
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        _writer: Option<&mut dyn MessageWriter>,
    ) {
        let req = match message.into_params::<N::Params>() {
            Ok(r) => r,
            Err(_) => return,
        };

        (self.f)(context, req.params.into()).await;
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
