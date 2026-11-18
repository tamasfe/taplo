# query.header_keys

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 414-417)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn header_keys(&self) -> Keys {
        self.header_key()
            .map_or_else(Keys::empty, |keys| Keys::from_syntax(keys.into()))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn header_keys(&self) -> Keys {
        self.header_key()
            .map_or_else(Keys::empty, |keys| Keys::from_syntax(keys.into()))
    }
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
