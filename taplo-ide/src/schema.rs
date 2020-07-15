use crate::{analytics::Key, WorldState};
use schemars::{
    schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec},
    Map,
};
use serde::{Deserialize, Serialize};

pub const EXTENSION_KEY: &str = "evenBetterToml";
pub const BUILTIN_SCHEME: &str = "toml_builtin";

pub fn register_built_in_schemas(world: &mut WorldState) {
    register_cargo_schema(world);
}

pub fn register_cargo_schema(world: &mut WorldState) {
    let cargo_schema =
        serde_json::from_str::<RootSchema>(include_str!("../schemas/cargo.json")).unwrap();
    let cargo_schema_name = format!("{}://cargo", BUILTIN_SCHEME);

    // This is supplied from the configuration.
    // let cargo_re = Regex::new(r#".*Cargo\.toml"#).unwrap();
    // world
    //     .schema_associations
    //     .insert(cargo_re.into(), cargo_schema_name.clone());

    world.schemas.insert(cargo_schema_name, cargo_schema);
}

#[derive(Debug, Clone)]
pub struct ExtendedSchema<'s> {
    pub schema: &'s SchemaObject,
    pub ext: ExtMeta,
}

impl<'s> ExtendedSchema<'s> {
    pub fn resolved(defs: &'s Map<String, Schema>, schema: &'s SchemaObject) -> Self {
        let mut s = ExtendedSchema {
            ext: get_ext(schema),
            schema,
        };

        if s.schema.is_ref() {
            if let Some(resolved_s) = resolve_object_ref(defs, s.clone()) {
                s = resolved_s;
            }
        }

        if s.ext == ExtMeta::default() {
            s.ext = get_ext(s.schema);
        }

        s
    }
}

impl<'s> From<&'s SchemaObject> for ExtendedSchema<'s> {
    fn from(schema: &'s SchemaObject) -> Self {
        ExtendedSchema {
            ext: get_ext(schema),
            schema,
        }
    }
}

pub fn get_schema_objects<'s>(keys: Vec<Key>, schema: &'s RootSchema) -> Vec<ExtendedSchema<'s>> {
    get_schema_objects_impl(keys, &schema.definitions, (&schema.schema).into())
}

pub(crate) fn get_ext(schema: &SchemaObject) -> ExtMeta {
    schema
        .extensions
        .get(EXTENSION_KEY)
        .and_then(|v| {
            serde_json::from_value::<ExtMeta>(v.clone())
                .map_err(|e| {
                    log_error!("invalid schema extension: {}", e);
                })
                .ok()
        })
        .unwrap_or_default()
}

