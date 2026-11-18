# mod.format_syntax

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 260-270)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn format_syntax(node: SyntaxNode, options: Options) -> String {
    let mut s = format_impl(node, options.clone(), Context::default());

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    s
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn format_syntax(node: SyntaxNode, options: Options) -> String {
    let mut s = format_impl(node, options.clone(), Context::default());

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    s
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
