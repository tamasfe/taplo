# from_syntax.from_syntax_invalid

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\from_syntax.rs` (lines 174-185)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from_syntax_invalid(syntax: SyntaxElement) -> Self {
        KeyInner {
            errors: Shared::new(Vec::from([Error::UnexpectedSyntax {
                syntax: syntax.clone(),
            }])),
            is_valid: false,
            value: Default::default(),
            syntax: Some(syntax),
            additional_syntaxes: Default::default(),
        }
        .wrap()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from_syntax_invalid(syntax: SyntaxElement) -> Self {
        KeyInner {
            errors: Shared::new(Vec::from([Error::UnexpectedSyntax {
                syntax: syntax.clone(),
            }])),
            is_valid: false,
            value: Default::default(),
            syntax: Some(syntax),
            additional_syntaxes: Default::default(),
        }
        .wrap()
    }
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
