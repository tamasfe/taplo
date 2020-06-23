use crate::{analytics::Key, WorldState};
use regex::Regex;
use schemars::{
    schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec},
    Map,
};
use serde::{Deserialize, Serialize};

pub(crate) const EXTENSION_KEY: &'static str = "evenBetterToml";

pub(crate) fn register_built_in_schemas(world: &mut WorldState) {
    register_cargo_schema(world);
}

pub(crate) fn register_cargo_schema(world: &mut WorldState) {
    let cargo_schema =
        serde_json::from_str::<RootSchema>(include_str!("../schemas/cargo.json")).unwrap();
    let cargo_schema_name = "toml_builtin://Cargo";
    let cargo_re = Regex::new(r#".*Cargo\.toml"#).unwrap();

    world
        .schema_associations
        .insert(cargo_re.into(), cargo_schema_name.into());

    world.schemas.insert(cargo_schema_name.into(), cargo_schema);
}

pub(crate) fn get_schema_objects<'s>(
    keys: Vec<Key>,
    schema: &'s RootSchema,
) -> Vec<&'s SchemaObject> {
    get_schema_objects_impl(keys, &schema.definitions, &schema.schema)
}

fn get_schema_objects_impl<'s>(
    mut keys: Vec<Key>,
    defs: &'s Map<String, Schema>,
    mut schema: &'s SchemaObject,
) -> Vec<&'s SchemaObject> {
    if schema.is_ref() {
        schema = match resolve_object_ref(defs, &schema) {
            Some(s) => s,
            None => return Vec::new(),
        }
    }

    if keys.is_empty() {
        let subs = collect_subschemas(defs, schema);
        let mut schemas = vec![schema];
        schemas.extend(subs);

        return schemas;
    } else {
        let mut schemas = Vec::new();
        let subs = collect_subschemas(defs, schema);

        for sub in subs {
            schemas.extend(get_schema_objects_impl(keys.clone(), defs, sub));
        }

        let key = keys.remove(0);

        match key {
            Key::Index(idx) => {
                if let Some(items) = schema.array.as_ref().and_then(|arr| arr.items.as_ref()) {
                    match items {
                        schemars::schema::SingleOrVec::Single(item_schema) => {
                            if let Schema::Object(item_obj) = &**item_schema {
                                schemas.extend(get_schema_objects_impl(keys, defs, item_obj));
                            }
                        }
                        schemars::schema::SingleOrVec::Vec(item_schemas) => {
                            if let Some(item_obj) = item_schemas.get(idx).and_then(|s| match s {
                                Schema::Bool(_) => None,
                                Schema::Object(o) => Some(o),
                            }) {
                                schemas.extend(get_schema_objects_impl(keys, defs, item_obj));
                            }
                        }
                    }
                }
            }
            Key::Property(prop) => {
                if let Some(obj) = &schema.object {
                    for (key, prop_schema) in &obj.properties {
                        if &prop == key {
                            if let Schema::Object(prop_obj) = prop_schema {
                                schemas.extend(get_schema_objects_impl(keys, defs, prop_obj));
                            }
                            return schemas;
                        }
                    }

                    for (pattern, prop_schema) in &obj.pattern_properties {
                        let re = regex::Regex::new(pattern).unwrap();

                        if re.is_match(&prop) {
                            if let Schema::Object(prop_obj) = prop_schema {
                                schemas.extend(get_schema_objects_impl(keys, defs, prop_obj));
                            }
                            return schemas;
                        }
                    }

                    if let Some(additional_schema) = &obj.additional_properties {
                        if let Schema::Object(add_obj) = &**additional_schema {
                            schemas.extend(get_schema_objects_impl(keys, defs, add_obj));
                        }
                    }
                }
            }
        }

        schemas
    }
}

fn collect_subschemas<'s>(
    defs: &'s Map<String, Schema>,
    schema: &'s SchemaObject,
) -> Vec<&'s SchemaObject> {
    let mut schemas = Vec::new();

    if let Some(subs) = &schema.subschemas {
        if let Some(one_ofs) = &subs.one_of {
            for one_of in one_ofs {
                match one_of {
                    Schema::Bool(_) => {}
                    Schema::Object(o) => {
                        if let Some(s) = resolve_object_ref(defs, o) {
                            schemas.push(s)
                        }
                    }
                }
            }
        }

        if let Some(any_ofs) = &subs.any_of {
            for any_of in any_ofs {
                match any_of {
                    Schema::Bool(_) => {}
                    Schema::Object(o) => {
                        if let Some(s) = resolve_object_ref(defs, o) {
                            schemas.push(s)
                        }
                    }
                }
            }
        }

        if let Some(all_ofs) = &subs.all_of {
            for all_of in all_ofs {
                match all_of {
                    Schema::Bool(_) => {}
                    Schema::Object(o) => {
                        if let Some(s) = resolve_object_ref(defs, o) {
                            schemas.push(s)
                        }
                    }
                }
            }
        }
    }

    schemas
}

#[allow(dead_code)]
pub(crate) fn resolve_ref<'s>(
    defs: &'s Map<String, Schema>,
    schema: &'s Schema,
) -> Option<&'s Schema> {
    if !schema.is_ref() {
        return Some(schema);
    }

    match schema {
        Schema::Bool(_) => Some(schema),
        Schema::Object(o) => {
            if let Some(r) = &o.reference {
                local_definition(r)
                    .and_then(|local_def| defs.get(local_def).and_then(|s| resolve_ref(defs, s)))
            } else {
                Some(schema)
            }
        }
    }
}

pub(crate) fn resolve_object_ref<'s>(
    defs: &'s Map<String, Schema>,
    obj: &'s SchemaObject,
) -> Option<&'s SchemaObject> {
    if let Some(r) = &obj.reference {
        local_definition(r).and_then(|local_def| {
            defs.get(local_def).and_then(|s| match s {
                Schema::Bool(_) => None,
                Schema::Object(o) => resolve_object_ref(defs, o),
            })
        })
    } else {
        Some(obj)
    }
}

pub(crate) fn local_definition(rf: &str) -> Option<&str> {
    if rf.starts_with("#/definitions/") {
        Some(rf.trim_start_matches("#/definitions/"))
    } else {
        None
    }
}

pub(crate) fn contains_type(ty: InstanceType, schema: &Schema) -> bool {
    match schema {
        Schema::Bool(b) => *b,
        Schema::Object(obj) => object_contains_type(ty, obj),
    }
}

pub(crate) fn object_contains_type(ty: InstanceType, obj: &SchemaObject) -> bool {
    obj.instance_type
        .as_ref()
        .map(|tys| match tys {
            SingleOrVec::Single(tp) => **tp == ty,
            SingleOrVec::Vec(tps) => tps.contains(&ty),
        })
        .unwrap_or_default()
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExtDocs {
    pub(crate) default_value: Option<String>,
    pub(crate) enum_values: Option<Vec<Option<String>>>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExtMeta {
    pub(crate) link: Option<String>,
    pub(crate) docs: ExtDocs
}
