# mod.format_key

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 789-802)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_key(node: SyntaxNode, formatted: &mut String, _options: &Options, _context: &Context) {
    // Idents and periods without whitespace
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(_) => {}
            NodeOrToken::Token(t) => match t.kind() {
                WHITESPACE | NEWLINE => {}
                _ => {
                    *formatted += t.text();
                }
            },
        }
    }
}
```

## Docstring

_No docstring provided._

## Source

```
fn format_key(node: SyntaxNode, formatted: &mut String, _options: &Options, _context: &Context) {
    // Idents and periods without whitespace
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(_) => {}
            NodeOrToken::Token(t) => match t.kind() {
                WHITESPACE | NEWLINE => {}
                _ => {
                    *formatted += t.text();
                }
            },
        }
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
