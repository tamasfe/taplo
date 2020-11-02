use crate::{
    config::Config,
    external::{eprint_line, get_paths_by_glob, get_stdin_source, print_message, read_file},
};
use clap::ArgMatches;
use pretty_lint::{PrettyLint, Severity};
use schemars::schema::RootSchema;
use std::collections::HashSet;
use taplo::{dom, rowan::TextRange, util::coords::Mapper};
use verify::Verifier;

pub(crate) struct LintResult {
    pub matched_document_count: usize,
    pub excluded_document_count: usize,
    pub error_count: usize,
}

pub(crate) fn lint(config: Config, m: &ArgMatches) -> LintResult {
    let mut res = LintResult {
        matched_document_count: 0,
        excluded_document_count: 0,
        error_count: 0,
    };

    let mut schema = None;

    if let Some(schema_path) = m.value_of("schema") {
        match read_file(schema_path.as_ref()) {
            Ok(schema_content) => match serde_json::from_str::<RootSchema>(&schema_content) {
                Ok(rs) => {
                    schema = Some(rs);
                }
                Err(err) => {
                    print_message(
                        Severity::Error,
                        "error",
                        &format!("could not load schema: {}", err),
                    );
                    res.error_count += 1;
                    return res;
                }
            },
            Err(err) => {
                print_message(
                    Severity::Error,
                    "error",
                    &format!("could not load schema: {}", err),
                );

                res.error_count += 1;
                return res;
            }
        }
    }

    if schema.is_none() {
        if let Some(schema_opts) = &config.global_options.schema {
            if schema_opts.enabled.unwrap_or(false) {
                if let Some(schema_path) = &schema_opts.path {
                    match read_file(schema_path.as_ref()) {
                        Ok(schema_content) => {
                            match serde_json::from_str::<RootSchema>(&schema_content) {
                                Ok(rs) => {
                                    schema = Some(rs);
                                }
                                Err(err) => {
                                    print_message(
                                        Severity::Error,
                                        "error",
                                        &format!("could not load schema: {}", err),
                                    );
                                    res.error_count += 1;
                                    return res;
                                }
                            }
                        }
                        Err(err) => {
                            print_message(
                                Severity::Error,
                                "error",
                                &format!("could not load schema: {}", err),
                            );

                            res.error_count += 1;
                            return res;
                        }
                    }
                }
            }
        }
    }

    if let Some(files) = m.values_of("files") {
        lint_paths(&config, schema.as_ref(), files, &mut res);
    } else {
        lint_paths(
            &config,
            schema.as_ref(),
            config.get_include_paths().iter().map(|s| s.as_ref()),
            &mut res,
        );
    }

    res
}

fn lint_paths<'i, F: Iterator<Item = &'i str>>(
    config: &Config,
    schema: Option<&RootSchema>,
    files: F,
    res: &mut LintResult,
) {
    for val in files {
        if val == "-" {
            let src = match get_stdin_source() {
                Ok(s) => s,
                Err(e) => {
                    print_message(Severity::Error, "error", &e.to_string());
                    continue;
                }
            };

            lint_source(None, schema, &src, res);
            res.matched_document_count += 1;
            continue;
        }

        match config.is_excluded(val) {
            Ok(excluded) => {
                if excluded {
                    res.excluded_document_count += 1;
                    continue;
                }
            }
            Err(err) => {
                print_message(Severity::Error, "error", &err.to_string());
                return;
            }
        }

        match get_paths_by_glob(val) {
            Ok((sources, errors)) => {
                for err in errors {
                    print_message(Severity::Error, "error", &err.to_string());
                    res.error_count += 1;
                }

                for path in sources {
                    match read_file(&path) {
                        Ok(src) => {
                            lint_source(path.to_str(), schema, &src, res);
                            res.matched_document_count += 1;
                        }
                        Err(err) => {
                            print_message(Severity::Error, "error", &err.to_string());
                            res.error_count += 1;
                        }
                    }
                }
            }
            Err(e) => {
                print_message(Severity::Error, "error", &e.to_string());
                break;
            }
        }
    }
}

