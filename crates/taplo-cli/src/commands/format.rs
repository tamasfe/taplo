use std::path::Path;

use crate::{args::FormatCommand, Taplo};
use anyhow::anyhow;
use codespan_reporting::files::SimpleFile;
use taplo::{formatter, parser};
use taplo_common::{config::Config, environment::Environment};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl<E: Environment> Taplo<E> {
    pub async fn execute_format(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        if matches!(cmd.files.get(0).map(|it| it.as_str()), Some("-")) {
            return self.format_stdin(cmd).await;
        } else {
            return self.format_files(cmd).await;
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn format_stdin(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        let mut source = String::new();
        self.env.stdin().read_to_string(&mut source).await?;

        let config = self.load_config(&cmd.general).await?;

        let p = parser::parse(&source);

        if !p.errors.is_empty() {
            self.print_parse_errors(&SimpleFile::new("-", source.as_str()), &p.errors)
                .await?;

            if !cmd.force {
                return Err(anyhow!("no formatting was done due to syntax errors"));
            }
        }

        let mut format_opts = self.format_options(&config, &cmd)?;
        config.update_format_options(Path::new("-"), &mut format_opts);

        let dom = p.into_dom();

        let formatted = formatter::format_with_path_scopes(
            dom,
            format_opts,
            config.format_scopes(Path::new("-")),
        )
        .map_err(|err| anyhow!("invalid key pattern: {err}"))?;

        if cmd.check {
            if source != formatted {
                return Err(anyhow!("the input was not properly formatted"));
            }
        } else {
            self.env.stdout().write_all(formatted.as_bytes()).await?;
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn format_files(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        let config = self.load_config(&cmd.general).await?;

        let cwd = self
            .env
            .cwd()
            .ok_or_else(|| anyhow!("could not figure the current working directory"))?;

        let format_opts = self.format_options(&config, &cmd)?;

        let files = self
            .collect_files(&cwd, &config, cmd.files.into_iter())
            .await?;

        let mut result = Ok(());

        for path in files {
            let mut format_opts = format_opts.clone();
            config.update_format_options(&path, &mut format_opts);

            let f = self.env.read_file(&path).await?;
            let source = String::from_utf8_lossy(&f).into_owned();

            let p = parser::parse(&source);

            if !p.errors.is_empty() {
                self.print_parse_errors(
                    &SimpleFile::new(&*path.to_string_lossy(), source.as_str()),
                    &p.errors,
                )
                .await?;

                if !cmd.force {
                    result = Err(anyhow!(
                        "some files were not formatted due to syntax errors"
                    ));
                    continue;
                }
            }

            let dom = p.into_dom();

            let formatted =
                formatter::format_with_path_scopes(dom, format_opts, config.format_scopes(&path))
                    .map_err(|err| anyhow!("invalid key pattern: {err}"))?;

            if cmd.check {
                if source != formatted {
                    tracing::error!(?path, "the file is not properly formatted");
                    result = Err(anyhow!("some files were not properly formatted"));
                }
            } else if source != formatted {
                self.env.write_file(&path, formatted.as_bytes()).await?;
            }
        }

        result
    }

    fn format_options(
        &self,
        config: &Config,
        cmd: &FormatCommand,
    ) -> Result<formatter::Options, anyhow::Error> {
        let mut format_opts = formatter::Options::default();

        if let Some(opts) = config.global_options.formatting.clone() {
            format_opts.update(opts);
        }

        format_opts.update_from_str(cmd.options.iter().filter_map(|s| {
            let mut split = s.split('=');
            let k = split.next();
            let v = split.next();

            if let (Some(k), Some(v)) = (k, v) {
                Some((k, v))
            } else {
                tracing::error!(option = %s, "malformed formatter option");
                None
            }
        }))?;

        Ok(format_opts)
    }
}
