# mod.check_underscores

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 878-896)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn check_underscores(s: &str, radix: u32) -> bool {
    if s.starts_with('_') || s.ends_with('_') {
        return false;
    }

    let mut last_char = 0 as char;

    for c in s.chars() {
        if c == '_' && !last_char.is_digit(radix) {
            return false;
        }
        if !c.is_digit(radix) && last_char == '_' {
            return false;
        }
        last_char = c;
    }

    true
}
```

## Docstring

_No docstring provided._

## Source

```
fn check_underscores(s: &str, radix: u32) -> bool {
    if s.starts_with('_') || s.ends_with('_') {
        return false;
    }

    let mut last_char = 0 as char;

    for c in s.chars() {
        if c == '_' && !last_char.is_digit(radix) {
            return false;
        }
        if !c.is_digit(radix) && last_char == '_' {
            return false;
        }
        last_char = c;
    }

    true
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
