# conversion.convert_to_toml

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\conversion.rs` (lines 40-68)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn convert_to_toml<E: Environment>(
    _context: Context<World<E>>,
    params: Params<ConvertToTomlParams>,
) -> Result<ConvertToTomlResponse, Error> {
    let p = params.required()?;

    let parse = parse(&p.text);
    if parse.errors.is_empty() {
        return Ok(ConvertToTomlResponse {
            text: Some(p.text),
            error: None,
        });
    }

    let dom = match serde_json::from_str::<Node>(&p.text) {
        Ok(dom) => dom,
        Err(err) => {
            return Ok(ConvertToTomlResponse {
                text: None,
                error: Some(err.to_string()),
            })
        }
    };

    Ok(ConvertToTomlResponse {
        text: Some(dom.to_toml(false, false)),
        error: None,
    })
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn convert_to_toml<E: Environment>(
    _context: Context<World<E>>,
    params: Params<ConvertToTomlParams>,
) -> Result<ConvertToTomlResponse, Error> {
    let p = params.required()?;

    let parse = parse(&p.text);
    if parse.errors.is_empty() {
        return Ok(ConvertToTomlResponse {
            text: Some(p.text),
            error: None,
        });
    }

    let dom = match serde_json::from_str::<Node>(&p.text) {
        Ok(dom) => dom,
        Err(err) => {
            return Ok(ConvertToTomlResponse {
                text: None,
                error: Some(err.to_string()),
            })
        }
    };

    Ok(ConvertToTomlResponse {
        text: Some(dom.to_toml(false, false)),
        error: None,
    })
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
