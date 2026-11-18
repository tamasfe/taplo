# Module: lib

**File:** `\crates\taplo-wasm\src\lib.rs`

## Overview

Module `lib` contains 38 analyzed entities.

```
use environment::WasmEnvironment;
use serde::Serialize;
use std::path::Path;
use taplo::{dom::Node, formatter, parser::parse};
use taplo_common::{config::Config, schema::Schemas};
use url::Url;
use wasm_bindgen::prelude::*;

mod environment;
#[cfg(feature = "lsp")]
mod lsp;

#[derive(Serialize)]
struct Range {
    start: u32,
    end: u32,
}

#[derive(Serialize)]
struct LintError {
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Range>,
    error: String,
}

#[derive(Serialize)]
struct LintResult {
    errors: Vec<LintError>,
}

#[wasm_bindgen]
pub fn initialize() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
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

#[wasm_bindgen]
pub async fn lint(env: JsValue, toml: String, config: JsValue) -> Result<JsValue, JsError> {
    let mut config = if config.is_undefined() {
        Config::default()
    } else {
        serde_wasm_bindgen::from_value(config)?
    };
    let env = WasmEnvironment::from(env);
    config
        .prepare(&env, Path::new("/"))
   
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `lib.handle_request` | function | No description |
| `lib.poll` | function | No description |
| `lib.is_terminated` | function | No description |
| `lib.deref` | function | No description |
| `lib.is_initialized` | function | No description |
| `lib.from` | function | No description |
| `lib.as_err` | function | No description |
| `lib.write_notification` | function | No description |
| `lib.on_notification` | function | No description |
| `lib.task_done` | function | No description |
| `lib.is_shutting_down` | function | No description |
| `lib.write_request` | function | No description |
| `lib.handle_message` | function | No description |
| `lib.new` | function | No description |
| `lib.token` | function | No description |
| `lib.cancel` | function | No description |
| `lib.required` | function | No description |
| `lib.defer` | function | No description |
| `lib.cancel_token` | function | No description |
| `lib.optional` | function | No description |

## Git History

- Commits: 8
- Authors: 2
- Bug fixes: 5
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
