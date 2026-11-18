# diagnostics.collect_dom_errors

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\diagnostics.rs` (lines 126-278)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn collect_dom_errors(
    doc: &DocumentState,
    dom: &Node,
    document_url: &Url,
    diags: &mut Vec<Diagnostic>,
) {
    if let Err(errors) = dom.validate() {
        for error in errors {
            match &error {
                taplo::dom::Error::ConflictingKeys { key, other } => {
                    let range = doc
                        .mapper
                        .range(key.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    let other_range = doc
                        .mapper
                        .range(other.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    diags.push(Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range: other_range,
                            },
                            message: "other key defined here".into(),
                        }])),
                        ..Default::default()
                    });

                    diags.push(Diagnostic {
                        range: other_range,
                        severity: Some(DiagnosticSeverity::HINT),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range,
                            },
                            message: "other key defined here".into(),
                        }])),
                        ..Default::default()
                    });
                }
                taplo::dom::Error::ExpectedTable {
                    not_table,
                    required_by,
                } => {
                    let range = doc
                        .mapper
                        .range(not_table.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    let other_range = doc
                        .mapper
                        .range(required_by.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    diags.push(Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range: other_range,
                            },
                            message: "required by this key".into(),
                        }])),
                        ..Default::default()
                    });

                    diags.push(Diagnostic {
                        range: other_range,
                        severity: Some(DiagnosticSeverity::HINT),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range,
                            },
                            message: "table defined here".into(),
                        }])),
                        ..Default::default()
                    });
                }
                taplo::dom::Error::ExpectedArrayOfTables {
                    not_array_of_tables,
                    required_by,
                } => {
                    let range = doc
                        .mapper
                        .range(not_array_of_tables.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    let other_range = doc
                        .mapper
                        .range(required_by.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    diags.push(Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range: other_range,
                            },
                            message: "required by this key".into(),
                        }])),
                        ..Default::default()
                    });

                    diags.push(Diagnostic {
                        range: other_range,
                        severity: Some(DiagnosticSeverity::HINT),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range,
                            },
                            message: "array of tables defined here".into(),
                        }])),
                        ..Default::default()
                    });
                }
                taplo::dom::Error::InvalidEscapeSequence { string: _ }
                | taplo::dom::Error::Query(_) => {}
                taplo::dom::Error::UnexpectedSyntax { syntax } => {
                    tracing::error!("unexpected syntax in dom: {syntax:#?}");
                }
            }
        }
    }
}
```

## Docstring

_No docstring provided._

## Source

```
fn collect_dom_errors(
    doc: &DocumentState,
    dom: &Node,
    document_url: &Url,
    diags: &mut Vec<Diagnostic>,
) {
    if let Err(errors) = dom.validate() {
        for error in errors {
            match &error {
                taplo::dom::Error::ConflictingKeys { key, other } => {
                    let range = doc
                        .mapper
                        .range(key.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    let other_range = doc
                        .mapper
                        .range(other.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    diags.push(Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range: other_range,
                            },
                            message: "other key defined here".into(),
                        }])),
                        ..Default::default()
                    });

                    diags.push(Diagnostic {
                        range: other_range,
                        severity: Some(DiagnosticSeverity::HINT),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range,
                            },
                            message: "other key defined here".into(),
                        }])),
                        ..Default::default()
                    });
                }
                taplo::dom::Error::ExpectedTable {
                    not_table,
                    required_by,
                } => {
                    let range = doc
                        .mapper
                        .range(not_table.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    let other_range = doc
                        .mapper
                        .range(required_by.text_ranges().next().unwrap())
                        .unwrap()
                        .into_lsp();

                    diags.push(Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range: other_range,
                            },
                            message: "required by this key".into(),
                        }])),
                        ..Default::default()
                    });

                    diags.push(Diagnostic {
                        range: other_range,
                        severity: Some(DiagnosticSeverity::HINT),
                        source: Some("Even Better TOML".into()),
                        message: error.to_string(),
                        related_information: Some(Vec::from([DiagnosticRelatedInformation {
                            location: Location {
                                uri: document_url.clone(),
                                range,
                            },
                            message: "table defined here".into(),
                        }])),
                        ..Default::default()
                    });
                }
                taplo::dom::Error::ExpectedArrayOfTables {
                    not_array_of_tables,
... (truncated)
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
