use once_cell::sync::Lazy;
use schemars::schema::RootSchema;
use std::collections::HashMap;

/// The key of the schema extension property.
pub const EXTENSION_KEY: &str = "evenBetterToml";

/// The scheme of the built-in schemas.
pub const BUILTIN_SCHEME: &str = "taplo";

pub static BUILTIN_SCHEMAS: Lazy<HashMap<String, RootSchema>> = Lazy::new(|| {
    let mut schemas = HashMap::new();
    
    schemas.insert(
        format!("{}://cargo@Cargo.toml", BUILTIN_SCHEME),
        serde_json::from_str(include_str!("../schemas/Cargo.json")).unwrap(),
    );
    schemas.insert(
        format!("{}://python@pyproject.toml", BUILTIN_SCHEME),
        serde_json::from_str(include_str!("../schemas/pyproject.json")).unwrap(),
    );
    schemas.insert(
        format!("{}://taplo@taplo.toml", BUILTIN_SCHEME),
        serde_json::from_str(include_str!("../schemas/taplo.json")).unwrap(),
    );

    schemas
});

pub static REGEX_ASSOCIATIONS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut associations = HashMap::new();
   
    associations.insert(".*/Cargo\\.toml".to_string(), "taplo://cargo@Cargo.toml".to_string());
    associations.insert(".*/pyproject\\.toml".to_string(), "taplo://python@pyproject.toml".to_string());
    associations.insert(".*/\\.?taplo\\.toml".to_string(), "taplo://taplo@taplo.toml".to_string());

    associations
});

