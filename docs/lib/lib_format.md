# lib.format

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lib.rs` (lines 37-70)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn format(
    env: JsValue,
    toml: &str,
    options: JsValue,
    config: JsValue,
) -> Result<String, JsError> {
    let mut config = if config.is_undefined() {
        Config::default()
    } else {
        serde_wasm_bindgen::from_value(config)?
    };

    let env = WasmEnvironment::from(env);
    config
        .prepare(&env, Path::new("/"))
        .map_err(|err| JsError::new(&err.to_string()))?;

    let camel_opts: formatter::OptionsIncompleteCamel = serde_wasm_bindgen::from_value(options)?;
    let mut options = formatter::Options::default();
    if let Some(cfg_opts) = config.global_options.formatting.clone() {
        options.update(cfg_opts);
    }
    options.update_camel(camel_opts);

    let syntax = parse(toml);
    let error_ranges = syntax.errors.iter().map(|e| e.range).collect::<Vec<_>>();

    Ok(formatter::format_with_path_scopes(
        syntax.into_dom(),
        options,
        &error_ranges,
        config.format_scopes(Path::new("")),
    )?)
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn format(
    env: JsValue,
    toml: &str,
    options: JsValue,
    config: JsValue,
) -> Result<String, JsError> {
    let mut config = if config.is_undefined() {
        Config::default()
    } else {
        serde_wasm_bindgen::from_value(config)?
    };

    let env = WasmEnvironment::from(env);
    config
        .prepare(&env, Path::new("/"))
        .map_err(|err| JsError::new(&err.to_string()))?;

    let camel_opts: formatter::OptionsIncompleteCamel = serde_wasm_bindgen::from_value(options)?;
    let mut options = formatter::Options::default();
    if let Some(cfg_opts) = config.global_options.formatting.clone() {
        options.update(cfg_opts);
    }
    options.update_camel(camel_opts);

    let syntax = parse(toml);
    let error_ranges = syntax.errors.iter().map(|e| e.range).collect::<Vec<_>>();

    Ok(formatter::format_with_path_scopes(
        syntax.into_dom(),
        options,
        &error_ranges,
        config.format_scopes(Path::new("")),
    )?)
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
