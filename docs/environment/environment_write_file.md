# environment.write_file

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 306-318)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn write_file(&self, path: &Path, bytes: &[u8]) -> Result<(), anyhow::Error> {
        let path_str = JsValue::from_str(&path.to_string_lossy());
        let this = JsValue::null();
        let res: JsValue = self
            .js_write_file
            .call2(&this, &path_str, &JsValue::from(Uint8Array::from(bytes)))
            .unwrap();
        let future = JsFuture::from(Promise::from(res))
            .await
            .map_err(|err| anyhow!("{:?}", err))?;

        Ok(serde_wasm_bindgen::from_value(future).map_err(|err| anyhow!("{err}"))?)
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn write_file(&self, path: &Path, bytes: &[u8]) -> Result<(), anyhow::Error> {
        let path_str = JsValue::from_str(&path.to_string_lossy());
        let this = JsValue::null();
        let res: JsValue = self
            .js_write_file
            .call2(&this, &path_str, &JsValue::from(Uint8Array::from(bytes)))
            .unwrap();
        let future = JsFuture::from(Promise::from(res))
            .await
            .map_err(|err| anyhow!("{:?}", err))?;

        Ok(serde_wasm_bindgen::from_value(future).map_err(|err| anyhow!("{err}"))?)
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
