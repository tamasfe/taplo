#![allow(dead_code)]
#![allow(unused_variables)]

use std::process::exit;

use taplo_cli::run;

fn main() {
    if !run(std::env::args()) {
        exit(1)
    }
}
