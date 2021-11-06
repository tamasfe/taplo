use crate::{utils::LspExt, World, WorldState};
use lsp_async_stub::{Context, RequestWriter};
use lsp_types::*;
use schemars::schema::{InstanceType, Metadata, RootSchema, SingleOrVec};
use taplo::{
    dom::{self, NodeSyntax},
    parser::Parse,
    util::coords::Mapper,
    verify::NodeSpan,
};
use verify::{
    schemars::errors::{Error, ErrorValue},
    Verifier,
};

#[cfg(target_arch = "wasm32")]
use crate::external::UrlExt;

pub async fn publish_diagnostics(mut context: Context<World>, uri: Url) {
    let w = context.world().read();

    let doc = match w.documents.get(&uri) {
        Some(d) => d.clone(),
        None => {
            // Doesn't exist anymore
            return;
        }
    };

    let excluded = w
        .taplo_config
        .as_ref()
        .map(|c| {
            c.is_excluded(uri.to_file_path().unwrap().to_str().unwrap())
                .ok()
                .unwrap_or(false)
        })
        .unwrap_or(false);

    if excluded {
        drop(w);
        context
            .write_notification::<notification::PublishDiagnostics, _>(Some(
                PublishDiagnosticsParams {
                    uri: uri.clone(),
                    diagnostics: vec![Diagnostic {
                        range: Default::default(),
                        severity: Some(DiagnosticSeverity::Hint),
                        code: None,
                        code_description: None,
                        source: Some("Even Better TOML".into()),
                        message: "this file was excluded based on Taplo config".to_string(),
                        tags: None,
                        data: None,
                        ..Default::default()
                    }],
                    version: None,
                },
            ))
            .await
            .unwrap_or_else(|err| log_error!("{}", err));

        return;
    }

    let mut diags = collect_toml_diagnostics(&uri, &doc.parse, &doc.mapper);
    drop(w);

    context
        .write_notification::<notification::PublishDiagnostics, _>(Some(PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: diags.clone(),
            version: None,
        }))
        .await
        .unwrap_or_else(|err| log_error!("{}", err));

    // Schema-related validations

    // If there are errors already, further ones
    // could be misleading
    if !diags.is_empty() {
        return;
    }

    let w = context.world().read();

    if !w.configuration.schema.enabled.unwrap_or_default() {
        return;
    }

    let mut schema_diag = Vec::new();
    match w.get_schema_name(&uri) {
        Some(schema_path) => {
            drop(w);
            match WorldState::get_schema(&uri, &schema_path, context.clone()).await {
                Ok(s) => {
                    schema_diag = collect_schema_diagnostics(&s, &doc.parse, &uri, &doc.mapper);
                }
                Err(err) => {
                    log_error!("failed to load schema: {}", err);
                }
            }
        }
        None => drop(w),
    };

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
    }
}

pub async fn clear_diagnostics(mut context: Context<World>, uri: Url) {
    context
        .write_notification::<notification::PublishDiagnostics, _>(Some(PublishDiagnosticsParams {
            uri,
            diagnostics: Vec::new(),
            version: None,
        }))
        .await
        .unwrap_or_else(|err| log_error!("{}", err));
}

