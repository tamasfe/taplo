use crate::Taplo;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{Ansi, NoColor},
    },
};
use itertools::Itertools;
use std::ops::Range;
use taplo::{dom, parser, rowan::TextRange};
use taplo_common::{environment::Environment, schema::NodeValidationError};
use tokio::io::AsyncWriteExt;

impl<E: Environment> Taplo<E> {
    pub(crate) async fn print_parse_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: &[parser::Error],
    ) -> Result<(), anyhow::Error> {
        let mut out_diag = Vec::<u8>::new();

        let config = codespan_reporting::term::Config::default();

        for error in errors.iter().unique_by(|e| e.range) {
            let diag = Diagnostic::error()
                .with_message("invalid TOML")
                .with_labels(Vec::from([
                    Label::primary((), std_range(error.range)).with_message(&error.message)
                ]));

            if self.colors {
                term::emit(&mut Ansi::new(&mut out_diag), &config, file, &diag)?;
            } else {
                term::emit(&mut NoColor::new(&mut out_diag), &config, file, &diag)?;
            }
        }

        self.env.stderr().write_all(&out_diag).await?;

        Ok(())
    }

    pub(crate) async fn print_semantic_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: &[dom::Error],
    ) -> Result<(), anyhow::Error> {
        let mut out_diag = Vec::<u8>::new();

        let config = codespan_reporting::term::Config::default();

        for error in errors.iter() {
            // TODO
            // let diag = match error {
            //     dom::Error::DuplicateKey { first, second } => Diagnostic::error()
            //         .with_message(error.to_string())
            //         .with_labels(Vec::from([
            //             Label::primary((), std_range(first.text_ranges()[0]))
            //                 .with_message("duplicate key"),
            //             Label::secondary((), std_range(second.text_ranges()[0]))
            //                 .with_message("duplicate found here"),
            //         ])),
            //     dom::Error::DottedKeyConflict { first, second } => Diagnostic::error()
            //         .with_message(error.to_string())
            //         .with_labels(Vec::from([
            //             Label::primary((), std_range(first.text_ranges()[0]))
            //                 .with_message("first dotted key"),
            //             Label::secondary((), std_range(second.text_ranges()[0]))
            //                 .with_message("overlapping dotted key here"),
            //         ])),
            //     dom::Error::ExpectedTableArray { target, key } => Diagnostic::error()
            //         .with_message(error.to_string())
            //         .with_labels(Vec::from([
            //             Label::primary((), std_range(target.text_ranges()[0]))
            //                 .with_message("conflicting key"),
            //             Label::secondary((), std_range(key.text_ranges()[0]))
            //                 .with_message("array of tables here"),
            //         ])),
            //     dom::Error::ExpectedTable { target, key } => Diagnostic::error()
            //         .with_message(error.to_string())
            //         .with_labels(Vec::from([
            //             Label::primary((), std_range(target.text_ranges()[0]))
            //                 .with_message("expected table here"),
            //             Label::secondary((), std_range(key.text_ranges()[0]))
            //                 .with_message("required by this table"),
            //         ])),
            //     dom::Error::InlineTable { target, key } => Diagnostic::error()
            //         .with_message(error.to_string())
            //         .with_labels(Vec::from([
            //             Label::primary((), std_range(target.text_ranges()[0]))
            //                 .with_message("inline table here"),
            //             Label::secondary((), std_range(key.text_ranges()[0]))
            //                 .with_message("modified here"),
            //         ])),
            //     dom::Error::SubTableBeforeTableArray { target, key } => Diagnostic::error()
            //         .with_message(error.to_string())
            //         .with_labels(Vec::from([
            //             Label::primary((), std_range(target.text_ranges()[0]))
            //                 .with_message("subtable here"),
            //             Label::secondary((), std_range(key.text_ranges()[0]))
            //                 .with_message("array of tables here"),
            //         ])),
            //     dom::Error::Spanned { range, message } => Diagnostic::error()
            //         .with_message(error.to_string())
            //         .with_labels(Vec::from([
            //             Label::primary((), std_range(*range)).with_message(message)
            //         ])),
            //     dom::Error::Generic(_) => Diagnostic::error().with_message(error.to_string()),
            // };

            // if self.colors {
            //     term::emit(&mut Ansi::new(&mut out_diag), &config, file, &diag)?;
            // } else {
            //     term::emit(&mut NoColor::new(&mut out_diag), &config, file, &diag)?;
            // }
        }

        self.env.stderr().write_all(&out_diag).await?;

        Ok(())
    }

    pub(crate) async fn print_schema_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: &[NodeValidationError],
    ) -> Result<(), anyhow::Error> {
        let config = codespan_reporting::term::Config::default();

        let mut out_diag = Vec::<u8>::new();
        for err in errors {
            let msg = err.error.to_string();
            for text_range in err.node.text_ranges() {
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
        self.env.stderr().write_all(&out_diag).await?;

        Ok(())
    }
}

fn std_range(range: TextRange) -> Range<usize> {
    let start: usize = u32::from(range.start()) as _;
    let end: usize = u32::from(range.end()) as _;
    start..end
}
