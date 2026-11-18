# nodes.ArrayInner

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 306-311)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) struct ArrayInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) kind: ArrayKind,
    pub(crate) items: Shared<Vec<Node>>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) struct ArrayInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) kind: ArrayKind,
    pub(crate) items: Shared<Vec<Node>>,
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
