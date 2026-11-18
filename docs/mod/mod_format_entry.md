# mod.format_entry

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 747-787)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_entry(node: SyntaxNode, options: &Options, context: &Context) -> FormattedEntry {
    let mut key = String::new();
    let mut value = String::new();
    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                KEY => {
                    format_key(n, &mut key, options, context);
                }
                VALUE => {
                    let val = format_value(n, options, context);
                    let c = val.trailing_comment();

                    if c.is_some() {
                        debug_assert!(comment.is_none());
                        comment = c;
                    }

                    val.write_to(&mut value, options);
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(t) => {
                if let COMMENT = t.kind() {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().into())
                }
            }
        }
    }

    FormattedEntry {
        syntax: node.into(),
        key,
        cleaned_key: OnceCell::new(),
        value,
        comment,
    }
}
```

## Docstring

_No docstring provided._

## Source

```
fn format_entry(node: SyntaxNode, options: &Options, context: &Context) -> FormattedEntry {
    let mut key = String::new();
    let mut value = String::new();
    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                KEY => {
                    format_key(n, &mut key, options, context);
                }
                VALUE => {
                    let val = format_value(n, options, context);
                    let c = val.trailing_comment();

                    if c.is_some() {
                        debug_assert!(comment.is_none());
                        comment = c;
                    }

                    val.write_to(&mut value, options);
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(t) => {
                if let COMMENT = t.kind() {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().into())
                }
            }
        }
    }

    FormattedEntry {
        syntax: node.into(),
        key,
        cleaned_key: OnceCell::new(),
        value,
        comment,
    }
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
