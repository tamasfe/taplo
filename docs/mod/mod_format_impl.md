# mod.format_impl

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 348-363)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_impl(node: SyntaxNode, options: Options, context: Context) -> String {
    assert!(node.kind() == ROOT);
    let mut formatted = format_root(node, &options, &context);

    if formatted.ends_with("\r\n") {
        formatted.truncate(formatted.len() - 2);
    } else if formatted.ends_with('\n') {
        formatted.truncate(formatted.len() - 1);
    }

    if options.trailing_newline {
        formatted += options.newline();
    }

    formatted
}
```

## Docstring

_No docstring provided._

## Source

```
fn format_impl(node: SyntaxNode, options: Options, context: Context) -> String {
    assert!(node.kind() == ROOT);
    let mut formatted = format_root(node, &options, &context);

    if formatted.ends_with("\r\n") {
        formatted.truncate(formatted.len() - 2);
    } else if formatted.ends_with('\n') {
        formatted.truncate(formatted.len() - 1);
    }

    if options.trailing_newline {
        formatted += options.newline();
    }

    formatted
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
