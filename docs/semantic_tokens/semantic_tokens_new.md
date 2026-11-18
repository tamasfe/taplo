# semantic_tokens.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\semantic_tokens.rs` (lines 121-127)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn new(mapper: &'b Mapper) -> Self {
        Self {
            tokens: Vec::new(),
            mapper,
            last_range: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn new(mapper: &'b Mapper) -> Self {
        Self {
            tokens: Vec::new(),
            mapper,
            last_range: None,
        }
    }
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