fn lint_source(path: Option<&str>, schema: Option<&RootSchema>, src: &str, res: &mut LintResult) {
    let parse = taplo::parser::parse(src);

    let mapper = Mapper::new(src);

    // So that same range lint aren't applied twice.
    let mut ranges: HashSet<TextRange> = HashSet::new();

    let fpath = match path {
        Some(p) => p,
        None => "",
    };

    if !parse.errors.is_empty() {
        for err in &parse.errors {
            if ranges.contains(&err.range) {
                continue;
            }
            res.error_count += 1;

            eprint_line(
                &PrettyLint::error(src)
                    .with_file_path(fpath)
                    .with_message("invalid syntax")
                    .at({
                        let r = mapper.range(err.range).unwrap();
                        pretty_lint::Span {
                            start: pretty_lint::Position {
                                line: r.start.line as usize + 1,
                                col: r.start.character as usize + 1,
                            },
                            end: pretty_lint::Position {
                                line: r.end.line as usize + 1,
                                col: r.end.character as usize + 1,
                            },
                        }
                    })
                    .with_inline_message(&err.message)
                    .to_string(),
            );
            eprint_line("");

            ranges.insert(err.range);
        }

        return;
    }

    let dom = parse.into_dom();
    ranges.clear();

    if !dom.errors().is_empty() {
        for err in dom.errors() {
            match err {
                dom::Error::DuplicateKey { first, second } => {
                    eprint_line(
                        &PrettyLint::error(src)
                            .with_file_path(fpath)
                            .with_message(&format!(
                                r#"duplicate key "{}""#,
                                second.full_key_string()
                            ))
                            .at({
                                let r = mapper.range(second.text_ranges()[0]).unwrap();
                                pretty_lint::Span {
                                    start: pretty_lint::Position {
                                        line: r.start.line as usize + 1,
                                        col: r.start.character as usize + 1,
                                    },
                                    end: pretty_lint::Position {
                                        line: r.end.line as usize + 1,
                                        col: r.end.character as usize + 1,
                                    },
                                }
                            })
                            .with_inline_message("defined here")
                            .and(
                                PrettyLint::error(src)
                                    .with_file_path(fpath)
                                    .at({
                                        let r = mapper.range(first.text_ranges()[0]).unwrap();
                                        pretty_lint::Span {
                                            start: pretty_lint::Position {
                                                line: r.start.line as usize + 1,
                                                col: r.start.character as usize + 1,
                                            },
                                            end: pretty_lint::Position {
                                                line: r.end.line as usize + 1,
                                                col: r.end.character as usize + 1,
                                            },
                                        }
                                    })
                                    .with_inline_message("already defined here"),
                            )
                            .to_string(),
                    );
                    eprint_line("");
                }
                dom::Error::ExpectedTableArray { target, key } => {
                    eprint_line(
                        &PrettyLint::error(src)
                            .with_file_path(fpath)
                            .with_message(&format!(
                                r#"entry "{}" conflicts with array of tables"#,
                                key.full_key_string()
                            ))
                            .at({
                                let r = mapper.range(key.text_ranges()[0]).unwrap();
                                pretty_lint::Span {
                                    start: pretty_lint::Position {
                                        line: r.start.line as usize + 1,
                                        col: r.start.character as usize + 1,
                                    },
                                    end: pretty_lint::Position {
                                        line: r.end.line as usize + 1,
                                        col: r.end.character as usize + 1,
                                    },
                                }
                            })
                            .with_inline_message("entry defined here")
                            .and(
                                PrettyLint::error(src)
                                    .with_file_path(fpath)
                                    .at({
                                        let r = mapper.range(target.text_ranges()[0]).unwrap();
                                        pretty_lint::Span {
                                            start: pretty_lint::Position {
                                                line: r.start.line as usize + 1,
                                                col: r.start.character as usize + 1,
                                            },
                                            end: pretty_lint::Position {
                                                line: r.end.line as usize + 1,
                                                col: r.end.character as usize + 1,
                                            },
                                        }
                                    })
                                    .with_inline_message("array of tables here"),
                            )
                            .to_string(),
                    );
                    eprint_line("");
                }
                dom::Error::ExpectedTable { target, key } => {
                    eprint_line(
                        &PrettyLint::error(src)
                            .with_file_path(fpath)
                            .with_message(&format!(
                                r#"expected "{}" to be a table"#,
                                target.full_key_string()
                            ))
                            .at({
                                let r = mapper.range(target.text_ranges()[0]).unwrap();
                                pretty_lint::Span {
                                    start: pretty_lint::Position {
                                        line: r.start.line as usize + 1,
                                        col: r.start.character as usize + 1,
                                    },
                                    end: pretty_lint::Position {
                                        line: r.end.line as usize + 1,
                                        col: r.end.character as usize + 1,
                                    },
                                }
                            })
                            .with_inline_message("entry defined here")
                            .and(
                                PrettyLint::error(src)
                                    .with_file_path(fpath)
                                    .at({
                                        let r = mapper.range(key.text_ranges()[0]).unwrap();
                                        pretty_lint::Span {
                                            start: pretty_lint::Position {
                                                line: r.start.line as usize + 1,
                                                col: r.start.character as usize + 1,
                                            },
                                            end: pretty_lint::Position {
                                                line: r.end.line as usize + 1,
                                                col: r.end.character as usize + 1,
                                            },
                                        }
                                    })
                                    .with_inline_message("required by this"),
                            )
                            .to_string(),
                    );
                    eprint_line("");
                }
                dom::Error::InlineTable { target, key } => {
                    eprint_line(
                        &PrettyLint::error(src)
                            .with_file_path(fpath)
                            .with_message(&format!(
                                r#"inline table "{}" cannot be modified"#,
                                target.full_key_string()
                            ))
                            .at({
                                let r = mapper.range(target.text_ranges()[0]).unwrap();
                                pretty_lint::Span {
                                    start: pretty_lint::Position {
                                        line: r.start.line as usize + 1,
                                        col: r.start.character as usize + 1,
                                    },
                                    end: pretty_lint::Position {
                                        line: r.end.line as usize + 1,
                                        col: r.end.character as usize + 1,
                                    },
                                }
                            })
                            .with_inline_message("inline table here")
                            .and(
                                PrettyLint::error(src)
                                    .with_file_path(fpath)
                                    .at({
                                        let r = mapper.range(key.text_ranges()[0]).unwrap();
                                        pretty_lint::Span {
                                            start: pretty_lint::Position {
                                                line: r.start.line as usize + 1,
                                                col: r.start.character as usize + 1,
                                            },
                                            end: pretty_lint::Position {
                                                line: r.end.line as usize + 1,
                                                col: r.end.character as usize + 1,
                                            },
                                        }
                                    })
                                    .with_inline_message("modified here"),
                            )
                            .to_string(),
                    );
                    eprint_line("");
                }
                dom::Error::Spanned { range, message } => {
                    eprint_line(
                        &PrettyLint::error(src)
                            .with_file_path(fpath)
                            .with_message(message)
                            .at({
                                let r = mapper.range(*range).unwrap();
                                pretty_lint::Span {
                                    start: pretty_lint::Position {
                                        line: r.start.line as usize + 1,
                                        col: r.start.character as usize + 1,
                                    },
                                    end: pretty_lint::Position {
                                        line: r.end.line as usize + 1,
                                        col: r.end.character as usize + 1,
                                    },
                                }
                            })
                            .with_inline_message("inline table here")
                            .to_string(),
                    );
                    eprint_line("");
                }
                dom::Error::Generic(g) => {
                    if fpath.is_empty() {
                        print_message(Severity::Error, "error", g);
                    } else {
                        print_message(Severity::Error, "error", &format!("({}) {}", fpath, g));
                    }
                }
                dom::Error::DottedKeyConflict { first, second } => {
                    eprint_line(
                        &PrettyLint::error(src)
                            .with_file_path(fpath)
                            .with_message(
                                r#"conflicting dotted keys"#,
                            )
                            .at({
                                let r = mapper.range(first.text_ranges()[0]).unwrap();
                                pretty_lint::Span {
                                    start: pretty_lint::Position {
                                        line: r.start.line as usize + 1,
                                        col: r.start.character as usize + 1,
                                    },
                                    end: pretty_lint::Position {
                                        line: r.end.line as usize + 1,
                                        col: r.end.character as usize + 1,
                                    },
                                }
                            })
                            .with_inline_message("conflicting keys here")
                            .and(
                                PrettyLint::error(src)
                                    .with_file_path(fpath)
                                    .at({
                                        let r = mapper.range(second.text_ranges()[0]).unwrap();
                                        pretty_lint::Span {
                                            start: pretty_lint::Position {
                                                line: r.start.line as usize + 1,
                                                col: r.start.character as usize + 1,
                                            },
                                            end: pretty_lint::Position {
                                                line: r.end.line as usize + 1,
                                                col: r.end.character as usize + 1,
                                            },
                                        }
                                    })
                                    .with_inline_message("already defined here"),
                            )
                            .with_notes(
                                &[
                                    "entries sharing dotted keys cannot fully define intermediate tables and values",
                                    "make sure that the path consists of equal amount of keys to avoid conflicts",
                                ]
                            )
                            .to_string(),
                    );
                    eprint_line("");
                }
                dom::Error::SubTableBeforeTableArray { target, key } => {
                    eprint_line(
                        &PrettyLint::error(src)
                            .with_file_path(fpath)
                            .with_message(r#"subtable is before array of tables"#)
                            .at({
                                let r = mapper.range(target.text_ranges()[0]).unwrap();
                                pretty_lint::Span {
                                    start: pretty_lint::Position {
                                        line: r.start.line as usize + 1,
                                        col: r.start.character as usize + 1,
                                    },
                                    end: pretty_lint::Position {
                                        line: r.end.line as usize + 1,
                                        col: r.end.character as usize + 1,
                                    },
                                }
                            })
                            .with_inline_message("array of tables here")
                            .and(
                                PrettyLint::error(src)
                                    .with_file_path(fpath)
                                    .at({
                                        let r = mapper.range(key.text_ranges()[0]).unwrap();
                                        pretty_lint::Span {
                                            start: pretty_lint::Position {
                                                line: r.start.line as usize + 1,
                                                col: r.start.character as usize + 1,
                                            },
                                            end: pretty_lint::Position {
                                                line: r.end.line as usize + 1,
                                                col: r.end.character as usize + 1,
                                            },
                                        }
                                    })
                                    .with_inline_message("already defined here"),
                            )
                            .to_string(),
                    );
                    eprint_line("");
                }
            };

            res.error_count += 1;
        }

        return;
    }

    if let Some(s) = schema {
        if let Err(errors) = s.verify_value(&dom) {
            for err in errors {
                let range = match err.span {
                    Some(s) => s.0,
                    None => {
                        continue;
                    }
                };

                let err_str: &str = &err.value.to_string();

                // TODO better messages
                let mut p_lint = PrettyLint::error(src)
                    .with_message("failed schema validation")
                    .at({
                        let r = mapper.range(range).unwrap();
                        pretty_lint::Span {
                            start: pretty_lint::Position {
                                line: r.start.line as usize + 1,
                                col: r.start.character as usize + 1,
                            },
                            end: pretty_lint::Position {
                                line: r.end.line as usize + 1,
                                col: r.end.character as usize + 1,
                            },
                        }
                    })
                    .with_inline_message(err_str);

                if let Some(p) = path {
                    p_lint = p_lint.with_file_path(p);
                }

                eprint_line(&p_lint.to_string());
                eprint_line("");

                res.error_count += 1;
            }
        }
    }
}
