use crate::args::{Colors, TaploArgs};
use std::io;
use taplo_common::environment::Environment;
use tracing_subscriber::{
    fmt::format::FmtSpan, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    EnvFilter,
};

pub fn setup_stderr_logging(e: impl Environment, taplo: &TaploArgs) {
    let span_events = if taplo.log_spans {
        FmtSpan::NEW | FmtSpan::CLOSE
    } else {
        FmtSpan::NONE
    };

    let registry = tracing_subscriber::registry();

    let env_filter = match e.env_var("RUST_LOG") {
        Some(log) => EnvFilter::new(log),
        None => EnvFilter::default().add_directive(tracing::Level::INFO.into()),
    };

    if taplo.verbose {
        registry
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(match taplo.colors {
                        Colors::Auto => e.atty_stderr(),
                        Colors::Always => true,
                        Colors::Never => false,
                    })
                    .with_span_events(span_events)
                    .event_format(tracing_subscriber::fmt::format().pretty().with_ansi(
                        match taplo.colors {
                            Colors::Auto => e.atty_stderr(),
                            Colors::Always => true,
                            Colors::Never => false,
                        },
                    ))
                    .with_writer(io::stderr),
            )
            .init();
    } else {
        registry
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(match taplo.colors {
                        Colors::Auto => e.atty_stderr(),
                        Colors::Always => true,
                        Colors::Never => false,
                    })
                    .event_format(
                        tracing_subscriber::fmt::format()
                            .compact()
                            .with_source_location(false)
                            .with_target(false)
                            .without_time()
                            .with_ansi(match taplo.colors {
                                Colors::Auto => e.atty_stderr(),
                                Colors::Always => true,
                                Colors::Never => false,
                            }),
                    )
                    .without_time()
                    .with_file(false)
                    .with_line_number(false)
                    .with_span_events(span_events)
                    .with_writer(io::stderr),
            )
            .init();
    }
}
