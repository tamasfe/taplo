# conversion.convert_to_json

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\conversion.rs` (lines 14-37)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn convert_to_json<E: Environment>(
    _context: Context<World<E>>,
    params: Params<ConvertToJsonParams>,
) -> Result<ConvertToJsonResponse, Error> {
    let p = params.required()?;

    if serde_json::from_str::<Value>(&p.text).is_ok() {
        return Ok(ConvertToJsonResponse {
            text: Some(p.text),
            error: None,
        });
    }

    match serde_json::to_string_pretty(&parse(&p.text).into_dom()) {
        Ok(text) => Ok(ConvertToJsonResponse {
            text: Some(text),
            error: None,
        }),
        Err(err) => Ok(ConvertToJsonResponse {
            text: None,
            error: Some(err.to_string()),
        }),
    }
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn convert_to_json<E: Environment>(
    _context: Context<World<E>>,
    params: Params<ConvertToJsonParams>,
) -> Result<ConvertToJsonResponse, Error> {
    let p = params.required()?;

    if serde_json::from_str::<Value>(&p.text).is_ok() {
        return Ok(ConvertToJsonResponse {
            text: Some(p.text),
            error: None,
        });
    }

    match serde_json::to_string_pretty(&parse(&p.text).into_dom()) {
        Ok(text) => Ok(ConvertToJsonResponse {
            text: Some(text),
            error: None,
        }),
        Err(err) => Ok(ConvertToJsonResponse {
            text: None,
            error: Some(err.to_string()),
        }),
    }
}
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
