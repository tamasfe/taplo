# mod.new_directive

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 302-310)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new_directive(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            syntax: None,
            value: OnceCell::from(CommentValue::Directive {
                name: name.into(),
                value: value.into(),
            }),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new_directive(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            syntax: None,
            value: OnceCell::from(CommentValue::Directive {
                name: name.into(),
                value: value.into(),
            }),
        }
    }
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
