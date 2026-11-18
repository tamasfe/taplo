# completion.new_entry_snippet

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\completion.rs` (lines 681-684)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn new_entry_snippet(keys: &Keys, schema: &Value, single_quote: bool) -> String {
    let value = default_value_snippet(schema, 0, single_quote);
    format!("{keys} = {value}")
}
```

## Docstring

_No docstring provided._

## Source

```
fn new_entry_snippet(keys: &Keys, schema: &Value, single_quote: bool) -> String {
    let value = default_value_snippet(schema, 0, single_quote);
    format!("{keys} = {value}")
}
```

## Git History

- Commits: 7
- Authors: 2
- Bug fixes: 3
- Features: 3

## Callers

None

## Callees

None
