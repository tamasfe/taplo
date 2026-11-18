# mod.into_iter

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 192-194)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn into_iter(self) -> Self::IntoIter {
        Vec::from(&*self.keys).into_iter()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn into_iter(self) -> Self::IntoIter {
        Vec::from(&*self.keys).into_iter()
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
