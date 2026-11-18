# format.format_files

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\format.rs` (lines 204-281)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 19
- Authors: 6
- Bug fixes: 6
- Features: 2

## Callers

None

## Callees

None