fn get_schema_objects_impl<'s>(
    mut keys: Vec<Key>,
    defs: &'s Map<String, Schema>,
    mut schema: ExtendedSchema<'s>,
) -> Vec<ExtendedSchema<'s>> {
    if schema.schema.is_ref() {
        schema = match resolve_object_ref(defs, schema) {
            Some(s) => s,
            None => return Vec::new(),
        }
    }

    if keys.is_empty() {
        let subs = collect_subschemas(defs, schema.clone());
        let mut schemas = vec![schema];
        schemas.extend(subs);

        schemas
    } else {
        let mut schemas = Vec::new();
        let subs = collect_subschemas(defs, schema.clone());

        for sub in subs {
            schemas.extend(get_schema_objects_impl(keys.clone(), defs, sub));
        }

        let key = keys.remove(0);

        match key {
            Key::Index(idx) => {
                if let Some(items) = schema
                    .schema
                    .array
                    .as_ref()
                    .and_then(|arr| arr.items.as_ref())
                {
                    match items {
                        schemars::schema::SingleOrVec::Single(item_schema) => {
                            if let Schema::Object(item_obj) = &**item_schema {
                                schemas.extend(get_schema_objects_impl(
                                    keys,
                                    defs,
                                    item_obj.into(),
                                ));
                            }
                        }
                        schemars::schema::SingleOrVec::Vec(item_schemas) => {
                            if let Some(item_obj) = item_schemas.get(idx).and_then(|s| match s {
                                Schema::Bool(_) => None,
                                Schema::Object(o) => Some(o),
                            }) {
                                schemas.extend(get_schema_objects_impl(
                                    keys,
                                    defs,
                                    item_obj.into(),
                                ));
                            }
                        }
                    }
                }
            }
            Key::Property(prop) => {
                if let Some(obj) = &schema.schema.object {
                    for (key, prop_schema) in &obj.properties {
                        if &prop == key {
                            if let Schema::Object(prop_obj) = prop_schema {
                                schemas.extend(get_schema_objects_impl(
                                    keys,
                                    defs,
                                    prop_obj.into(),
                                ));
                            }
                            return schemas;
                        }
                    }

                    for (pattern, prop_schema) in &obj.pattern_properties {
                        let re = regex::Regex::new(pattern).unwrap();

                        if re.is_match(&prop) {
                            if let Schema::Object(prop_obj) = prop_schema {
                                schemas.extend(get_schema_objects_impl(
                                    keys,
                                    defs,
                                    prop_obj.into(),
                                ));
                            }
                            return schemas;
                        }
                    }

                    if let Some(additional_schema) = &obj.additional_properties {
                        if let Schema::Object(add_obj) = &**additional_schema {
                            schemas.extend(get_schema_objects_impl(keys, defs, add_obj.into()));
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
    schema: ExtendedSchema<'s>,
) -> Vec<ExtendedSchema<'s>> {
    let mut schemas: Vec<ExtendedSchema> = Vec::new();
    let ext = schema.ext;

    if let Some(subs) = &schema.schema.subschemas {
        if let Some(one_ofs) = &subs.one_of {
            for one_of in one_ofs {
                match one_of {
                    Schema::Bool(_) => {}
                    Schema::Object(o) => {
                        if let Some(s) = resolve_object_ref(defs, o.into()) {
                            let mut s_ext: ExtendedSchema = s;
                            if ext != ExtMeta::default() {
                                s_ext.ext = ext.clone();
                            }
                            schemas.push(s_ext)
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
                        if let Some(s) = resolve_object_ref(defs, o.into()) {
                            let mut s_ext: ExtendedSchema = s;
                            if ext != ExtMeta::default() {
                                s_ext.ext = ext.clone();
                            }
                            schemas.push(s_ext)
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
                        if let Some(s) = resolve_object_ref(defs, o.into()) {
                            let mut s_ext: ExtendedSchema = s;
                            if ext != ExtMeta::default() {
                                s_ext.ext = ext.clone();
                            }
                            schemas.push(s_ext)
                        }
                    }
                }
            }
        }
    }

    schemas
}

pub fn resolve_ref<'s>(defs: &'s Map<String, Schema>, schema: &'s Schema) -> Option<&'s Schema> {
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

pub fn resolve_object_ref<'s>(
    defs: &'s Map<String, Schema>,
    obj: ExtendedSchema<'s>,
) -> Option<ExtendedSchema<'s>> {
    if let Some(r) = &obj.schema.reference {
        local_definition(r).and_then(|local_def| {
            defs.get(local_def).and_then(|s| match s {
                Schema::Bool(_) => None,
                Schema::Object(o) => {
                    let ext = obj.ext;
                    resolve_object_ref(defs, o.into()).map(|mut s| {
                        if ext != ExtMeta::default() {
                            s.ext = ext.clone();
                        }
                        s
                    })
                }
            })
        })
    } else {
        Some(obj)
    }
}

pub fn local_definition(rf: &str) -> Option<&str> {
    if rf.starts_with("#/definitions/") {
        Some(rf.trim_start_matches("#/definitions/"))
    } else {
        None
    }
}

pub fn contains_type(ty: InstanceType, schema: &Schema) -> bool {
    match schema {
        Schema::Bool(b) => *b,
        Schema::Object(obj) => object_contains_type(ty, obj),
    }
}

pub fn object_contains_type(ty: InstanceType, obj: &SchemaObject) -> bool {
    obj.instance_type
        .as_ref()
        .map(|tys| match tys {
            SingleOrVec::Single(tp) => **tp == ty,
            SingleOrVec::Vec(tps) => tps.contains(&ty),
        })
        .unwrap_or_default()
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExtDocs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<Option<String>>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExtLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<Option<String>>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExtMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<ExtLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<ExtDocs>,
}
