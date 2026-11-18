# printing.std_range

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\printing.rs` (lines 151-155)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn std_range(range: TextRange) -> Range<usize> {
    let start: usize = u32::from(range.start()) as _;
    let end: usize = u32::from(range.end()) as _;
    start..end
}
```

## Docstring

_No docstring provided._

## Source

```
fn std_range(range: TextRange) -> Range<usize> {
    let start: usize = u32::from(range.start()) as _;
    let end: usize = u32::from(range.end()) as _;
    start..end
}
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
