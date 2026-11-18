# query.in_entry_keys

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 286-289)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn in_entry_keys(&self) -> bool {
        self.entry_key()
            .is_some_and(|k| k.text_range().contains(self.offset))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn in_entry_keys(&self) -> bool {
        self.entry_key()
            .is_some_and(|k| k.text_range().contains(self.offset))
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
