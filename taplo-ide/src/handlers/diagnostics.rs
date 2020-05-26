use lsp_types::*;
use taplo::{dom, parser::Parse, util::coords::Mapper};

pub fn collect_diagnostics(uri: &Url, parse: &Parse, mapper: &Mapper) -> Vec<Diagnostic> {
    let mut diag: Vec<Diagnostic> = parse
        .errors
        .iter()
        .map(|e| {
            let range = mapper.range(e.range).unwrap();
            Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some("Even Better TOML".into()),
                message: e.message.clone(),
                related_information: None,
                tags: None,
            }
        })
        .collect();

    if !diag.is_empty() {
        // Only syntax errors
        return diag;
    }

    for err in parse.clone().into_dom().errors() {
        match err {
            dom::Error::DuplicateKey { first, second } => {
                let first_range = mapper.range(first.text_range()).unwrap();
                let second_range = mapper.range(second.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: first_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"duplicate key "{}""#, first.full_key_string()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range,
                            uri: uri.clone(),
                        },
                        message: "other declaration".into(),
                    }]),
                    tags: None,
                });

                diag.push(Diagnostic {
                    range: second_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"duplicate key "{}""#, first.full_key_string()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: first_range,
                            uri: uri.clone(),
                        },
                        message: "first declaration".into(),
                    }]),
                    tags: None,
                });
            }
            dom::Error::ExpectedTable { target, key } => {
                let target_range = mapper.range(target.text_range()).unwrap();
                let second_range = mapper.range(key.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"expected table for "{}""#, target.full_key_string()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"required by "{}""#, key.full_key_string()),
                    }]),
                    tags: None,
                });
            }
            dom::Error::ExpectedTableArray { target, key } => {
                let target_range = mapper.range(target.text_range()).unwrap();
                let second_range = mapper.range(key.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: "array conflicts with array of tables".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range,
                            uri: uri.clone(),
                        },
                        message: "array of tables declaration".to_string(),
                    }]),
                    tags: None,
                });

                diag.push(Diagnostic {
                    range: second_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: "array conflicts with array of tables".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: target_range,
                            uri: uri.clone(),
                        },
                        message: "array declaration".to_string(),
                    }]),
                    tags: None,
                });
            }
            dom::Error::InlineTable { target, key } => {
                let target_range = mapper.range(target.text_range()).unwrap();
                let second_range = mapper.range(key.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: "inline table cannot be modified".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"modified here by "{}""#, key.full_key_string()),
                    }]),
                    tags: None,
                });

                diag.push(Diagnostic {
                    range: second_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: "inline table cannot be modified".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: target_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"inline table "{}" here"#, target.full_key_string()),
                    }]),
                    tags: None,
                });
            }
            dom::Error::TopLevelTableDefined { table, key } => {
                let table_range = mapper.range(table.text_range()).unwrap();
                let key_range = mapper.range(key.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: table_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: "table conflicts with entry".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: key_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"entry here "{}""#, key.full_key_string()),
                    }]),
                    tags: None,
                });

                diag.push(Diagnostic {
                    range: key_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: "entry conflicts with table".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: table_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"table "{}" here"#, table.full_key_string()),
                    }]),
                    tags: None,
                });
            }
            dom::Error::Spanned { range, message } => {
                let r = mapper.range(range.clone()).unwrap();

                diag.push(Diagnostic {
                    range: r,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: message.clone(),
                    related_information: None,
                    tags: None,
                });
            }

            dom::Error::Generic(_) => {
                // todo show this as well somewhere?
            }
        }
    }

    diag
}
