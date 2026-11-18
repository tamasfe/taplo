# mod.collect_schemas

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 328-459)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn collect_schemas(
        &self,
        root_url: &Url,
        schema: &Value,
        value: &Value,
        full_path: Keys,
        path: &Keys,
        schemas: &mut Vec<(Keys, Arc<Value>)>,
    ) -> Result<(), anyhow::Error> {
        if !schema.is_object() {
            return Ok(());
        }

        if let Some(r) = schema.schema_ref() {
            let url = reference_url(root_url, r)
                .ok_or_else(|| anyhow!("could not determine schema URL"))?;
            let schema = self.resolve_schema(url).await?;
            return self
                .collect_schemas(root_url, &schema, value, full_path.clone(), path, schemas)
                .await;
        }

        if let Some(one_ofs) = schema["oneOf"].as_array() {
            for one_of in one_ofs {
                self.collect_schemas(root_url, one_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        if let Some(any_ofs) = schema["anyOf"].as_array() {
            for any_of in any_ofs {
                self.collect_schemas(root_url, any_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        if let Some(all_ofs) = schema["allOf"].as_array() {
            for all_of in all_ofs {
                self.collect_schemas(root_url, all_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        let include_self = schema["allOf"].is_null();

        let Some(key) = path.iter().next() else {
            if include_self {
                schemas.push((full_path.clone(), Arc::new(schema.clone())));
            }
            return Ok(());
        };

        let child_path = path.skip_left(1);

        match key {
            KeyOrIndex::Key(k) => {
                // For array of tables.
                self.collect_schemas(
                    root_url,
                    &schema["items"][k.value()],
                    value,
                    full_path.join(k.clone()),
                    &child_path,
                    schemas,
                )
                .await?;

                self.collect_schemas(
                    root_url,
                    &schema["properties"][k.value()],
                    &value[k.value()],
                    full_path.join(k.clone()),
                    &child_path,
                    schemas,
                )
                .await?;

                self.collect_schemas(
                    root_url,
                    &schema["additionalProperties"],
                    &value[k.value()],
                    full_path.join(k.clone()),
                    &child_path,
                    schemas,
                )
                .await?;

                if let Some(pattern_props) = schema["patternProperties"].as_object() {
                    for (pattern, pattern_schema) in pattern_props {
                        if let Ok(re) = Regex::new(pattern) {
                            if re.is_match(k.value()) {
                                self.collect_schemas(
                                    root_url,
                                    pattern_schema,
                                    &value[k.value()],
                                    full_path.join(k.clone()),
                                    &child_path,
                                    schemas,
                                )
                                .await?;
                            }
                        }
                    }
                }
            }
            KeyOrIndex::Index(idx) => {
                if schema["items"].is_array() {
                    self.collect_schemas(
                        root_url,
                        &schema["items"][idx],
                        &value[idx],
                        full_path.join(*idx),
                        &child_path,
                        schemas,
                    )
                    .await?;
                } else {
                    self.collect_schemas(
                        root_url,
                        &schema["items"],
                        &value[idx],
                        full_path.join(*idx),
                        &child_path,
                        schemas,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn collect_schemas(
        &self,
        root_url: &Url,
        schema: &Value,
        value: &Value,
        full_path: Keys,
        path: &Keys,
        schemas: &mut Vec<(Keys, Arc<Value>)>,
    ) -> Result<(), anyhow::Error> {
        if !schema.is_object() {
            return Ok(());
        }

        if let Some(r) = schema.schema_ref() {
            let url = reference_url(root_url, r)
                .ok_or_else(|| anyhow!("could not determine schema URL"))?;
            let schema = self.resolve_schema(url).await?;
            return self
                .collect_schemas(root_url, &schema, value, full_path.clone(), path, schemas)
                .await;
        }

        if let Some(one_ofs) = schema["oneOf"].as_array() {
            for one_of in one_ofs {
                self.collect_schemas(root_url, one_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        if let Some(any_ofs) = schema["anyOf"].as_array() {
            for any_of in any_ofs {
                self.collect_schemas(root_url, any_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        if let Some(all_ofs) = schema["allOf"].as_array() {
            for all_of in all_ofs {
                self.collect_schemas(root_url, all_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        let include_self = schema["allOf"].is_null();

        let Some(key) = path.iter().next() else {
            if include_self {
                schemas.push((full_path.clone(), Arc::new(schema.clone())));
            }
            return Ok(());
        };

        let child_path = path.skip_left(1);

        match key {
            KeyOrIndex::Key(k) => {
                // For array of tables.
                self.collect_schemas(
                    root_url,
                    &schema["items"][k.value()],
                    value,
                    full_path.join(k.clone()),
                    &child_path,
                    schemas,
                )
                .await?;

                self.collect_schemas(
                    root_url,
                    &schema["properties"][k.value()],
                    &value[k.value()],
                    full_path.join(k.clone()),
                    &child_path,
                    schemas,
                )
                .await?;

                self.collect_schemas(
                    root_url,
                    &schema["additionalProperties"],
                    &value[k.value()],
                    full_path.join(k.clone()),
                    &child_path,
                    schemas,
                )
                .await?;

                if let Some(pattern_props) = schema["patternProperties"].as_object() {
                    for (pattern, pattern_schema) in pattern_props {
                        if let Ok(re) = Regex::new(pattern) {
                            if re.is_match(k.value()) {
                                self.collect_schemas(
                                    root_url,
                                    pattern_schema,
                                    &value[k.value()],
                                    full_path.join(k.clone()),
                                    &child_path,
                                    schemas,
                                )
                                .await?;
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
