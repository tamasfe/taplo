# query.full_range

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 456-467)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn full_range(keys: &Keys, node: &Node) -> TextRange {
    let Some(last_key) = keys
        .iter()
        .filter_map(KeyOrIndex::as_key)
        .next_back()
        .map(Key::text_ranges)
    else {
        return join_ranges(node.text_ranges(true));
    };

    join_ranges(last_key.chain(node.text_ranges(true)))
}
```

## Docstring

_No docstring provided._

## Source

```
fn full_range(keys: &Keys, node: &Node) -> TextRange {
    let Some(last_key) = keys
        .iter()
        .filter_map(KeyOrIndex::as_key)
        .next_back()
        .map(Key::text_ranges)
    else {
        return join_ranges(node.text_ranges(true));
    };

    join_ranges(last_key.chain(node.text_ranges(true)))
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
