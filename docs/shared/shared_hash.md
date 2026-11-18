# shared.hash

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\shared.rs` (lines 14-16)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.load().hash(state)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.load().hash(state)
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
