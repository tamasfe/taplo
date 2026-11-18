# mod.skip_right

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 173-175)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn skip_right(&self, n: usize) -> Self {
        Self::new(self.keys.iter().rev().skip(n).cloned().rev())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn skip_right(&self, n: usize) -> Self {
        Self::new(self.keys.iter().rev().skip(n).cloned().rev())
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
