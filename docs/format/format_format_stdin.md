# format.format_stdin

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\format.rs` (lines 24-81)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
