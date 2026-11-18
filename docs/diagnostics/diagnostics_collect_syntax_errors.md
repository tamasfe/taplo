# diagnostics.collect_syntax_errors

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\diagnostics.rs` (lines 108-123)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn collect_syntax_errors(doc: &DocumentState, diags: &mut Vec<Diagnostic>) {
    diags.extend(doc.parse.errors.iter().map(|e| {
        let range = doc.mapper.range(e.range).unwrap_or_default().into_lsp();
        Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("Even Better TOML".into()),
            message: e.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        }
    }));
}
```

## Docstring

_No docstring provided._

## Source

```
fn collect_syntax_errors(doc: &DocumentState, diags: &mut Vec<Diagnostic>) {
    diags.extend(doc.parse.errors.iter().map(|e| {
        let range = doc.mapper.range(e.range).unwrap_or_default().into_lsp();
        Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("Even Better TOML".into()),
            message: e.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        }
    }));
}
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
