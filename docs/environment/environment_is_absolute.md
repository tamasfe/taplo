# environment.is_absolute

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 328-334)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn is_absolute(&self, path: &Path) -> bool {
        let path_str = JsValue::from_str(&path.to_string_lossy());
        let this = JsValue::null();
        let res: JsValue = self.js_is_absolute.call1(&this, &path_str).unwrap();

        res.is_truthy()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn is_absolute(&self, path: &Path) -> bool {
        let path_str = JsValue::from_str(&path.to_string_lossy());
        let this = JsValue::null();
        let res: JsValue = self.js_is_absolute.call1(&this, &path_str).unwrap();

        res.is_truthy()
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
