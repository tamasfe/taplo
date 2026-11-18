# toml_test.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\toml_test.rs` (lines 85-90)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new(node: &'a Node) -> Self {
        Self {
            r#type: TomlTestType::of(node),
            node,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new(node: &'a Node) -> Self {
        Self {
            r#type: TomlTestType::of(node),
            node,
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