// Syntax and TOML rules validations
fn collect_toml_diagnostics(uri: &Url, parse: &Parse, mapper: &Mapper) -> Vec<Diagnostic> {
    let mut diag: Vec<Diagnostic> = parse
        .errors
        .iter()
        .map(|e| {
            let range = mapper.range(e.range).unwrap_or_default().into_lsp();
            Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                code_description: None,
                source: Some("Even Better TOML".into()),
                message: e.message.clone(),
                related_information: None,
                tags: None,
                data: None,
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
                let first_range = mapper.range(first.syntax().text_range()).unwrap();
                let second_range = mapper.range(second.syntax().text_range()).unwrap();

                diag.push(Diagnostic {
                    range: first_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"duplicate key "{}""#, first.full_key_string()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: "other declaration".into(),
                    }]),
                    tags: None,
                    data: None,
                });

                diag.push(Diagnostic {
                    range: second_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"duplicate key "{}""#, first.full_key_string()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: first_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: "first declaration".into(),
                    }]),
                    tags: None,
                    data: None,
                });
            }
            dom::Error::ExpectedTable { target, key } => {
                let target_range = mapper.range(target.syntax().text_range()).unwrap();
                let second_range = mapper.range(key.syntax().text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"expected table for "{}""#, target.full_key_string()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: format!(r#"required by "{}""#, key.full_key_string()),
                    }]),
                    tags: None,
                    data: None,
                });
            }
            dom::Error::ExpectedTableArray { target, key } => {
                let target_range = mapper.range(target.syntax().text_range()).unwrap();
                let second_range = mapper.range(key.syntax().text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: "array conflicts with array of tables".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: "array of tables declaration".to_string(),
                    }]),
                    tags: None,
                    data: None,
                });

                diag.push(Diagnostic {
                    range: second_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: "array conflicts with array of tables".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: target_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: "array declaration".to_string(),
                    }]),
                    tags: None,
                    data: None,
                });
            }
            dom::Error::InlineTable { target, key } => {
                let target_range = mapper.range(target.syntax().text_range()).unwrap();
                let second_range = mapper.range(key.syntax().text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: "inline table cannot be modified".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: format!(r#"modified here by "{}""#, key.full_key_string()),
                    }]),
                    tags: None,
                    data: None,
                });

                diag.push(Diagnostic {
                    range: second_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: "inline table cannot be modified".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: target_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: format!(r#"inline table "{}" here"#, target.full_key_string()),
                    }]),
                    tags: None,
                    data: None,
                });
            }

            dom::Error::Spanned { range, message } => {
                let r = mapper.range(*range).unwrap();

                diag.push(Diagnostic {
                    range: r.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: message.clone(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }

            dom::Error::Generic(err) => {
                diag.push(Diagnostic {
                    range: Default::default(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: err.clone(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
            dom::Error::DottedKeyConflict { first, second } => {
                let first_range = mapper.range(first.syntax().text_range()).unwrap();
                let second_range = mapper.range(second.syntax().text_range()).unwrap();

                diag.push(Diagnostic {
                    range: second_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,code_description:None,
                    source: Some("Even Better TOML".into()),
                    message: "conflicting dotted keys, entries with overlapping keys must have the same amount of keys".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: first_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: "conflicting dotted keys here".to_string(),
                    }]),
                    tags: None, data: None
                });

                diag.push(Diagnostic {
                    range: first_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,code_description:None,
                    source: Some("Even Better TOML".into()),
                    message: "conflicting dotted keys, entries with overlapping keys must have the same amount of keys".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: "conflicting dotted keys here".to_string(),
                    }]),
                    tags: None, data: None
                });
            }
            dom::Error::SubTableBeforeTableArray { target, key } => {
                let target_range = mapper.range(target.syntax().text_range()).unwrap();
                let key_range = mapper.range(key.syntax().text_range()).unwrap();

                diag.push(Diagnostic {
                    range: key_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: "array of tables conflicting with subtable above".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: target_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: "subtable here".to_string(),
                    }]),
                    tags: None,
                    data: None,
                });

                diag.push(Diagnostic {
                    range: target_range.into_lsp(),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    code_description: None,
                    source: Some("Even Better TOML".into()),
                    message: "subtable declared before array of tables".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: key_range.into_lsp(),
                            uri: uri.clone(),
                        },
                        message: "array of tables here".to_string(),
                    }]),
                    tags: None,
                    data: None,
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
                    .unwrap()
                    .into_lsp(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                code_description: None,
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
                data: None,
            });
        }
        ErrorValue::NoPatternMatch { pattern: _ } => {
            diags.push(Diagnostic {
                range: error
                    .span
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default()
                    .into_lsp(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                code_description: None,
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
                data: None,
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
                    .unwrap_or_default()
                    .into_lsp(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                code_description: None,
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
                data: None,
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
                    .unwrap_or_default().into_lsp(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,code_description:None,
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
                tags: None, data: None
            });
        }
        ErrorValue::NotUnique { first, duplicate } => {
            diags.push(Diagnostic {
                range: first
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default()
                    .into_lsp(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                code_description: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: error.value.to_string(),
                related_information: Some(vec![DiagnosticRelatedInformation {
                    location: Location {
                        range: duplicate
                            .map(|span| mapper.range(span.0).unwrap())
                            .unwrap_or_default()
                            .into_lsp(),
                        uri: uri.clone(),
                    },
                    message: "duplicate value".into(),
                }]),
                tags: None,
                data: None,
            });
            diags.push(Diagnostic {
                range: duplicate
                    .map(|span| mapper.range(span.0).unwrap())
                    .unwrap_or_default()
                    .into_lsp(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                code_description: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: error.value.to_string(),
                related_information: Some(vec![DiagnosticRelatedInformation {
                    location: Location {
                        range: first
                            .map(|span| mapper.range(span.0).unwrap())
                            .unwrap_or_default()
                            .into_lsp(),
                        uri: uri.clone(),
                    },
                    message: "duplicate value".into(),
                }]),
                tags: None,
                data: None,
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
        ErrorValue::Custom(_) => {
            // Incomplete DOM tree errors are expected.
        }
        error_value => {
            diags.push(Diagnostic {
                range: error
                    .span
                    .and_then(|span| mapper.range(span.0))
                    .unwrap_or_default()
                    .into_lsp(),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                code_description: None,
                source: Some(format!(
                    "Even Better TOML{}",
                    schema_in_message(&error.meta)
                )),
                message: error_value.to_string(),
                related_information: None,
                tags: None,
                data: None,
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
