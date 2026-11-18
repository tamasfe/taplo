# environment.now

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 224-229)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn now(&self) -> OffsetDateTime {
        let this = JsValue::null();
        let res: JsValue = self.js_now.call0(&this).unwrap();
        let s: String = js_sys::Date::from(res).to_iso_string().into();
        OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339).unwrap()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn now(&self) -> OffsetDateTime {
        let this = JsValue::null();
        let res: JsValue = self.js_now.call0(&this).unwrap();
        let s: String = js_sys::Date::from(res).to_iso_string().into();
        OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339).unwrap()
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
