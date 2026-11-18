# mod.indent

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 237-239)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn indent<'o>(&self, opts: &'o Options) -> impl Iterator<Item = &'o str> {
        repeat(opts.indent_string.as_ref()).take(self.indent_level)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn indent<'o>(&self, opts: &'o Options) -> impl Iterator<Item = &'o str> {
        repeat(opts.indent_string.as_ref()).take(self.indent_level)
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
