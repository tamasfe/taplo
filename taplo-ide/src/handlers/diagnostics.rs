use crate::World;
use lsp_async_stub::{Context, RequestWriter};
use lsp_types::*;
use schemars::schema::{InstanceType, Metadata, RootSchema, SingleOrVec};
use taplo::{dom::{self, Common}, parser::Parse, util::coords::Mapper, verify::NodeSpan};
use verify::{
    schemars::errors::{Error, ErrorValue},
    Verifier,
};

// TODO(schema)
#[allow(unreachable_code)]
pub async fn publish_diagnostics(mut context: Context<World>, uri: Url) {
    let w = context.world().lock().await;
    let doc = w.documents.get(&uri).unwrap().clone();

    let diags = collect_toml_diagnostics(&uri, &doc.parse, &doc.mapper);
    drop(w);

    context
        .write_notification::<notification::PublishDiagnostics, _>(Some(PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: diags.clone(),
            version: None,
        }))
        .await
        .unwrap_or_else(|err| log_error!("{}", err));

    // TODO(schema)
    return;

    // Schema-related validations
    let w = context.world().lock().await;

    let mut schema_diag = Vec::new();
    let mut unresolved_schema_name = None;

    if let Some(schema_name) = w.get_schema_name(&uri) {
        if let Some(s) = w.get_schema(schema_name) {
            schema_diag = collect_schema_diagnostics(s, &doc.parse, &uri, &doc.mapper);
        } else {
            unresolved_schema_name = Some(schema_name.to_string());
        }
    }
    drop(w);

    if !schema_diag.is_empty() {
        diags.extend(schema_diag.into_iter());
        context
            .write_notification::<notification::PublishDiagnostics, _>(Some(
                PublishDiagnosticsParams {
                    uri: uri.clone(),
                    diagnostics: diags.clone(),
                    version: None,
                },
            ))
            .await
            .unwrap_or_else(|err| log_error!("{}", err));
    } else if let Some(_schema_name) = unresolved_schema_name {
        // resolve external
        todo!()
    }
}

// Syntax and TOML rules validations
fn collect_toml_diagnostics(uri: &Url, parse: &Parse, mapper: &Mapper) -> Vec<Diagnostic> {
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

    let dom = parse.clone().into_dom();

    for err in dom.errors() {
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
                let r = mapper.range(*range).unwrap();

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

            dom::Error::Generic(err) => {
                diag.push(Diagnostic {
                    range: Default::default(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: err.clone(),
                    related_information: None,
                    tags: None,
                });
            }
        }
    }

    diag
}

// JSON Schema validation
fn collect_schema_diagnostics(
    schema: &RootSchema,
    parse: &Parse,
    uri: &Url,
    mapper: &Mapper,
) -> Vec<Diagnostic> {
    match schema.verify_value(&parse.clone().into_dom()) {
        Ok(_) => Vec::new(),
        Err(errors) => {
            let mut diag = Vec::with_capacity(errors.len());

            for error in errors {
                diag.extend(diags_from_error(error, uri, mapper))
            }

            diag
        }
    }
}

