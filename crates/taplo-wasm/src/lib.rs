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

#[wasm_bindgen]
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

#[wasm_bindgen]
pub fn from_json(json: &str) -> Result<String, JsError> {
    let dom: Node = serde_json::from_str(json)?;
    Ok(dom.to_toml(false, false))
}

#[cfg(feature = "cli")]
#[wasm_bindgen]
pub async fn run_cli(env: JsValue, args: JsValue) -> Result<(), JsError> {
    use clap::Parser;
    use environment::WasmEnvironment;
    use taplo_cli::{
        args::{Colors, TaploArgs},
        Taplo,
    };
    use taplo_common::{environment::Environment, log::setup_stderr_logging};
    use tokio::io::AsyncWriteExt;
    use tracing::Instrument;

    let env = WasmEnvironment::from(env);
    let args: Vec<String> = serde_wasm_bindgen::from_value(args)?;

    let cli = match TaploArgs::try_parse_from(args) {
        Ok(v) => v,
        Err(error) => {
            env.stdout().write_all(error.to_string().as_bytes()).await?;
            return Err(JsError::new("operation failed"));
        }
    };

    setup_stderr_logging(
        env.clone(),
        cli.log_spans,
        cli.verbose,
        match cli.colors {
            Colors::Auto => None,
            Colors::Always => Some(true),
            Colors::Never => Some(false),
        },
    );

    match Taplo::new(env.clone())
        .execute(cli)
        .instrument(tracing::info_span!("taplo"))
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => {
            tracing::error!(error = %format!("{error:#}"), "operation failed");
            Err(JsError::new("operation failed"))
        }
    }
}

#[cfg(feature = "lsp")]
#[wasm_bindgen]
pub fn create_lsp(env: JsValue, lsp_interface: JsValue) -> lsp::TaploWasmLsp {
    
    use taplo_common::log::setup_stderr_logging;

    let env = WasmEnvironment::from(env);

    // for (key, value) in env.env_vars() {
    //     std::env::set_var(key, value);
    // }

    setup_stderr_logging(env.clone(), false, false, None);

    lsp::TaploWasmLsp {
        server: taplo_lsp::create_server(),
        world: taplo_lsp::create_world(env),
        lsp_interface: lsp::WasmLspInterface::from(lsp_interface),
    }
}
