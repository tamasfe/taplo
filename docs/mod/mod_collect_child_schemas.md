# mod.collect_child_schemas

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 496-599)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn collect_child_schemas(
        &self,
        root_url: &Url,
        schema: &Value,
        root_path: &Keys,
        path: &Keys,
        mut depth: usize,
        schemas: &mut Vec<(Keys, Keys, Arc<Value>)>,
    ) {
        if !schema.is_object() || depth == 0 {
            return;
        }

        if let Some(schema) = self.ref_schema_value(root_url, schema).await {
            return self
                .collect_child_schemas(root_url, &schema, root_path, path, depth, schemas)
                .await;
        }

        if let Some(one_ofs) = schema["oneOf"].as_array() {
            for one_of in one_ofs {
                self.collect_child_schemas(root_url, one_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        if let Some(any_ofs) = schema["anyOf"].as_array() {
            for any_of in any_ofs {
                self.collect_child_schemas(root_url, any_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        // Deal with the { "description": "Foo", "allOf": [{ "$ref": "Bar" }] }
        // pattern.
        let composed = [
            !schema["allOf"].is_null(),
            !schema["oneOf"].is_null(),
            !schema["anyOf"].is_null(),
        ]
        .into_iter()
        .filter(|b| *b)
        .count()
            == 1
            && schema["properties"].is_null();

        if let Some(all_ofs) = schema["allOf"].as_array() {
            if !all_ofs.is_empty() && composed {
                let mut schema = schema.clone();
                if let Some(obj) = schema["allOf"].as_object_mut() {
                    obj.remove("allOf");
                }

                let mut merged_all_of = Value::Object(serde_json::Map::default());

                for all_of in all_ofs {
                    merged_all_of.merge(match self.ref_schema_value(root_url, all_of).await {
                        Some(ref schema) => schema,
                        None => all_of,
                    });
                }

                merged_all_of.merge(&schema);

                self.collect_child_schemas(
                    root_url,
                    &merged_all_of,
                    root_path,
                    path,
                    depth,
                    schemas,
                )
                .await;
            }
            // TODO: handle allOfs in regular schemas.
            // doing so currently will overflow the stack.
        }

        let include_self = !composed;

        if include_self {
            schemas.push((
                root_path.extend(path.clone()),
                path.clone(),
                Arc::new(schema.clone()),
            ));
        }

        depth -= 1;

        if let Some(map) = schema["properties"].as_object() {
            for (k, v) in map {
                self.collect_child_schemas(
                    root_url,
                    v,
                    root_path,
                    &path.join(Key::from(k)),
                    depth,
                    schemas,
                )
                .await;
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn collect_child_schemas(
        &self,
        root_url: &Url,
        schema: &Value,
        root_path: &Keys,
        path: &Keys,
        mut depth: usize,
        schemas: &mut Vec<(Keys, Keys, Arc<Value>)>,
    ) {
        if !schema.is_object() || depth == 0 {
            return;
        }

        if let Some(schema) = self.ref_schema_value(root_url, schema).await {
            return self
                .collect_child_schemas(root_url, &schema, root_path, path, depth, schemas)
                .await;
        }

        if let Some(one_ofs) = schema["oneOf"].as_array() {
            for one_of in one_ofs {
                self.collect_child_schemas(root_url, one_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        if let Some(any_ofs) = schema["anyOf"].as_array() {
            for any_of in any_ofs {
                self.collect_child_schemas(root_url, any_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        // Deal with the { "description": "Foo", "allOf": [{ "$ref": "Bar" }] }
        // pattern.
        let composed = [
            !schema["allOf"].is_null(),
            !schema["oneOf"].is_null(),
            !schema["anyOf"].is_null(),
        ]
        .into_iter()
        .filter(|b| *b)
        .count()
            == 1
            && schema["properties"].is_null();

        if let Some(all_ofs) = schema["allOf"].as_array() {
            if !all_ofs.is_empty() && composed {
                let mut schema = schema.clone();
                if let Some(obj) = schema["allOf"].as_object_mut() {
                    obj.remove("allOf");
                }

                let mut merged_all_of = Value::Object(serde_json::Map::default());

                for all_of in all_ofs {
                    merged_all_of.merge(match self.ref_schema_value(root_url, all_of).await {
                        Some(ref schema) => schema,
                        None => all_of,
                    });
                }

                merged_all_of.merge(&schema);

                self.collect_child_schemas(
                    root_url,
                    &merged_all_of,
                    root_path,
                    path,
                    depth,
                    schemas,
                )
                .await;
            }
            // TODO: handle allOfs in regular schemas.
            // doing so currently will overflow the stack.
        }

        let include_self = !composed;

        if include_self {
            schemas.push((
                root_path.extend(path.clone()),
                path.clone(),
                Arc::new(schema.clone()),
            ));
        }

        depth -= 1;

        if let Some(map) = schema["properties"].as_object() {
            for (k, v) in map {
                self.collect_child_schemas(
                    root_url,
                    v,
                    root_path,
                    &path.join(Key::from(k)),
                    depth,
                    schemas,
                )
... (truncated)
```

## Git History

- Commits: 13
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
