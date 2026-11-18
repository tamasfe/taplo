# nodes.value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 372-379)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn value(&self) -> bool {
        *self.inner.value.get_or_init(|| {
            self.syntax()
                .and_then(|s| s.as_token())
                .and_then(|s| s.text().parse().ok())
                .unwrap_or_default()
        })
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn value(&self) -> bool {
        *self.inner.value.get_or_init(|| {
            self.syntax()
                .and_then(|s| s.as_token())
                .and_then(|s| s.text().parse().ok())
                .unwrap_or_default()
        })
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
