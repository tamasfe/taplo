# environment.glob_files

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 285-292)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn glob_files(&self, glob: &str) -> Result<Vec<std::path::PathBuf>, anyhow::Error> {
        let this = JsValue::null();
        let res: JsValue = self
            .js_glob_files
            .call1(&this, &JsValue::from_str(glob))
            .unwrap();
        serde_wasm_bindgen::from_value(res).map_err(|err| anyhow!("{err}"))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn glob_files(&self, glob: &str) -> Result<Vec<std::path::PathBuf>, anyhow::Error> {
        let this = JsValue::null();
        let res: JsValue = self
            .js_glob_files
            .call1(&this, &JsValue::from_str(glob))
            .unwrap();
        serde_wasm_bindgen::from_value(res).map_err(|err| anyhow!("{err}"))
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
