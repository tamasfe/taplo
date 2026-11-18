# mod.value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 324-329)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn value(&self) -> &str {
        match self.value_internal() {
            CommentValue::Comment(s) => s,
            CommentValue::Directive { value, .. } => value,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn value(&self) -> &str {
        match self.value_internal() {
            CommentValue::Comment(s) => s,
            CommentValue::Directive { value, .. } => value,
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
