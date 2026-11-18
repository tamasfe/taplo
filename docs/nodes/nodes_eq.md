# nodes.eq

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 284-290)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn eq(&self, other: &Self) -> bool {
        if !self.inner.is_valid || !other.inner.is_valid {
            return false;
        }

        self.value().eq(other.value())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn eq(&self, other: &Self) -> bool {
        if !self.inner.is_valid || !other.inner.is_valid {
            return false;
        }

        self.value().eq(other.value())
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
