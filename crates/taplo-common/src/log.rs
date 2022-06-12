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
    let span_events = if spans {
        FmtSpan::NEW | FmtSpan::CLOSE
    } else {
        FmtSpan::NONE
    };

    let registry = tracing_subscriber::registry();

    let env_filter = match e.env_var("RUST_LOG") {
        Some(log) => EnvFilter::new(log),
        None => EnvFilter::default().add_directive(tracing::Level::INFO.into()),
    };

    if verbose {
        registry
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(match colors {
                        None => e.atty_stderr(),
                        Some(v) => v,
                    })
                    .with_span_events(span_events)
                    .event_format(tracing_subscriber::fmt::format().pretty().with_ansi(
                        match colors {
                            None => e.atty_stderr(),
                            Some(v) => v,
                        },
                    ))
                    .with_writer(move || BlockingWrite(e.stderr())),
            )
            .try_init()
            .ok();
    } else {
        registry
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(match colors {
                        None => e.atty_stderr(),
                        Some(v) => v,
                    })
                    .event_format(
                        tracing_subscriber::fmt::format()
                            .compact()
                            .with_source_location(false)
                            .with_target(false)
                            .without_time()
                            .with_ansi(match colors {
                                None => e.atty_stderr(),
                                Some(v) => v,
                            }),
                    )
                    .without_time()
                    .with_file(false)
                    .with_line_number(false)
                    .with_span_events(span_events)
                    .with_writer(move || BlockingWrite(e.stderr())),
            )
            .try_init()
            .ok();
    }
}
