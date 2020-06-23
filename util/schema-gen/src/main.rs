use schemars::gen::{SchemaGenerator, SchemaSettings};

mod defs;

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
}

fn generate_cargo_schema(out_dir: &str) {
    let gen = SchemaGenerator::new(SchemaSettings::default().with(|s| {
        s.option_add_null_type = false;
    }));

    let schema = gen.into_root_schema_for::<defs::cargo::Manifest>();
}