pub mod util {
    use schemars::{
        schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec},
        Map,
    };
    use serde_crate::{Deserialize, Serialize};
    use smallvec::{smallvec, SmallVec};

    use crate::dom::{self, PathKey};

    use super::EXTENSION_KEY;

    #[derive(Debug, Clone)]
    pub struct ExtendedSchema<'s> {
        pub schema: &'s SchemaObject,
        pub ext: ExtMeta,
    }

    impl<'s> ExtendedSchema<'s> {
        pub fn is(&self, ty: InstanceType) -> bool {
            match &self.schema.instance_type {
                Some(t) => match t {
                    SingleOrVec::Single(s) => **s == ty,
                    SingleOrVec::Vec(s) => s.iter().any(|s| *s == ty),
                },
                None => ty == InstanceType::Object,
            }
        }

        pub fn is_hidden(&self) -> bool {
            self.ext.hidden.unwrap_or(false)
        }

        pub fn is_array_of_objects(&self, defs: &'s Map<String, Schema>) -> bool {
            self.schema
                .array
                .as_ref()
                .and_then(|arr| {
                    arr.items.as_ref().and_then(|items| match items {
                        SingleOrVec::Single(s) => {
                            ExtendedSchema::resolved(defs, &*s).map(|s| s.is(InstanceType::Object))
                        }
                        SingleOrVec::Vec(_) => Some(false),
                    })
                })
                .unwrap_or(false)
        }

        pub fn resolved_object(defs: &'s Map<String, Schema>, schema: &'s SchemaObject) -> Self {
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

        pub fn resolved(defs: &'s Map<String, Schema>, schema: &'s Schema) -> Option<Self> {
            match schema {
                Schema::Bool(_) => None,
                Schema::Object(o) => Some(Self::resolved_object(defs, o)),
            }
        }

        /// Collect all descendants with their paths relative to this schema,
        /// and an indicator whether the schema (property) is required.
        ///
        /// This doesn't include the schema itself.
        pub fn descendants(
            &self,
            defs: &'s Map<String, Schema>,
            max_depth: usize,
        ) -> Vec<(dom::Path, ExtendedSchema<'s>, bool)> {
            let mut schemas = Vec::new();

            self.collect_descendants(defs, dom::Path::new(), 0, max_depth, &mut schemas);

            schemas
        }

        fn collect_descendants(
            &self,
            defs: &'s Map<String, Schema>,
            path: dom::Path,
            depth: usize,
            max_depth: usize,
            schemas: &mut Vec<(dom::Path, ExtendedSchema<'s>, bool)>,
        ) {
            if depth == max_depth {
                return;
            }

            if let Some(o) = &self.schema.object {
                for (key, schema) in &o.properties {
                    if let Some(schema) = ExtendedSchema::resolved(defs, schema) {
                        let p = path.join(key);
                        schema.collect_descendants(defs, p.clone(), depth + 1, max_depth, schemas);
                        schemas.push((p, schema, o.required.contains(key)));
                    }
                }
            }

            for sub in collect_subschemas(defs, self.clone()) {
                sub.collect_descendants(defs, path.clone(), depth, max_depth, schemas)
            }
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

    pub fn get_schema_objects<'s>(
        path: dom::Path,
        schema: &'s RootSchema,
        subschemas: bool,
    ) -> SmallVec<[ExtendedSchema<'s>; 10]> {
        get_schema_objects_impl(
            path,
            &schema.definitions,
            (&schema.schema).into(),
            subschemas,
        )
    }

    pub fn get_ext(schema: &SchemaObject) -> ExtMeta {
        schema
            .extensions
            .get(EXTENSION_KEY)
            .and_then(|v| serde_json::from_value::<ExtMeta>(v.clone()).ok())
            .unwrap_or_default()
    }

    fn get_schema_objects_impl<'s>(
        path: dom::Path,
        defs: &'s Map<String, Schema>,
        mut schema: ExtendedSchema<'s>,
        subschemas: bool,
    ) -> SmallVec<[ExtendedSchema<'s>; 10]> {
        if schema.schema.is_ref() {
            schema = match resolve_object_ref(defs, schema) {
                Some(s) => s,
                None => return SmallVec::new(),
            }
        }

        if path.is_empty() {
            let mut schemas = smallvec![schema.clone()];

            if subschemas {
                let subs = collect_subschemas(defs, schema);
                schemas.extend(subs);
            }

            schemas
        } else {
            let mut schemas = SmallVec::new();

            let subs = collect_subschemas(defs, schema.clone());

            for sub in subs {
                schemas.extend(get_schema_objects_impl(path.clone(), defs, sub, subschemas));
            }

            let key = path.keys().next().unwrap();

            match key {
                PathKey::Index(idx) => {
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
                                        path.skip_left(1),
                                        defs,
                                        item_obj.into(),
                                        subschemas,
                                    ));
                                }
                            }
                            schemars::schema::SingleOrVec::Vec(item_schemas) => {
                                if let Some(item_obj) = item_schemas.get(*idx).and_then(|s| match s
                                {
                                    Schema::Bool(_) => None,
                                    Schema::Object(o) => Some(o),
                                }) {
                                    schemas.extend(get_schema_objects_impl(
                                        path.skip_left(1),
                                        defs,
                                        item_obj.into(),
                                        subschemas,
                                    ));
                                }
                            }
                        }
                    }
                }
                PathKey::Key(key) => {
                    if let Some(obj) = &schema.schema.object {
                        for (property, prop_schema) in &obj.properties {
                            if key == property {
                                if let Schema::Object(prop_obj) = prop_schema {
                                    schemas.extend(get_schema_objects_impl(
                                        path.skip_left(1),
                                        defs,
                                        prop_obj.into(),
                                        subschemas,
                                    ));
                                }
                                return schemas;
                            }
                        }

                        for (pattern, prop_schema) in &obj.pattern_properties {
                            let re = regex::Regex::new(pattern).unwrap();

                            if re.is_match(key) {
                                if let Schema::Object(prop_obj) = prop_schema {
                                    schemas.extend(get_schema_objects_impl(
                                        path.skip_left(1),
                                        defs,
                                        prop_obj.into(),
                                        subschemas,
                                    ));
                                }
                                return schemas;
                            }
                        }

                        if let Some(additional_schema) = &obj.additional_properties {
                            if let Schema::Object(add_obj) = &**additional_schema {
                                schemas.extend(get_schema_objects_impl(
                                    path.skip_left(1),
                                    defs,
                                    add_obj.into(),
                                    subschemas,
                                ));
                            }
                        }
                    }
                }
            }

            schemas
        }
    }

    pub fn collect_subschemas<'s>(
        defs: &'s Map<String, Schema>,
        schema: ExtendedSchema<'s>,
    ) -> SmallVec<[ExtendedSchema<'s>; 10]> {
        let mut schemas = SmallVec::new();

        if let Some(subs) = &schema.schema.subschemas {
            if let Some(one_ofs) = &subs.one_of {
                for one_of in one_ofs {
                    if let Some(s) = ExtendedSchema::resolved(defs, one_of) {
                        schemas.push(s);
                    }
                }
            }

            if let Some(any_ofs) = &subs.any_of {
                for any_of in any_ofs {
                    if let Some(s) = ExtendedSchema::resolved(defs, any_of) {
                        schemas.push(s);
                    }
                }
            }

            if let Some(all_ofs) = &subs.all_of {
                for all_of in all_ofs {
                    if let Some(s) = ExtendedSchema::resolved(defs, all_of) {
                        schemas.push(s);
                    }
                }
            }
        }

        schemas
    }

    pub fn resolve_ref<'s>(
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
                    local_definition(r).and_then(|local_def| {
                        defs.get(local_def).and_then(|s| resolve_ref(defs, s))
                    })
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
    #[serde(crate = "serde_crate")]
    pub struct ExtDocs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub main: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub const_value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enum_values: Option<Vec<Option<String>>>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    #[serde(rename_all = "camelCase")]
    #[serde(crate = "serde_crate")]
    pub struct ExtLinks {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enum_values: Option<Vec<Option<String>>>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    #[serde(rename_all = "camelCase")]
    #[serde(crate = "serde_crate")]
    pub struct ExtMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hidden: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub links: Option<ExtLinks>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub docs: Option<ExtDocs>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub init_keys: Option<Vec<String>>,
    }
}
