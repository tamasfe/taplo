# iter.exactly

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\iter.rs` (lines 46-48)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn exactly(self, count: usize) -> ExactIter<Self> {
        ExactIter::new(count, self)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn exactly(self, count: usize) -> ExactIter<Self> {
        ExactIter::new(count, self)
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
