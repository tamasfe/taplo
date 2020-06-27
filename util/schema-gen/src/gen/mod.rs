use schemars::schema::SchemaObject;

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
        schemars::visit::visit_schema_object(&mut KeyVisitor(keys.clone()), schema);

        if let Some(obj) = &mut schema.object {
            for (k, s) in &mut obj.properties {
                let mut new_keys = keys.clone();
                new_keys.push(k.clone());

                schemars::visit::visit_schema(&mut KeysVisitor(new_keys), s);
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
