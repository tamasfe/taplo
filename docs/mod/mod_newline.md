# mod.newline

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 181-187)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn newline(&self) -> &'static str {
        if self.crlf {
            "\r\n"
        } else {
            "\n"
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn newline(&self) -> &'static str {
        if self.crlf {
            "\r\n"
        } else {
            "\n"
        }
    }
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
