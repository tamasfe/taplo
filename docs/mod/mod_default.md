# mod.default

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 207-214)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn default() -> Self {
        Self {
            indent_level: Default::default(),
            force_multiline: Default::default(),
            errors: Rc::from([]),
            scopes: Default::default(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn default() -> Self {
        Self {
            indent_level: Default::default(),
            force_multiline: Default::default(),
            errors: Rc::from([]),
            scopes: Default::default(),
        }
    }
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
