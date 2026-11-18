# syntax.add_all

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\syntax.rs` (lines 5-16)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn add_all(node: SyntaxNode, builder: &mut GreenNodeBuilder) {
    builder.start_node(node.kind().into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => add_all(n, builder),
            NodeOrToken::Token(t) => builder.token(t.kind().into(), t.text()),
        }
    }

    builder.finish_node()
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn add_all(node: SyntaxNode, builder: &mut GreenNodeBuilder) {
    builder.start_node(node.kind().into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => add_all(n, builder),
            NodeOrToken::Token(t) => builder.token(t.kind().into(), t.text()),
        }
    }

    builder.finish_node()
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
