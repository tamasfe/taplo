# to_toml.fmt

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\to_toml.rs` (lines 188-190)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_toml_impl(f, Keys::empty(), false, false, false)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_toml_impl(f, Keys::empty(), false, false, false)
    }
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
