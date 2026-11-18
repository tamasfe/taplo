# node.syntax

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 43-54)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn syntax(&self) -> Option<&SyntaxElement> {
        match self {
            Node::Table(n) => n.syntax(),
            Node::Array(n) => n.syntax(),
            Node::Bool(n) => n.syntax(),
            Node::Str(n) => n.syntax(),
            Node::Integer(n) => n.syntax(),
            Node::Float(n) => n.syntax(),
            Node::Date(n) => n.syntax(),
            Node::Invalid(n) => n.syntax(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn syntax(&self) -> Option<&SyntaxElement> {
        match self {
            Node::Table(n) => n.syntax(),
            Node::Array(n) => n.syntax(),
            Node::Bool(n) => n.syntax(),
            Node::Str(n) => n.syntax(),
            Node::Integer(n) => n.syntax(),
            Node::Float(n) => n.syntax(),
            Node::Date(n) => n.syntax(),
            Node::Invalid(n) => n.syntax(),
        }
    }
```

## Git History

- Commits: 8
- Authors: 6
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
