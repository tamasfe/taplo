# lsp.send

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lsp.rs` (lines 19-33)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn send(&self, message: JsValue) -> Result<(), JsError> {
        let message: lsp_async_stub::rpc::Message = serde_wasm_bindgen::from_value(message)?;
        let world = self.world.clone();
        let writer = self.lsp_interface.clone();

        let msg_fut = self.server.handle_message(world, message, writer);

        spawn_local(async move {
            if let Err(err) = msg_fut.await {
                tracing::error!(error = %err, "lsp message error");
            }
        });

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn send(&self, message: JsValue) -> Result<(), JsError> {
        let message: lsp_async_stub::rpc::Message = serde_wasm_bindgen::from_value(message)?;
        let world = self.world.clone();
        let writer = self.lsp_interface.clone();

        let msg_fut = self.server.handle_message(world, message, writer);

        spawn_local(async move {
            if let Err(err) = msg_fut.await {
                tracing::error!(error = %err, "lsp message error");
            }
        });

        Ok(())
    }
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
