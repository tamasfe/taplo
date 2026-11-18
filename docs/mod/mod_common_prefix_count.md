# mod.common_prefix_count

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 154-159)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn common_prefix_count(&self, other: &Self) -> usize {
        self.iter()
            .zip(other.iter())
            .take_while(|(a, b)| a == b)
            .count()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn common_prefix_count(&self, other: &Self) -> usize {
        self.iter()
            .zip(other.iter())
            .take_while(|(a, b)| a == b)
            .count()
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
