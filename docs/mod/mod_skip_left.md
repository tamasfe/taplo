# mod.skip_left

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 169-171)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn skip_left(&self, n: usize) -> Self {
        Self::new(self.keys.iter().skip(n).cloned())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn skip_left(&self, n: usize) -> Self {
        Self::new(self.keys.iter().skip(n).cloned())
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
