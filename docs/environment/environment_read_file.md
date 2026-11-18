# environment.read_file

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 294-304)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn read_file(&self, path: &Path) -> Result<Vec<u8>, anyhow::Error> {
        let path_str = JsValue::from_str(&path.to_string_lossy());
        let this = JsValue::null();
        let res: JsValue = self.js_read_file.call1(&this, &path_str).unwrap();

        let ret = JsFuture::from(Promise::from(res))
            .await
            .map_err(|err| anyhow!("{:?}", err))?;

        Ok(Uint8Array::from(ret).to_vec())
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn read_file(&self, path: &Path) -> Result<Vec<u8>, anyhow::Error> {
        let path_str = JsValue::from_str(&path.to_string_lossy());
        let this = JsValue::null();
        let res: JsValue = self.js_read_file.call1(&this, &path_str).unwrap();

        let ret = JsFuture::from(Promise::from(res))
            .await
            .map_err(|err| anyhow!("{:?}", err))?;

        Ok(Uint8Array::from(ret).to_vec())
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
