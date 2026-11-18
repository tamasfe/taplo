# nodes.TableInner

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 58-64)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) struct TableInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) header: bool,
    pub(crate) kind: TableKind,
    pub(crate) entries: Shared<Entries>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) struct TableInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) header: bool,
    pub(crate) kind: TableKind,
    pub(crate) entries: Shared<Entries>,
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
