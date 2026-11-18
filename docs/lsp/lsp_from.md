# lsp.from

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lsp.rs` (lines 42-48)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from(val: JsValue) -> Self {
        Self {
            js_on_message: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_message"))
                .unwrap()
                .into(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from(val: JsValue) -> Self {
        Self {
            js_on_message: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_message"))
                .unwrap()
                .into(),
        }
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
