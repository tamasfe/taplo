# mod.newlines

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 189-191)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn newlines(&self, count: usize) -> impl Iterator<Item = &'static str> {
        repeat(self.newline()).take(usize::min(count, self.allowed_blank_lines + 1))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn newlines(&self, count: usize) -> impl Iterator<Item = &'static str> {
        repeat(self.newline()).take(usize::min(count, self.allowed_blank_lines + 1))
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
