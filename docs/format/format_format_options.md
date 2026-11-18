# format.format_options

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\format.rs` (lines 283-306)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
