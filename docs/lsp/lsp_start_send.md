# lsp.start_send

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lsp.rs` (lines 61-70)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn start_send(
        self: std::pin::Pin<&mut Self>,
        message: rpc::Message,
    ) -> Result<(), Self::Error> {
        let this = JsValue::null();
        self.js_on_message
            .call1(&this, &serde_wasm_bindgen::to_value(&message).unwrap())
            .unwrap();
        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn start_send(
        self: std::pin::Pin<&mut Self>,
        message: rpc::Message,
    ) -> Result<(), Self::Error> {
        let this = JsValue::null();
        self.js_on_message
            .call1(&this, &serde_wasm_bindgen::to_value(&message).unwrap())
            .unwrap();
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
