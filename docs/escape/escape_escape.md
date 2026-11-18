# escape.escape

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\escape.rs` (lines 57-76)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '\u{0008}' => escaped.push_str(r#"\b"#),
            '\u{0009}' => escaped.push_str(r#"\t"#),
            '\u{000A}' => escaped.push_str(r#"\n"#),
            '\u{000C}' => escaped.push_str(r#"\f"#),
            '\u{000D}' => escaped.push_str(r#"\r"#),
            '\u{0022}' => escaped.push_str(r#"\""#),
            '\u{005C}' => escaped.push_str(r#"\\"#),
            _ => {
                escaped.push(c);
            }
        }
    }

    escaped
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '\u{0008}' => escaped.push_str(r#"\b"#),
            '\u{0009}' => escaped.push_str(r#"\t"#),
            '\u{000A}' => escaped.push_str(r#"\n"#),
            '\u{000C}' => escaped.push_str(r#"\f"#),
            '\u{000D}' => escaped.push_str(r#"\r"#),
            '\u{0022}' => escaped.push_str(r#"\""#),
            '\u{005C}' => escaped.push_str(r#"\\"#),
            _ => {
                escaped.push(c);
            }
        }
    }

    escaped
}
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
