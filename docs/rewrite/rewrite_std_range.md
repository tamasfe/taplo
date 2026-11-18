# rewrite.std_range

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\rewrite.rs` (lines 132-136)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn std_range(range: TextRange) -> Range<usize> {
    let start: usize = u32::from(range.start()) as usize;
    let end: usize = u32::from(range.end()) as usize;
    start..end
}
```

## Docstring

_No docstring provided._

## Source

```
fn std_range(range: TextRange) -> Range<usize> {
    let start: usize = u32::from(range.start()) as usize;
    let end: usize = u32::from(range.end()) as usize;
    start..end
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
