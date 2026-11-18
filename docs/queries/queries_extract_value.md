# queries.extract_value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\queries.rs` (lines 211-234)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn extract_value(node: &Node, separator: &str) -> Result<String, anyhow::Error> {
    Ok(match node {
        Node::Table(_) => {
            return Err(anyhow!(
                r#"cannot print tables with the given output format, specify a different output format (e.g. with `-o json`) "#
            ))
        }
        Node::Array(arr) => {
            let mut values = Vec::new();

            for node in arr.items().read().iter() {
                values.push(extract_value(node, separator)?);
            }

            values.join(separator)
        }
        Node::Bool(b) => b.value().to_string(),
        Node::Str(s) => s.value().to_string(),
        Node::Integer(i) => i.value().to_string(),
        Node::Float(f) => f.value().to_string(),
        Node::Date(d) => d.value().to_string(),
        Node::Invalid(_) => "".into(),
    })
}
```

## Docstring

_No docstring provided._

## Source

```
fn extract_value(node: &Node, separator: &str) -> Result<String, anyhow::Error> {
    Ok(match node {
        Node::Table(_) => {
            return Err(anyhow!(
                r#"cannot print tables with the given output format, specify a different output format (e.g. with `-o json`) "#
            ))
        }
        Node::Array(arr) => {
            let mut values = Vec::new();

            for node in arr.items().read().iter() {
                values.push(extract_value(node, separator)?);
            }

            values.join(separator)
        }
        Node::Bool(b) => b.value().to_string(),
        Node::Str(s) => s.value().to_string(),
        Node::Integer(i) => i.value().to_string(),
        Node::Float(f) => f.value().to_string(),
        Node::Date(d) => d.value().to_string(),
        Node::Invalid(_) => "".into(),
    })
}
```

## Git History

- Commits: 8
- Authors: 2
- Bug fixes: 0
- Features: 3

## Callers

None

## Callees

None
