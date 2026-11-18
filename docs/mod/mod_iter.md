# mod.iter

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 261-263)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn iter(&self) -> impl Iterator<Item = &(Key, Node)> {
        self.all.iter()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn iter(&self) -> impl Iterator<Item = &(Key, Node)> {
        self.all.iter()
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
