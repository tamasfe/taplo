# query.position_info_at

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 50-68)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn position_info_at(
        root: &Node,
        syntax: &SyntaxNode,
        offset: TextSize,
    ) -> Option<PositionInfo> {
        let syntax = match syntax.token_at_offset(offset) {
            taplo::rowan::TokenAtOffset::None => return None,
            taplo::rowan::TokenAtOffset::Single(s) => s,
            taplo::rowan::TokenAtOffset::Between(_, right) => right,
        };

        Some(PositionInfo {
            syntax,
            dom_node: root
                .flat_iter()
                .filter(|(k, n)| full_range(k, n).contains(offset))
                .max_by_key(|(k, _)| k.len()),
        })
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn position_info_at(
        root: &Node,
        syntax: &SyntaxNode,
        offset: TextSize,
    ) -> Option<PositionInfo> {
        let syntax = match syntax.token_at_offset(offset) {
            taplo::rowan::TokenAtOffset::None => return None,
            taplo::rowan::TokenAtOffset::Single(s) => s,
            taplo::rowan::TokenAtOffset::Between(_, right) => right,
        };

        Some(PositionInfo {
            syntax,
            dom_node: root
                .flat_iter()
                .filter(|(k, n)| full_range(k, n).contains(offset))
                .max_by_key(|(k, _)| k.len()),
        })
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
