use std::io::{self, Write};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*, util::SubscriberInitExt, EnvFilter};

use crate::environment::Environment;

struct BlockingWrite<W: AsyncWrite>(W);

impl<W: AsyncWrite + Unpin> Write for BlockingWrite<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            futures::executor::block_on(self.0.write(buf))
        }

        // On WASM we cannot do blocking writes without blocking
        // the event loop, so we simply do not wait.
        #[cfg(target_arch = "wasm32")]
        {
            use futures::FutureExt;

            let _ = self.0.write_all(buf).boxed_local().poll_unpin(
                &mut futures::task::Context::from_waker(&futures::task::noop_waker()),
            );

            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            futures::executor::block_on(self.0.flush())
        }

        // On WASM we cannot do blocking writes without blocking
        // the event loop, so we simply do not wait.
        #[cfg(target_arch = "wasm32")]
        {
            use futures::FutureExt;

            let _ =
                self.0
                    .flush()
                    .boxed_local()
                    .poll_unpin(&mut futures::task::Context::from_waker(
                        &futures::task::noop_waker(),
                    ));

            Ok(())
        }
    }
}

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
