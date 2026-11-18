# printing.print_semantic_errors

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\printing.rs` (lines 50-116)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn print_semantic_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: impl Iterator<Item = dom::Error>,
    ) -> Result<(), anyhow::Error> {
        let mut out_diag = Vec::<u8>::new();

        let config = codespan_reporting::term::Config::default();

        for error in errors {
            let diag = match &error {
                dom::Error::ConflictingKeys { key, other } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([
                        Label::primary((), std_range(key.text_ranges().next().unwrap()))
                            .with_message("duplicate key"),
                        Label::secondary((), std_range(other.text_ranges().next().unwrap()))
                            .with_message("duplicate found here"),
                    ])),
                dom::Error::ExpectedArrayOfTables {
                    not_array_of_tables,
                    required_by,
                } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([
                        Label::primary(
                            (),
                            std_range(not_array_of_tables.text_ranges().next().unwrap()),
                        )
                        .with_message("expected array of tables"),
                        Label::secondary((), std_range(required_by.text_ranges().next().unwrap()))
                            .with_message("required by this key"),
                    ])),
                dom::Error::ExpectedTable {
                    not_table,
                    required_by,
                } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([
                        Label::primary((), std_range(not_table.text_ranges().next().unwrap()))
                            .with_message("expected table"),
                        Label::secondary((), std_range(required_by.text_ranges().next().unwrap()))
                            .with_message("required by this key"),
                    ])),
                dom::Error::InvalidEscapeSequence { string } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([Label::primary(
                        (),
                        std_range(string.text_range()),
                    )
                    .with_message("the string contains invalid escape sequences")])),
                _ => {
                    unreachable!("this is a bug")
                }
            };

            if self.colors {
                term::emit(&mut Ansi::new(&mut out_diag), &config, file, &diag)?;
            } else {
                term::emit(&mut NoColor::new(&mut out_diag), &config, file, &diag)?;
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
    pub(crate) async fn print_semantic_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: impl Iterator<Item = dom::Error>,
    ) -> Result<(), anyhow::Error> {
        let mut out_diag = Vec::<u8>::new();

        let config = codespan_reporting::term::Config::default();

        for error in errors {
            let diag = match &error {
                dom::Error::ConflictingKeys { key, other } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([
                        Label::primary((), std_range(key.text_ranges().next().unwrap()))
                            .with_message("duplicate key"),
                        Label::secondary((), std_range(other.text_ranges().next().unwrap()))
                            .with_message("duplicate found here"),
                    ])),
                dom::Error::ExpectedArrayOfTables {
                    not_array_of_tables,
                    required_by,
                } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([
                        Label::primary(
                            (),
                            std_range(not_array_of_tables.text_ranges().next().unwrap()),
                        )
                        .with_message("expected array of tables"),
                        Label::secondary((), std_range(required_by.text_ranges().next().unwrap()))
                            .with_message("required by this key"),
                    ])),
                dom::Error::ExpectedTable {
                    not_table,
                    required_by,
                } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([
                        Label::primary((), std_range(not_table.text_ranges().next().unwrap()))
                            .with_message("expected table"),
                        Label::secondary((), std_range(required_by.text_ranges().next().unwrap()))
                            .with_message("required by this key"),
                    ])),
                dom::Error::InvalidEscapeSequence { string } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([Label::primary(
                        (),
                        std_range(string.text_range()),
                    )
                    .with_message("the string contains invalid escape sequences")])),
                _ => {
                    unreachable!("this is a bug")
                }
            };

            if self.colors {
                term::emit(&mut Ansi::new(&mut out_diag), &config, file, &diag)?;
            } else {
                term::emit(&mut NoColor::new(&mut out_diag), &config, file, &diag)?;
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
