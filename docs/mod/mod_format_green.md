# mod.format_green

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 243-245)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn format_green(green: GreenNode, options: Options) -> String {
    format_syntax(SyntaxNode::new_root(green), options)
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn format_green(green: GreenNode, options: Options) -> String {
    format_syntax(SyntaxNode::new_root(green), options)
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
