# nodes.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 179-188)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new(key: impl Into<String>) -> Self {
        KeyInner {
            errors: Default::default(),
            syntax: None,
            is_valid: true,
            value: OnceCell::from(key.into()),
            additional_syntaxes: Default::default(),
        }
        .wrap()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new(key: impl Into<String>) -> Self {
        KeyInner {
            errors: Default::default(),
            syntax: None,
            is_valid: true,
            value: OnceCell::from(key.into()),
            additional_syntaxes: Default::default(),
        }
        .wrap()
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
