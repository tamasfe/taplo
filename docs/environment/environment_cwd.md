# environment.cwd

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 336-341)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn cwd(&self) -> Option<std::path::PathBuf> {
        let this = JsValue::null();
        let res: JsValue = self.js_cwd.call0(&this).unwrap();

        res.as_string().map(Into::into)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn cwd(&self) -> Option<std::path::PathBuf> {
        let this = JsValue::null();
        let res: JsValue = self.js_cwd.call0(&this).unwrap();

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
