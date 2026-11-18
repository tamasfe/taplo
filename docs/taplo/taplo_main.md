# taplo.main

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\bin\taplo.rs` (lines 11-37)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn main() {
    let cli = TaploArgs::parse();
    setup_stderr_logging(
        NativeEnvironment::new(),
        cli.log_spans,
        cli.verbose,
        match cli.colors {
            Colors::Auto => None,
            Colors::Always => Some(true),
            Colors::Never => Some(false),
        },
    );

    match Taplo::new(NativeEnvironment::new())
        .execute(cli)
        .instrument(tracing::info_span!("taplo"))
        .await
    {
        Ok(_) => {
            exit(0);
        }
        Err(error) => {
            tracing::error!(error = %format!("{error:#}"), "operation failed");
            exit(1);
        }
    }
}
```

## Docstring

_No docstring provided._

## Source

```
async fn main() {
    let cli = TaploArgs::parse();
    setup_stderr_logging(
        NativeEnvironment::new(),
        cli.log_spans,
        cli.verbose,
        match cli.colors {
            Colors::Auto => None,
            Colors::Always => Some(true),
            Colors::Never => Some(false),
        },
    );

    match Taplo::new(NativeEnvironment::new())
        .execute(cli)
        .instrument(tracing::info_span!("taplo"))
        .await
    {
        Ok(_) => {
            exit(0);
        }
        Err(error) => {
            tracing::error!(error = %format!("{error:#}"), "operation failed");
            exit(1);
        }
    }
}
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
