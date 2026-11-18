# printing.print_schema_errors

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\printing.rs` (lines 119-148)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn print_schema_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: &[NodeValidationError],
    ) -> Result<(), anyhow::Error> {
        let config = codespan_reporting::term::Config::default();

        let mut out_diag = Vec::<u8>::new();
        for err in errors {
            let msg = err.error.to_string();
            for text_range in err.text_ranges() {
                let diag = Diagnostic::error()
                    .with_message(err.error.to_string())
                    .with_labels(Vec::from([
                        Label::primary((), std_range(text_range)).with_message(&msg)
                    ]));

                if self.colors {
                    term::emit(&mut Ansi::new(&mut out_diag), &config, file, &diag)?;
                } else {
                    term::emit(&mut NoColor::new(&mut out_diag), &config, file, &diag)?;
                };
            }
        }
        let mut stderr = self.env.stderr();
        stderr.write_all(&out_diag).await?;
        stderr.flush().await?;

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) async fn print_schema_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: &[NodeValidationError],
    ) -> Result<(), anyhow::Error> {
        let config = codespan_reporting::term::Config::default();

        let mut out_diag = Vec::<u8>::new();
        for err in errors {
            let msg = err.error.to_string();
            for text_range in err.text_ranges() {
                let diag = Diagnostic::error()
                    .with_message(err.error.to_string())
                    .with_labels(Vec::from([
                        Label::primary((), std_range(text_range)).with_message(&msg)
                    ]));

                if self.colors {
                    term::emit(&mut Ansi::new(&mut out_diag), &config, file, &diag)?;
                } else {
                    term::emit(&mut NoColor::new(&mut out_diag), &config, file, &diag)?;
                };
            }
        }
        let mut stderr = self.env.stderr();
        stderr.write_all(&out_diag).await?;
        stderr.flush().await?;

        Ok(())
    }
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
