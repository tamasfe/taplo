# associations.add_from_document

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 171-255)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn add_from_document(&self, doc_url: &Url, root: &Node) {
        self.retain(|(rule, assoc)| match rule {
            AssociationRule::Url(u) => {
                !(u == doc_url
                    && (assoc.meta["source"] == source::DIRECTIVE
                        || assoc.meta["source"] == source::SCHEMA_FIELD))
            }
            _ => true,
        });

        for comment in root.header_comments() {
            if let Some("schema") = comment.directive() {
                let value = comment.value();

                if value.is_empty() {
                    tracing::warn!("empty schema directive");
                    continue;
                }

                let schema_url: Url = match value.parse() {
                    Ok(url) => url,
                    Err(error) => {
                        tracing::debug!(%error, "invalid url in directive, assuming file path instead");

                        if self.env.is_absolute(Path::new(value)) {
                            match format!("file://{value}").parse() {
                                Ok(u) => u,
                                Err(error) => {
                                    tracing::error!(%error, "invalid schema directive");
                                    continue;
                                }
                            }
                        } else {
                            match doc_url.join(value) {
                                Ok(u) => u,
                                Err(error) => {
                                    tracing::error!(%error, "invalid schema directive");
                                    continue;
                                }
                            }
                        }
                    }
                };

                self.associations.write().push((
                    AssociationRule::Url(doc_url.clone()),
                    SchemaAssociation {
                        url: schema_url,
                        priority: priority::DIRECTIVE,
                        meta: json!({ "source": source::DIRECTIVE }),
                    },
                ));
                break;
            }
        }

        if let Node::Str(s) = root.get("$schema") {
            let schema_url: Url = if s.value().starts_with('.') {
                match doc_url.join(s.value()) {
                    Ok(s) => s,
                    Err(error) => {
                        tracing::error!(%error, "invalid schema url or path given in the `$schema` field");
                        return;
                    }
                }
            } else {
                match s.value().parse() {
                    Ok(s) => s,
                    Err(error) => {
                        tracing::error!(%error, "invalid schema url or path given in the `$schema` field");
                        return;
                    }
                }
            };

            self.associations.write().push((
                AssociationRule::Url(doc_url.clone()),
                SchemaAssociation {
                    url: schema_url,
                    priority: priority::SCHEMA_FIELD,
                    meta: json!({ "source": source::SCHEMA_FIELD }),
                },
            ));
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn add_from_document(&self, doc_url: &Url, root: &Node) {
        self.retain(|(rule, assoc)| match rule {
            AssociationRule::Url(u) => {
                !(u == doc_url
                    && (assoc.meta["source"] == source::DIRECTIVE
                        || assoc.meta["source"] == source::SCHEMA_FIELD))
            }
            _ => true,
        });

        for comment in root.header_comments() {
            if let Some("schema") = comment.directive() {
                let value = comment.value();

                if value.is_empty() {
                    tracing::warn!("empty schema directive");
                    continue;
                }

                let schema_url: Url = match value.parse() {
                    Ok(url) => url,
                    Err(error) => {
                        tracing::debug!(%error, "invalid url in directive, assuming file path instead");

                        if self.env.is_absolute(Path::new(value)) {
                            match format!("file://{value}").parse() {
                                Ok(u) => u,
                                Err(error) => {
                                    tracing::error!(%error, "invalid schema directive");
                                    continue;
                                }
                            }
                        } else {
                            match doc_url.join(value) {
                                Ok(u) => u,
                                Err(error) => {
                                    tracing::error!(%error, "invalid schema directive");
                                    continue;
                                }
                            }
                        }
                    }
                };

                self.associations.write().push((
                    AssociationRule::Url(doc_url.clone()),
                    SchemaAssociation {
                        url: schema_url,
                        priority: priority::DIRECTIVE,
                        meta: json!({ "source": source::DIRECTIVE }),
                    },
                ));
                break;
            }
        }

        if let Node::Str(s) = root.get("$schema") {
            let schema_url: Url = if s.value().starts_with('.') {
                match doc_url.join(s.value()) {
                    Ok(s) => s,
                    Err(error) => {
                        tracing::error!(%error, "invalid schema url or path given in the `$schema` field");
                        return;
                    }
                }
            } else {
                match s.value().parse() {
                    Ok(s) => s,
                    Err(error) => {
                        tracing::error!(%error, "invalid schema url or path given in the `$schema` field");
                        return;
                    }
                }
            };

            self.associations.write().push((
                AssociationRule::Url(doc_url.clone()),
                SchemaAssociation {
                    url: schema_url,
                    priority: priority::SCHEMA_FIELD,
                    meta: json!({ "source": source::SCHEMA_FIELD }),
                },
            ));
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
