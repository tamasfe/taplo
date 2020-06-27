use getopts::Options;
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
};
use std::{env, fs, path::Path};

use gen::{cargo::CargoVisitor, KeysVisitor, RemoveKeysVisitor};

mod defs;
mod gen;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt(
        "o",
        "output",
        "output directory for the generated schemas",
        "DIR",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            return;
        }
    };

    let output_dir = matches.opt_str("o").unwrap();

    generate_cargo_schema(&output_dir);
}

fn generate_cargo_schema(out_dir: &str) {
    let gen = SchemaGenerator::new(
        SchemaSettings::default()
            .with(|s| {
                s.option_add_null_type = false;
            })
            .with_visitor(KeysVisitor::default())
            .with_visitor(CargoVisitor)
            .with_visitor(RemoveKeysVisitor),
    );

    let schema = gen.into_root_schema_for::<defs::cargo::Manifest>();

    let mut f = fs::File::create(&Path::new(out_dir).join("cargo.json")).unwrap();

    serde_json::to_writer_pretty(&mut f, &schema).unwrap();
}
