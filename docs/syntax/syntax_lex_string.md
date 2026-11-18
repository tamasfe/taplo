# syntax.lex_string

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\syntax.rs` (lines 129-151)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn lex_string(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();
    let mut escaped = false;

    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = !escaped;
            continue;
        }

        if c == '"' && !escaped {
            lex.bump(total_len);
            return true;
        }

        escaped = false;
    }
    false
}
```

## Docstring

_No docstring provided._

## Source

```
fn lex_string(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();
    let mut escaped = false;

    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = !escaped;
            continue;
        }

        if c == '"' && !escaped {
            lex.bump(total_len);
            return true;
        }

        escaped = false;
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
