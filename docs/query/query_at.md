# query.at

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 34-48)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn at(root: &Node, offset: TextSize) -> Self {
        let syntax = root.syntax().cloned().unwrap().into_node().unwrap();

        Query {
            offset,
            before: offset
                .checked_sub(TextSize::from(1))
                .and_then(|offset| Self::position_info_at(root, &syntax, offset)),
            after: if offset >= syntax.text_range().end() {
                None
            } else {
                Self::position_info_at(root, &syntax, offset)
            },
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn at(root: &Node, offset: TextSize) -> Self {
        let syntax = root.syntax().cloned().unwrap().into_node().unwrap();

        Query {
            offset,
            before: offset
                .checked_sub(TextSize::from(1))
                .and_then(|offset| Self::position_info_at(root, &syntax, offset)),
            after: if offset >= syntax.text_range().end() {
                None
            } else {
                Self::position_info_at(root, &syntax, offset)
            },
        }
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
