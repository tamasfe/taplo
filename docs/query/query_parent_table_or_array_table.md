# query.parent_table_or_array_table

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 212-243)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn parent_table_or_array_table(&self, root: &Node) -> (Keys, Node) {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(s) => s.syntax.clone(),
            None => return (Keys::empty(), root.clone()),
        };

        let last_header = root
            .syntax()
            .unwrap()
            .as_node()
            .unwrap()
            .descendants()
            .skip(1)
            .filter(|n| matches!(n.kind(), TABLE_HEADER | TABLE_ARRAY_HEADER))
            .take_while(|n| n.text_range().end() <= syntax.text_range().end())
            .last();

        let Some(last_header) = last_header else {
            return (Keys::empty(), root.clone());
        };

        let keys = Keys::from_syntax(
            last_header
                .descendants()
                .find(|n| n.kind() == KEY)
                .unwrap()
                .into(),
        );
        let node = root.path(&keys).unwrap();

        (keys, node)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn parent_table_or_array_table(&self, root: &Node) -> (Keys, Node) {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(s) => s.syntax.clone(),
            None => return (Keys::empty(), root.clone()),
        };

        let last_header = root
            .syntax()
            .unwrap()
            .as_node()
            .unwrap()
            .descendants()
            .skip(1)
            .filter(|n| matches!(n.kind(), TABLE_HEADER | TABLE_ARRAY_HEADER))
            .take_while(|n| n.text_range().end() <= syntax.text_range().end())
            .last();

        let Some(last_header) = last_header else {
            return (Keys::empty(), root.clone());
        };

        let keys = Keys::from_syntax(
            last_header
                .descendants()
                .find(|n| n.kind() == KEY)
                .unwrap()
                .into(),
        );
        let node = root.path(&keys).unwrap();

        (keys, node)
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
