# mod.contains

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 161-163)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn contains(&self, other: &Self) -> bool {
        self.len() >= other.len() && self.common_prefix_count(other) == other.len()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn contains(&self, other: &Self) -> bool {
        self.len() >= other.len() && self.common_prefix_count(other) == other.len()
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
