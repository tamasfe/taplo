use std::path::Path;

use crate::{args::LintCommand, Taplo};
use anyhow::{anyhow, Context};
use codespan_reporting::files::SimpleFile;
use serde_json::json;
use taplo::parser;
use taplo_common::{
    environment::Environment,
    schema::associations::{AssociationRule, SchemaAssociation, DEFAULT_CATALOGS},
};
use tokio::io::AsyncReadExt;
use url::Url;

impl<E: Environment> Taplo<E> {
    pub async fn execute_lint(&mut self, cmd: LintCommand) -> Result<(), anyhow::Error> {
        self.schemas
            .cache()
            .set_cache_path(cmd.general.cache_path.clone());

        let config = self.load_config(&cmd.general).await?;

        if !cmd.no_schema {
            if let Some(schema_url) = cmd.schema.clone() {
                self.schemas.associations().add(
                    AssociationRule::regex(".*")?,
                    SchemaAssociation {
                        meta: json!({"source": "command-line"}),
                        url: schema_url,
                        priority: 999,
                    },
                );
            } else {
                self.schemas.associations().add_from_config(&config);

                for catalog in &cmd.schema_catalog {
                    self.schemas
                        .associations()
                        .add_from_catalog(catalog)
                        .await
                        .with_context(|| "failed to load schema catalog")?;
                }

                if cmd.default_schema_catalogs {
                    for catalog in DEFAULT_CATALOGS {
                        self.schemas
                            .associations()
                            .add_from_catalog(&Url::parse(catalog).unwrap())
                            .await
                            .with_context(|| "failed to load schema catalog")?;
                    }
                }
            }
        }

        if matches!(cmd.files.first().map(|it| it.as_str()), Some("-")) {
            self.lint_stdin(cmd).await
        } else {
            self.lint_files(cmd).await
        }
    }

    #[tracing::instrument(skip_all)]
    async fn lint_stdin(&self, _cmd: LintCommand) -> Result<(), anyhow::Error> {
        let mut source = String::new();
        self.env.stdin().read_to_string(&mut source).await?;
        self.lint_source("-", &source).await
    }

    #[tracing::instrument(skip_all)]
    async fn lint_files(&mut self, cmd: LintCommand) -> Result<(), anyhow::Error> {
        let config = self.load_config(&cmd.general).await?;

        let cwd = self
            .env
            .cwd_normalized()
            .ok_or_else(|| anyhow!("could not figure the current working directory"))?;

        let files = self
            .collect_files(&cwd, &config, cmd.files.into_iter())
            .await?;

        let mut result = Ok(());

        for file in files {
            if let Err(error) = self.lint_file(&file).await {
                tracing::error!(%error, path = ?file, "invalid file");
                result = Err(anyhow!("some files were not valid"));
            }
        }

        result
    }

    async fn lint_file(&self, file: &Path) -> Result<(), anyhow::Error> {
        let source = self.env.read_file(file).await?;
        let source = String::from_utf8(source)?;
        self.lint_source(&file.to_string_lossy(), &source).await
    }

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
}
