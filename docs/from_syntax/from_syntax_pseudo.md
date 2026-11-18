# from_syntax.pseudo

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\from_syntax.rs` (lines 102-111)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn pseudo(key: &Key, header: bool) -> Self {
        TableInner {
            errors: Default::default(),
            syntax: key.syntax().cloned(),
            header,
            kind: TableKind::Pseudo,
            entries: Default::default(),
        }
        .wrap()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn pseudo(key: &Key, header: bool) -> Self {
        TableInner {
            errors: Default::default(),
            syntax: key.syntax().cloned(),
            header,
            kind: TableKind::Pseudo,
            entries: Default::default(),
        }
        .wrap()
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
