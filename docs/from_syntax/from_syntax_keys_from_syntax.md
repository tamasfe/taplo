# from_syntax.keys_from_syntax

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\from_syntax.rs` (lines 389-401)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn keys_from_syntax(syntax: &SyntaxElement) -> impl ExactSizeIterator<Item = Key> {
    assert!(syntax.kind() == KEY);

    syntax
        .as_node()
        .map(|n| n.children_with_tokens())
        .map(|children| {
            let iter = children.filter(|c| c.kind() == IDENT);
            let c = iter.clone().count();
            Either::Left(iter.map(Key::from_syntax).exactly(c))
        })
        .unwrap_or_else(|| Either::Right(core::iter::empty()))
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) fn keys_from_syntax(syntax: &SyntaxElement) -> impl ExactSizeIterator<Item = Key> {
    assert!(syntax.kind() == KEY);

    syntax
        .as_node()
        .map(|n| n.children_with_tokens())
        .map(|children| {
            let iter = children.filter(|c| c.kind() == IDENT);
            let c = iter.clone().count();
            Either::Left(iter.map(Key::from_syntax).exactly(c))
        })
        .unwrap_or_else(|| Either::Right(core::iter::empty()))
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
