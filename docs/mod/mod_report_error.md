# mod.report_error

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 161-171)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn report_error(&mut self, message: &str) -> ParserResult<()> {
        let span = self.lexer.span();
        self.add_error(&Error {
            range: TextRange::new(
                TextSize::from(span.start as u32),
                TextSize::from(span.end as u32),
            ),
            message: message.into(),
        });
        Err(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn report_error(&mut self, message: &str) -> ParserResult<()> {
        let span = self.lexer.span();
        self.add_error(&Error {
            range: TextRange::new(
                TextSize::from(span.start as u32),
                TextSize::from(span.end as u32),
            ),
            message: message.into(),
        });
        Err(())
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
