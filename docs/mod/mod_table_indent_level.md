# mod.table_indent_level

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 611-624)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn table_indent_level(
    history: &[(Keys, usize)],
    current_key: &Keys,
    default_indent: usize,
) -> usize {
    history
        .iter()
        .rev()
        .find_map(|(previous_key, indent)| {
            (current_key.contains(previous_key) && current_key != previous_key)
                .then_some(*indent + 1)
        })
        .unwrap_or(default_indent)
}
```

## Docstring

_No docstring provided._

## Source

```
fn table_indent_level(
    history: &[(Keys, usize)],
    current_key: &Keys,
    default_indent: usize,
) -> usize {
    history
        .iter()
        .rev()
        .find_map(|(previous_key, indent)| {
            (current_key.contains(previous_key) && current_key != previous_key)
                .then_some(*indent + 1)
        })
        .unwrap_or(default_indent)
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
