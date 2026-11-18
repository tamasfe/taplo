# associations.add_from_config

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 257-301)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn add_from_config(&self, config: &Config) {
        for rule in &config.rule {
            let Some(file_rule) = rule.file_rule.clone() else {
                continue;
            };

            if let Some(schema_opts) = &rule.options.schema {
                if let Some(url) = &schema_opts.url {
                    if schema_opts.enabled.unwrap_or(true) {
                        self.associations.write().push((
                            file_rule.into(),
                            SchemaAssociation {
                                url: url.clone(),
                                meta: json!({
                                    "source": source::CONFIG,
                                }),
                                priority: priority::CONFIG_RULE,
                            },
                        ));
                    }
                }
            }
        }

        let Some(file_rule) = config.file_rule.clone() else {
            return;
        };

        if let Some(schema_opts) = &config.global_options.schema {
            if let Some(url) = &schema_opts.url {
                if schema_opts.enabled.unwrap_or(true) {
                    self.associations.write().push((
                        file_rule.into(),
                        SchemaAssociation {
                            url: url.clone(),
                            meta: json!({
                                "source": source::CONFIG,
                            }),
                            priority: priority::CONFIG,
                        },
                    ));
                }
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn add_from_config(&self, config: &Config) {
        for rule in &config.rule {
            let Some(file_rule) = rule.file_rule.clone() else {
                continue;
            };

            if let Some(schema_opts) = &rule.options.schema {
                if let Some(url) = &schema_opts.url {
                    if schema_opts.enabled.unwrap_or(true) {
                        self.associations.write().push((
                            file_rule.into(),
                            SchemaAssociation {
                                url: url.clone(),
                                meta: json!({
                                    "source": source::CONFIG,
                                }),
                                priority: priority::CONFIG_RULE,
                            },
                        ));
                    }
                }
            }
        }

        let Some(file_rule) = config.file_rule.clone() else {
            return;
        };

        if let Some(schema_opts) = &config.global_options.schema {
            if let Some(url) = &schema_opts.url {
                if schema_opts.enabled.unwrap_or(true) {
                    self.associations.write().push((
                        file_rule.into(),
                        SchemaAssociation {
                            url: url.clone(),
                            meta: json!({
                                "source": source::CONFIG,
                            }),
                            priority: priority::CONFIG,
                        },
                    ));
                }
            }
        }
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
