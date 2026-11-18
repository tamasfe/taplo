# environment.to_file_path

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 320-326)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn to_file_path(&self, url: &Url) -> Option<std::path::PathBuf> {
        let url_str = JsValue::from_str(url.as_str());
        let this = JsValue::null();
        let res: JsValue = self.js_to_file_path.call1(&this, &url_str).unwrap();

        res.as_string().map(Into::into)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn to_file_path(&self, url: &Url) -> Option<std::path::PathBuf> {
        let url_str = JsValue::from_str(url.as_str());
        let this = JsValue::null();
        let res: JsValue = self.js_to_file_path.call1(&this, &url_str).unwrap();

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
