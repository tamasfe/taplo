# lib.lint

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lib.rs` (lines 73-140)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn lint(env: JsValue, toml: String, config: JsValue) -> Result<JsValue, JsError> {
    let mut config = if config.is_undefined() {
        Config::default()
    } else {
        serde_wasm_bindgen::from_value(config)?
    };
    let env = WasmEnvironment::from(env);
    config
        .prepare(&env, Path::new("/"))
        .map_err(|err| JsError::new(&err.to_string()))?;

    let syntax = parse(&toml);

    if !syntax.errors.is_empty() {
        return Ok(serde_wasm_bindgen::to_value(&LintResult {
            errors: syntax
                .errors
                .into_iter()
                .map(|err| LintError {
                    range: Range {
                        start: err.range.start().into(),
                        end: err.range.end().into(),
                    }
                    .into(),
                    error: err.to_string(),
                })
                .collect(),
        })?);
    }

    let dom = syntax.into_dom();

    if let Err(errors) = dom.validate() {
        return Ok(serde_wasm_bindgen::to_value(&LintResult {
            errors: errors
                .map(|err| LintError {
                    range: None,
                    error: err.to_string(),
                })
                .collect(),
        })?);
    }

    let schemas = Schemas::new(env, Default::default());
    schemas.associations().add_from_config(&config);

    if let Some(schema) = schemas
        .associations()
        .association_for(&Url::parse("file:///__.toml").unwrap())
    {
        let schema_errors = schemas
            .validate(&schema.url, &serde_json::to_value(&dom).unwrap())
            .await
            .map_err(|err| JsError::new(&err.to_string()))?;

        return Ok(serde_wasm_bindgen::to_value(&LintResult {
            errors: schema_errors
                .into_iter()
                .map(|err| LintError {
                    range: None,
                    error: err.to_string(),
                })
                .collect(),
        })?);
    }

    todo!()
}
```

## Docstring

_No docstring provided._

## Source

```
pub async fn lint(env: JsValue, toml: String, config: JsValue) -> Result<JsValue, JsError> {
    let mut config = if config.is_undefined() {
        Config::default()
    } else {
        serde_wasm_bindgen::from_value(config)?
    };
    let env = WasmEnvironment::from(env);
    config
        .prepare(&env, Path::new("/"))
        .map_err(|err| JsError::new(&err.to_string()))?;

    let syntax = parse(&toml);

    if !syntax.errors.is_empty() {
        return Ok(serde_wasm_bindgen::to_value(&LintResult {
            errors: syntax
                .errors
                .into_iter()
                .map(|err| LintError {
                    range: Range {
                        start: err.range.start().into(),
                        end: err.range.end().into(),
                    }
                    .into(),
                    error: err.to_string(),
                })
                .collect(),
        })?);
    }

    let dom = syntax.into_dom();

    if let Err(errors) = dom.validate() {
        return Ok(serde_wasm_bindgen::to_value(&LintResult {
            errors: errors
                .map(|err| LintError {
                    range: None,
                    error: err.to_string(),
                })
                .collect(),
        })?);
    }

    let schemas = Schemas::new(env, Default::default());
    schemas.associations().add_from_config(&config);

    if let Some(schema) = schemas
        .associations()
        .association_for(&Url::parse("file:///__.toml").unwrap())
    {
        let schema_errors = schemas
            .validate(&schema.url, &serde_json::to_value(&dom).unwrap())
            .await
            .map_err(|err| JsError::new(&err.to_string()))?;

        return Ok(serde_wasm_bindgen::to_value(&LintResult {
            errors: schema_errors
                .into_iter()
                .map(|err| LintError {
                    range: None,
                    error: err.to_string(),
                })
                .collect(),
        })?);
    }

    todo!()
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