// more user-friendly errors.
fn diags_from_error(error: Error<NodeSpan>, uri: &Url, mapper: &Mapper) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    match error.value {
        ErrorValue::InvalidType {
            mut expected,
            actual,
        } => {
            // null is not a valid type in TOML, in that case the value simply non-existent.
            expected = match expected {
                SingleOrVec::Single(ty) => {
                    if *ty == InstanceType::Null {
                        return diags;
                    }
                    SingleOrVec::Single(ty)
                }
                SingleOrVec::Vec(tys) => {
                    let not_null: Vec<InstanceType> = tys
                        .into_iter()
                        .filter(|ty| *ty != InstanceType::Null)
                        .collect();

                    if not_null.is_empty() {
                        return diags;
                    }

                    SingleOrVec::Vec(not_null)
                }
            };

            diags.push(Diagnostic {
                range: error
                    .span
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: match expected {
                    SingleOrVec::Single(ty) => format!(
                        "invalid type: expected {expected}, not {actual}",
                        expected = format_ty(*ty),
                        actual = format_ty(actual)
                    ),
                    SingleOrVec::Vec(tys) => format!(
                        "invalid type: expected one of {expected}, not {actual}",
                        expected = format_multiple(
                            &tys.iter().map(|ty| format_ty(*ty)).collect::<Vec<String>>(),
                            "or"
                        ),
                        actual = format_ty(actual)
                    ),
                },
                related_information: None,
                tags: None,
            });
        }
        ErrorValue::NoPatternMatch { pattern: _ } => {
            diags.push(Diagnostic {
                range: error
                    .span
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: error
                    .meta
                    .and_then(|meta| meta.title)
                    .map(|title| format!("the value is not a valid {}", title))
                    .unwrap_or_else(|| "the value doesn't match the given pattern".to_string()),
                related_information: None,
                tags: None,
            });
        }
        ErrorValue::NoneValid {
            exclusive,
            schemas,
            errors,
        } => {
            diags.push(Diagnostic {
                range: error
                    .span
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: if schemas.iter().all(|meta| match meta {
                    Some(m) => m.title.is_some(),
                    None => false,
                }) {
                    if exclusive {
                        format!(
                            "the value to be one of {}",
                            format_multiple(
                                &schemas
                                    .iter()
                                    .map(|m| {
                                        m.as_ref().unwrap().title.as_ref().unwrap().to_string()
                                    })
                                    .collect::<Vec<String>>(),
                                "or"
                            )
                        )
                    } else {
                        format!(
                            "the value to match at least one of {}",
                            format_multiple(
                                &schemas
                                    .iter()
                                    .map(|m| {
                                        m.as_ref().unwrap().title.as_ref().unwrap().to_string()
                                    })
                                    .collect::<Vec<String>>(),
                                "or"
                            )
                        )
                    }
                } else if exclusive {
                    format!(
                        "the value must be valid for one of the {} schemas",
                        schemas.len()
                    )
                } else {
                    format!(
                        "the value must be valid for at least one of the {} schemas",
                        schemas.len()
                    )
                },
                related_information: None,
                tags: None,
            });
            for errs in errors {
                for err in errs {
                    diags.extend(diags_from_error(err, uri, mapper))
                }
            }
        }
        ErrorValue::MoreThanOneValid { schemas, matched } => {
            diags.push(Diagnostic {
                range: error
                    .span
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: format!(
                    "Expected the value to be exactly one of {schemas}, but it is valid for {matched}",
                    schemas = format_multiple(
                        &schemas
                            .iter()
                            .map(|m| {
                                m.as_ref().and_then(|m| m.title.clone())
                                    .unwrap_or_else(|| "unnamed".to_string())
                            })
                            .collect::<Vec<String>>(),
                        "or"
                    ),
                    matched = format_multiple(
                        &matched
                            .iter()
                            .map(|m| {
                                m.as_ref().and_then(|m| m.title.clone())
                                    .unwrap_or_else(|| "unnamed".to_string())
                            })
                            .collect::<Vec<String>>(),
                        "and"
                    )
                ),
                related_information: None,
                tags: None,
            });
        }
        ErrorValue::NotUnique { first, duplicate } => {
            diags.push(Diagnostic {
                range: first
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: error.value.to_string(),
                related_information: Some(vec![DiagnosticRelatedInformation {
                    location: Location {
                        range: duplicate
                            .map(|span| mapper.range(span.0).unwrap())
                            .unwrap_or_default(),
                        uri: uri.clone(),
                    },
                    message: "duplicate value".into(),
                }]),
                tags: None,
            });
            diags.push(Diagnostic {
                range: duplicate
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: error.value.to_string(),
                related_information: Some(vec![DiagnosticRelatedInformation {
                    location: Location {
                        range: first
                            .map(|span| mapper.range(span.0).unwrap())
                            .unwrap_or_default(),
                        uri: uri.clone(),
                    },
                    message: "duplicate value".into(),
                }]),
                tags: None,
            });
        }
        ErrorValue::InvalidSchema(err) => log_warn!(
            "{span:?} {err}",
            span = error
                .span
                .map(|span| mapper.range(span.0).unwrap_or_default())
                .unwrap_or_default(),
            err = err
        ),
        error_value => {
            diags.push(Diagnostic {
                range: error
                    .span
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: error_value.to_string(),
                related_information: None,
                tags: None,
            });
        }
    }

    diags
}

fn schema_in_message(meta: &Option<Box<Metadata>>) -> String {
    meta.as_ref()
        .and_then(|meta| meta.title.clone())
        .map(|t| format!(" ({})", t))
        .unwrap_or_default()
}

fn format_ty(ty: InstanceType) -> String {
    let mut ty_string = format!("{:?}", ty).to_lowercase();
    if ty_string == "object" {
        ty_string = "table".to_string();
    }

    ty_string
}

// Pretty formatted lists
fn format_multiple(values: &[String], conj: &str) -> String {
    let mut formatted = String::new();

    for (i, v) in values.iter().enumerate() {
        if i != 0 {
            if i == values.len() - 1 {
                formatted += " ";
                formatted += conj;
                formatted += " ";
            } else {
                formatted += ", ";
            }
        }
        formatted += r#"""#;
        formatted += v;
        formatted += r#"""#;
    }

    formatted
}
