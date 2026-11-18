# iter.size_hint

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\iter.rs` (lines 34-36)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
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
