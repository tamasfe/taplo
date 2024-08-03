use clap::Parser;
use std::process::exit;
use taplo_cli::{
    args::{Colors, TaploArgs},
    Taplo,
};
use taplo_common::{environment::native::NativeEnvironment, log::setup_stderr_logging};
use tracing::Instrument;

#[tokio::main]
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
