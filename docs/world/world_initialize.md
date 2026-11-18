# world.initialize

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\world.rs` (lines 148-216)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn initialize(
        &mut self,
        context: Context<World<E>>,
        env: &impl Environment,
    ) -> Result<(), anyhow::Error> {
        if let Err(error) = self
            .load_config(env, &context.world().default_config.load())
            .await
        {
            tracing::warn!(%error, "failed to load workspace configuration");
        }

        if !self.config.schema.enabled {
            return Ok(());
        }

        self.schemas.cache().set_expiration_times(
            Duration::from_secs(self.config.schema.cache.memory_expiration),
            Duration::from_secs(self.config.schema.cache.disk_expiration),
        );

        self.schemas
            .associations()
            .add_from_config(&self.taplo_config);

        for (pattern, schema_url) in &self.config.schema.associations {
            let pattern = match Regex::new(pattern) {
                Ok(p) => p,
                Err(error) => {
                    tracing::error!(%error, "invalid association pattern");
                    continue;
                }
            };

            let url = if schema_url.starts_with("./") {
                self.root.join(schema_url)
            } else {
                schema_url.parse()
            };

            let url = match url {
                Ok(u) => u,
                Err(error) => {
                    tracing::error!(%error, url = %schema_url, "invalid schema url");
                    continue;
                }
            };

            self.schemas.associations().add(
                AssociationRule::Regex(pattern),
                SchemaAssociation {
                    url,
                    meta: json!({
                        "source": source::LSP_CONFIG,
                    }),
                    priority: priority::LSP_CONFIG,
                },
            );
        }

        for catalog in &self.config.schema.catalogs {
            if let Err(error) = self.schemas.associations().add_from_catalog(catalog).await {
                tracing::error!(%error, "failed to add schemas from catalog");
            }
        }

        self.emit_associations(context).await;
        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) async fn initialize(
        &mut self,
        context: Context<World<E>>,
        env: &impl Environment,
    ) -> Result<(), anyhow::Error> {
        if let Err(error) = self
            .load_config(env, &context.world().default_config.load())
            .await
        {
            tracing::warn!(%error, "failed to load workspace configuration");
        }

        if !self.config.schema.enabled {
            return Ok(());
        }

        self.schemas.cache().set_expiration_times(
            Duration::from_secs(self.config.schema.cache.memory_expiration),
            Duration::from_secs(self.config.schema.cache.disk_expiration),
        );

        self.schemas
            .associations()
            .add_from_config(&self.taplo_config);

        for (pattern, schema_url) in &self.config.schema.associations {
            let pattern = match Regex::new(pattern) {
                Ok(p) => p,
                Err(error) => {
                    tracing::error!(%error, "invalid association pattern");
                    continue;
                }
            };

            let url = if schema_url.starts_with("./") {
                self.root.join(schema_url)
            } else {
                schema_url.parse()
            };

            let url = match url {
                Ok(u) => u,
                Err(error) => {
                    tracing::error!(%error, url = %schema_url, "invalid schema url");
                    continue;
                }
            };

            self.schemas.associations().add(
                AssociationRule::Regex(pattern),
                SchemaAssociation {
                    url,
                    meta: json!({
                        "source": source::LSP_CONFIG,
                    }),
                    priority: priority::LSP_CONFIG,
                },
            );
        }

        for catalog in &self.config.schema.catalogs {
            if let Err(error) = self.schemas.associations().add_from_catalog(catalog).await {
                tracing::error!(%error, "failed to add schemas from catalog");
            }
        }

        self.emit_associations(context).await;
        Ok(())
    }
```

## Git History

- Commits: 13
- Authors: 3
- Bug fixes: 4
- Features: 5

## Callers

None

## Callees

None
