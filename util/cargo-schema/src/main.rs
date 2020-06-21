use schemars::gen::{SchemaGenerator, SchemaSettings};

mod defs;

fn main() {
    let gen = SchemaGenerator::new(SchemaSettings::default().with(|s| {
        s.option_add_null_type = false;
    }));

    let schema = gen.into_root_schema_for::<defs::Manifest>();

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
