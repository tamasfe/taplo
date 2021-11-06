use crate::{
    config::Config,
    external::{get_paths_by_glob, get_schema, read_file, read_stdin},
    print_message,
};
use clap::ArgMatches;
use pretty_lint::{PrettyLint, Severity};
use regex::Regex;
use schemars::schema::RootSchema;
use std::collections::HashSet;
use taplo::{dom, rowan::TextRange, schema::SchemaIndex, util::coords::Mapper};
use verify::Verifier;

pub(crate) struct LintResult {
    pub matched_document_count: usize,
    pub excluded_document_count: usize,
    pub error_count: usize,
}

pub(crate) async fn lint(
    config: Config,
    m: &ArgMatches,
    schema_index: Option<SchemaIndex>,
) -> LintResult {
    let mut res = LintResult {
        matched_document_count: 0,
        excluded_document_count: 0,
        error_count: 0,
    };

    let mut schema = None;

    if let Some(schema_path) = m.value_of("schema") {
        match get_schema(schema_path, m.value_of("cache-path")).await {
            Ok(s) => schema = Some(s.0),
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

    let cache_path = m.value_of("cache-path").map(|s| s.to_string());

    if let Some(files) = m.values_of("files") {
        lint_paths(
            &config,
            schema_index.as_ref(),
            schema,
            cache_path,
            files,
            &mut res,
            false,
        )
        .await;
    } else {
        lint_paths(
            &config,
            schema_index.as_ref(),
            schema,
            cache_path,
            config.get_include_paths().iter().map(|s| s.as_ref()),
            &mut res,
            true,
        )
        .await;
    }

    res
}

async fn lint_paths<'i, F: Iterator<Item = &'i str>>(
    config: &Config,
    schema_index: Option<&SchemaIndex>,
    schema: Option<RootSchema>,
    cache_path: Option<String>,
    files: F,
    res: &mut LintResult,
    allow_exclude: bool,
) {
    for val in files {
        if val == "-" {
            let src = match read_stdin() {
                Ok(s) => s,
                Err(e) => {
                    print_message(Severity::Error, "error", &e.to_string());
                    res.error_count += 1;
                    continue;
                }
            };

            lint_source(None, schema.as_ref(), &src, res);
            res.matched_document_count += 1;
            continue;
        }

        match get_paths_by_glob(val) {
            Ok(sources) => {
                for path in sources {
                    match read_file(path.to_str().unwrap()).await {
                        Ok(src) => {
                            res.matched_document_count += 1;

                            if allow_exclude {
                                // Don't format taplo config files unless asked explicitly.
                                if let Some(file_name) = path.file_name() {
                                    if file_name == "taplo.toml" || file_name == ".taplo.toml" {
                                        // Don't count it as excluded.
                                        continue;
                                    }
                                }

                                if let Some(p) = path.to_str() {
                                    match config.is_excluded(p) {
                                        Ok(excluded) => {
                                            if excluded {
                                                res.excluded_document_count += 1;
                                                continue;
                                            }
                                        }
                                        Err(err) => {
                                            print_message(
                                                Severity::Error,
                                                "error",
                                                &err.to_string(),
                                            );
                                            res.error_count += 1;
                                            return;
                                        }
                                    }
                                }
                            }

                            let str_src = match std::str::from_utf8(&src) {
                                Ok(s) => s,
                                Err(err) => {
                                    print_message(
                                        Severity::Error,
                                        "error",
                                        &format!("file {:?} is not valid UTF-8: {}", path, err),
                                    );
                                    res.error_count += 1;
                                    continue;
                                }
                            };

                            match &schema {
                                Some(s) => {
                                    lint_source(path.to_str(), Some(s), str_src, res);
                                }
                                None => {
                                    let schema_path = match config
                                        .get_schema_path(path.to_str().unwrap())
                                    {
                                        Ok(p) => match p {
                                            Some(p) => p,
                                            None => match schema_index {
                                                Some(idx) => {
                                                    match idx.schemas.iter().find(|s| {
                                                        s.extra.patterns.iter().any(|p| {
                                                            Regex::new(p)
                                                                .ok()
                                                                .map(|r| {
                                                                    r.is_match(
                                                                        path.to_str().unwrap(),
                                                                    )
                                                                })
                                                                .unwrap_or(false)
                                                        })
                                                    }) {
                                                        Some(s) => s.url.clone(),
                                                        None => {
                                                            lint_source(
                                                                path.to_str(),
                                                                None,
                                                                str_src,
                                                                res,
                                                            );
                                                            continue;
                                                        }
                                                    }
                                                }
                                                None => {
                                                    lint_source(path.to_str(), None, str_src, res);
                                                    continue;
                                                }
                                            },
                                        },
                                        Err(err) => {
                                            print_message(
                                                Severity::Error,
                                                "error",
                                                &format!("invalid config: {}", err),
                                            );
                                            res.error_count += 1;
                                            continue;
                                        }
                                    };

                                    match get_schema(&schema_path, cache_path.as_ref()).await {
                                        Ok(schema) => {
                                            lint_source(
                                                path.to_str(),
                                                Some(&schema.0),
                                                str_src,
                                                res,
                                            );
                                        }
                                        Err(err) => {
                                            print_message(
                                                Severity::Error,
                                                "error",
                                                &format!("could not load schema: {}", err),
                                            );
                                            res.error_count += 1;
                                            continue;
                                        }
                                    };
                                }
                            }
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
                res.error_count += 1;
                break;
            }
        }
    }
}

fn lint_source(path: Option<&str>, schema: Option<&RootSchema>, src: &str, res: &mut LintResult) {
    let parse = taplo::parser::parse(src);

    let mapper = Mapper::new_utf16(src, false);

    // So that same range lint aren't applied twice.
    let mut ranges: HashSet<TextRange> = HashSet::new();

    let fpath = path.unwrap_or("");

    if !parse.errors.is_empty() {
        for err in &parse.errors {
            if ranges.contains(&err.range) {
                continue;
            }
            res.error_count += 1;

            eprintln!(
                "{}",
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
                                col: r.end.character as usize,
                            },
                        }
                    })
                    .with_inline_message(&err.message)
                    .to_string(),
            );

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
                    eprintln!(
                        "{}",
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
                                        col: r.end.character as usize,
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
                                                col: r.end.character as usize,
                                            },
                                        }
                                    })
                                    .with_inline_message("already defined here"),
                            )
                            .to_string(),
                    );
                }
                dom::Error::ExpectedTableArray { target, key } => {
                    eprintln!(
                        "{}",
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
                                        col: r.end.character as usize,
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
                                                col: r.end.character as usize,
                                            },
                                        }
                                    })
                                    .with_inline_message("array of tables here"),
                            )
                            .to_string(),
                    );
                }
                dom::Error::ExpectedTable { target, key } => {
                    eprintln!(
                        "{}",
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
                                        col: r.end.character as usize,
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
                                                col: r.end.character as usize,
                                            },
                                        }
                                    })
                                    .with_inline_message("required by this"),
                            )
                            .to_string(),
                    );
                }
                dom::Error::InlineTable { target, key } => {
                    eprintln!(
                        "{}",
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
                                        col: r.end.character as usize,
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
                                                col: r.end.character as usize,
                                            },
                                        }
                                    })
                                    .with_inline_message("modified here"),
                            )
                            .to_string(),
                    );
                }
                dom::Error::Spanned { range, message } => {
                    eprintln!(
                        "{}",
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
                                        col: r.end.character as usize,
                                    },
                                }
                            })
                            .with_inline_message("inline table here")
                            .to_string(),
                    );
                }
                dom::Error::Generic(g) => {
                    if fpath.is_empty() {
                        print_message(Severity::Error, "error", g);
                    } else {
                        print_message(Severity::Error, "error", &format!("({}) {}", fpath, g));
                    }
                }
                dom::Error::DottedKeyConflict { first, second } => {
                    eprintln!("{}",
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
                                        col: r.end.character as usize,
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
                                                col: r.end.character as usize,
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
                }
                dom::Error::SubTableBeforeTableArray { target, key } => {
                    eprintln!(
                        "{}",
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
                                        col: r.end.character as usize,
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
                                                col: r.end.character as usize,
                                            },
                                        }
                                    })
                                    .with_inline_message("already defined here"),
                            )
                            .to_string(),
                    );
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
                                col: r.end.character as usize,
                            },
                        }
                    })
                    .with_inline_message(err_str);

                if let Some(p) = path {
                    p_lint = p_lint.with_file_path(p);
                }

                eprintln!("{}", &p_lint.to_string());

                res.error_count += 1;
            }
        }
    }
}
