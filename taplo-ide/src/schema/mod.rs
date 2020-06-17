use crate::WorldState;
use regex::Regex;
use schemars::schema::RootSchema;

pub(crate) fn register_built_in_schemas(world: &mut WorldState) {
    register_cargo_schema(world);
}

pub(crate) fn register_cargo_schema(world: &mut WorldState) {
    let cargo_schema = serde_json::from_str::<RootSchema>(include_str!("../../schemas/cargo.json")).unwrap();
    let cargo_schema_name = "toml_builtin://Cargo";
    let cargo_re = Regex::new(r#".*Cargo\.toml"#).unwrap();

    world
        .schema_associations
        .insert(cargo_re.into(), cargo_schema_name.into());

    world.schemas.insert(cargo_schema_name.into(), cargo_schema);
}
