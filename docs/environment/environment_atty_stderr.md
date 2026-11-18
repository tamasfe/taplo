# environment.atty_stderr

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 267-271)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn atty_stderr(&self) -> bool {
        let this = JsValue::null();
        let res: JsValue = self.js_atty_stderr.call0(&this).unwrap();
        res.as_bool().unwrap_or(false)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn atty_stderr(&self) -> bool {
        let this = JsValue::null();
        let res: JsValue = self.js_atty_stderr.call0(&this).unwrap();
        res.as_bool().unwrap_or(false)
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
