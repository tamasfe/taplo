# environment.env_vars

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 259-265)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn env_vars(&self) -> Vec<(String, String)> {
        let this = JsValue::null();
        let res: JsValue = self.js_env_vars.call0(&this).unwrap();
        serde_wasm_bindgen::from_value(res)
            .map_err(|err| anyhow!("{err}"))
            .unwrap_or_default()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn env_vars(&self) -> Vec<(String, String)> {
        let this = JsValue::null();
        let res: JsValue = self.js_env_vars.call0(&this).unwrap();
        serde_wasm_bindgen::from_value(res)
            .map_err(|err| anyhow!("{err}"))
            .unwrap_or_default()
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
