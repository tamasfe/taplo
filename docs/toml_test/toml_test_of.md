# toml_test.of

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\toml_test.rs` (lines 60-76)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn of(node: &Node) -> Option<Self> {
        match node {
            Node::Bool(_) => Some(TomlTestType::Bool),
            Node::Integer(_) => Some(TomlTestType::Integer),
            Node::Float(_) => Some(TomlTestType::Float),
            Node::Str(_) => Some(TomlTestType::String),
            Node::Date(d) => match d.value() {
                DateTimeValue::OffsetDateTime(_) => Some(TomlTestType::DateTime),
                DateTimeValue::LocalDateTime(_) => Some(TomlTestType::DateTimeLocal),
                DateTimeValue::Date(_) => Some(TomlTestType::DateLocal),
                DateTimeValue::Time(_) => Some(TomlTestType::TimeLocal),
            },
            Node::Array(_) => None,
            Node::Table(_) => None,
            Node::Invalid(_) => unreachable!(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn of(node: &Node) -> Option<Self> {
        match node {
            Node::Bool(_) => Some(TomlTestType::Bool),
            Node::Integer(_) => Some(TomlTestType::Integer),
            Node::Float(_) => Some(TomlTestType::Float),
            Node::Str(_) => Some(TomlTestType::String),
            Node::Date(d) => match d.value() {
                DateTimeValue::OffsetDateTime(_) => Some(TomlTestType::DateTime),
                DateTimeValue::LocalDateTime(_) => Some(TomlTestType::DateTimeLocal),
                DateTimeValue::Date(_) => Some(TomlTestType::DateLocal),
                DateTimeValue::Time(_) => Some(TomlTestType::TimeLocal),
            },
            Node::Array(_) => None,
            Node::Table(_) => None,
            Node::Invalid(_) => unreachable!(),
        }
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
