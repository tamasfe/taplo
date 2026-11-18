# mod.directive

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 316-322)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn directive(&self) -> Option<&str> {
        if let CommentValue::Directive { name, .. } = self.value_internal() {
            Some(name)
        } else {
            None
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn directive(&self) -> Option<&str> {
        if let CommentValue::Directive { name, .. } = self.value_internal() {
            Some(name)
        } else {
            None
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
