# config.fmt

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 320-328)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("name", &self.name)
            .field("include", &self.include)
            .field("exclude", &self.exclude)
            .field("keys", &self.keys)
            .field("options", &self.options)
            .finish()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("name", &self.name)
            .field("include", &self.include)
            .field("exclude", &self.exclude)
            .field("keys", &self.keys)
            .field("options", &self.options)
            .finish()
    }
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 4
- Features: 1

## Callers

None

## Callees

None
