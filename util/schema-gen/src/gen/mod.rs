use schemars::{
    schema::{Schema, SchemaObject, SingleOrVec},
    visit::Visitor,
    Map,
};

pub mod cargo;

#[derive(Debug, Clone)]
struct KeyVisitor(Vec<String>);

impl KeyVisitor {
    const KEYS_NAME: &'static str = "__taplo_key";
}

impl schemars::visit::Visitor for KeyVisitor {
    fn visit_root_schema(&mut self, _root: &mut schemars::schema::RootSchema) {
        unimplemented!()
    }

    fn visit_schema_object(&mut self, schema: &mut schemars::schema::SchemaObject) {
        schema
            .extensions
            .insert(KeyVisitor::KEYS_NAME.into(), self.0.clone().into());
        // We don't go further on purpose
    }
}

#[derive(Debug, Default, Clone)]
pub struct KeysVisitor(Vec<String>);

impl schemars::visit::Visitor for KeysVisitor {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        let mut keys = self.0.clone();

        if let Some(s) = schema.metadata.as_ref().and_then(|m| m.title.clone()) {
            keys.push(s)
        }

        KeyVisitor(keys.clone()).visit_schema_object(schema);
        visit_schema_object(&mut KeyVisitor(keys.clone()), schema);

        if let Some(obj) = &mut schema.object {
            for (k, s) in &mut obj.properties {
                let mut new_keys = keys.clone();
                new_keys.push(k.clone());

                schemars::visit::visit_schema(&mut KeysVisitor(new_keys), s);
            }
        }
    }
}

// Filtered visit
pub fn visit_schema_object<V: schemars::visit::Visitor + ?Sized>(
    v: &mut V,
    schema: &mut SchemaObject,
) {
    if let Some(sub) = &mut schema.subschemas {
        visit_vec(v, &mut sub.all_of);
        visit_vec(v, &mut sub.any_of);
        visit_vec(v, &mut sub.one_of);
        visit_box(v, &mut sub.not);
        visit_box(v, &mut sub.if_schema);
        visit_box(v, &mut sub.then_schema);
        visit_box(v, &mut sub.else_schema);
    }

    if let Some(arr) = &mut schema.array {
        visit_single_or_vec(v, &mut arr.items);
        visit_box(v, &mut arr.additional_items);
        visit_box(v, &mut arr.contains);
    }

    if let Some(obj) = &mut schema.object {
        visit_map_values(v, &mut obj.properties);
        visit_map_values(v, &mut obj.pattern_properties);
        // visit_box(v, &mut obj.additional_properties);
        visit_box(v, &mut obj.property_names);
    }
}

fn visit_box<V: Visitor + ?Sized>(v: &mut V, target: &mut Option<Box<Schema>>) {
    if let Some(s) = target {
        v.visit_schema(s)
    }
}

fn visit_vec<V: Visitor + ?Sized>(v: &mut V, target: &mut Option<Vec<Schema>>) {
    if let Some(vec) = target {
        for s in vec {
            v.visit_schema(s)
        }
    }
}

fn visit_map_values<V: Visitor + ?Sized>(v: &mut V, target: &mut Map<String, Schema>) {
    for s in target.values_mut() {
        v.visit_schema(s)
    }
}

fn visit_single_or_vec<V: Visitor + ?Sized>(v: &mut V, target: &mut Option<SingleOrVec<Schema>>) {
    match target {
        None => {}
        Some(SingleOrVec::Single(s)) => v.visit_schema(s),
        Some(SingleOrVec::Vec(vec)) => {
            for s in vec {
                v.visit_schema(s)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RemoveKeysVisitor;

impl schemars::visit::Visitor for RemoveKeysVisitor {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        schema.extensions.remove(KeyVisitor::KEYS_NAME);

        schemars::visit::visit_schema_object(self, schema)
    }
}
