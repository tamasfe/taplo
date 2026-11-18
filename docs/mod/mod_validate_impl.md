# mod.validate_impl

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 126-184)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn validate_impl(
        &self,
        validator: &JSONSchema,
        value: &Value,
    ) -> Result<Vec<ValidationError<'static>>, anyhow::Error> {
        // The following loop is required for retrieving external schemas.
        //
        // We don't know if any external schemas are required until we reach
        // a validation path that requires it, so we might have to loop many times
        // to fully validate according to a schema that has many nested references.
        loop {
            match validator.validate(value) {
                Ok(()) => return Ok(Vec::new()),
                Err(errors) => {
                    let errors: Vec<_> = errors
                        .map(|err| ValidationError {
                            instance: Cow::Owned(err.instance.into_owned()),
                            kind: err.kind,
                            instance_path: err.instance_path,
                            schema_path: err.schema_path,
                        })
                        .collect();

                    // We check whether there were any external schema errors,
                    // and retrieve the schemas accordingly.
                    let mut external_schema_requests: FuturesUnordered<_> = errors
                        .iter()
                        .filter_map(|err| {
                            if let ValidationErrorKind::Resolver { url, .. } = &err.kind {
                                Some(async {
                                    let value = self.load_schema(url).await?;
                                    drop(self.cache.store(url.clone(), value));
                                    Result::<(), anyhow::Error>::Ok(())
                                })
                            } else {
                                None
                            }
                        })
                        .collect();

                    // There are no external schemas to retrieve,
                    // return the errors as-is.
                    if external_schema_requests.is_empty() {
                        drop(external_schema_requests);
                        return Ok(errors);
                    }

                    // Retrieve external schemas, and return on the first failure.
                    while let Some(external_schema_result) = external_schema_requests.next().await {
                        external_schema_result?;
                    }

                    // Try validation again, now with external schemas
                    // resolved and cached.
                    continue;
                }
            };
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn validate_impl(
        &self,
        validator: &JSONSchema,
        value: &Value,
    ) -> Result<Vec<ValidationError<'static>>, anyhow::Error> {
        // The following loop is required for retrieving external schemas.
        //
        // We don't know if any external schemas are required until we reach
        // a validation path that requires it, so we might have to loop many times
        // to fully validate according to a schema that has many nested references.
        loop {
            match validator.validate(value) {
                Ok(()) => return Ok(Vec::new()),
                Err(errors) => {
                    let errors: Vec<_> = errors
                        .map(|err| ValidationError {
                            instance: Cow::Owned(err.instance.into_owned()),
                            kind: err.kind,
                            instance_path: err.instance_path,
                            schema_path: err.schema_path,
                        })
                        .collect();

                    // We check whether there were any external schema errors,
                    // and retrieve the schemas accordingly.
                    let mut external_schema_requests: FuturesUnordered<_> = errors
                        .iter()
                        .filter_map(|err| {
                            if let ValidationErrorKind::Resolver { url, .. } = &err.kind {
                                Some(async {
                                    let value = self.load_schema(url).await?;
                                    drop(self.cache.store(url.clone(), value));
                                    Result::<(), anyhow::Error>::Ok(())
                                })
                            } else {
                                None
                            }
                        })
                        .collect();

                    // There are no external schemas to retrieve,
                    // return the errors as-is.
                    if external_schema_requests.is_empty() {
                        drop(external_schema_requests);
                        return Ok(errors);
                    }

                    // Retrieve external schemas, and return on the first failure.
                    while let Some(external_schema_result) = external_schema_requests.next().await {
                        external_schema_result?;
                    }

                    // Try validation again, now with external schemas
                    // resolved and cached.
                    continue;
                }
            };
        }
    }
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
