# node.errors

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 56-67)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn errors(&self) -> &Shared<Vec<Error>> {
        match self {
            Node::Table(n) => n.errors(),
            Node::Array(n) => n.errors(),
            Node::Bool(n) => n.errors(),
            Node::Str(n) => n.errors(),
            Node::Integer(n) => n.errors(),
            Node::Float(n) => n.errors(),
            Node::Date(n) => n.errors(),
            Node::Invalid(n) => n.errors(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn errors(&self) -> &Shared<Vec<Error>> {
        match self {
            Node::Table(n) => n.errors(),
            Node::Array(n) => n.errors(),
            Node::Bool(n) => n.errors(),
            Node::Str(n) => n.errors(),
            Node::Integer(n) => n.errors(),
            Node::Float(n) => n.errors(),
            Node::Date(n) => n.errors(),
            Node::Invalid(n) => n.errors(),
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
