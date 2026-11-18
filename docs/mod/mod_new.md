# mod.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 102-112)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn new(source: &'p str) -> Self {
        Parser {
            current_token: None,
            skip_whitespace: true,
            key_pattern_syntax: false,
            error_whitelist: 0,
            lexer: SyntaxKind::lexer(source),
            builder: Default::default(),
            errors: Default::default(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) fn new(source: &'p str) -> Self {
        Parser {
            current_token: None,
            skip_whitespace: true,
            key_pattern_syntax: false,
            error_whitelist: 0,
            lexer: SyntaxKind::lexer(source),
            builder: Default::default(),
            errors: Default::default(),
        }
    }
```

## Git History

- Commits: 8
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
