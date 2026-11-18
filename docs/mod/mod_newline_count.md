# mod.newline_count

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 1227-1229)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn newline_count(&self) -> usize {
        self.chars().filter(|c| c == &'\n').count()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn newline_count(&self) -> usize {
        self.chars().filter(|c| c == &'\n').count()
    }
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
