# lib.to_json

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lib.rs` (lines 143-157)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn to_json(toml: &str) -> Result<String, JsError> {
    let syntax = parse(toml);

    if !syntax.errors.is_empty() {
        return Err(JsError::new("the given input contains syntax errors"));
    }

    let dom = syntax.into_dom();

    if dom.validate().is_err() {
        return Err(JsError::new("the given input contains errors"));
    }

    Ok(serde_json::to_string(&dom).unwrap())
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn to_json(toml: &str) -> Result<String, JsError> {
    let syntax = parse(toml);

    if !syntax.errors.is_empty() {
        return Err(JsError::new("the given input contains syntax errors"));
    }

    let dom = syntax.into_dom();

    if dom.validate().is_err() {
        return Err(JsError::new("the given input contains errors"));
    }

    Ok(serde_json::to_string(&dom).unwrap())
}
```

## Git History

- Commits: 8
- Authors: 2
- Bug fixes: 5
- Features: 2

## Callers

None

## Callees

None
