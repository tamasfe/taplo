use clap::StructOpt;
use std::process::exit;
use taplo_cli::{args::TaploArgs, log::setup_stderr_logging, Taplo};
use taplo_common::environment::native::NativeEnvironment;
use tracing::Instrument;

#[tokio::main]
async fn main() {
    let cli = TaploArgs::parse();
    setup_stderr_logging(NativeEnvironment, &cli);

    match Taplo::new(NativeEnvironment)
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
