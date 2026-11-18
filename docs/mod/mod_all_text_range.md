# mod.all_text_range

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 177-184)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn all_text_range(&self) -> TextRange {
        join_ranges(
            self.keys
                .iter()
                .filter_map(KeyOrIndex::as_key)
                .flat_map(|k| k.text_ranges()),
        )
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn all_text_range(&self) -> TextRange {
        join_ranges(
            self.keys
                .iter()
                .filter_map(KeyOrIndex::as_key)
                .flat_map(|k| k.text_ranges()),
        )
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
