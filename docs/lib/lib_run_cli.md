# lib.run_cli

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lib.rs` (lines 167-211)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
