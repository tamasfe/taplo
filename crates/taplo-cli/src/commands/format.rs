use std::{
    mem,
    path::{Path, PathBuf},
};

use crate::{args::FormatCommand, Taplo};
use anyhow::anyhow;
use codespan_reporting::files::SimpleFile;

use taplo::{formatter, parser};
use taplo_common::{config::Config, environment::Environment, util::Normalize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl<E: Environment> Taplo<E> {
    pub async fn execute_format(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        if matches!(cmd.files.get(0).map(|it| it.as_str()), Some("-")) {
            self.format_stdin(cmd).await
        } else {
            self.format_files(cmd).await
        }
    }

    #[tracing::instrument(skip_all)]
    async fn format_stdin(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        let mut source = String::new();
        self.env.stdin().read_to_string(&mut source).await?;

        let config = self.load_config(&cmd.general).await?;
        let display_path = cmd.stdin_filepath.as_deref().unwrap_or("-");

        let p = parser::parse(&source);

        if !p.errors.is_empty() {
            self.print_parse_errors(&SimpleFile::new(display_path, source.as_str()), &p.errors)
                .await?;

            if !cmd.force {
                return Err(anyhow!("no formatting was done due to syntax errors"));
            }
        }

        let format_opts = self.format_options(&config, &cmd, Path::new(display_path))?;

        let error_ranges = p.errors.iter().map(|e| e.range).collect::<Vec<_>>();

        let dom = p.into_dom();

        let formatted = formatter::format_with_path_scopes(
            dom,
            format_opts,
            &error_ranges,
            config.format_scopes(&PathBuf::from(display_path).normalize()),
        )
        .map_err(|err| anyhow!("invalid key pattern: {err}"))?;

        if cmd.check {
            if source != formatted {
                return Err(anyhow!("the input was not properly formatted"));
            }
        } else {
            let mut stdout = self.env.stdout();
            stdout.write_all(formatted.as_bytes()).await?;
            stdout.flush().await?;
        }

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn format_files(&mut self, mut cmd: FormatCommand) -> Result<(), anyhow::Error> {
        if cmd.stdin_filepath.is_some() {
            tracing::warn!("using `--stdin-filepath` has no effect unless input comes from stdin")
        }

        let config = self.load_config(&cmd.general).await?;

        let cwd = self
            .env
            .cwd_normalized()
            .ok_or_else(|| anyhow!("could not figure the current working directory"))?;

        let files = self
            .collect_files(&cwd, &config, mem::take(&mut cmd.files).into_iter())
            .await?;

        let mut result = Ok(());

        for path in files {
            let format_opts = self.format_options(&config, &cmd, &path)?;

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

            let error_ranges = p.errors.iter().map(|e| e.range).collect::<Vec<_>>();

            let dom = p.into_dom();

            let formatted = formatter::format_with_path_scopes(
                dom,
                format_opts,
                &error_ranges,
                config.format_scopes(&path),
            )
            .map_err(|err| anyhow!("invalid key pattern: {err}"))?;

            if cmd.check {
                if source != formatted {
                    tracing::error!(?path, "the file is not properly formatted");

                    println!("diff --git a/{path} b/{path}", path = path.display());
                    println!("--- a/{path}", path = path.display());
                    println!("+++ b/{path}", path = path.display());

                    const CONTEXT_LINES: usize = 7;
                    let hunks = prettydiff::diff_lines(&source, &formatted);
                    let hunks = hunks.diff();
                    let hunkcount = hunks.len();
                    let mut acc = Vec::<String>::with_capacity(hunkcount);
                    for (idx, diff_op) in hunks.into_iter().enumerate() {
                        use ansi_term::Colour::{self, Green, Red};
                        use prettydiff::basic::DiffOp;

                        fn apply_color<'a>(
                            s: &'a [&'a str],
                            prefix: &'a str,
                            color: Colour,
                        ) -> impl IntoIterator<Item = String> + 'a {
                            s.iter()
                                .map(move |&s| color.paint(prefix.to_owned() + s).to_string())
                        }

                        match diff_op {
                            DiffOp::Equal(slices) => {
                                if slices.len() < 2 + CONTEXT_LINES * 2
                                    && idx > 0
                                    && idx + 1 < hunkcount
                                {
                                    acc.extend(
                                        slices[0..usize::max(CONTEXT_LINES, slices.len())]
                                            .into_iter()
                                            .map(ToOwned::to_owned)
                                            .map(ToOwned::to_owned),
                                    );
                                } else {
                                    if idx > 0 {
                                        acc.extend(
                                            slices[0..usize::max(CONTEXT_LINES, slices.len())]
                                                .into_iter()
                                                .map(ToOwned::to_owned)
                                                .map(ToOwned::to_owned),
                                        );
                                    }
                                    if idx + 1 < hunkcount {
                                        acc.extend(
                                            slices[(slices.len().saturating_sub(CONTEXT_LINES))..]
                                                .into_iter()
                                                .map(ToOwned::to_owned)
                                                .map(ToOwned::to_owned),
                                        );
                                    }
                                }
                            }
                            DiffOp::Insert(ins) => {
                                acc.extend(apply_color(ins, "+", Green));
                            }
                            DiffOp::Remove(rem) => {
                                acc.extend(apply_color(rem, "-", Red));
                            }
                            DiffOp::Replace(rem, ins) => {
                                acc.extend(apply_color(rem, "-", Red));
                                acc.extend(apply_color(ins, "+", Green));
                            }
                        }
                    }
                    println!("{}", acc.join("\n"));

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
        path: &Path,
    ) -> Result<formatter::Options, anyhow::Error> {
        let mut format_opts = formatter::Options::default();
        config.update_format_options(path, &mut format_opts);

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
