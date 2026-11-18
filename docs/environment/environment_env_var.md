# environment.env_var

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 250-257)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn env_var(&self, name: &str) -> Option<String> {
        let this = JsValue::null();
        let res: JsValue = self
            .js_env_var
            .call1(&this, &JsValue::from_str(name))
            .unwrap();
        res.as_string()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn env_var(&self, name: &str) -> Option<String> {
        let this = JsValue::null();
        let res: JsValue = self
            .js_env_var
            .call1(&this, &JsValue::from_str(name))
            .unwrap();
        res.as_string()
    }
```

## Git History

- Commits: 6
- Authors: 2
- Bug fixes: 3
- Features: 2

## Callers

None

## Callees

None
