# mod.multi_line_string

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 51-71)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn multi_line_string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t'
                && c != '\n'
                && c != '\r'
                && (('\u{0000}'..='\u{0008}').contains(&c)
                    || ('\u{000A}'..='\u{001F}').contains(&c)
                    || c == '\u{007F}')
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) fn multi_line_string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t'
                && c != '\n'
                && c != '\r'
                && (('\u{0000}'..='\u{0008}').contains(&c)
                    || ('\u{000A}'..='\u{001F}').contains(&c)
                    || c == '\u{007F}')
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }
```

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
