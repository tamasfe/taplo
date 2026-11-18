# associations.add_from_catalog

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 102-168)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn add_from_catalog(&self, url: &Url) -> Result<(), anyhow::Error> {
        let index = self.load_catalog(url).await?;
        match index {
            SchemaCatalog::SchemaStore(index) => {
                for schema in &index.schemas {
                    match GlobRule::new(&schema.file_match, [] as [&str; 0]) {
                        Ok(rule) => {
                            self.associations.write().push((
                                rule.into(),
                                SchemaAssociation {
                                    url: schema.url.clone(),
                                    meta: json!({
                                        "name": schema.name,
                                        "description": schema.description,
                                        "source": source::CATALOG,
                                        "catalog_url": url,
                                    }),
                                    priority: priority::CATALOG,
                                },
                            ));
                        }
                        Err(error) => {
                            tracing::warn!(
                                %error,
                                schema_name = %schema.name,
                                source = %url,
                                "invalid glob pattern(s)"
                            );
                        }
                    }
                }
            }
            SchemaCatalog::Taplo(index) => {
                for schema in &index.schemas {
                    for pattern in &schema.extra.patterns {
                        let regex = match Regex::new(pattern) {
                            Ok(pat) => pat,
                            Err(error) => {
                                tracing::warn!(
                                    %error,
                                    pattern = %pattern,
                                    schema_name = %schema.title,
                                    "invalid regex pattern"
                                );
                                continue;
                            }
                        };

                        self.associations.write().push((
                            regex.into(),
                            SchemaAssociation {
                                url: schema.url.clone(),
                                meta: json!({
                                    "name": schema.title,
                                    "description": schema.description,
                                    "source": source::CATALOG,
                                    "catalog_url": url,
                                }),
                                priority: priority::CATALOG,
                            },
                        ));
                    }
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
    pub async fn add_from_catalog(&self, url: &Url) -> Result<(), anyhow::Error> {
        let index = self.load_catalog(url).await?;
        match index {
            SchemaCatalog::SchemaStore(index) => {
                for schema in &index.schemas {
                    match GlobRule::new(&schema.file_match, [] as [&str; 0]) {
                        Ok(rule) => {
                            self.associations.write().push((
                                rule.into(),
                                SchemaAssociation {
                                    url: schema.url.clone(),
                                    meta: json!({
                                        "name": schema.name,
                                        "description": schema.description,
                                        "source": source::CATALOG,
                                        "catalog_url": url,
                                    }),
                                    priority: priority::CATALOG,
                                },
                            ));
                        }
                        Err(error) => {
                            tracing::warn!(
                                %error,
                                schema_name = %schema.name,
                                source = %url,
                                "invalid glob pattern(s)"
                            );
                        }
                    }
                }
            }
            SchemaCatalog::Taplo(index) => {
                for schema in &index.schemas {
                    for pattern in &schema.extra.patterns {
                        let regex = match Regex::new(pattern) {
                            Ok(pat) => pat,
                            Err(error) => {
                                tracing::warn!(
                                    %error,
                                    pattern = %pattern,
                                    schema_name = %schema.title,
                                    "invalid regex pattern"
                                );
                                continue;
                            }
                        };

                        self.associations.write().push((
                            regex.into(),
                            SchemaAssociation {
                                url: schema.url.clone(),
                                meta: json!({
                                    "name": schema.title,
                                    "description": schema.description,
                                    "source": source::CATALOG,
                                    "catalog_url": url,
                                }),
                                priority: priority::CATALOG,
                            },
                        ));
                    }
                }
            }
        }
        Ok(())
    }
```

## Git History

- Commits: 16
- Authors: 5
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
