# lint.lint_source

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\lint.rs` (lines 101-155)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn lint_source(&self, file_path: &str, source: &str) -> Result<(), anyhow::Error> {
        let parse = parser::parse(source);

        self.print_parse_errors(&SimpleFile::new(file_path, source), &parse.errors)
            .await?;

        if !parse.errors.is_empty() {
            return Err(anyhow!("syntax errors found"));
        }

        let dom = parse.into_dom();

        if let Err(errors) = dom.validate() {
            self.print_semantic_errors(&SimpleFile::new(file_path, source), errors)
                .await?;

            return Err(anyhow!("semantic errors found"));
        }

        let config = self.config.as_ref().unwrap();

        if !config.is_schema_enabled(Path::new(file_path)) {
            tracing::debug!("schema validation disabled for config file");
            return Ok(());
        }

        let file_uri: Url = format!("file://{file_path}").parse().unwrap();

        self.schemas
            .associations()
            .add_from_document(&file_uri, &dom);

        if let Some(schema_association) = self.schemas.associations().association_for(&file_uri) {
            tracing::debug!(
                schema.url = %schema_association.url,
                schema.name = schema_association.meta["name"].as_str().unwrap_or(""),
                schema.source = schema_association.meta["source"].as_str().unwrap_or(""),
                "using schema"
            );

            let errors = self
                .schemas
                .validate_root(&schema_association.url, &dom)
                .await?;

            if !errors.is_empty() {
                self.print_schema_errors(&SimpleFile::new(file_path, source), &errors)
                    .await?;

                return Err(anyhow!("schema validation failed"));
            }
        }

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn lint_source(&self, file_path: &str, source: &str) -> Result<(), anyhow::Error> {
        let parse = parser::parse(source);

        self.print_parse_errors(&SimpleFile::new(file_path, source), &parse.errors)
            .await?;

        if !parse.errors.is_empty() {
            return Err(anyhow!("syntax errors found"));
        }

        let dom = parse.into_dom();

        if let Err(errors) = dom.validate() {
            self.print_semantic_errors(&SimpleFile::new(file_path, source), errors)
                .await?;

            return Err(anyhow!("semantic errors found"));
        }

        let config = self.config.as_ref().unwrap();

        if !config.is_schema_enabled(Path::new(file_path)) {
            tracing::debug!("schema validation disabled for config file");
            return Ok(());
        }

        let file_uri: Url = format!("file://{file_path}").parse().unwrap();

        self.schemas
            .associations()
            .add_from_document(&file_uri, &dom);

        if let Some(schema_association) = self.schemas.associations().association_for(&file_uri) {
            tracing::debug!(
                schema.url = %schema_association.url,
                schema.name = schema_association.meta["name"].as_str().unwrap_or(""),
                schema.source = schema_association.meta["source"].as_str().unwrap_or(""),
                "using schema"
            );

            let errors = self
                .schemas
                .validate_root(&schema_association.url, &dom)
                .await?;

            if !errors.is_empty() {
                self.print_schema_errors(&SimpleFile::new(file_path, source), &errors)
                    .await?;

                return Err(anyhow!("schema validation failed"));
            }
        }

        Ok(())
    }
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 1
- Features: 3

## Callers

None

## Callees

None
