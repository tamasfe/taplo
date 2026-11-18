# syntax.lex_string_literal

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\syntax.rs` (lines 215-228)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn lex_string_literal(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();
    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == '\'' {
            lex.bump(total_len);
            return true;
        }
    }
    false
}
```

## Docstring

_No docstring provided._

## Source

```
fn lex_string_literal(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();
    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == '\'' {
            lex.bump(total_len);
            return true;
        }
    }
    false
}
```

## Git History

- Commits: 5
- Authors: 3
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
