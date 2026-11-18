# from_syntax.entry_from_syntax

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\from_syntax.rs` (lines 403-458)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn entry_from_syntax(syntax: SyntaxElement) -> (Key, Node) {
    assert!(syntax.kind() == ENTRY);

    let mut keys = keys_from_syntax(
        &syntax
            .as_node()
            .and_then(|n| n.first_child())
            .map(Into::into)
            .unwrap_or_else(|| syntax.clone()),
    );
    let first_key = keys
        .next()
        .unwrap_or_else(|| Key::from_syntax_invalid(syntax.clone()));

    let mut value = syntax
        .as_node()
        .and_then(|n| n.first_child())
        .and_then(|k| k.next_sibling())
        .map(|n| Node::from_syntax(n.into()))
        .unwrap_or_else(|| Invalid::from_syntax(syntax).into());

    let mut top_pseudo_table: Option<Table> = None;
    let mut last_pseudo_table: Option<Table> = None;
    while let Some(pseudo_key) = keys.next() {
        let new_pseudo_table = Table::pseudo(&pseudo_key, false);

        match (top_pseudo_table.take(), last_pseudo_table.take()) {
            (None, None) => {
                let top_pt = Table::pseudo(&first_key, false);
                top_pseudo_table = Some(top_pt.clone());

                if keys.len() == 0 {
                    top_pt.add_entry(pseudo_key, value);
                    value = top_pt.into();
                } else {
                    top_pt.add_entry(pseudo_key, new_pseudo_table.clone().into());
                    last_pseudo_table = Some(new_pseudo_table.clone());
                }
            }
            (Some(top_pt), Some(last_pt)) => {
                if keys.len() == 0 {
                    last_pt.add_entry(pseudo_key, value);
                    value = top_pt.into();
                    break;
                } else {
                    top_pseudo_table = Some(top_pt);
                    last_pt.add_entry(pseudo_key, new_pseudo_table.clone().into());
                    last_pseudo_table = Some(new_pseudo_table);
                }
            }
            _ => unreachable!(),
        }
    }

    (first_key, value)
}
```

## Docstring

_No docstring provided._

## Source

```
fn entry_from_syntax(syntax: SyntaxElement) -> (Key, Node) {
    assert!(syntax.kind() == ENTRY);

    let mut keys = keys_from_syntax(
        &syntax
            .as_node()
            .and_then(|n| n.first_child())
            .map(Into::into)
            .unwrap_or_else(|| syntax.clone()),
    );
    let first_key = keys
        .next()
        .unwrap_or_else(|| Key::from_syntax_invalid(syntax.clone()));

    let mut value = syntax
        .as_node()
        .and_then(|n| n.first_child())
        .and_then(|k| k.next_sibling())
        .map(|n| Node::from_syntax(n.into()))
        .unwrap_or_else(|| Invalid::from_syntax(syntax).into());

    let mut top_pseudo_table: Option<Table> = None;
    let mut last_pseudo_table: Option<Table> = None;
    while let Some(pseudo_key) = keys.next() {
        let new_pseudo_table = Table::pseudo(&pseudo_key, false);

        match (top_pseudo_table.take(), last_pseudo_table.take()) {
            (None, None) => {
                let top_pt = Table::pseudo(&first_key, false);
                top_pseudo_table = Some(top_pt.clone());

                if keys.len() == 0 {
                    top_pt.add_entry(pseudo_key, value);
                    value = top_pt.into();
                } else {
                    top_pt.add_entry(pseudo_key, new_pseudo_table.clone().into());
                    last_pseudo_table = Some(new_pseudo_table.clone());
                }
            }
            (Some(top_pt), Some(last_pt)) => {
                if keys.len() == 0 {
                    last_pt.add_entry(pseudo_key, value);
                    value = top_pt.into();
                    break;
                } else {
                    top_pseudo_table = Some(top_pt);
                    last_pt.add_entry(pseudo_key, new_pseudo_table.clone().into());
                    last_pseudo_table = Some(new_pseudo_table);
                }
            }
            _ => unreachable!(),
        }
    }

    (first_key, value)
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
