# environment.find_config_file

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 343-353)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn find_config_file(&self, from: &Path) -> Option<std::path::PathBuf> {
        let path_str = JsValue::from_str(&from.to_string_lossy());
        let this = JsValue::null();
        let res: JsValue = self.js_find_config_file.call1(&this, &path_str).unwrap();

        if res.is_undefined() {
            return None;
        }

        res.as_string().map(Into::into)
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn find_config_file(&self, from: &Path) -> Option<std::path::PathBuf> {
        let path_str = JsValue::from_str(&from.to_string_lossy());
        let this = JsValue::null();
        let res: JsValue = self.js_find_config_file.call1(&this, &path_str).unwrap();

        if res.is_undefined() {
            return None;
        }

        res.as_string().map(Into::into)
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
