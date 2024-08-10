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
        if matches!(cmd.files.first().map(|it| it.as_str()), Some("-")) {
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
        let display_path = match cmd.stdin_filepath.as_deref() {
            Some(filepath) if self.env.is_absolute(filepath.as_ref()) => {
                PathBuf::from(filepath).normalize()
            }
            Some(filepath) => {
                let cwd = self
                    .env
                    .cwd_normalized()
                    .ok_or_else(|| anyhow!("could not figure the current working directory"))?;
                cwd.join(filepath).normalize()
            }
            None => PathBuf::from("-"),
        };
        let p = parser::parse(&source);

        if !p.errors.is_empty() {
            self.print_parse_errors(
                &SimpleFile::new(&display_path.to_string_lossy(), source.as_str()),
                &p.errors,
            )
            .await?;

            if !cmd.force {
                return Err(anyhow!("no formatting was done due to syntax errors"));
            }
        }

        let format_opts = self.format_options(&config, &cmd, &display_path)?;

        let error_ranges = p.errors.iter().map(|e| e.range).collect::<Vec<_>>();

        let dom = p.into_dom();

        let formatted = formatter::format_with_path_scopes(
            dom,
            format_opts,
            &error_ranges,
            config.format_scopes(&display_path),
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

    #[cfg(target_arch = "wasm32")]
    async fn print_diff(
        &self,
        _path: impl AsRef<Path>,
        _original: &str,
        _formatted: &str,
    ) -> Result<(), anyhow::Error> {
        tracing::warn!("the `--diff` flag is not available in this build yet");
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn print_diff(
        &self,
        path: impl AsRef<Path>,
        original: &str,
        formatted: &str,
    ) -> Result<(), anyhow::Error> {
        let path = path.as_ref();
        let mut stdout = self.env.stdout();

        // print to stdout
        macro_rules! echo {
            ($($args:tt)*) => {
                let msg = format!("{}\n", std::format_args!($($args)*));
                stdout.write_all_buf(&mut msg.as_str().as_bytes()).await?;
            }
        }

        echo!("diff a/{path} b/{path}", path = path.display());
        echo!("--- a/{path}", path = path.display());
        echo!("+++ b/{path}", path = path.display());

        // How many lines of context to print:
        const CONTEXT_LINES: usize = 7;

        let hunks = prettydiff::diff_lines(original, formatted);
        let hunks = hunks.diff();
        let hunkcount = hunks.len();
        let mut acc = Vec::<String>::with_capacity(hunkcount);

        let mut pre_line = 0_usize;
        let mut post_line = 0_usize;

        for (idx, diff_op) in hunks.into_iter().enumerate() {
            use ansi_term::Colour::{self, Green, Red};
            use prettydiff::basic::DiffOp;

            // apply the given color and prefix to the set of strings `s`
            fn apply_color<'a>(
                s: &'a [&'a str],
                prefix: &'a str,
                color: Colour,
            ) -> impl IntoIterator<Item = String> + 'a {
                s.iter()
                    .map(move |&s| color.paint(prefix.to_owned() + s).to_string())
            }

            let mut pre_length = 0_usize;
            let mut post_length = 0_usize;

            // length of a net diff op
            match diff_op {
                DiffOp::Equal(slices) => {
                    if slices.len() < CONTEXT_LINES * 2 && idx > 0 && idx + 1 < hunkcount {
                        acc.extend(slices[..].iter().map(|&s| s.to_owned()));
                        pre_length += slices.len();
                        post_length += slices.len();
                    } else {
                        if idx > 0 {
                            let end = usize::min(CONTEXT_LINES, slices.len());
                            acc.extend(slices[0..end].iter().map(|&s| s.to_owned()));
                            pre_length += end;
                            post_length += end;
                        }
                        // context before the hunk within the file

                        // context after the hunk within the file
                        if idx + 1 < hunkcount {
                            let skip = slices.len().saturating_sub(CONTEXT_LINES);
                            acc.extend(slices[skip..].iter().map(|&s| s.to_owned()));
                            let delta = slices.len().saturating_sub(skip);
                            pre_length += delta;
                            post_length += delta;
                        }
                    }
                }
                DiffOp::Insert(ins) => {
                    acc.extend(apply_color(ins, "+", Green));
                    post_length += ins.len();
                }
                DiffOp::Remove(rem) => {
                    acc.extend(apply_color(rem, "-", Red));
                    pre_length += rem.len();
                }
                DiffOp::Replace(rem, ins) => {
                    acc.extend(apply_color(rem, "-", Red));
                    acc.extend(apply_color(ins, "+", Green));
                    pre_length += rem.len();
                    post_length += ins.len();
                }
            };
            echo!(
                "@@ -{},{} +{},{} @@",
                pre_line,
                pre_length,
                post_line,
                post_length
            );
            echo!("{}", acc.join("\n"));

            pre_line += pre_length;
            post_line += post_length;
            acc.clear();
        }

        stdout.flush().await?;
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

            if source != formatted {
                if cmd.diff {
                    if let Err(e) = self.print_diff(&path, &source, &formatted).await {
                        self.env
                            .stderr()
                            .write_all(
                                format!("Failed to write diff to stdout: {:?}", e)
                                    .as_str()
                                    .as_bytes(),
                            )
                            .await?;
                    }
                }

                if cmd.check {
                    tracing::error!(?path, "the file is not properly formatted");
                    result = Err(anyhow!("some files were not properly formatted"));
                } else {
                    self.env.write_file(&path, formatted.as_bytes()).await?;
                }
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
