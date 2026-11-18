# iter.ExactIterExt

- **Type:** trait
- **Language:** rust
- **File:** `\crates\taplo\src\util\iter.rs` (lines 41-43)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub(crate) trait ExactIterExt: Iterator + Sized {
    fn exactly(self, count: usize) -> ExactIter<Self>;
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) trait ExactIterExt: Iterator + Sized {
    fn exactly(self, count: usize) -> ExactIter<Self>;
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
