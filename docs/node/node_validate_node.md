# node.validate_node

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 69-80)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn validate_node(&self) -> Result<(), &Shared<Vec<Error>>> {
        match self {
            Node::Table(n) => n.validate_node(),
            Node::Array(n) => n.validate_node(),
            Node::Bool(n) => n.validate_node(),
            Node::Str(n) => n.validate_node(),
            Node::Integer(n) => n.validate_node(),
            Node::Float(n) => n.validate_node(),
            Node::Date(n) => n.validate_node(),
            Node::Invalid(n) => n.validate_node(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn validate_node(&self) -> Result<(), &Shared<Vec<Error>>> {
        match self {
            Node::Table(n) => n.validate_node(),
            Node::Array(n) => n.validate_node(),
            Node::Bool(n) => n.validate_node(),
            Node::Str(n) => n.validate_node(),
            Node::Integer(n) => n.validate_node(),
            Node::Float(n) => n.validate_node(),
            Node::Date(n) => n.validate_node(),
            Node::Invalid(n) => n.validate_node(),
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
