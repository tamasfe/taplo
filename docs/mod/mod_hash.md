# mod.hash

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 226-228)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dotted.hash(state);
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dotted.hash(state);
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
