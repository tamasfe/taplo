use std::process::exit;
use taplo_cli::run;

#[tokio::main]
async fn main() {
    if !run(std::env::args().skip(1)).await {
        exit(1)
    }
}
