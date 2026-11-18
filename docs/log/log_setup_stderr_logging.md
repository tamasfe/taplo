# log.setup_stderr_logging

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\log.rs` (lines 55-103)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn setup_stderr_logging(e: impl Environment, spans: bool, verbose: bool, colors: Option<bool>) {
    let registry = tracing_subscriber::registry();

    let colors = match colors {
        None => e.atty_stderr(),
        Some(v) => v,
    };

    let registry = registry.with(match e.env_var("RUST_LOG") {
        Some(log) => EnvFilter::new(log),
        None => EnvFilter::default().add_directive(tracing::Level::INFO.into()),
    });

    let event_format = tracing_subscriber::fmt::format().pretty().with_ansi(colors);

    let layer = tracing_subscriber::fmt::layer()
        .with_ansi(colors)
        .with_writer(move || BlockingWrite(e.stderr()));

    let layer = if spans {
        layer.with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    } else {
        layer
    };

    if verbose {
        registry
            .with(layer.event_format(event_format))
            .try_init()
            .ok();
    } else {
        registry
            .with(
                layer
                    .event_format(
                        event_format
                            .compact()
                            .with_source_location(false)
                            .with_target(false)
                            .without_time(),
                    )
                    .without_time()
                    .with_file(false)
                    .with_line_number(false),
            )
            .try_init()
            .ok();
    }
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn setup_stderr_logging(e: impl Environment, spans: bool, verbose: bool, colors: Option<bool>) {
    let registry = tracing_subscriber::registry();

    let colors = match colors {
        None => e.atty_stderr(),
        Some(v) => v,
    };

    let registry = registry.with(match e.env_var("RUST_LOG") {
        Some(log) => EnvFilter::new(log),
        None => EnvFilter::default().add_directive(tracing::Level::INFO.into()),
    });

    let event_format = tracing_subscriber::fmt::format().pretty().with_ansi(colors);

    let layer = tracing_subscriber::fmt::layer()
        .with_ansi(colors)
        .with_writer(move || BlockingWrite(e.stderr()));

    let layer = if spans {
        layer.with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    } else {
        layer
    };

    if verbose {
        registry
            .with(layer.event_format(event_format))
            .try_init()
            .ok();
    } else {
        registry
            .with(
                layer
                    .event_format(
                        event_format
                            .compact()
                            .with_source_location(false)
                            .with_target(false)
                            .without_time(),
                    )
                    .without_time()
                    .with_file(false)
                    .with_line_number(false),
            )
            .try_init()
            .ok();
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
