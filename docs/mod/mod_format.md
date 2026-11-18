# mod.format

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 248-257)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn format(src: &str, options: Options) -> String {
    let p = crate::parser::parse(src);

    let ctx = Context {
        errors: p.errors.iter().map(|err| err.range).collect(),
        ..Context::default()
    };

    format_impl(p.into_syntax(), options, ctx)
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn format(src: &str, options: Options) -> String {
    let p = crate::parser::parse(src);

    let ctx = Context {
        errors: p.errors.iter().map(|err| err.range).collect(),
        ..Context::default()
    };

    format_impl(p.into_syntax(), options, ctx)
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
