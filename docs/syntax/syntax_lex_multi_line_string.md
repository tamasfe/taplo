# syntax.lex_multi_line_string

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\syntax.rs` (lines 153-213)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn lex_multi_line_string(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();

    let mut total_len = 0;
    let mut quote_count = 0;

    let mut escaped = false;

    // As the string can contain ",
    // we can end up with more than 3 "-s at
    // the end, in that case we need to include all
    // in the string.
    let mut quotes_found = false;

    for c in remainder.chars() {
        if quotes_found {
            if c != '"' {
                if quote_count >= 6 {
                    return false;
                }

                lex.bump(total_len);
                return true;
            } else {
                quote_count += 1;
                total_len += c.len_utf8();
                continue;
            }
        }
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = true;
            continue;
        }

        if c == '"' && !escaped {
            quote_count += 1;
        } else {
            quote_count = 0;
        }

        if quote_count == 3 {
            quotes_found = true;
        }

        escaped = false;
    }

    // End of input
    if quotes_found {
        if quote_count >= 6 {
            return false;
        }

        lex.bump(total_len);
        true
    } else {
        false
    }
}
```

## Docstring

_No docstring provided._

## Source

```
fn lex_multi_line_string(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();

    let mut total_len = 0;
    let mut quote_count = 0;

    let mut escaped = false;

    // As the string can contain ",
    // we can end up with more than 3 "-s at
    // the end, in that case we need to include all
    // in the string.
    let mut quotes_found = false;

    for c in remainder.chars() {
        if quotes_found {
            if c != '"' {
                if quote_count >= 6 {
                    return false;
                }

                lex.bump(total_len);
                return true;
            } else {
                quote_count += 1;
                total_len += c.len_utf8();
                continue;
            }
        }
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = true;
            continue;
        }

        if c == '"' && !escaped {
            quote_count += 1;
        } else {
            quote_count = 0;
        }

        if quote_count == 3 {
            quotes_found = true;
        }

        escaped = false;
    }

    // End of input
    if quotes_found {
        if quote_count >= 6 {
            return false;
        }

        lex.bump(total_len);
        true
    } else {
        false
    }
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
